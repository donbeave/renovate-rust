use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// A paginated API item that can be cached.
///
/// Mirrors `ApiPageItem` from `lib/modules/platform/github/types.ts`.
pub trait ApiPageItem: Clone + PartialEq {
    fn number(&self) -> u64;
    fn updated_at(&self) -> &str;
}

/// Serializable cache storage for paginated GitHub API items.
///
/// Mirrors `ApiPageCache<T>` from `lib/modules/platform/github/types.ts`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiPageCache<T> {
    pub items: HashMap<u64, T>,
    #[serde(rename = "lastModified", skip_serializing_if = "Option::is_none")]
    pub last_modified: Option<String>,
}

impl<T> Default for ApiPageCache<T> {
    fn default() -> Self {
        Self {
            items: HashMap::new(),
            last_modified: None,
        }
    }
}

/// In-memory cache for paginated GitHub API responses (PRs, issues, etc.).
///
/// Mirrors `ApiCache<T>` from `lib/modules/platform/github/api-cache.ts`.
#[derive(Debug)]
pub struct ApiCache<T> {
    cache: ApiPageCache<T>,
}

impl<T: ApiPageItem> ApiCache<T> {
    pub fn new(cache: ApiPageCache<T>) -> Self {
        Self { cache }
    }

    pub fn get_items(&self) -> Vec<&T> {
        let mut items: Vec<&T> = self.cache.items.values().collect();
        items.sort_by_key(|i| i.number());
        items
    }

    pub fn get_item(&self, number: u64) -> Option<&T> {
        self.cache.items.get(&number)
    }

    pub fn update_item(&mut self, item: T) {
        self.cache.items.insert(item.number(), item);
    }

    pub fn get_last_modified(&self) -> Option<&str> {
        self.cache.last_modified.as_deref()
    }

    pub fn update_last_modified(&mut self, timestamp: &str) {
        let should_update = match &self.cache.last_modified {
            None => true,
            Some(current) => timestamp > current.as_str(),
        };
        if should_update {
            self.cache.last_modified = Some(timestamp.to_string());
        }
    }

    /// Copies items from a page into the cache.
    ///
    /// `page` must be sorted by `updated_at` descending (most recent first).
    /// Returns `true` when the next page is likely to contain fresh items.
    pub fn reconcile(&mut self, page: &[T]) -> bool {
        if page.is_empty() {
            return false;
        }

        let mut need_next_page = true;
        let mut last_modified = self.cache.last_modified.clone();

        for new_item in page {
            let number = new_item.number();
            let old_item = self.cache.items.get(&number);

            let item_new_time = new_item.updated_at();
            let item_old_time = old_item.map(|i| i.updated_at().to_string());

            if old_item != Some(new_item) {
                self.cache.items.insert(number, new_item.clone());
            }

            need_next_page = match &item_old_time {
                Some(old_time) => old_time.as_str() < item_new_time,
                None => true,
            };

            let cache_old_time = last_modified.as_deref();
            if cache_old_time.map_or(true, |t| item_new_time > t) {
                last_modified = Some(item_new_time.to_string());
            }
        }

        self.cache.last_modified = last_modified;

        need_next_page
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestItem {
        number: u64,
        updated_at: String,
    }

    impl ApiPageItem for TestItem {
        fn number(&self) -> u64 {
            self.number
        }
        fn updated_at(&self) -> &str {
            &self.updated_at
        }
    }

    fn item(number: u64, year: u32) -> TestItem {
        TestItem {
            number,
            updated_at: format!("{:04}-01-01T00:00:00.000Z", year),
        }
    }

    fn cache_from(items: Vec<TestItem>, last_modified: Option<&str>) -> ApiPageCache<TestItem> {
        let mut map = HashMap::new();
        for i in items {
            map.insert(i.number, i);
        }
        ApiPageCache {
            items: map,
            last_modified: last_modified.map(|s| s.to_string()),
        }
    }

    // Ported: "stores and retrieves items" — modules/platform/github/api-cache.spec.ts line 12
    #[test]
    fn stores_and_retrieves_items() {
        let item1 = item(1, 2001);
        let item2 = item(2, 2002);
        let mut api_cache = ApiCache::new(cache_from(
            vec![item1.clone()],
            Some("2001-01-01T00:00:00.000Z"),
        ));

        assert_eq!(api_cache.get_item(1), Some(&item1));
        assert_eq!(api_cache.get_item(2), None);

        api_cache.update_item(item2.clone());
        assert_eq!(api_cache.get_item(2), Some(&item2));

        let items = api_cache.get_items();
        assert!(items.contains(&&item1));
        assert!(items.contains(&&item2));
        assert_eq!(items.len(), 2);
    }

    // Ported: "maps items" — modules/platform/github/api-cache.spec.ts line 29
    #[test]
    fn get_items_maps_items() {
        let item1 = item(1, 2001);
        let item2 = item(2, 2002);
        let api_cache = ApiCache::new(cache_from(vec![item1.clone(), item2.clone()], None));
        let items = api_cache.get_items();
        assert_eq!(items.len(), 2);
        assert!(items.contains(&&item1));
        assert!(items.contains(&&item2));
    }

    // Ported: "resets cache on item update" — modules/platform/github/api-cache.spec.ts line 46
    #[test]
    fn get_items_resets_on_item_update() {
        let item1 = item(1, 2001);
        let item1_updated = TestItem {
            number: 1,
            updated_at: "2003-01-01T00:00:00.000Z".to_string(),
        };
        let mut api_cache = ApiCache::new(cache_from(vec![item1], None));
        api_cache.update_item(item1_updated.clone());
        let items = api_cache.get_items();
        assert_eq!(items.len(), 1);
        assert_eq!(items[0], &item1_updated);
    }

    // Ported: "resets cache on page reconcile" — modules/platform/github/api-cache.spec.ts line 69
    #[test]
    fn get_items_resets_on_page_reconcile() {
        let item1 = item(1, 2001);
        let item1_updated = TestItem {
            number: 1,
            updated_at: "2003-01-01T00:00:00.000Z".to_string(),
        };
        let mut api_cache = ApiCache::new(cache_from(vec![item1], None));
        api_cache.reconcile(&[item1_updated.clone()]);
        assert_eq!(api_cache.get_item(1), Some(&item1_updated));
    }

    // Ported: "returns undefined when no lastModified in cache" — modules/platform/github/api-cache.spec.ts line 94
    #[test]
    fn get_last_modified_returns_none_when_not_set() {
        let api_cache = ApiCache::new(cache_from(vec![], None));
        assert_eq!(api_cache.get_last_modified(), None);
    }

    // Ported: "returns stored value when present" — modules/platform/github/api-cache.spec.ts line 100
    #[test]
    fn get_last_modified_returns_stored_value() {
        let api_cache = ApiCache::new(cache_from(vec![], Some("2001-01-01T00:00:00.000Z")));
        assert_eq!(
            api_cache.get_last_modified(),
            Some("2001-01-01T00:00:00.000Z")
        );
    }

    // Ported: "returns updated value after reconcile" — modules/platform/github/api-cache.spec.ts line 106
    #[test]
    fn get_last_modified_returns_updated_after_reconcile() {
        let mut api_cache = ApiCache::new(cache_from(vec![], Some("2001-01-01T00:00:00.000Z")));
        api_cache.reconcile(&[item(1, 2003)]);
        assert_eq!(
            api_cache.get_last_modified(),
            Some("2003-01-01T00:00:00.000Z")
        );
    }

    // Ported: "sets lastModified when not present" — modules/platform/github/api-cache.spec.ts line 116
    #[test]
    fn update_last_modified_sets_when_absent() {
        let mut api_cache = ApiCache::new(cache_from(vec![], None));
        api_cache.update_last_modified("2001-01-01T00:00:00.000Z");
        assert_eq!(
            api_cache.get_last_modified(),
            Some("2001-01-01T00:00:00.000Z")
        );
    }

    // Ported: "advances lastModified to newer timestamp" — modules/platform/github/api-cache.spec.ts line 124
    #[test]
    fn update_last_modified_advances_to_newer() {
        let mut api_cache = ApiCache::new(cache_from(vec![], Some("2001-01-01T00:00:00.000Z")));
        api_cache.update_last_modified("2003-01-01T00:00:00.000Z");
        assert_eq!(
            api_cache.get_last_modified(),
            Some("2003-01-01T00:00:00.000Z")
        );
    }

    // Ported: "does not regress lastModified to older timestamp" — modules/platform/github/api-cache.spec.ts line 132
    #[test]
    fn update_last_modified_does_not_regress() {
        let mut api_cache = ApiCache::new(cache_from(vec![], Some("2003-01-01T00:00:00.000Z")));
        api_cache.update_last_modified("2001-01-01T00:00:00.000Z");
        assert_eq!(
            api_cache.get_last_modified(),
            Some("2003-01-01T00:00:00.000Z")
        );
    }

    // Ported: "returns false for empty page" — modules/platform/github/api-cache.spec.ts line 142
    #[test]
    fn reconcile_returns_false_for_empty_page() {
        let mut api_cache = ApiCache::new(cache_from(vec![], None));
        assert!(!api_cache.reconcile(&[]));
    }

    // Ported: "appends new items" — modules/platform/github/api-cache.spec.ts line 152
    #[test]
    fn reconcile_appends_new_items() {
        let item1 = item(1, 2001);
        let item2 = item(2, 2002);
        let mut api_cache = ApiCache::new(cache_from(
            vec![item1.clone()],
            Some("2001-01-01T00:00:00.000Z"),
        ));
        let result = api_cache.reconcile(&[item2.clone()]);
        assert!(result); // item2 is newer than lastModified
        assert_eq!(api_cache.get_item(1), Some(&item1));
        assert_eq!(api_cache.get_item(2), Some(&item2));
    }

    // Ported: "handles updated items" — modules/platform/github/api-cache.spec.ts line 175
    #[test]
    fn reconcile_handles_updated_items() {
        let item1 = item(1, 2001);
        let item1_updated = TestItem {
            number: 1,
            updated_at: "2003-01-01T00:00:00.000Z".to_string(),
        };
        let mut api_cache =
            ApiCache::new(cache_from(vec![item1], Some("2001-01-01T00:00:00.000Z")));
        api_cache.reconcile(&[item1_updated.clone()]);
        assert_eq!(api_cache.get_item(1), Some(&item1_updated));
        assert_eq!(
            api_cache.get_last_modified(),
            Some("2003-01-01T00:00:00.000Z")
        );
    }

    // Ported: "ignores page overlap" — modules/platform/github/api-cache.spec.ts line 199
    #[test]
    fn reconcile_ignores_page_overlap() {
        let item1 = item(1, 2001);
        let item2 = item(2, 2002);
        let mut api_cache = ApiCache::new(cache_from(
            vec![item1.clone(), item2.clone()],
            Some("2002-01-01T00:00:00.000Z"),
        ));
        // Reconcile with same items (no changes)
        let result = api_cache.reconcile(&[item2.clone(), item1.clone()]);
        // item2's old time equals its new time, so needNextPage = false for item1
        // item1's old time (2001) < item2's time (2002) is handled item by item
        // Final result depends on last item processed: item1 old=2001, new=2001 → needNextPage = false
        assert!(!result);
        assert_eq!(api_cache.get_item(1), Some(&item1));
        assert_eq!(api_cache.get_item(2), Some(&item2));
    }

    // Ported: "does not require new page if all items are old" — modules/platform/github/api-cache.spec.ts line 226
    #[test]
    fn reconcile_does_not_require_next_page_if_all_old() {
        let item1 = item(1, 2001);
        let item2 = item(2, 2002);
        let mut api_cache = ApiCache::new(cache_from(
            vec![item1.clone(), item2.clone()],
            Some("2002-01-01T00:00:00.000Z"),
        ));
        // Page contains item2 then item1 (desc order by updated_at), both already known
        let result = api_cache.reconcile(&[item2.clone(), item1.clone()]);
        assert!(!result);
    }
}
