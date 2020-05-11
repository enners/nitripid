#[macro_use]
extern crate log;

use async_std::task;
use env_logger;
use std::path;
use tide;

mod configuration;
mod controller;

mod service;

struct State {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let settings = configuration::Settings::new(path::PathBuf::default())
        .expect("failed to load configuration");
    env_logger::init();

    let login_svc = service::login::LoginSvc {};

    let state = State {};
    let address = format!("{}:{}", settings.server.address, settings.server.http.port);
    let mut httpd = tide::with_state(state);
    httpd
        .at("/")
        .nest(controller::web::LoginController::add_router(login_svc));

    task::block_on(async {
        httpd.listen(address).await?;
        Ok(())
    })
}
