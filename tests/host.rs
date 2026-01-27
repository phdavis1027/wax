#![deny(warnings)]
use wax::host::Authority;

#[tokio::test]
async fn exact() {
    let filter = wax::host::exact("known.com");

    // no authority
    let req = wax::test::request();
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in URI
    let req = wax::test::request().path("http://known.com/about-us");
    assert!(req.filter(&filter).await.is_ok());

    let req = wax::test::request().path("http://unknown.com/about-us");
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in Host header
    let req = wax::test::request()
        .header("host", "known.com")
        .path("/about-us");
    assert!(req.filter(&filter).await.is_ok());

    let req = wax::test::request()
        .header("host", "unknown.com")
        .path("/about-us");
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in both - matching
    let req = wax::test::request()
        .header("host", "known.com")
        .path("http://known.com/about-us");
    assert!(req.filter(&filter).await.is_ok());

    let req = wax::test::request()
        .header("host", "unknown.com")
        .path("http://unknown.com/about-us");
    assert!(req.filter(&filter).await.unwrap_err().is_not_found());

    // specified in both - mismatch
    let req = wax::test::request()
        .header("host", "known.com")
        .path("http://known2.com/about-us");
    assert!(req
        .filter(&filter)
        .await
        .unwrap_err()
        .find::<wax::reject::InvalidHeader>()
        .is_some());

    // bad host header - invalid chars
    let req = wax::test::request()
        .header("host", "😭")
        .path("http://known.com/about-us");
    assert!(req
        .filter(&filter)
        .await
        .unwrap_err()
        .find::<wax::reject::InvalidHeader>()
        .is_some());

    // bad host header - invalid format
    let req = wax::test::request()
        .header("host", "hello space.com")
        .path("http://known.com//about-us");
    assert!(req
        .filter(&filter)
        .await
        .unwrap_err()
        .find::<wax::reject::InvalidHeader>()
        .is_some());
}

#[tokio::test]
async fn optional() {
    let filter = wax::host::optional();

    let req = wax::test::request().header("host", "example.com");
    assert_eq!(
        req.filter(&filter).await.unwrap(),
        Some(Authority::from_static("example.com"))
    );

    let req = wax::test::request();
    assert_eq!(req.filter(&filter).await.unwrap(), None);
}
