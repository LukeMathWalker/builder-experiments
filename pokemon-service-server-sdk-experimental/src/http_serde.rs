// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn deser_payload_capture_pokemon_capture_pokemon_input_events(
    body: &mut aws_smithy_http::body::SdkBody,
) -> std::result::Result<
    aws_smithy_http::event_stream::Receiver<
        crate::model::AttemptCapturingPokemonEvent,
        crate::error::AttemptCapturingPokemonEventError,
    >,
    aws_smithy_json::deserialize::Error,
> {
    let unmarshaller = crate::event_stream_serde::AttemptCapturingPokemonEventUnmarshaller::new();
    let body = std::mem::replace(body, aws_smithy_http::body::SdkBody::taken());
    Ok(aws_smithy_http::event_stream::Receiver::new(
        unmarshaller,
        body,
    ))
}

pub fn deser_header_get_storage_get_storage_input_passcode(
    header_map: &http::HeaderMap,
) -> std::result::Result<
    std::option::Option<std::string::String>,
    aws_smithy_http::header::ParseError,
> {
    let headers = header_map.get_all("passcode").iter();
    aws_smithy_http::header::one_or_none(headers)
}
