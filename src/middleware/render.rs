use hbs::{HandlebarsEngine, DirectorySource};

pub fn template_render(paths: Vec<&str>) -> HandlebarsEngine {
    let mut tempalte = HandlebarsEngine::new();
    // add a directory source, all files with .html suffix
    // will be loaded as template
    for path in paths.iter() {
        tempalte.add(Box::new(DirectorySource::new(path, ".html")));
    }

    // load templates from all registered sources
    if let Err(r) = tempalte.reload() {
        panic!("{}", r);
    }
    tempalte
}
