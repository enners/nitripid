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

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let cfg = Settings::new(PathBuf::default()).expect("failed to load configuration");

    let templates = tera::TeraEngine::new("web/templates/**/*.html");
    let te = templates.clone(); //TMPL_ENGINE.clone();
    let login_svc = service::login::LoginSvc { tmpl_engine: te };
    let web_controller = controller::web::WebController {
        login_svc: login_svc,
    };

    let web_cx = web::Context {};

    let address = format!("{}:{}", cfg.server.address, cfg.server.http.port);
    let mut httpd = tide::new();
    httpd.at("/").nest(controller::web::WebController::router(
        web_cx,
        web_controller,
    ));

    task::block_on(async {
        httpd.listen(address).await?;
        Ok(())
    })
}
