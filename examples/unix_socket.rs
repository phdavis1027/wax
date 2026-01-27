#![deny(warnings)]

#[cfg(unix)]
#[tokio::main]
async fn main() {
    use tokio::net::UnixListener;

    pretty_env_logger::init();

    let socket = "/tmp/wax.sock";

    let listener = UnixListener::bind(socket).unwrap();
    wax::serve(wax::fs::dir("examples/dir"))
        .incoming(listener)
        .graceful(async { tokio::signal::ctrl_c().await.unwrap() })
        .run()
        .await;

    std::fs::remove_file(socket).unwrap();
}

#[cfg(not(unix))]
#[tokio::main]
async fn main() {
    panic!("Must run under Unix-like platform!");
}
