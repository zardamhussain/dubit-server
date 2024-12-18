use serde::Serialize;

#[derive(Serialize)]
pub struct JSONResponse<T> {
    msg: String,
    data: T,
}

impl<T: Serialize> JSONResponse<T> {
    pub fn new(msg: &str, data: T) -> Self {
        Self { msg: msg.to_string(), data }
    }
}