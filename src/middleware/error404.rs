use iron::{Request, Response, IronResult, AfterMiddleware};
use iron::error::{IronError};
use iron::status;
use router::{NoRoute};

/// Structure for actions with 404 error
/// and other Errors
pub struct Error404;

/// Methods for Erros middleware
impl AfterMiddleware for Error404 {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        println!("Error middleware: \n{:?}\n", err.error);

        if let Some(_) = err.error.downcast::<NoRoute>() {
            Ok(Response::with((status::NotFound, "Custom 404 response")))
        } else {
            Err(err)
        }
    }
}
