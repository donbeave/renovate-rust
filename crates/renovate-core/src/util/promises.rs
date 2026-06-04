use std::error::Error;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use tokio::sync::Semaphore;

/// @parity lib/util/promises.ts full
/// Mirrors `all` / `map` from `lib/util/promises.ts`.
pub async fn all<T, E>(tasks: Vec<Task<T, E>>, options: Option<AllOptions>) -> Result<Vec<T>, PromiseError>
where
    T: Send + 'static,
    E: Error + Send + Sync + 'static,
{
    let options = options.unwrap_or_default();
    let results = run_all_concurrent(tasks, options.concurrency).await;
    handle_results(results, options.stop_on_error)
}

pub async fn map<Element, NewElement, M, Fut, E>(
    input: impl IntoIterator<Item = Element>,
    mapper: M,
    options: Option<MapOptions>,
) -> Result<Vec<NewElement>, PromiseError>
where
    Element: Send + 'static,
    NewElement: Send + 'static,
    M: Fn(Element) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = Result<NewElement, E>> + Send + 'static,
    E: Error + Send + Sync + 'static,
{
    let options = options.unwrap_or_default();
    let mut tasks: Vec<Task<NewElement, E>> = Vec::new();
    let mapper = Arc::new(mapper);

    for item in input {
        let mapper = Arc::clone(&mapper);
        tasks.push(Box::new(move || {
            Box::pin(mapper(item)) as Pin<Box<TaskFuture<NewElement, E>>>
        }));
    }

    let results = run_all_concurrent(tasks, options.concurrency).await;
    handle_results(results, options.stop_on_error)
}

type TaskFuture<T, E> = dyn Future<Output = Result<T, E>> + Send + 'static;

type Task<T, E> = Box<dyn FnOnce() -> Pin<Box<TaskFuture<T, E>>> + Send>;

async fn run_all_concurrent<T, E>(tasks: Vec<Task<T, E>>, concurrency: usize) -> Vec<Result<T, E>>
where
    T: Send + 'static,
    E: Error + Send + Sync + 'static,
{
    let semaphore = Arc::new(Semaphore::new(concurrency.max(1)));
    let mut handles = Vec::with_capacity(tasks.len());

    for task in tasks {
        let permit = semaphore
            .clone()
            .acquire_owned()
            .await
            .expect("semaphore closed");
        handles.push(tokio::spawn(async move {
            let _permit = permit;
            task().await
        }));
    }

    let mut results = Vec::with_capacity(handles.len());
    for handle in handles {
        results.push(handle.await.expect("task panicked"));
    }
    results
}

fn handle_results<T, E>(results: Vec<Result<T, E>>, stop_on_error: bool) -> Result<Vec<T>, PromiseError>
where
    E: Error + Send + Sync + 'static,
{
    let mut outputs: Vec<T> = Vec::new();
    let mut errors: Vec<(String, Box<dyn Error + Send + Sync>)> = Vec::new();

    for result in results {
        match result {
            Ok(value) => outputs.push(value),
            Err(err) => {
                let message = err.to_string();
                errors.push((message, Box::new(err)));
            }
        }
    }

    if errors.is_empty() {
        return Ok(outputs);
    }

    if stop_on_error {
        let (_, first_error) = errors
            .into_iter()
            .next()
            .expect("errors is non-empty");
        return Err(PromiseError::Single(first_error));
    }

    if let Some(index) = errors.iter().position(|(message, _)| is_external_host_error(message)) {
        let (_, err) = errors.remove(index);
        return Err(PromiseError::ExternalHost(err));
    }

    if errors.len() == 1 || errors.iter().all(|(message, _)| message == &errors[0].0) {
        let (_, err) = errors
            .into_iter()
            .next()
            .expect("errors is non-empty");
        return Err(PromiseError::Single(err));
    }

    let only_errors = errors.into_iter().map(|(_, err)| err).collect();
    Err(PromiseError::Aggregate(only_errors))
}

fn is_external_host_error(message: &str) -> bool {
    message.contains("ExternalHostError")
}

#[derive(Debug)]
pub enum PromiseError {
    ExternalHost(Box<dyn Error + Send + Sync>),
    Single(Box<dyn Error + Send + Sync>),
    Aggregate(Vec<Box<dyn Error + Send + Sync>>),
}

impl std::fmt::Display for PromiseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ExternalHost(err) => write!(f, "{err}"),
            Self::Single(err) => write!(f, "{err}"),
            Self::Aggregate(errors) => {
                let messages: Vec<String> = errors.iter().map(|err| err.to_string()).collect();
                write!(f, "{}", messages.join(", "))
            }
        }
    }
}

impl Error for PromiseError {}

#[derive(Debug, Clone)]
pub struct AllOptions {
    pub concurrency: usize,
    pub stop_on_error: bool,
}

impl Default for AllOptions {
    fn default() -> Self {
        Self {
            concurrency: 5,
            stop_on_error: false,
        }
    }
}

pub type MapOptions = AllOptions;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt;
    use std::io;

    #[derive(Debug, Clone)]
    enum TestPromiseError {
        ExternalHost(String),
        Unknown,
        Message(String),
    }

    impl fmt::Display for TestPromiseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                Self::ExternalHost(msg) => f.write_str(msg),
                Self::Unknown => f.write_str("fail"),
                Self::Message(message) => f.write_str(message),
            }
        }
    }

    impl Error for TestPromiseError {}

    fn make_i32_task(value: i32) -> Task<i32, io::Error> {
        Box::new(move || Box::pin(async move { Ok::<_, io::Error>(value) }) as Pin<Box<TaskFuture<i32, io::Error>>>)
    }

    fn make_string_task_ok(value: &'static str) -> Task<&'static str, TestPromiseError> {
        Box::new(move || {
            Box::pin(async move { Ok::<_, TestPromiseError>(value) })
                as Pin<Box<TaskFuture<&'static str, TestPromiseError>>>
        })
    }

    fn make_string_task_err(message: TestPromiseError) -> Task<&'static str, TestPromiseError> {
        Box::new(move || {
            Box::pin(async move { Err::<&'static str, TestPromiseError>(message) })
                as Pin<Box<TaskFuture<&'static str, TestPromiseError>>>
        })
    }

    fn make_err_task(message: String) -> Task<(), TestPromiseError> {
        Box::new(move || {
            Box::pin(async move { Err::<(), TestPromiseError>(TestPromiseError::Message(message.clone())) })
                as Pin<Box<TaskFuture<(), TestPromiseError>>>
        })
    }

    #[tokio::test]
    async fn test_promises_all_works() {
        assert_eq!(
            all(
                vec![
                    make_i32_task(1),
                    make_i32_task(2),
                    make_i32_task(3),
                ],
                None,
            )
            .await
            .expect("all resolves all tasks"),
            vec![1, 2, 3],
        );
    }

    #[tokio::test]
    async fn test_promises_map_works() {
        assert_eq!(
            map(
                vec![1, 2, 3],
                |value| async move { Ok::<_, io::Error>(value + 1) },
                None,
            )
            .await
            .expect("map resolves"),
            vec![2, 3, 4],
        );
    }

    #[tokio::test]
    async fn test_promises_all_first_external_host_error() {
        match all(
            vec![
                make_string_task_ok("ok"),
                make_string_task_err(TestPromiseError::Unknown),
                make_string_task_err(TestPromiseError::ExternalHost(
                    "ExternalHostError: network failed".to_owned(),
                )),
            ],
            None,
        )
        .await
        .unwrap_err()
        {
            PromiseError::ExternalHost(err) => {
                assert!(err.to_string().contains("ExternalHostError"));
            }
            _ => panic!("Expected ExternalHost variant"),
        }
    }

    #[tokio::test]
    async fn test_promises_all_same_error_message() {
        let message = "some error".to_owned();
        match all(
            vec![
                make_err_task(message.clone()),
                make_err_task(message.clone()),
                make_err_task(message),
            ],
            None,
        )
        .await
        .unwrap_err()
        {
            PromiseError::Single(_) => {}
            _ => panic!("Expected single error with identical messages"),
        }
    }

    #[tokio::test]
    async fn test_promises_map_aggregate_for_different_error_messages() {
        match map(
            vec![1, 2, 3],
            |value| async move { Err::<(), io::Error>(io::Error::other(format!("error {value}"))) },
            None,
        )
        .await
        .unwrap_err()
        {
            PromiseError::Aggregate(_) => {}
            _ => panic!("Expected aggregate errors"),
        }
    }

    #[tokio::test]
    async fn test_promises_all_rethrows_when_stop_on_error_true() {
        match all(
            vec![
                Box::new(|| {
                    Box::pin(async { Ok::<_, io::Error>("ok") })
                        as Pin<Box<TaskFuture<&'static str, io::Error>>>
                }),
                Box::new(|| {
                    Box::pin(async { Ok::<_, io::Error>("ok") })
                        as Pin<Box<TaskFuture<&'static str, io::Error>>>
                }),
                Box::new(|| {
                    Box::pin(async { Err::<_, io::Error>(io::Error::other("fail")) })
                        as Pin<Box<TaskFuture<&'static str, io::Error>>>
                }),
            ],
            Some(AllOptions {
                concurrency: 5,
                stop_on_error: true,
            }),
        )
        .await
        .unwrap_err()
        {
            PromiseError::Single(err) => {
                assert_eq!(err.to_string(), "fail");
            }
            _ => panic!("Expected single error when stop_on_error is true"),
        }
    }
}
