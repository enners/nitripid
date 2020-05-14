use crate::service::jinja::Jinja;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;

pub struct Svc<T>
where
    T: 'static + Jinja,
{
    pub tmpl_engine: &'static T,
}

pub struct LoginRequest<T>
where
    T: Jinja + 'static,
{
    pub svc: Svc<T>,
    pub usrname: &'static str,
    pub passwd: &'static str,
}

#[derive(Serialize)]
pub struct PasswordLoginForm<'a> {
    pub usrname: &'a str,
    pub passwd: &'a str,
}

#[derive(Copy, Clone)]
pub struct LoginSvc;

impl<T> Service<LoginRequest<T>> for LoginSvc
where
    T: 'static + Jinja,
{
    type Response = String;
    type Error = String;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: LoginRequest<T>) -> Self::Future {
        let page = req
            .svc
            .tmpl_engine
            .render(
                "login.html",
                PasswordLoginForm {
                    usrname: &"usrname",
                    passwd: &"passwd",
                },
            )
            .to_owned();
        let page = async { Result::from(page) };
        Box::pin(page)
    }
}
