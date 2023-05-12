use match_request::{match_request, Error, Params};
use hyper::{Request, Response, Body};
use futures::future::{BoxFuture};
use crate::routes::{unknowed_route, foo_bar, user_profile};

// A boxed type definition for your async views.
type RouterHandler = Box<dyn Fn(Request<Body>, Params) -> BoxFuture<'static, Response<Body>> + Send + Sync>;


macro_rules! route_handler {
    ($closure:expr) => {{
        #[allow(unused_mut)]
        let mut closure = $closure;
        let b: RouterHandler
         = Box::new(move |req, params| {
            Box::pin(closure(req, params))
        });
        b
    }};
}

// An example request router.
pub async fn router(req: Request<Body>) -> Result<Response<Body>, Error> {
    let method = req.method();
    let path = req.uri().path();
    
    let (handler, params) = match_request!(method, path, {
        "/foo/bar" => {
            GET => route_handler!(foo_bar),
        },
        "/user/:name" => {
            GET => route_handler!(user_profile),
        },
        "/*" => {
            GET => route_handler!(unknowed_route),
        }
    })?;

    Ok(handler(req, params).await)
}