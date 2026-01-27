#![deny(warnings)]
use wax::http::header::{HeaderMap, HeaderValue};
use wax::Filter;

#[tokio::test]
async fn header() {
    let header = wax::reply::with::header("foo", "bar");

    let no_header = wax::any().map(wax::reply).with(&header);

    let req = wax::test::request();
    let resp = req.reply(&no_header).await;
    assert_eq!(resp.headers()["foo"], "bar");

    let prev_header = wax::reply::with::header("foo", "sean");
    let yes_header = wax::any().map(wax::reply).with(prev_header).with(header);

    let req = wax::test::request();
    let resp = req.reply(&yes_header).await;
    assert_eq!(resp.headers()["foo"], "bar", "replaces header");
}

#[tokio::test]
async fn headers() {
    let mut headers = HeaderMap::new();
    headers.insert("server", HeaderValue::from_static("wax"));
    headers.insert("foo", HeaderValue::from_static("bar"));

    let headers = wax::reply::with::headers(headers);

    let no_header = wax::any().map(wax::reply).with(&headers);

    let req = wax::test::request();
    let resp = req.reply(&no_header).await;
    assert_eq!(resp.headers()["foo"], "bar");
    assert_eq!(resp.headers()["server"], "wax");

    let prev_header = wax::reply::with::header("foo", "sean");
    let yes_header = wax::any().map(wax::reply).with(prev_header).with(headers);

    let req = wax::test::request();
    let resp = req.reply(&yes_header).await;
    assert_eq!(resp.headers()["foo"], "bar", "replaces header");
}

#[tokio::test]
async fn default_header() {
    let def_header = wax::reply::with::default_header("foo", "bar");

    let no_header = wax::any().map(wax::reply).with(&def_header);

    let req = wax::test::request();
    let resp = req.reply(&no_header).await;

    assert_eq!(resp.headers()["foo"], "bar");

    let header = wax::reply::with::header("foo", "sean");
    let yes_header = wax::any().map(wax::reply).with(header).with(def_header);

    let req = wax::test::request();
    let resp = req.reply(&yes_header).await;

    assert_eq!(resp.headers()["foo"], "sean", "doesn't replace header");
}
