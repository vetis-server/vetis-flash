use crate::tokio::FlashHook;
use serde::{Deserialize, Serialize};
use vetis::{
    Str, VetisHook, VetisResult,
    errors::{ConfigError, VetisError},
    hook::HookConfig,
    matcher::MatcherConfig,
};

#[cfg(feature = "runtime-tokio")]
/// Tokio runtime support module
pub mod tokio;

#[cfg(test)]
mod tests;

/// Builder for creating `FlashHookConfig` instances.
pub struct FlashHookConfigBuilder {
    matcher: Option<Box<dyn MatcherConfig>>,
    response: Option<Str>,
    template: Option<Str>,
    status_code: u16,
    headers: Option<Vec<(Str, Str)>>,
}

impl FlashHookConfigBuilder {
    /// Add a Matcher to hook
    ///
    /// # Arguments
    ///
    /// * `matcher` - Matcher configuration
    ///
    /// # Notes
    ///
    /// Add a Matcher configuration to the hook, matchers are
    /// useful to add determine if a particular hook will
    /// be used.
    pub fn matcher<M>(mut self, matcher: M) -> Self
    where
        M: MatcherConfig + 'static,
    {
        self.matcher = Some(Box::new(matcher));
        self
    }

    /// Allow set the response of flash path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn response(mut self, extensions: &str) -> Self {
        self.response = Some(extensions.into());
        self
    }

    /// Allow set the template of flash path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn template(mut self, template: &str) -> Self {
        self.template = Some(template.into());
        self
    }

    /// Allow set the status code of flash path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn status_code(mut self, status_code: u16) -> Self {
        self.status_code = status_code;
        self
    }

    /// Allow set the headers of flash path.
    ///
    /// # Returns
    ///
    /// * `Self` - The builder.
    pub fn headers(mut self, heders: Vec<(Str, Str)>) -> Self {
        self.headers = Some(heders);
        self
    }

    /// Build the `StaticHookConfig` with the configured settings.
    ///
    /// # Returns
    ///
    /// * `VetisResult<FlashConfig>` - The `StaticHookConfig` with the configured settings.
    pub fn build(self) -> VetisResult<FlashHookConfig> {
        if self
            .response
            .is_none()
            && self
                .template
                .is_none()
        {
            return Err(VetisError::Config(ConfigError::Hook(
                "You must provide response or template file".into(),
            )));
        }

        Ok(FlashHookConfig {
            matcher: self
                .matcher
                .map_or(None, |val| Some(val)),
            response: self.response,
            template: self.template,
            status_code: self.status_code,
            headers: self
                .headers
                .map_or(None, |val| Some(val.into_boxed_slice())),
        })
    }
}

/// Configuration for static file serving.
#[derive(Clone, Serialize, Deserialize)]
pub struct FlashHookConfig {
    matcher: Option<Box<dyn MatcherConfig>>,
    response: Option<Str>,
    template: Option<Str>,
    status_code: u16,
    headers: Option<Box<[(Str, Str)]>>,
}

#[typetag::serde(name = "flash")]
impl HookConfig for FlashHookConfig {
    fn matcher(&self) -> Option<&Box<dyn MatcherConfig>> {
        self.matcher
            .as_ref()
    }

    fn boxed_clone(&self) -> Box<dyn HookConfig> {
        Box::new(self.clone())
    }

    fn boxed_hook(&self) -> Box<dyn VetisHook> {
        Box::new(FlashHook::new(self.clone()))
    }
}

impl FlashHookConfig {
    /// Allow create a new `FlashHookConfigBuilder` with default settings.
    ///
    /// # Returns
    ///
    /// * `StaticHookConfigBuilder` - The builder.
    pub fn builder() -> FlashHookConfigBuilder {
        FlashHookConfigBuilder {
            matcher: None,
            response: None,
            template: None,
            status_code: 200,
            headers: None,
        }
    }

    /// Returns response
    ///
    /// # Returns
    ///
    /// * `&str` - The response.
    pub fn response(&self) -> &Option<Str> {
        &self.response
    }

    /// Returns template
    ///
    /// # Returns
    ///
    /// * `&str` - The template.
    pub fn template(&self) -> &Option<Str> {
        &self.template
    }

    /// Returns status code
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<String>>` - The index_files.
    pub fn status_code(&self) -> u16 {
        self.status_code
    }

    /// Returns headers
    ///
    /// # Returns
    ///
    /// * `&Option<Vec<String, String>>` - The headers.
    pub fn headers(&self) -> &Option<Box<[(Str, Str)]>> {
        &self.headers
    }
}
