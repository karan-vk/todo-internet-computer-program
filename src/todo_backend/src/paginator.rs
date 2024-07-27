use candid::CandidType;
use serde::Deserialize;

/// Default number of items per page if not specified.
const DEFAULT_PAGE_SIZE: u32 = 5;

/// Maximum number of items allowed per page.
const MAX_PAGE_SIZE: u32 = 100;

/// Struct to handle pagination logic.
#[derive(CandidType, Deserialize, Default)]
pub struct Paginator {
    /// Page number, 1-indexed.
    /// The default value 0 also refers to the first page.
    #[serde(default)]
    page: u32,

    /// Optional limit on the number of items per page.
    limit: Option<u32>,
}

impl Paginator {
    /// Returns the current page number, ensuring it is at least 1.
    /// This method ensures that the page number is always valid and 1-indexed.
    pub fn page(&self) -> u32 {
        u32::max(self.page, 1)
    }

    /// Returns the number of items per page.
    /// If the limit is not set, it defaults to `DEFAULT_PAGE_SIZE`.
    /// The limit is capped at `MAX_PAGE_SIZE` to prevent excessive data fetching.
    pub fn limit(&self) -> usize {
        let limit = self.limit.unwrap_or(DEFAULT_PAGE_SIZE);
        u32::min(limit, MAX_PAGE_SIZE) as usize
    }

    /// Calculates the number of items to skip based on the current page and limit.
    /// This is useful for database queries or in-memory collections to fetch the correct subset of items.
    pub fn skip(&self) -> usize {
        (self.page() - 1) as usize * self.limit()
    }
}
