use iron::{Request, Response, IronResult, AfterMiddleware};
use iron::error::{IronError};
use iron::status;
use router::{NoRoute};
use hbs::{HandlebarsEngine, DirectorySource};

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

pub fn template(tempalte: &mut HandlebarsEngine, path: &str) {
    // add a directory source, all files with .html suffix
    // will be loaded as template
    tempalte.add(Box::new(DirectorySource::new(path, ".html")));

    // load templates from all registered sources
    if let Err(r) = tempalte.reload() {
        panic!("{}", r);
    }
}