#[macro_use]
extern crate lazy_static;

use async_std::task;
use configuration::Settings;
use controller::web;
use env_logger;
use path::PathBuf;
use service::template::tera;
use std::path;
use tide;

mod configuration;
mod controller;
mod service;

lazy_static! {
    static ref TMPL_ENGINE: tera::TeraEngine = tera::TeraEngine::new("web/templates/**/*.html");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cfg = Settings::new(PathBuf::default()).expect("failed to load configuration");

    let login_svc = service::login::LoginSvc {};
    let web_cx = web::Context {
        login: login_svc,
        tmpl_engine: &*TMPL_ENGINE,
    };

    let address = format!("{}:{}", cfg.server.address, cfg.server.http.port);
    let mut httpd = tide::new();
    httpd
        .at("/")
        .nest(controller::web::WebController::router(web_cx));

    task::block_on(async {
        httpd.listen(address).await?;
        Ok(())
    })
}
