use crate::service::jinja::tera;
use async_std::task;
use controller::web;
use env_logger;
use std::path;
use tide;

mod configuration;
mod controller;
mod service;

struct State {}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let settings = configuration::Settings::new(path::PathBuf::default())
        .expect("failed to load configuration");

    let login_svc = service::login::LoginSvc {};
    let tera = tera::TeraEngine::new("web/templates");
    let web_cx = web::Context {
        login: login_svc,
        tmpl_engine: &tera,
    };

    let state = State {};
    let address = format!("{}:{}", settings.server.address, settings.server.http.port);
    let mut httpd = tide::with_state(state);
    httpd
        .at("/")
        .nest(controller::web::LoginController::router(web_cx));

    task::block_on(async {
        httpd.listen(address).await?;
        Ok(())
    })
}
