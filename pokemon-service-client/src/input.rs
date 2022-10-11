// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
use std::fmt::Write;

/// See [`CheckHealthInput`](crate::input::CheckHealthInput).
pub mod check_health_input {

    /// A builder for [`CheckHealthInput`](crate::input::CheckHealthInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`CheckHealthInput`](crate::input::CheckHealthInput).
        pub fn build(
            self,
        ) -> Result<crate::input::CheckHealthInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::CheckHealthInput {})
        }
    }
}
impl CheckHealthInput {
    /// Consumes the builder and constructs an Operation<[`CheckHealth`](crate::operation::CheckHealth)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::CheckHealth,
            aws_smithy_http::retry::DefaultResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::CheckHealthInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/ping").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::CheckHealthInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::CheckHealth::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CheckHealth",
            "PokemonService",
        ));
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`CheckHealthInput`](crate::input::CheckHealthInput).
    pub fn builder() -> crate::input::check_health_input::Builder {
        crate::input::check_health_input::Builder::default()
    }
}

/// See [`DoNothingInput`](crate::input::DoNothingInput).
pub mod do_nothing_input {

    /// A builder for [`DoNothingInput`](crate::input::DoNothingInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DoNothingInput`](crate::input::DoNothingInput).
        pub fn build(
            self,
        ) -> Result<crate::input::DoNothingInput, aws_smithy_http::operation::BuildError> {
            Ok(crate::input::DoNothingInput {})
        }
    }
}
impl DoNothingInput {
    /// Consumes the builder and constructs an Operation<[`DoNothing`](crate::operation::DoNothing)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::DoNothing,
            aws_smithy_http::retry::DefaultResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::DoNothingInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/do-nothing").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::DoNothingInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let op =
            aws_smithy_http::operation::Operation::new(request, crate::operation::DoNothing::new())
                .with_metadata(aws_smithy_http::operation::Metadata::new(
                    "DoNothing",
                    "PokemonService",
                ));
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`DoNothingInput`](crate::input::DoNothingInput).
    pub fn builder() -> crate::input::do_nothing_input::Builder {
        crate::input::do_nothing_input::Builder::default()
    }
}

/// See [`GetPokemonSpeciesInput`](crate::input::GetPokemonSpeciesInput).
pub mod get_pokemon_species_input {

    /// A builder for [`GetPokemonSpeciesInput`](crate::input::GetPokemonSpeciesInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Consumes the builder and constructs a [`GetPokemonSpeciesInput`](crate::input::GetPokemonSpeciesInput).
        pub fn build(
            self,
        ) -> Result<crate::input::GetPokemonSpeciesInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::GetPokemonSpeciesInput { name: self.name })
        }
    }
}
impl GetPokemonSpeciesInput {
    /// Consumes the builder and constructs an Operation<[`GetPokemonSpecies`](crate::operation::GetPokemonSpecies)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetPokemonSpecies,
            aws_smithy_http::retry::DefaultResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetPokemonSpeciesInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let input_1 = &_input.name;
                let input_1 = input_1.as_ref().ok_or(
                    aws_smithy_http::operation::BuildError::MissingField {
                        field: "name",
                        details: "cannot be empty or unset",
                    },
                )?;
                let name = aws_smithy_http::label::fmt_string(input_1, false);
                if name.is_empty() {
                    return Err(aws_smithy_http::operation::BuildError::MissingField {
                        field: "name",
                        details: "cannot be empty or unset",
                    });
                }
                write!(output, "/pokemon-species/{name}", name = name)
                    .expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetPokemonSpeciesInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetPokemonSpecies::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetPokemonSpecies",
            "PokemonService",
        ));
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetPokemonSpeciesInput`](crate::input::GetPokemonSpeciesInput).
    pub fn builder() -> crate::input::get_pokemon_species_input::Builder {
        crate::input::get_pokemon_species_input::Builder::default()
    }
}

/// See [`GetServerStatisticsInput`](crate::input::GetServerStatisticsInput).
pub mod get_server_statistics_input {

    /// A builder for [`GetServerStatisticsInput`](crate::input::GetServerStatisticsInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`GetServerStatisticsInput`](crate::input::GetServerStatisticsInput).
        pub fn build(
            self,
        ) -> Result<crate::input::GetServerStatisticsInput, aws_smithy_http::operation::BuildError>
        {
            Ok(crate::input::GetServerStatisticsInput {})
        }
    }
}
impl GetServerStatisticsInput {
    /// Consumes the builder and constructs an Operation<[`GetServerStatistics`](crate::operation::GetServerStatistics)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetServerStatistics,
            aws_smithy_http::retry::DefaultResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetServerStatisticsInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                write!(output, "/stats").expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetServerStatisticsInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetServerStatistics::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetServerStatistics",
            "PokemonService",
        ));
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetServerStatisticsInput`](crate::input::GetServerStatisticsInput).
    pub fn builder() -> crate::input::get_server_statistics_input::Builder {
        crate::input::get_server_statistics_input::Builder::default()
    }
}

/// See [`CapturePokemonInput`](crate::input::CapturePokemonInput).
pub mod capture_pokemon_input {

    /// A builder for [`CapturePokemonInput`](crate::input::CapturePokemonInput).
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) events: std::option::Option<
            aws_smithy_http::event_stream::EventStreamSender<
                crate::model::AttemptCapturingPokemonEvent,
                crate::error::AttemptCapturingPokemonEventError,
            >,
        >,
        pub(crate) region: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn events(
            mut self,
            input: aws_smithy_http::event_stream::EventStreamSender<
                crate::model::AttemptCapturingPokemonEvent,
                crate::error::AttemptCapturingPokemonEventError,
            >,
        ) -> Self {
            self.events = Some(input);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_events(
            mut self,
            input: std::option::Option<
                aws_smithy_http::event_stream::EventStreamSender<
                    crate::model::AttemptCapturingPokemonEvent,
                    crate::error::AttemptCapturingPokemonEventError,
                >,
            >,
        ) -> Self {
            self.events = input;
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn region(mut self, input: impl Into<std::string::String>) -> Self {
            self.region = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_region(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.region = input;
            self
        }
        /// Consumes the builder and constructs a [`CapturePokemonInput`](crate::input::CapturePokemonInput).
        pub fn build(
            self,
        ) -> Result<crate::input::CapturePokemonInput, aws_smithy_http::operation::BuildError>
        {
            Ok(
                crate::input::CapturePokemonInput {
                    events: self.events
                        .ok_or(
                            aws_smithy_http::operation::BuildError::MissingField { field: "events", details: "events was not specified but it is required when building CapturePokemonInput" }
                        )?
                    ,
                    region: self.region
                    ,
                }
            )
        }
    }
}
impl CapturePokemonInput {
    /// Consumes the builder and constructs an Operation<[`CapturePokemon`](crate::operation::CapturePokemon)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::CapturePokemon,
            aws_smithy_http::retry::DefaultResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::CapturePokemonInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let input_2 = &_input.region;
                let input_2 = input_2.as_ref().ok_or(
                    aws_smithy_http::operation::BuildError::MissingField {
                        field: "region",
                        details: "cannot be empty or unset",
                    },
                )?;
                let region = aws_smithy_http::label::fmt_string(input_2, false);
                if region.is_empty() {
                    return Err(aws_smithy_http::operation::BuildError::MissingField {
                        field: "region",
                        details: "cannot be empty or unset",
                    });
                }
                write!(output, "/capture-pokemon-event/{region}", region = region)
                    .expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::CapturePokemonInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                Ok(builder.method("POST").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder = aws_smithy_http::header::set_request_header_if_absent(
                builder,
                http::header::CONTENT_TYPE,
                "application/vnd.amazon.eventstream",
            );
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from({
            let error_marshaller =
                crate::event_stream_serde::AttemptCapturingPokemonEventErrorMarshaller::new();
            let marshaller =
                crate::event_stream_serde::AttemptCapturingPokemonEventMarshaller::new();
            let signer = _config.new_event_stream_signer(properties.clone());
            let adapter: aws_smithy_http::event_stream::MessageStreamAdapter<_, _> = self
                .events
                .into_body_stream(marshaller, error_marshaller, signer);
            let body: aws_smithy_http::body::SdkBody = hyper::Body::wrap_stream(adapter).into();
            body
        });
        if let Some(content_length) = body.content_length() {
            request = aws_smithy_http::header::set_request_header_if_absent(
                request,
                http::header::CONTENT_LENGTH,
                content_length,
            );
        }
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::CapturePokemon::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "CapturePokemon",
            "PokemonService",
        ));
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`CapturePokemonInput`](crate::input::CapturePokemonInput).
    pub fn builder() -> crate::input::capture_pokemon_input::Builder {
        crate::input::capture_pokemon_input::Builder::default()
    }
}

/// See [`GetStorageInput`](crate::input::GetStorageInput).
pub mod get_storage_input {

    /// A builder for [`GetStorageInput`](crate::input::GetStorageInput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) user: std::option::Option<std::string::String>,
        pub(crate) passcode: std::option::Option<std::string::String>,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn user(mut self, input: impl Into<std::string::String>) -> Self {
            self.user = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_user(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.user = input;
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn passcode(mut self, input: impl Into<std::string::String>) -> Self {
            self.passcode = Some(input.into());
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_passcode(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.passcode = input;
            self
        }
        /// Consumes the builder and constructs a [`GetStorageInput`](crate::input::GetStorageInput).
        pub fn build(
            self,
        ) -> Result<crate::input::GetStorageInput, aws_smithy_http::operation::BuildError> {
            Ok(crate::input::GetStorageInput {
                user: self.user,
                passcode: self.passcode,
            })
        }
    }
}
impl GetStorageInput {
    /// Consumes the builder and constructs an Operation<[`GetStorage`](crate::operation::GetStorage)>
    #[allow(unused_mut)]
    #[allow(clippy::let_and_return)]
    #[allow(clippy::needless_borrow)]
    pub async fn make_operation(
        &self,
        _config: &crate::config::Config,
    ) -> std::result::Result<
        aws_smithy_http::operation::Operation<
            crate::operation::GetStorage,
            aws_smithy_http::retry::DefaultResponseRetryClassifier,
        >,
        aws_smithy_http::operation::BuildError,
    > {
        let mut request = {
            fn uri_base(
                _input: &crate::input::GetStorageInput,
                output: &mut String,
            ) -> Result<(), aws_smithy_http::operation::BuildError> {
                let input_3 = &_input.user;
                let input_3 = input_3.as_ref().ok_or(
                    aws_smithy_http::operation::BuildError::MissingField {
                        field: "user",
                        details: "cannot be empty or unset",
                    },
                )?;
                let user = aws_smithy_http::label::fmt_string(input_3, false);
                if user.is_empty() {
                    return Err(aws_smithy_http::operation::BuildError::MissingField {
                        field: "user",
                        details: "cannot be empty or unset",
                    });
                }
                write!(output, "/pokedex/{user}", user = user).expect("formatting should succeed");
                Ok(())
            }
            #[allow(clippy::unnecessary_wraps)]
            fn update_http_builder(
                input: &crate::input::GetStorageInput,
                builder: http::request::Builder,
            ) -> std::result::Result<http::request::Builder, aws_smithy_http::operation::BuildError>
            {
                let mut uri = String::new();
                uri_base(input, &mut uri)?;
                let builder = crate::http_serde::add_headers_get_storage(input, builder)?;
                Ok(builder.method("GET").uri(uri))
            }
            let mut builder = update_http_builder(&self, http::request::Builder::new())?;
            builder
        };
        let mut properties = aws_smithy_http::property_bag::SharedPropertyBag::new();
        #[allow(clippy::useless_conversion)]
        let body = aws_smithy_http::body::SdkBody::from("");
        let request = request.body(body).expect("should be valid request");
        let mut request = aws_smithy_http::operation::Request::from_parts(request, properties);
        request
            .properties_mut()
            .insert(aws_smithy_http::http_versions::DEFAULT_HTTP_VERSION_LIST.clone());
        let op = aws_smithy_http::operation::Operation::new(
            request,
            crate::operation::GetStorage::new(),
        )
        .with_metadata(aws_smithy_http::operation::Metadata::new(
            "GetStorage",
            "PokemonService",
        ));
        Ok(op)
    }
    /// Creates a new builder-style object to manufacture [`GetStorageInput`](crate::input::GetStorageInput).
    pub fn builder() -> crate::input::get_storage_input::Builder {
        crate::input::get_storage_input::Builder::default()
    }
}

/// A request to access Pokémon storage.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetStorageInput {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub user: std::option::Option<std::string::String>,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub passcode: std::option::Option<std::string::String>,
}
impl GetStorageInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn user(&self) -> std::option::Option<&str> {
        self.user.as_deref()
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn passcode(&self) -> std::option::Option<&str> {
        self.passcode.as_deref()
    }
}
impl std::fmt::Debug for GetStorageInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetStorageInput");
        formatter.field("user", &self.user);
        formatter.field("passcode", &self.passcode);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetPokemonSpeciesInput {
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
}
impl GetPokemonSpeciesInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
}
impl std::fmt::Debug for GetPokemonSpeciesInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetPokemonSpeciesInput");
        formatter.field("name", &self.name);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CheckHealthInput {}
impl std::fmt::Debug for CheckHealthInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CheckHealthInput");
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
pub struct CapturePokemonInput {
    #[allow(missing_docs)] // documentation missing in model
    pub events: aws_smithy_http::event_stream::EventStreamSender<
        crate::model::AttemptCapturingPokemonEvent,
        crate::error::AttemptCapturingPokemonEventError,
    >,
    #[allow(missing_docs)] // documentation missing in model
    #[doc(hidden)]
    pub region: std::option::Option<std::string::String>,
}
impl CapturePokemonInput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn events(
        &self,
    ) -> &aws_smithy_http::event_stream::EventStreamSender<
        crate::model::AttemptCapturingPokemonEvent,
        crate::error::AttemptCapturingPokemonEventError,
    > {
        &self.events
    }
    #[allow(missing_docs)] // documentation missing in model
    pub fn region(&self) -> std::option::Option<&str> {
        self.region.as_deref()
    }
}
impl std::fmt::Debug for CapturePokemonInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CapturePokemonInput");
        formatter.field("events", &self.events);
        formatter.field("region", &self.region);
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DoNothingInput {}
impl std::fmt::Debug for DoNothingInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DoNothingInput");
        formatter.finish()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetServerStatisticsInput {}
impl std::fmt::Debug for GetServerStatisticsInput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetServerStatisticsInput");
        formatter.finish()
    }
}
