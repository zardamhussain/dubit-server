use flate2::{read::GzEncoder, Compression};
use rocket::{fairing, Build, Request, Response, Rocket};

pub struct Gzip;

#[rocket::async_trait]
impl fairing::Fairing for Gzip {
    fn info(&self) -> fairing::Info {
        fairing::Info {
            name: "Gzip compression",
            kind: fairing::Kind::Response | fairing::Kind::Ignite | fairing::Kind::Request,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        println!("🚀 Rocket is starting up!");
        Ok(rocket)
    }

    async fn on_response<'r>(&self, request: &'r Request<'_>, response: &mut Response<'r>) {
        use std::io::{Cursor, Read};

        if request
            .headers()
            .get("Accept-Encoding")
            .any(|e| e.to_lowercase().contains("gzip"))
        {
            let body_bytes = response.body_mut().to_bytes().await.unwrap();
            let mut encoder = GzEncoder::new(&body_bytes[..], Compression::fast());
            let mut buf = Vec::with_capacity(body_bytes.len());
            let size_read = encoder.read_to_end(&mut buf).unwrap();
            response.set_sized_body(size_read, Cursor::new(buf));
            response.set_raw_header("Content-Encoding", "gzip");
        }
    }
}
