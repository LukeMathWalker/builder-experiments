// Code generated by software.amazon.smithy.rust.codegen.smithy-rs. DO NOT EDIT.
pub fn serialize_structure_crate_error_master_ball_unsuccessful(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::error::MasterBallUnsuccessful,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_1) = &input.message {
        object.key("message").string(var_1.as_str());
    }
    Ok(())
}

pub fn serialize_structure_crate_model_capturing_payload(
    object: &mut aws_smithy_json::serialize::JsonObjectWriter,
    input: &crate::model::CapturingPayload,
) -> Result<(), aws_smithy_http::operation::SerializationError> {
    if let Some(var_2) = &input.name {
        object.key("name").string(var_2.as_str());
    }
    if let Some(var_3) = &input.pokeball {
        object.key("pokeball").string(var_3.as_str());
    }
    Ok(())
}
