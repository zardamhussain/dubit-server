use jsonwebtoken::{decode, decode_header, Algorithm, DecodingKey, Validation};
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
struct ServiceAccount {
    #[serde(rename = "project_id")]
    project_id: String,
}

#[derive(Debug, Deserialize)]
pub struct FirebaseUser {
    #[serde(rename = "sub")]
    pub user_id: String,
    auth_time: Option<usize>,

    pub email: Option<String>,
    email_verified: Option<bool>,
    pub name: Option<String>,
    pub picture: Option<String>,

    #[serde(flatten)]
    other: HashMap<String, serde_json::Value>,
}

pub async fn verify_firebase_id_token(id_token: &str, project_id: &str) -> Option<FirebaseUser> {
    let header = decode_header(id_token).ok()?;
    let kid = header.kid.ok_or("No 'kid' found in token header").ok()?;

    let jwks_url =
        "https://www.googleapis.com/robot/v1/metadata/x509/securetoken@system.gserviceaccount.com";
    let jwks: HashMap<String, String> = Client::new()
        .get(jwks_url)
        .send()
        .await
        .ok()?
        .json()
        .await
        .ok()?;

    let public_key_pem = jwks.get(&kid)?;

    let mut validation = Validation::new(Algorithm::RS256);
    validation.set_issuer(&[format!("https://securetoken.google.com/{}", project_id)]);
    validation.set_audience(&[project_id]);

    let token_data = decode::<FirebaseUser>(
        id_token,
        &DecodingKey::from_rsa_pem(public_key_pem.as_bytes()).ok()?,
        &validation,
    )
    .ok()?;

    Some(token_data.claims)
}
