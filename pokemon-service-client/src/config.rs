// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Service config.
///
pub struct Config {
    retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
}
impl std::fmt::Debug for Config {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut config = f.debug_struct("Config");
        config.finish()
    }
}
impl Config {
    /// Constructs a config builder.
    pub fn builder() -> Builder {
        Builder::default()
    }
    /// Return a reference to the retry configuration contained in this config, if any.
    pub fn retry_config(&self) -> Option<&aws_smithy_types::retry::RetryConfig> {
        self.retry_config.as_ref()
    }

    /// Return a cloned Arc containing the async sleep implementation from this config, if any.
    pub fn sleep_impl(
        &self,
    ) -> Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>> {
        self.sleep_impl.clone()
    }

    /// Return a reference to the timeout configuration contained in this config, if any.
    pub fn timeout_config(&self) -> Option<&aws_smithy_types::timeout::TimeoutConfig> {
        self.timeout_config.as_ref()
    }
    /// Creates a new Event Stream `SignMessage` implementor.
    pub fn new_event_stream_signer(
        &self,
        _properties: aws_smithy_http::property_bag::SharedPropertyBag,
    ) -> impl aws_smithy_eventstream::frame::SignMessage {
        aws_smithy_eventstream::frame::NoOpSigner {}
    }
}
/// Builder for creating a `Config`.
#[derive(Default)]
pub struct Builder {
    retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
}
impl Builder {
    /// Constructs a config builder.
    pub fn new() -> Self {
        Self::default()
    }
    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```no_run
    /// use pokemon_service_client::config::Config;
    /// use pokemon_service_client::config::retry::RetryConfig;
    ///
    /// let retry_config = RetryConfig::standard().with_max_attempts(5);
    /// let config = Config::builder().retry_config(retry_config).build();
    /// ```
    pub fn retry_config(mut self, retry_config: aws_smithy_types::retry::RetryConfig) -> Self {
        self.set_retry_config(Some(retry_config));
        self
    }

    /// Set the retry_config for the builder
    ///
    /// # Examples
    /// ```no_run
    /// use pokemon_service_client::config::{Builder, Config};
    /// use pokemon_service_client::config::retry::RetryConfig;
    ///
    /// fn disable_retries(builder: &mut Builder) {
    ///     let retry_config = RetryConfig::standard().with_max_attempts(1);
    ///     builder.set_retry_config(Some(retry_config));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// disable_retries(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_retry_config(
        &mut self,
        retry_config: Option<aws_smithy_types::retry::RetryConfig>,
    ) -> &mut Self {
        self.retry_config = retry_config;
        self
    }

    /// Set the sleep_impl for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pokemon_service_client::config::{AsyncSleep, Sleep, Config};
    ///
    /// #[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// let sleep_impl = std::sync::Arc::new(ForeverSleep);
    /// let config = Config::builder().sleep_impl(sleep_impl).build();
    /// ```
    pub fn sleep_impl(
        mut self,
        sleep_impl: std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>,
    ) -> Self {
        self.set_sleep_impl(Some(sleep_impl));
        self
    }

    /// Set the sleep_impl for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use pokemon_service_client::config::{AsyncSleep, Sleep, Builder, Config};
    ///
    /// #[derive(Debug)]
    /// pub struct ForeverSleep;
    ///
    /// impl AsyncSleep for ForeverSleep {
    ///     fn sleep(&self, duration: std::time::Duration) -> Sleep {
    ///         Sleep::new(std::future::pending())
    ///     }
    /// }
    ///
    /// fn set_never_ending_sleep_impl(builder: &mut Builder) {
    ///     let sleep_impl = std::sync::Arc::new(ForeverSleep);
    ///     builder.set_sleep_impl(Some(sleep_impl));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// set_never_ending_sleep_impl(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_sleep_impl(
        &mut self,
        sleep_impl: Option<std::sync::Arc<dyn aws_smithy_async::rt::sleep::AsyncSleep>>,
    ) -> &mut Self {
        self.sleep_impl = sleep_impl;
        self
    }

    /// Set the timeout_config for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// use pokemon_service_client::config::Config;
    /// use pokemon_service_client::config::timeout::TimeoutConfig;
    ///
    /// let timeout_config = TimeoutConfig::builder()
    ///     .operation_attempt_timeout(Duration::from_secs(1))
    ///     .build();
    /// let config = Config::builder().timeout_config(timeout_config).build();
    /// ```
    pub fn timeout_config(
        mut self,
        timeout_config: aws_smithy_types::timeout::TimeoutConfig,
    ) -> Self {
        self.set_timeout_config(Some(timeout_config));
        self
    }

    /// Set the timeout_config for the builder
    ///
    /// # Examples
    ///
    /// ```no_run
    /// # use std::time::Duration;
    /// use pokemon_service_client::config::{Builder, Config};
    /// use pokemon_service_client::config::timeout::TimeoutConfig;
    ///
    /// fn set_request_timeout(builder: &mut Builder) {
    ///     let timeout_config = TimeoutConfig::builder()
    ///         .operation_attempt_timeout(Duration::from_secs(1))
    ///         .build();
    ///     builder.set_timeout_config(Some(timeout_config));
    /// }
    ///
    /// let mut builder = Config::builder();
    /// set_request_timeout(&mut builder);
    /// let config = builder.build();
    /// ```
    pub fn set_timeout_config(
        &mut self,
        timeout_config: Option<aws_smithy_types::timeout::TimeoutConfig>,
    ) -> &mut Self {
        self.timeout_config = timeout_config;
        self
    }
    /// Builds a [`Config`].
    pub fn build(self) -> Config {
        Config {
            retry_config: self.retry_config,
            sleep_impl: self.sleep_impl,
            timeout_config: self.timeout_config,
        }
    }
}

pub use aws_smithy_async::rt::sleep::{AsyncSleep, Sleep};

/// Retry configuration
///
/// These are re-exported from `aws-smithy-types` for convenience.
pub mod retry {
    pub use aws_smithy_types::retry::{RetryConfig, RetryConfigBuilder, RetryMode};
}
/// Timeout configuration
///
/// These are re-exported from `aws-smithy-types` for convenience.
pub mod timeout {
    pub use aws_smithy_types::timeout::{TimeoutConfig, TimeoutConfigBuilder};
}