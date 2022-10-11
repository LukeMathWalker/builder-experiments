// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[allow(clippy::unnecessary_wraps)]
pub fn parse_check_health_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CheckHealthOutput, crate::error::CheckHealthError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::CheckHealthError::unhandled)?;
    Err(crate::error::CheckHealthError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_check_health_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CheckHealthOutput, crate::error::CheckHealthError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::check_health_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_do_nothing_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DoNothingOutput, crate::error::DoNothingError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::DoNothingError::unhandled)?;
    Err(crate::error::DoNothingError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_do_nothing_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::DoNothingOutput, crate::error::DoNothingError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::do_nothing_output::Builder::default();
        let _ = response;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_pokemon_species_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetPokemonSpeciesOutput, crate::error::GetPokemonSpeciesError>
{
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetPokemonSpeciesError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetPokemonSpeciesError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::error::GetPokemonSpeciesError {
            meta: generic,
            kind: crate::error::GetPokemonSpeciesErrorKind::ResourceNotFoundError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_error_json_err(response.body().as_ref(), output).map_err(crate::error::GetPokemonSpeciesError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetPokemonSpeciesError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_pokemon_species_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetPokemonSpeciesOutput, crate::error::GetPokemonSpeciesError>
{
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_pokemon_species_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_pokemon_species(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetPokemonSpeciesError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_server_statistics_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetServerStatisticsOutput,
    crate::error::GetServerStatisticsError,
> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetServerStatisticsError::unhandled)?;
    Err(crate::error::GetServerStatisticsError::generic(generic))
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_server_statistics_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<
    crate::output::GetServerStatisticsOutput,
    crate::error::GetServerStatisticsError,
> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_server_statistics_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_server_statistics(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetServerStatisticsError::unhandled)?;
        output.build()
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_capture_pokemon(
    op_response: &mut aws_smithy_http::operation::Response,
) -> std::result::Result<crate::output::CapturePokemonOutput, crate::error::CapturePokemonError> {
    #[allow(unused_variables)]
    let (response, properties) = op_response.parts_mut();
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::capture_pokemon_output::Builder::default();
        let _ = response;
        output = output.set_events(Some(
            crate::http_serde::deser_payload_capture_pokemon_capture_pokemon_output_events(
                response.body_mut(),
            )?,
        ));
        output
            .build()
            .map_err(crate::error::CapturePokemonError::unhandled)?
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_capture_pokemon_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::CapturePokemonOutput, crate::error::CapturePokemonError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::CapturePokemonError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::CapturePokemonError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "InvalidPokeballError" => crate::error::CapturePokemonError {
            meta: generic,
            kind: crate::error::CapturePokemonErrorKind::InvalidPokeballError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::invalid_pokeball_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_invalid_pokeball_error_json_err(response.body().as_ref(), output).map_err(crate::error::CapturePokemonError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "MasterBallUnsuccessful" => crate::error::CapturePokemonError {
            meta: generic,
            kind: crate::error::CapturePokemonErrorKind::MasterBallUnsuccessful({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::master_ball_unsuccessful::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_master_ball_unsuccessful_json_err(response.body().as_ref(), output).map_err(crate::error::CapturePokemonError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "UnsupportedRegionError" => crate::error::CapturePokemonError {
            meta: generic,
            kind: crate::error::CapturePokemonErrorKind::UnsupportedRegionError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::unsupported_region_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_unsupported_region_error_json_err(response.body().as_ref(), output).map_err(crate::error::CapturePokemonError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "ThrottlingError" => {
            crate::error::CapturePokemonError {
                meta: generic,
                kind: crate::error::CapturePokemonErrorKind::ThrottlingError({
                    #[allow(unused_mut)]
                    let mut tmp = {
                        #[allow(unused_mut)]
                        let mut output = crate::error::throttling_error::Builder::default();
                        let _ = response;
                        output = crate::json_deser::deser_structure_crate_error_throttling_error_json_err(response.body().as_ref(), output).map_err(crate::error::CapturePokemonError::unhandled)?;
                        output.build()
                    };
                    if (&tmp.message).is_none() {
                        tmp.message = _error_message;
                    }
                    tmp
                }),
            }
        }
        _ => crate::error::CapturePokemonError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_storage_error(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetStorageOutput, crate::error::GetStorageError> {
    let generic = crate::json_deser::parse_http_generic_error(response)
        .map_err(crate::error::GetStorageError::unhandled)?;
    let error_code = match generic.code() {
        Some(code) => code,
        None => return Err(crate::error::GetStorageError::unhandled(generic)),
    };

    let _error_message = generic.message().map(|msg| msg.to_owned());
    Err(match error_code {
        "ResourceNotFoundException" => crate::error::GetStorageError {
            meta: generic,
            kind: crate::error::GetStorageErrorKind::ResourceNotFoundError({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::resource_not_found_error::Builder::default();
                    let _ = response;
                    output = crate::json_deser::deser_structure_crate_error_resource_not_found_error_json_err(response.body().as_ref(), output).map_err(crate::error::GetStorageError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        "NotAuthorized" => crate::error::GetStorageError {
            meta: generic,
            kind: crate::error::GetStorageErrorKind::NotAuthorized({
                #[allow(unused_mut)]
                let mut tmp = {
                    #[allow(unused_mut)]
                    let mut output = crate::error::not_authorized::Builder::default();
                    let _ = response;
                    output =
                        crate::json_deser::deser_structure_crate_error_not_authorized_json_err(
                            response.body().as_ref(),
                            output,
                        )
                        .map_err(crate::error::GetStorageError::unhandled)?;
                    output.build()
                };
                if (&tmp.message).is_none() {
                    tmp.message = _error_message;
                }
                tmp
            }),
        },
        _ => crate::error::GetStorageError::generic(generic),
    })
}

#[allow(clippy::unnecessary_wraps)]
pub fn parse_get_storage_response(
    response: &http::Response<bytes::Bytes>,
) -> std::result::Result<crate::output::GetStorageOutput, crate::error::GetStorageError> {
    Ok({
        #[allow(unused_mut)]
        let mut output = crate::output::get_storage_output::Builder::default();
        let _ = response;
        output = crate::json_deser::deser_operation_crate_operation_get_storage(
            response.body().as_ref(),
            output,
        )
        .map_err(crate::error::GetStorageError::unhandled)?;
        output.build()
    })
}
