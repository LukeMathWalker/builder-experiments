// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
#[non_exhaustive]
#[derive(Debug)]
pub struct CapturePokemonEventsErrorMarshaller;

impl CapturePokemonEventsErrorMarshaller {
    pub fn new() -> Self {
        CapturePokemonEventsErrorMarshaller
    }
}
impl aws_smithy_eventstream::frame::MarshallMessage for CapturePokemonEventsErrorMarshaller {
    type Input = crate::error::CapturePokemonEventsError;
    fn marshall(
        &self,
        _input: Self::Input,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::Message,
        aws_smithy_eventstream::error::Error,
    > {
        let mut headers = Vec::new();
        headers.push(aws_smithy_eventstream::frame::Header::new(
            ":message-type",
            aws_smithy_eventstream::frame::HeaderValue::String("exception".into()),
        ));
        let payload = match _input {
            crate::error::CapturePokemonEventsError::InvalidPokeballError(inner) => {
                headers.push(aws_smithy_eventstream::frame::Header::new(
                    ":exception-type",
                    aws_smithy_eventstream::frame::HeaderValue::String("invalid_pokeball".into()),
                ));
                headers.push(aws_smithy_eventstream::frame::Header::new(
                    ":content-type",
                    aws_smithy_eventstream::frame::HeaderValue::String(
                        "application/vnd.amazon.eventstream".into(),
                    ),
                ));
                crate::operation_ser::serialize_structure_crate_error_invalid_pokeball_error(&inner)
                    .map_err(|err| {
                        aws_smithy_eventstream::error::Error::Marshalling(format!("{}", err))
                    })?
            }
            crate::error::CapturePokemonEventsError::ThrottlingError(inner) => {
                headers.push(aws_smithy_eventstream::frame::Header::new(
                    ":exception-type",
                    aws_smithy_eventstream::frame::HeaderValue::String("throttlingError".into()),
                ));
                headers.push(aws_smithy_eventstream::frame::Header::new(
                    ":content-type",
                    aws_smithy_eventstream::frame::HeaderValue::String(
                        "application/vnd.amazon.eventstream".into(),
                    ),
                ));
                crate::operation_ser::serialize_structure_crate_error_throttling_error(&inner)
                    .map_err(|err| {
                        aws_smithy_eventstream::error::Error::Marshalling(format!("{}", err))
                    })?
            }
        };
        Ok(aws_smithy_eventstream::frame::Message::new_from_parts(
            headers, payload,
        ))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct CapturePokemonEventsMarshaller;

impl CapturePokemonEventsMarshaller {
    pub fn new() -> Self {
        CapturePokemonEventsMarshaller
    }
}
impl aws_smithy_eventstream::frame::MarshallMessage for CapturePokemonEventsMarshaller {
    type Input = crate::model::CapturePokemonEvents;
    fn marshall(
        &self,
        input: Self::Input,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::Message,
        aws_smithy_eventstream::error::Error,
    > {
        let mut headers = Vec::new();
        headers.push(aws_smithy_eventstream::frame::Header::new(
            ":message-type",
            aws_smithy_eventstream::frame::HeaderValue::String("event".into()),
        ));
        let payload = match input {
            Self::Input::Event(inner) => {
                headers.push(aws_smithy_eventstream::frame::Header::new(
                    ":event-type",
                    aws_smithy_eventstream::frame::HeaderValue::String("event".into()),
                ));
                if let Some(value) = inner.name {
                    headers.push(aws_smithy_eventstream::frame::Header::new(
                        "name",
                        aws_smithy_eventstream::frame::HeaderValue::String(value.into()),
                    ));
                }
                if let Some(value) = inner.captured {
                    headers.push(aws_smithy_eventstream::frame::Header::new(
                        "captured",
                        aws_smithy_eventstream::frame::HeaderValue::Bool(value),
                    ));
                }
                if let Some(value) = inner.shiny {
                    headers.push(aws_smithy_eventstream::frame::Header::new(
                        "shiny",
                        aws_smithy_eventstream::frame::HeaderValue::Bool(value),
                    ));
                }
                headers.push(aws_smithy_eventstream::frame::Header::new(
                    ":content-type",
                    aws_smithy_eventstream::frame::HeaderValue::String(
                        "application/octet-stream".into(),
                    ),
                ));
                if let Some(inner_payload) = inner.pokedex_update {
                    inner_payload.into_inner()
                } else {
                    Vec::new()
                }
            }
        };
        Ok(aws_smithy_eventstream::frame::Message::new_from_parts(
            headers, payload,
        ))
    }
}

#[non_exhaustive]
#[derive(Debug)]
pub struct AttemptCapturingPokemonEventUnmarshaller;

impl AttemptCapturingPokemonEventUnmarshaller {
    pub fn new() -> Self {
        AttemptCapturingPokemonEventUnmarshaller
    }
}
impl aws_smithy_eventstream::frame::UnmarshallMessage for AttemptCapturingPokemonEventUnmarshaller {
    type Output = crate::model::AttemptCapturingPokemonEvent;
    type Error = crate::error::AttemptCapturingPokemonEventError;
    fn unmarshall(
        &self,
        message: &aws_smithy_eventstream::frame::Message,
    ) -> std::result::Result<
        aws_smithy_eventstream::frame::UnmarshalledMessage<Self::Output, Self::Error>,
        aws_smithy_eventstream::error::Error,
    > {
        let response_headers = aws_smithy_eventstream::smithy::parse_response_headers(message)?;
        match response_headers.message_type.as_str() {
            "event" => {
                match response_headers.smithy_type.as_str() {
                    "event" => {
                        let mut builder = crate::model::CapturingEvent::builder();
                        builder = builder.payload(
                            crate::json_deser::deser_structure_crate_model_capturing_payload_payload(&message.payload()[..])
                                            .map_err(|err| {
                                                aws_smithy_eventstream::error::Error::Unmarshalling(format!("failed to unmarshall payload: {}", err))
                                            })?
                        );
                        Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Event(
                            crate::model::AttemptCapturingPokemonEvent::Event(builder.build()),
                        ))
                    }
                    _unknown_variant => {
                        return Err(aws_smithy_eventstream::error::Error::Unmarshalling(
                            format!("unrecognized :event-type: {}", _unknown_variant),
                        ));
                    }
                }
            }
            "exception" => {
                if response_headers.smithy_type.as_str() == "masterball_unsuccessful" {
                    let mut builder = crate::error::MasterBallUnsuccessful::builder();
                    builder = crate::json_deser::deser_structure_crate_error_master_ball_unsuccessful_json_err(&message.payload()[..], builder)
                                                            .map_err(|err| {
                                                                aws_smithy_eventstream::error::Error::Unmarshalling(format!("failed to unmarshall masterball_unsuccessful: {}", err))
                                                            })?;
                    return Ok(aws_smithy_eventstream::frame::UnmarshalledMessage::Error(
                        crate::error::AttemptCapturingPokemonEventError::MasterBallUnsuccessful(
                            builder.build(),
                        ),
                    ));
                }
                return Err(aws_smithy_eventstream::error::Error::Unmarshalling(
                    format!(
                        "unrecognized exception: {}",
                        response_headers.smithy_type.as_str()
                    ),
                ));
            }
            value => {
                return Err(aws_smithy_eventstream::error::Error::Unmarshalling(
                    format!("unrecognized :message-type: {}", value),
                ));
            }
        }
    }
}
