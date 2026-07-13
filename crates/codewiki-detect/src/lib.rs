//! Repository stack detection boundary.

/// Capabilities planned for repository detection.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DetectionCapabilities {
    /// Whether language detection is part of this boundary.
    pub languages: bool,
    /// Whether package manager detection is part of this boundary.
    pub package_managers: bool,
    /// Whether framework/library signals are part of this boundary.
    pub frameworks: bool,
    /// Whether entrypoint and test/build discovery are part of this boundary.
    pub entrypoints: bool,
}

impl DetectionCapabilities {
    /// Return the scaffold capability set.
    pub fn scaffold() -> Self {
        Self {
            languages: true,
            package_managers: true,
            frameworks: true,
            entrypoints: true,
        }
    }

    /// Human-readable summary for CLI status output.
    pub fn summary(&self) -> &'static str {
        "languages, package managers, frameworks, entrypoints"
    }
}
