use rocket::{fairing::{Fairing, Info, Kind}, Rocket, Build};
use rocket::log::LogLevel;
pub struct Logger;

#[rocket::async_trait]
impl Fairing for Logger {
    fn info(&self) -> Info {
        Info {
            name: "Custom Logger",
            kind: Kind::Ignite | Kind::Request,
        }
    }

    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        println!("🚀 Rocket is starting up!");
        Ok(rocket)
    }
}