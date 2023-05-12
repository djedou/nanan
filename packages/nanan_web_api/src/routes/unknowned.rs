use match_request::{/*match_request, Error,*/ Params};
use hyper::{Request, Response, Body};

pub(crate) async fn foo_bar(_req: Request<Body>, _params: Params) -> Response<Body> {
    Response::new(Body::from("Foo bar"))
}

pub(crate) async fn user_profile(_req: Request<Body>, params: Params) -> Response<Body> {
    let name = params.get("name").unwrap();
    Response::new(Body::from(format!("Profile for {}", name)))
}

pub(crate) async fn unknowed_route(_req: Request<Body>, params: Params) -> Response<Body> {
    Response::new(Body::from(format!("Unknowed Route")))
}