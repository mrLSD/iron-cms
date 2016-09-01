use iron::{Request, Response, IronResult, AfterMiddleware};
use iron::error::{IronError};
use iron::status;
use router::{NoRoute};

pub struct Error404;

impl AfterMiddleware for Error404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Catch error: {}", err);

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}
