#![deny(warnings)]
#[macro_use]
extern crate wax;

use futures_util::future;
use wax::Filter;

#[tokio::test]
async fn path() {
    let _ = pretty_env_logger::try_init();

    let foo = wax::domain_is("foo");
    let bar = wax::domain_is(String::from("bar"));
    let foo_bar = foo.and(bar.clone());

    // /foo
    let foo_req = || wax::test::request().path("/foo");

    assert!(foo_req().matches(&foo).await);
    assert!(!foo_req().matches(&bar).await);
    assert!(!foo_req().matches(&foo_bar).await);

    // /foo/bar
    let foo_bar_req = || wax::test::request().path("/foo/bar");

    assert!(foo_bar_req().matches(&foo).await);
    assert!(!foo_bar_req().matches(&bar).await);
    assert!(foo_bar_req().matches(&foo_bar).await);
}

#[tokio::test]
async fn param() {
    let _ = pretty_env_logger::try_init();

    let num = wax::path::param::<u32>();

    let req = wax::test::request().path("/321");
    assert_eq!(req.filter(&num).await.unwrap(), 321);

    let s = wax::path::param::<String>();

    let req = wax::test::request().path("/wax");
    assert_eq!(req.filter(&s).await.unwrap(), "wax");

    // u32 doesn't extract a non-int
    let req = wax::test::request().path("/wax");
    assert!(!req.matches(&num).await);

    let combo = num.map(|n| n + 5).and(s);

    let req = wax::test::request().path("/42/vroom");
    assert_eq!(req.filter(&combo).await.unwrap(), (47, "vroom".to_string()));

    // empty segments never match
    let req = wax::test::request();
    assert!(
        !req.matches(&s).await,
        "param should never match an empty segment"
    );
}

#[tokio::test]
async fn end() {
    let _ = pretty_env_logger::try_init();

    let foo = wax::domain_is("foo");
    let end = wax::path::end();
    let foo_end = foo.and(end);

    assert!(
        wax::test::request().path("/").matches(&end).await,
        "end() matches /"
    );

    assert!(
        wax::test::request()
            .path("http://localhost:1234")
            .matches(&end)
            .await,
        "end() matches /"
    );

    assert!(
        wax::test::request()
            .path("http://localhost:1234?q=2")
            .matches(&end)
            .await,
        "end() matches empty path"
    );

    assert!(
        wax::test::request()
            .path("localhost:1234")
            .matches(&end)
            .await,
        "end() matches authority-form"
    );

    assert!(
        !wax::test::request().path("/foo").matches(&end).await,
        "end() doesn't match /foo"
    );

    assert!(
        wax::test::request().path("/foo").matches(&foo_end).await,
        "path().and(end()) matches /foo"
    );

    assert!(
        wax::test::request().path("/foo/").matches(&foo_end).await,
        "path().and(end()) matches /foo/"
    );
}

#[tokio::test]
async fn tail() {
    let tail = wax::path::tail();

    // matches full path
    let ex = wax::test::request()
        .path("/42/vroom")
        .filter(&tail)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "42/vroom");

    // matches index
    let ex = wax::test::request().path("/").filter(&tail).await.unwrap();
    assert_eq!(ex.as_str(), "");

    // doesn't include query
    let ex = wax::test::request()
        .path("/foo/bar?baz=quux")
        .filter(&tail)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "foo/bar");

    // doesn't include previously matched prefix
    let and = wax::domain_is("foo").and(tail);
    let ex = wax::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "bar");

    // sets unmatched path index to end
    let m = tail.and(wax::domain_is("foo"));
    assert!(!wax::test::request().path("/foo/bar").matches(&m).await);

    let m = tail.and(wax::path::end());
    assert!(wax::test::request().path("/foo/bar").matches(&m).await);

    let ex = wax::test::request()
        .path("localhost")
        .filter(&tail)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/");
}

#[tokio::test]
async fn or() {
    let _ = pretty_env_logger::try_init();

    // /foo/bar OR /foo/baz
    let foo = wax::domain_is("foo");
    let bar = wax::domain_is("bar");
    let baz = wax::domain_is("baz");
    let p = foo.and(bar.or(baz));

    // /foo/bar
    let req = wax::test::request().path("/foo/bar");

    assert!(req.matches(&p).await);

    // /foo/baz
    let req = wax::test::request().path("/foo/baz");

    assert!(req.matches(&p).await);

    // deeper nested ORs
    // /foo/bar/baz OR /foo/baz/bar OR /foo/bar/bar
    let p = foo
        .and(bar.and(baz).map(|| panic!("shouldn't match")))
        .or(foo.and(baz.and(bar)).map(|| panic!("shouldn't match")))
        .or(foo.and(bar.and(bar)));

    // /foo/baz
    let req = wax::test::request().path("/foo/baz/baz");
    assert!(!req.matches(&p).await);

    // /foo/bar/bar
    let req = wax::test::request().path("/foo/bar/bar");
    assert!(req.matches(&p).await);
}

#[tokio::test]
async fn or_else() {
    let _ = pretty_env_logger::try_init();

    let foo = wax::domain_is("foo");
    let bar = wax::domain_is("bar");

    let p = foo.and(bar.or_else(|_| future::ok::<_, std::convert::Infallible>(())));

    // /foo/bar
    let req = wax::test::request().path("/foo/nope");

    assert!(req.matches(&p).await);
}

#[tokio::test]
async fn path_macro() {
    let _ = pretty_env_logger::try_init();

    let req = wax::test::request().path("/foo/bar");
    let p = path!("foo" / "bar");
    assert!(req.matches(&p).await);

    let req = wax::test::request().path("/foo/bar");
    let p = path!(String / "bar");
    assert_eq!(req.filter(&p).await.unwrap(), "foo");

    let req = wax::test::request().path("/foo/bar");
    let p = path!("foo" / String);
    assert_eq!(req.filter(&p).await.unwrap(), "bar");

    // Requires path end

    let req = wax::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar");
    assert!(!req.matches(&p).await);

    let req = wax::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar").and(wax::domain_is("baz"));
    assert!(!req.matches(&p).await);

    // Prefix syntax

    let req = wax::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar" / ..);
    assert!(req.matches(&p).await);

    let req = wax::test::request().path("/foo/bar/baz");
    let p = path!("foo" / "bar" / ..).and(wax::path!("baz"));
    assert!(req.matches(&p).await);

    // Empty

    let req = wax::test::request().path("/");
    let p = path!();
    assert!(req.matches(&p).await);

    let req = wax::test::request().path("/foo");
    let p = path!();
    assert!(!req.matches(&p).await);
}

#[tokio::test]
async fn full_path() {
    let full_path = wax::path::full();

    let foo = wax::domain_is("foo");
    let bar = wax::domain_is("bar");
    let param = wax::path::param::<u32>();

    // matches full request path
    let ex = wax::test::request()
        .path("/42/vroom")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/42/vroom");

    // matches index
    let ex = wax::test::request()
        .path("/")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/");

    // does not include query
    let ex = wax::test::request()
        .path("/foo/bar?baz=quux")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/bar");

    // includes previously matched prefix
    let and = foo.and(full_path);
    let ex = wax::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/bar");

    // includes following matches
    let and = full_path.and(foo);
    let ex = wax::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/bar");

    // includes previously matched param
    let and = foo.and(param).and(full_path);
    let (_, ex) = wax::test::request()
        .path("/foo/123")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/foo/123");

    // does not modify matching
    let m = full_path.and(foo).and(bar);
    assert!(wax::test::request().path("/foo/bar").matches(&m).await);

    // doesn't panic on authority-form
    let ex = wax::test::request()
        .path("localhost:1234")
        .filter(&full_path)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "/");
}

#[tokio::test]
async fn peek() {
    let peek = wax::path::peek();

    let foo = wax::domain_is("foo");
    let bar = wax::domain_is("bar");
    let param = wax::path::param::<u32>();

    // matches full request path
    let ex = wax::test::request()
        .path("/42/vroom")
        .filter(&peek)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "42/vroom");

    // matches index
    let ex = wax::test::request().path("/").filter(&peek).await.unwrap();
    assert_eq!(ex.as_str(), "");

    // does not include query
    let ex = wax::test::request()
        .path("/foo/bar?baz=quux")
        .filter(&peek)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "foo/bar");

    // does not include previously matched prefix
    let and = foo.and(peek);
    let ex = wax::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "bar");

    // includes following matches
    let and = peek.and(foo);
    let ex = wax::test::request()
        .path("/foo/bar")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "foo/bar");

    // does not include previously matched param
    let and = foo.and(param).and(peek);
    let (_, ex) = wax::test::request()
        .path("/foo/123")
        .filter(&and)
        .await
        .unwrap();
    assert_eq!(ex.as_str(), "");

    // does not modify matching
    let and = peek.and(foo).and(bar);
    assert!(wax::test::request().path("/foo/bar").matches(&and).await);
}

#[tokio::test]
async fn peek_segments() {
    let peek = wax::path::peek();

    // matches full request path
    let ex = wax::test::request()
        .path("/42/vroom")
        .filter(&peek)
        .await
        .unwrap();

    assert_eq!(ex.segments().collect::<Vec<_>>(), &["42", "vroom"]);

    // matches index
    let ex = wax::test::request().path("/").filter(&peek).await.unwrap();

    let segs = ex.segments().collect::<Vec<_>>();
    assert_eq!(segs, Vec::<&str>::new());
}
