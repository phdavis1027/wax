#![deny(warnings)]

use wax::Filter;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let readme = wax::get()
        .and(wax::path::end())
        .and(wax::fs::file("./README.md"));

    // dir already requires GET...
    let examples = wax::domain_is("ex").and(wax::fs::dir("./examples/"));

    // GET / => README.md
    // GET /ex/... => ./examples/..
    let routes = readme.or(examples);

    wax::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
