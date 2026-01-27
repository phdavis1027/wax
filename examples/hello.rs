#![deny(warnings)]
use wax::Filter;

#[tokio::main]
async fn main() {
    // Match any request and return hello world!
    let routes = wax::any().map(|| "Hello, World!");

    wax::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}
