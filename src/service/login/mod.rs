use http_types;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use tower_service::Service;

pub struct PasswdLoginForm {
    pub usrname: &'static str,
    pub passwd: &'static str,
}

#[derive(Copy, Clone)]
pub struct LoginSvc;

impl Service<PasswdLoginForm> for LoginSvc {
    type Response = PasswdLoginForm;
    type Error = http_types::Error;
    type Future =
        Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, _: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: PasswdLoginForm) -> Self::Future {
        let r = async {
            Result::Ok(PasswdLoginForm {
                usrname: &"usrname",
                passwd: &"passwd",
            })
        };
        Box::pin(r)
    }
}
