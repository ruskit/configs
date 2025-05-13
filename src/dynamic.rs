/// # DynamicConfigs
///
/// A trait for application-specific configuration extensions.
///
/// Implementors of this trait can provide custom configuration loading logic
/// for application-specific settings that aren't covered by the standard
/// configuration objects.
///
/// ## Required Methods
///
/// * `load` - Load configuration values from environment variables, files, etc.
pub trait DynamicConfigs: Default {
    /// Load configuration values from the environment or other sources.
    fn load(&mut self);
}

/// # Empty
///
/// A placeholder implementation of `DynamicConfigs` that doesn't add any configuration.
///
/// This type can be used when no additional configuration beyond the standard
/// service configurations is needed.
#[derive(Debug, Default, Clone, Copy)]
pub struct Empty;

impl DynamicConfigs for Empty {
    fn load(&mut self) {}
}
