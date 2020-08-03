use crate::service::template::Renderer;
use serde::Serialize;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;

pub struct LoginRequest {
    pub usrname: &'static str,
    pub passwd: &'static str,
}

#[derive(Serialize)]
pub struct PasswordLoginForm<'a> {
    pub lang: &'a str,
    pub usrname: &'a str,
    pub passwd: &'a str,
}

#[derive(Copy, Clone)]
pub struct LoginSvc<T>
where
    T: Renderer,
{
    pub tmpl_engine: T,
}

impl<T> Service<LoginRequest> for LoginSvc<T>
where
    T: Renderer,
{
    type Response = String;
    type Error = String;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, _req: LoginRequest) -> Self::Future {
        let te = self.tmpl_engine.clone();
        let page = te
            .render(
                "login.html",
                PasswordLoginForm {
                    lang: &"en",
                    usrname: &"usrname",
                    passwd: &"passwd",
                },
            )
            .to_owned();
        let page = async { Result::from(page) };
        Box::pin(page)
    }
}
