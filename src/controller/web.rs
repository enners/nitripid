use crate::service;

use tide::{self, http};
use tower_service::Service;

pub struct Context<T>
where
    T: 'static + service::jinja::Jinja,
{
    pub login: service::login::LoginSvc,
    pub tmpl_engine: &'static T,
}

pub struct LoginController {}

impl LoginController {
    pub fn router<T>(cx: Context<T>) -> tide::Server<Context<T>>
    where
        T: service::jinja::Jinja + 'static,
    {
        let mut router = tide::with_state(cx);
        router
            .at("/login")
            .get(move |req: tide::Request<Context<T>>| get_login_page(req));
        router
    }
}

async fn get_login_page<T>(req: tide::Request<Context<T>>) -> tide::Result<http::Response>
where
    T: service::jinja::Jinja + 'static,
{
    let mut lc = req.state().login;
    let te = req.state().tmpl_engine;
    let res: std::result::Result<http::Response, http::Error> = lc
        .call(crate::service::login::LoginRequest {
            svc: service::login::Svc { tmpl_engine: te },
            usrname: "todo",
            passwd: "sorry",
        })
        .await
        .map(|b| {
            let mut r = http::Response::new(http::StatusCode::Ok);
            r.set_body(b);
            r
        })
        .map_err(|e| http::Error::from_str(http::StatusCode::InternalServerError, e));
    res
    //    tide::Result::from(res)
}
