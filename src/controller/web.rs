use crate::service;

use http_types as http;
use tide;
use tower_service::Service;

pub struct Services {
    login: service::login::LoginSvc,
}

pub struct LoginController {}

impl LoginController {
    pub fn add_router(login_svc: service::login::LoginSvc) -> tide::Server<Services> {
        let mut router = tide::with_state(Services { login: login_svc });
        router
            .at("/login")
            .get(move |req: tide::Request<Services>| get_login_page(req));
        router
    }
}

async fn get_login_page(req: tide::Request<Services>) -> tide::Result<http::Response> {
    let mut lc = req.state().login;
    let res = lc
        .call(crate::service::login::PasswdLoginForm {
            usrname: "todo",
            passwd: "sorry",
        })
        .await
        .map(|p| format!("{}:{}", p.usrname, p.passwd))
        .map(|b| {
            let mut r = http::Response::new(http::StatusCode::Ok);
            r.set_body(b);
            r
        })
        .map_err(|e| {
            let mut r = http::Response::new(http::StatusCode::InternalServerError);
            r.set_body(e.to_string());
            e
        });
    tide::Result::from(res)
}
