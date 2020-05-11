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
    info!("inside get login");
    let plf = lc
        .call(crate::service::login::PasswdLoginForm {
            usrname: "todo",
            passwd: "sorry",
        })
        .await?;
    let res = format!("{}:{}", plf.usrname, plf.passwd);
    Ok::<http::Response, http::Error>(http::Response::from(res))
}
