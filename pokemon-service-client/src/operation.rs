// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
/// Operation shape for `CheckHealth`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`check_health`](crate::client::Client::check_health).
///
/// See [`crate::client::fluent_builders::CheckHealth`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CheckHealth {
    _private: (),
}
impl CheckHealth {
    /// Creates a new builder-style object to manufacture [`CheckHealthInput`](crate::input::CheckHealthInput).
    pub fn builder() -> crate::input::check_health_input::Builder {
        crate::input::check_health_input::Builder::default()
    }
    /// Creates a new `CheckHealth` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for CheckHealth {
    type Output =
        std::result::Result<crate::output::CheckHealthOutput, crate::error::CheckHealthError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_check_health_error(response)
        } else {
            crate::operation_deser::parse_check_health_response(response)
        }
    }
}

/// Operation shape for `DoNothing`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`do_nothing`](crate::client::Client::do_nothing).
///
/// See [`crate::client::fluent_builders::DoNothing`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct DoNothing {
    _private: (),
}
impl DoNothing {
    /// Creates a new builder-style object to manufacture [`DoNothingInput`](crate::input::DoNothingInput).
    pub fn builder() -> crate::input::do_nothing_input::Builder {
        crate::input::do_nothing_input::Builder::default()
    }
    /// Creates a new `DoNothing` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for DoNothing {
    type Output = std::result::Result<crate::output::DoNothingOutput, crate::error::DoNothingError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_do_nothing_error(response)
        } else {
            crate::operation_deser::parse_do_nothing_response(response)
        }
    }
}

/// Operation shape for `GetPokemonSpecies`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_pokemon_species`](crate::client::Client::get_pokemon_species).
///
/// See [`crate::client::fluent_builders::GetPokemonSpecies`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetPokemonSpecies {
    _private: (),
}
impl GetPokemonSpecies {
    /// Creates a new builder-style object to manufacture [`GetPokemonSpeciesInput`](crate::input::GetPokemonSpeciesInput).
    pub fn builder() -> crate::input::get_pokemon_species_input::Builder {
        crate::input::get_pokemon_species_input::Builder::default()
    }
    /// Creates a new `GetPokemonSpecies` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetPokemonSpecies {
    type Output = std::result::Result<
        crate::output::GetPokemonSpeciesOutput,
        crate::error::GetPokemonSpeciesError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_pokemon_species_error(response)
        } else {
            crate::operation_deser::parse_get_pokemon_species_response(response)
        }
    }
}

/// Operation shape for `GetServerStatistics`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_server_statistics`](crate::client::Client::get_server_statistics).
///
/// See [`crate::client::fluent_builders::GetServerStatistics`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetServerStatistics {
    _private: (),
}
impl GetServerStatistics {
    /// Creates a new builder-style object to manufacture [`GetServerStatisticsInput`](crate::input::GetServerStatisticsInput).
    pub fn builder() -> crate::input::get_server_statistics_input::Builder {
        crate::input::get_server_statistics_input::Builder::default()
    }
    /// Creates a new `GetServerStatistics` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetServerStatistics {
    type Output = std::result::Result<
        crate::output::GetServerStatisticsOutput,
        crate::error::GetServerStatisticsError,
    >;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_server_statistics_error(response)
        } else {
            crate::operation_deser::parse_get_server_statistics_response(response)
        }
    }
}

/// Operation shape for `CapturePokemon`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`capture_pokemon`](crate::client::Client::capture_pokemon).
///
/// See [`crate::client::fluent_builders::CapturePokemon`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct CapturePokemon {
    _private: (),
}
impl CapturePokemon {
    /// Creates a new builder-style object to manufacture [`CapturePokemonInput`](crate::input::CapturePokemonInput).
    pub fn builder() -> crate::input::capture_pokemon_input::Builder {
        crate::input::capture_pokemon_input::Builder::default()
    }
    /// Creates a new `CapturePokemon` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseHttpResponse for CapturePokemon {
    type Output =
        std::result::Result<crate::output::CapturePokemonOutput, crate::error::CapturePokemonError>;
    fn parse_unloaded(
        &self,
        response: &mut aws_smithy_http::operation::Response,
    ) -> Option<Self::Output> {
        // This is an error, defer to the non-streaming parser
        if !response.http().status().is_success() && response.http().status().as_u16() != 200 {
            return None;
        }
        Some(crate::operation_deser::parse_capture_pokemon(response))
    }
    fn parse_loaded(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        // if streaming, we only hit this case if its an error
        crate::operation_deser::parse_capture_pokemon_error(response)
    }
}

/// Operation shape for `GetStorage`.
///
/// This is usually constructed for you using the the fluent builder returned by
/// [`get_storage`](crate::client::Client::get_storage).
///
/// See [`crate::client::fluent_builders::GetStorage`] for more details about the operation.
#[derive(std::default::Default, std::clone::Clone, std::fmt::Debug)]
pub struct GetStorage {
    _private: (),
}
impl GetStorage {
    /// Creates a new builder-style object to manufacture [`GetStorageInput`](crate::input::GetStorageInput).
    pub fn builder() -> crate::input::get_storage_input::Builder {
        crate::input::get_storage_input::Builder::default()
    }
    /// Creates a new `GetStorage` operation.
    pub fn new() -> Self {
        Self { _private: () }
    }
}
impl aws_smithy_http::response::ParseStrictResponse for GetStorage {
    type Output =
        std::result::Result<crate::output::GetStorageOutput, crate::error::GetStorageError>;
    fn parse(&self, response: &http::Response<bytes::Bytes>) -> Self::Output {
        if !response.status().is_success() && response.status().as_u16() != 200 {
            crate::operation_deser::parse_get_storage_error(response)
        } else {
            crate::operation_deser::parse_get_storage_response(response)
        }
    }
}

/// Operation customization and supporting types
pub mod customize;
