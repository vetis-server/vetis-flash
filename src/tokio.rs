use crate::FlashHookConfig;
use cel::Context;
use http::StatusCode;
use hyper_body_utils::HttpBody;
use std::path::{Path, PathBuf};
use tokio::fs;
use upon::value;
use vetis::{
    Request,
    errors::RuntimeError,
    hook::{Hook, HookConfig, HookPhase},
    matcher::Matcher,
};
use vetis_tokio::{
    Response, VetisFutureResult,
    errors::{HostError, VetisError},
};

/// Static path
pub struct FlashHook {
    matcher: Option<Box<dyn Matcher>>,
    config: FlashHookConfig,
}

unsafe impl Send for FlashHook {}
unsafe impl Sync for FlashHook {}

impl FlashHook {
    /// Create a new static path with provided configuration
    ///
    /// # Arguments
    ///
    /// * `config` - The configuration for the static path
    ///
    /// # Returns
    ///
    /// * `FlashHook` - The static path
    pub fn new(config: FlashHookConfig) -> FlashHook {
        FlashHook { matcher: None, config }
    }
}

impl Hook for FlashHook {
    fn setup<'a>(&'a mut self) -> VetisFutureResult<'a, ()> {
        Box::pin(async move {
            let matcher = if let Some(matcher_config) = self
                .config
                .matcher()
            {
                Some(matcher_config.boxed_matcher())
            } else {
                None
            };
            self.matcher = matcher;
            Ok(())
        })
    }

    fn has_matches(&self, context: &Context) -> bool {
        if let Some(matcher) = &self.matcher { matcher.is_match(context) } else { true }
    }

    fn phase(&self) -> HookPhase {
        HookPhase::Content
    }

    fn matcher_mut(&mut self) -> Option<&mut Box<dyn Matcher>> {
        self.matcher
            .as_mut()
    }

    fn handle<'a>(
        &'a self,
        request: Request,
        _response: Option<Response>,
    ) -> VetisFutureResult<'a, (Request, Option<Response>)> {
        let future = async move {
            if let Some(content) = self
                .config
                .response()
            {
                let response = Some(
                    Response::builder()
                        .status(http::StatusCode::OK)
                        .body(HttpBody::text(content)),
                );
                return Ok((request, response));
            } else if let Some(template) = self
                .config
                .template()
            {
                let template_path = Path::new(template);
                let full_template_path = if template_path.is_relative() {
                    if let Some(root_dir) = request.root_directory() {
                        PathBuf::from_iter([root_dir, template_path])
                    } else {
                        return Err(VetisError::Host(HostError::Runtime(RuntimeError::new(
                            "Root directory undefined",
                        ))));
                    }
                } else {
                    template_path.to_path_buf()
                };

                let template = fs::read_to_string(full_template_path)
                    .await
                    .map_err(|e| {
                        VetisError::Host(HostError::Runtime(RuntimeError::new(&e.to_string())))
                    })?;

                let mut engine = upon::Engine::new();
                engine
                    .add_template("main", template)
                    .map_err(|e| {
                        VetisError::Host(HostError::Runtime(RuntimeError::new(&e.to_string())))
                    })?;

                let output = engine
                    .template("main")
                    .render(value! { title: "Vetis!" })
                    .to_string()
                    .map_err(|e| {
                        VetisError::Host(HostError::Runtime(RuntimeError::new(&e.to_string())))
                    })?;

                let response = Some(
                    Response::builder()
                        .status(http::StatusCode::OK)
                        .body(HttpBody::text(&output)),
                );

                return Ok((request, response));
            }

            Err(VetisError::Host(HostError::Runtime(
                RuntimeError::new("Invalid flash content, must be either response or template.")
                    .with_status_code(StatusCode::NOT_FOUND),
            )))
        };

        Box::pin(future)
    }
}
