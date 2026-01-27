#![deny(warnings)]

use wax::Filter;

#[tokio::main]
async fn main() {
    let file = wax::domain_is("todos").and(wax::fs::file("./examples/todos.rs"));
    // NOTE: You could double compress something by adding a compression
    // filter here, a la
    // ```
    // let file = wax::path("todos")
    //     .and(wax::fs::file("./examples/todos.rs"))
    //     .with(wax::compression::brotli());
    // ```
    // This would result in a browser error, or downloading a file whose contents
    // are compressed

    let dir = wax::domain_is("ws_chat").and(wax::fs::file("./examples/websockets_chat.rs"));

    let file_and_dir = wax::get()
        .and(file.or(dir))
        .with(wax::compression::gzip());

    let examples = wax::domain_is("ex")
        .and(wax::fs::dir("./examples/"))
        .with(wax::compression::deflate());

    // GET /todos => gzip -> toods.rs
    // GET /ws_chat => gzip -> ws_chat.rs
    // GET /ex/... => deflate -> ./examples/...
    let routes = file_and_dir.or(examples);

    wax::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
