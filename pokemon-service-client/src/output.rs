// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.

/// Contents of the Pokémon storage.
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetStorageOutput {
    /// A list of Pokémon species.
    #[doc(hidden)]
    pub collection: std::option::Option<std::vec::Vec<std::string::String>>,
}
impl GetStorageOutput {
    /// A list of Pokémon species.
    pub fn collection(&self) -> std::option::Option<&[std::string::String]> {
        self.collection.as_deref()
    }
}
impl std::fmt::Debug for GetStorageOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetStorageOutput");
        formatter.field("collection", &self.collection);
        formatter.finish()
    }
}
/// See [`GetStorageOutput`](crate::output::GetStorageOutput).
pub mod get_storage_output {

    /// A builder for [`GetStorageOutput`](crate::output::GetStorageOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) collection: std::option::Option<std::vec::Vec<std::string::String>>,
    }
    impl Builder {
        /// Appends an item to `collection`.
        ///
        /// To override the contents of this collection use [`set_collection`](Self::set_collection).
        ///
        /// A list of Pokémon species.
        pub fn collection(mut self, input: impl Into<std::string::String>) -> Self {
            let mut v = self.collection.unwrap_or_default();
            v.push(input.into());
            self.collection = Some(v);
            self
        }
        /// A list of Pokémon species.
        pub fn set_collection(
            mut self,
            input: std::option::Option<std::vec::Vec<std::string::String>>,
        ) -> Self {
            self.collection = input;
            self
        }
        /// Consumes the builder and constructs a [`GetStorageOutput`](crate::output::GetStorageOutput).
        pub fn build(self) -> crate::output::GetStorageOutput {
            crate::output::GetStorageOutput {
                collection: self.collection,
            }
        }
    }
}
impl GetStorageOutput {
    /// Creates a new builder-style object to manufacture [`GetStorageOutput`](crate::output::GetStorageOutput).
    pub fn builder() -> crate::output::get_storage_output::Builder {
        crate::output::get_storage_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetPokemonSpeciesOutput {
    /// The name for this resource.
    #[doc(hidden)]
    pub name: std::option::Option<std::string::String>,
    /// A list of flavor text entries for this Pokémon species.
    #[doc(hidden)]
    pub flavor_text_entries: std::option::Option<std::vec::Vec<crate::model::FlavorText>>,
}
impl GetPokemonSpeciesOutput {
    /// The name for this resource.
    pub fn name(&self) -> std::option::Option<&str> {
        self.name.as_deref()
    }
    /// A list of flavor text entries for this Pokémon species.
    pub fn flavor_text_entries(&self) -> std::option::Option<&[crate::model::FlavorText]> {
        self.flavor_text_entries.as_deref()
    }
}
impl std::fmt::Debug for GetPokemonSpeciesOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetPokemonSpeciesOutput");
        formatter.field("name", &self.name);
        formatter.field("flavor_text_entries", &self.flavor_text_entries);
        formatter.finish()
    }
}
/// See [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
pub mod get_pokemon_species_output {

    /// A builder for [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) name: std::option::Option<std::string::String>,
        pub(crate) flavor_text_entries:
            std::option::Option<std::vec::Vec<crate::model::FlavorText>>,
    }
    impl Builder {
        /// The name for this resource.
        pub fn name(mut self, input: impl Into<std::string::String>) -> Self {
            self.name = Some(input.into());
            self
        }
        /// The name for this resource.
        pub fn set_name(mut self, input: std::option::Option<std::string::String>) -> Self {
            self.name = input;
            self
        }
        /// Appends an item to `flavor_text_entries`.
        ///
        /// To override the contents of this collection use [`set_flavor_text_entries`](Self::set_flavor_text_entries).
        ///
        /// A list of flavor text entries for this Pokémon species.
        pub fn flavor_text_entries(mut self, input: crate::model::FlavorText) -> Self {
            let mut v = self.flavor_text_entries.unwrap_or_default();
            v.push(input);
            self.flavor_text_entries = Some(v);
            self
        }
        /// A list of flavor text entries for this Pokémon species.
        pub fn set_flavor_text_entries(
            mut self,
            input: std::option::Option<std::vec::Vec<crate::model::FlavorText>>,
        ) -> Self {
            self.flavor_text_entries = input;
            self
        }
        /// Consumes the builder and constructs a [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
        pub fn build(self) -> crate::output::GetPokemonSpeciesOutput {
            crate::output::GetPokemonSpeciesOutput {
                name: self.name,
                flavor_text_entries: self.flavor_text_entries,
            }
        }
    }
}
impl GetPokemonSpeciesOutput {
    /// Creates a new builder-style object to manufacture [`GetPokemonSpeciesOutput`](crate::output::GetPokemonSpeciesOutput).
    pub fn builder() -> crate::output::get_pokemon_species_output::Builder {
        crate::output::get_pokemon_species_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct CheckHealthOutput {}
impl std::fmt::Debug for CheckHealthOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CheckHealthOutput");
        formatter.finish()
    }
}
/// See [`CheckHealthOutput`](crate::output::CheckHealthOutput).
pub mod check_health_output {

    /// A builder for [`CheckHealthOutput`](crate::output::CheckHealthOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`CheckHealthOutput`](crate::output::CheckHealthOutput).
        pub fn build(self) -> crate::output::CheckHealthOutput {
            crate::output::CheckHealthOutput {}
        }
    }
}
impl CheckHealthOutput {
    /// Creates a new builder-style object to manufacture [`CheckHealthOutput`](crate::output::CheckHealthOutput).
    pub fn builder() -> crate::output::check_health_output::Builder {
        crate::output::check_health_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
pub struct CapturePokemonOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub events: aws_smithy_http::event_stream::Receiver<
        crate::model::CapturePokemonEvents,
        crate::error::CapturePokemonEventsError,
    >,
}
impl CapturePokemonOutput {
    #[allow(missing_docs)] // documentation missing in model
    pub fn events(
        &self,
    ) -> &aws_smithy_http::event_stream::Receiver<
        crate::model::CapturePokemonEvents,
        crate::error::CapturePokemonEventsError,
    > {
        &self.events
    }
}
impl std::fmt::Debug for CapturePokemonOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("CapturePokemonOutput");
        formatter.field("events", &self.events);
        formatter.finish()
    }
}
/// See [`CapturePokemonOutput`](crate::output::CapturePokemonOutput).
pub mod capture_pokemon_output {

    /// A builder for [`CapturePokemonOutput`](crate::output::CapturePokemonOutput).
    #[derive(std::default::Default, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) events: std::option::Option<
            aws_smithy_http::event_stream::Receiver<
                crate::model::CapturePokemonEvents,
                crate::error::CapturePokemonEventsError,
            >,
        >,
    }
    impl Builder {
        #[allow(missing_docs)] // documentation missing in model
        pub fn events(
            mut self,
            input: aws_smithy_http::event_stream::Receiver<
                crate::model::CapturePokemonEvents,
                crate::error::CapturePokemonEventsError,
            >,
        ) -> Self {
            self.events = Some(input);
            self
        }
        #[allow(missing_docs)] // documentation missing in model
        pub fn set_events(
            mut self,
            input: std::option::Option<
                aws_smithy_http::event_stream::Receiver<
                    crate::model::CapturePokemonEvents,
                    crate::error::CapturePokemonEventsError,
                >,
            >,
        ) -> Self {
            self.events = input;
            self
        }
        /// Consumes the builder and constructs a [`CapturePokemonOutput`](crate::output::CapturePokemonOutput).
        pub fn build(
            self,
        ) -> Result<crate::output::CapturePokemonOutput, aws_smithy_http::operation::BuildError>
        {
            Ok(
                crate::output::CapturePokemonOutput {
                    events: self.events
                        .ok_or(
                            aws_smithy_http::operation::BuildError::MissingField { field: "events", details: "events was not specified but it is required when building CapturePokemonOutput" }
                        )?
                    ,
                }
            )
        }
    }
}
impl CapturePokemonOutput {
    /// Creates a new builder-style object to manufacture [`CapturePokemonOutput`](crate::output::CapturePokemonOutput).
    pub fn builder() -> crate::output::capture_pokemon_output::Builder {
        crate::output::capture_pokemon_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct DoNothingOutput {}
impl std::fmt::Debug for DoNothingOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("DoNothingOutput");
        formatter.finish()
    }
}
/// See [`DoNothingOutput`](crate::output::DoNothingOutput).
pub mod do_nothing_output {

    /// A builder for [`DoNothingOutput`](crate::output::DoNothingOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {}
    impl Builder {
        /// Consumes the builder and constructs a [`DoNothingOutput`](crate::output::DoNothingOutput).
        pub fn build(self) -> crate::output::DoNothingOutput {
            crate::output::DoNothingOutput {}
        }
    }
}
impl DoNothingOutput {
    /// Creates a new builder-style object to manufacture [`DoNothingOutput`](crate::output::DoNothingOutput).
    pub fn builder() -> crate::output::do_nothing_output::Builder {
        crate::output::do_nothing_output::Builder::default()
    }
}

#[allow(missing_docs)] // documentation missing in model
#[non_exhaustive]
#[derive(std::clone::Clone, std::cmp::PartialEq)]
pub struct GetServerStatisticsOutput {
    /// The number of calls executed by the server.
    #[doc(hidden)]
    pub calls_count: std::option::Option<i64>,
}
impl GetServerStatisticsOutput {
    /// The number of calls executed by the server.
    pub fn calls_count(&self) -> std::option::Option<i64> {
        self.calls_count
    }
}
impl std::fmt::Debug for GetServerStatisticsOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut formatter = f.debug_struct("GetServerStatisticsOutput");
        formatter.field("calls_count", &self.calls_count);
        formatter.finish()
    }
}
/// See [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
pub mod get_server_statistics_output {

    /// A builder for [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
    #[derive(std::default::Default, std::clone::Clone, std::cmp::PartialEq, std::fmt::Debug)]
    pub struct Builder {
        pub(crate) calls_count: std::option::Option<i64>,
    }
    impl Builder {
        /// The number of calls executed by the server.
        pub fn calls_count(mut self, input: i64) -> Self {
            self.calls_count = Some(input);
            self
        }
        /// The number of calls executed by the server.
        pub fn set_calls_count(mut self, input: std::option::Option<i64>) -> Self {
            self.calls_count = input;
            self
        }
        /// Consumes the builder and constructs a [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
        pub fn build(self) -> crate::output::GetServerStatisticsOutput {
            crate::output::GetServerStatisticsOutput {
                calls_count: self.calls_count,
            }
        }
    }
}
impl GetServerStatisticsOutput {
    /// Creates a new builder-style object to manufacture [`GetServerStatisticsOutput`](crate::output::GetServerStatisticsOutput).
    pub fn builder() -> crate::output::get_server_statistics_output::Builder {
        crate::output::get_server_statistics_output::Builder::default()
    }
}
