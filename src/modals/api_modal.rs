use serde::Serialize;

#[derive(Serialize)]
pub struct JSONResponse<T> {
    msg: String,
    data: Option<T>,
}

impl<T: Serialize> JSONResponse<T> {
    pub fn new(msg: &str, data: Option<T>) -> Self {
        Self {
            msg: msg.to_string(),
            data,
        }
    }
}
