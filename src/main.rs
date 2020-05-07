mod configuration;
mod controller;

use async_std::task;
use env_logger;
use std::path;
use tide;

pub struct AuthService {}

impl AuthService {
    pub fn new() -> AuthService {
        AuthService {}
    }
    pub async fn login(&self) -> Result<String, std::io::Error> {
        Ok(String::from("Hello World"))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = configuration::Settings::new(path::PathBuf::default())
        .expect("failed to load configuration");
    env_logger::init();
    let login_svc = AuthService::new();
    let state = State {};
    let address = format!("{}:{}", settings.server.address, settings.server.http.port);
    let mut httpd = tide::with_state(state);
    httpd
        .middleware(tide::log::LogMiddleware::new())
        .at("/login")
        .nest(controller::tide_web_controller::LoginController::add_router(login_svc));

    task::block_on(async {
        httpd.listen(address).await?;
        Ok(())
    })
}

struct State {}
