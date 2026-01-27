#![deny(warnings)]
use wax::Filter;

#[tokio::test]
async fn method() {
    let _ = pretty_env_logger::try_init();
    let get = wax::get().map(wax::reply);

    let req = wax::test::request();
    assert!(req.matches(&get).await);

    let req = wax::test::request().method("POST");
    assert!(!req.matches(&get).await);

    let req = wax::test::request().method("POST");
    let resp = req.reply(&get).await;
    assert_eq!(resp.status(), 405);
}

#[tokio::test]
async fn method_not_allowed_trumps_not_found() {
    let _ = pretty_env_logger::try_init();
    let get = wax::get().and(wax::domain_is("hello").map(wax::reply));
    let post = wax::post().and(wax::domain_is("bye").map(wax::reply));

    let routes = get.or(post);

    let req = wax::test::request().method("GET").path("/bye");

    let resp = req.reply(&routes).await;
    // GET was allowed, but only for /hello, so POST returning 405 is fine.
    assert_eq!(resp.status(), 405);
}

#[tokio::test]
async fn bad_request_trumps_method_not_allowed() {
    let _ = pretty_env_logger::try_init();
    let get = wax::get()
        .and(wax::domain_is("hello"))
        .and(wax::header::exact("foo", "bar"))
        .map(wax::reply);
    let post = wax::post().and(wax::domain_is("bye")).map(wax::reply);

    let routes = get.or(post);

    let req = wax::test::request().method("GET").path("/hello");

    let resp = req.reply(&routes).await;
    // GET was allowed, but header rejects with 400, should not
    // assume POST was the appropriate method.
    assert_eq!(resp.status(), 400);
}
