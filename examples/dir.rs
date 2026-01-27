#![deny(warnings)]

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    wax::serve(wax::fs::dir("examples/dir"))
        .run(([127, 0, 0, 1], 3030))
        .await;
}
