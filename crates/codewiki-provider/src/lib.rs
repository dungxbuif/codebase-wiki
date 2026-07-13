//! Replaceable code-intelligence provider boundary.

/// Minimal provider contract for future semantic exploration.
pub trait CodeIntelligenceProvider {
    /// Stable provider name for logs, evidence, and config.
    fn provider_name(&self) -> &'static str;
}

/// Placeholder provider used until the first real provider is wired.
#[derive(Debug, Default, Clone, Copy)]
pub struct NoopProvider;

impl CodeIntelligenceProvider for NoopProvider {
    fn provider_name(&self) -> &'static str {
        "noop"
    }
}
