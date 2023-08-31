use serde::Serializer;

#[allow(clippy::trivially_copy_pass_by_ref)]
pub fn bool_to_int_str<S>(value: &bool, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(if *value { "1" } else { "0" })
}
