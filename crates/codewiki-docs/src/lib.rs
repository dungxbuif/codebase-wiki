//! Generated wiki document boundary.

/// Planned generated docs layout.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WikiDocsLayout {
    /// Root folder for generated CodeWiki pages.
    pub generated_docs_root: &'static str,
}

impl Default for WikiDocsLayout {
    fn default() -> Self {
        Self {
            generated_docs_root: "docs/codewiki",
        }
    }
}
