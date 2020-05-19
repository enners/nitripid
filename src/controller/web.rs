use crate::service;
use http_types;
use service::login::LoginSvc;
use std::future::Future;
use std::pin::Pin;
use tide::{self, http};
use tower_service::Service;

pub struct Context {}

pub struct WebController<T>
where
    T: service::template::Renderer + 'static,
{
    pub login_svc: LoginSvc<T>,
}

impl<T> WebController<T>
where
    T: service::template::Renderer + 'static,
{
    pub fn router(ctx: Context, ctrl: WebController<T>) -> tide::Server<Context>
    where
        T: service::template::Renderer + 'static,
    {
        let mut router = tide::with_state(ctx);
        router
            .at("/static")
            .serve_dir("/home/knut/dev/rust/nitripid/web/static")
            .unwrap();
        router.at("/login").get(ctrl);
        router
    }
}

impl<T> tide::Endpoint<Context> for WebController<T>
where
    T: service::template::Renderer,
{
    fn call<'a>(
        &'a self,
        _req: tide::Request<Context>,
    ) -> Pin<Box<dyn Future<Output = tide::Result<tide::Response>> + Send + 'a>> {
        let lreq = crate::service::login::LoginRequest {
            usrname: "todo",
            passwd: "sorry",
        };
        let mut svc = self.login_svc.clone();
        Box::pin(async move {
            svc.call(lreq)
                .await
                .map(|b| {
                    let mut r = http_types::Response::new(http_types::StatusCode::Ok);
                    r.set_body(b);
                    r.insert_header("Content-Type", "text/html").unwrap();
                    tide::Response::from(r)
                })
                .map_err(|e| tide::Error::from_str(http::StatusCode::InternalServerError, e))
        })
    }
}
