use std::fmt;

#[derive(Debug)]
pub struct ExecError {
    pub message: String,
    pub cmd: String,
    pub stderr: String,
    pub stdout: String,
    pub exit_code: Option<i32>,
    pub source: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl ExecError {
    pub fn new(message: impl Into<String>, cmd: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            cmd: cmd.into(),
            stderr: String::new(),
            stdout: String::new(),
            exit_code: None,
            source: None,
        }
    }

    pub fn with_output(mut self, stdout: String, stderr: String, exit_code: Option<i32>) -> Self {
        self.stdout = stdout;
        self.stderr = stderr;
        self.exit_code = exit_code;
        self
    }

    pub fn with_source(mut self, err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        self.source = Some(err);
        self
    }
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "exec error: {} (cmd: {})", self.message, self.cmd)?;
        if let Some(code) = self.exit_code {
            write!(f, ", exit_code: {}", code)?;
        }
        if !self.stderr.is_empty() {
            write!(f, ", stderr: {}", self.stderr.chars().take(200).collect::<String>())?;
        }
        Ok(())
    }
}

impl std::error::Error for ExecError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &dyn std::error::Error)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exec_error_new() {
        let err = ExecError::new("failed", "cmd");
        assert_eq!(err.message, "failed");
        assert_eq!(err.cmd, "cmd");
        assert!(err.stderr.is_empty());
        assert!(err.stdout.is_empty());
        assert_eq!(err.exit_code, None);
    }

    #[test]
    fn exec_error_with_output() {
        let err = ExecError::new("failed", "cmd")
            .with_output("out".into(), "err".into(), Some(1));
        assert_eq!(err.stdout, "out");
        assert_eq!(err.stderr, "err");
        assert_eq!(err.exit_code, Some(1));
    }

    #[test]
    fn exec_error_display_includes_message_and_cmd() {
        let err = ExecError::new("failed", "cmd");
        let s = format!("{}", err);
        assert!(s.contains("failed"));
        assert!(s.contains("cmd"));
    }

    #[test]
    fn exec_error_display_includes_exit_code() {
        let err = ExecError::new("failed", "cmd").with_output("".into(), "".into(), Some(42));
        let s = format!("{}", err);
        assert!(s.contains("42"));
    }

    #[test]
    fn exec_error_display_includes_stderr() {
        let err = ExecError::new("failed", "cmd").with_output("".into(), "stderr content".into(), None);
        let s = format!("{}", err);
        assert!(s.contains("stderr content"));
    }

    #[test]
    fn exec_error_with_source() {
        let err = ExecError::new("failed", "cmd").with_source(Box::new(std::io::Error::other("source")));
        assert!(err.source.is_some());
    }
}
