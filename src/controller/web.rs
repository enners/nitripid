use crate::service;

use tide::{self, http};
use tower_service::Service;

pub struct Context<'a, T: service::jinja::Jinja> {
    pub login: service::login::LoginSvc,
    pub tmpl_engine: &'a T,
}

pub struct LoginController {}

impl LoginController {
    pub fn router<T>(cx: Context<'static, T>) -> tide::Server<Context<T>>
    where
        T: service::jinja::Jinja,
    {
        let mut router = tide::with_state(cx);
        router
            .at("/login")
            .get(move |req: tide::Request<Context<'static, T>>| get_login_page(req));
        router
    }
}

async fn get_login_page<T>(req: tide::Request<Context<'_, T>>) -> tide::Result<http::Response>
where
    T: service::jinja::Jinja,
{
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
