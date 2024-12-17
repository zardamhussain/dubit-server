use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};

pub struct Authorization {
    pub token: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Authorization {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let header = req.headers().get_one("Authorization");

        if let Some(header_value) = header {
            if header_value.starts_with("Bearer ") {
                let token = header_value[7..].to_string(); // Extract token after "Bearer "
                
                if is_valid_token(&token).await {
                    return Outcome::Success(Authorization { token });
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}

async fn is_valid_token(token: &str) -> bool {
    token == "valid_token_example"
}
