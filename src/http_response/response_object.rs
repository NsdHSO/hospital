use crate::http_response::http_code_w::HttpCodeW;
use serde::Serialize;

#[derive(Debug)]
pub struct ResponseObject<T> {
    pub message: T,
    pub code: HttpCodeW,
}

impl<T: Serialize> Serialize for ResponseObject<T> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut state = serializer.serialize_struct("ResponseObject", 2)?;
        state.serialize_field("message", &self.message)?;
        state.serialize_field("code", &self.code)?;
        state.end()
    }
}
