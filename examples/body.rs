#![deny(warnings)]

use serde_derive::{Deserialize, Serialize};

use wax::Filter;

#[derive(Deserialize, Serialize)]
struct Employee {
    name: String,
    rate: u32,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    // POST /employees/:rate  {"name":"Sean","rate":2}
    let promote = wax::post()
        .and(wax::domain_is("employees"))
        .and(wax::path::param::<u32>())
        // Only accept bodies smaller than 16kb...
        .and(wax::body::content_length_limit(1024 * 16))
        .and(wax::body::json())
        .map(|rate, mut employee: Employee| {
            employee.rate = rate;
            wax::reply::json(&employee)
        });

    wax::serve(promote).run(([127, 0, 0, 1], 3030)).await
}
