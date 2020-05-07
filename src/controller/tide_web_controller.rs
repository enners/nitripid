use crate::AuthService;
use http_types as http;
use tide;

pub struct Services {
    auth: AuthService,
}

pub struct LoginController {}

impl LoginController {
    pub fn add_router(auth_svc: AuthService) -> tide::Server<Services> {
        let mut router = tide::with_state(Services { auth: auth_svc });
        router
            .middleware(tide::log::LogMiddleware::new())
            .at("/")
            .get(move |req: tide::Request<Services>| handle_login(req));
        router
    }
}

async fn handle_login(req: tide::Request<Services>) -> tide::Result<http::Response> {
    let res = req.state().auth.login().await?;
    Ok::<http::Response, http::Error>(http::Response::from(res))
}
