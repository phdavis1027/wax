#![deny(warnings)]
use std::convert::Infallible;
use wax::Filter;

#[tokio::test]
async fn flattens_tuples() {
    let _ = pretty_env_logger::try_init();

    let str1 = wax::any().map(|| "wax");
    let true1 = wax::any().map(|| true);
    let unit1 = wax::any();

    // just 1 value
    let ext = wax::test::request().filter(&str1).await.unwrap();
    assert_eq!(ext, "wax");

    // just 1 unit
    let ext = wax::test::request().filter(&unit1).await.unwrap();
    assert_eq!(ext, ());

    // combine 2 values
    let and = str1.and(true1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("wax", true));

    // combine 2 reversed
    let and = true1.and(str1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, (true, "wax"));

    // combine 1 with unit
    let and = str1.and(unit1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, "wax");

    let and = unit1.and(str1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, "wax");

    // combine 3 values
    let and = str1.and(str1).and(true1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("wax", "wax", true));

    // combine 2 with unit
    let and = str1.and(unit1).and(true1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("wax", true));

    let and = unit1.and(str1).and(true1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("wax", true));

    let and = str1.and(true1).and(unit1);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("wax", true));

    // nested tuples
    let str_true_unit = str1.and(true1).and(unit1);
    let unit_str_true = unit1.and(str1).and(true1);

    let and = str_true_unit.and(unit_str_true);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("wax", true, "wax", true));

    let and = unit_str_true.and(unit1).and(str1).and(str_true_unit);
    let ext = wax::test::request().filter(&and).await.unwrap();
    assert_eq!(ext, ("wax", true, "wax", "wax", true));
}

#[tokio::test]
async fn map() {
    let _ = pretty_env_logger::try_init();

    let ok = wax::any().map(wax::reply);

    let req = wax::test::request();
    let resp = req.reply(&ok).await;
    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn or() {
    let _ = pretty_env_logger::try_init();

    // Or can be combined with an infallible filter
    let a = wax::path::param::<u32>();
    let b = wax::any().map(|| 41i32);
    let f = a.or(b);

    let _: Result<_, Infallible> = wax::test::request().filter(&f).await;
}

#[tokio::test]
async fn or_else() {
    let _ = pretty_env_logger::try_init();

    let a = wax::path::param::<u32>();
    let f = a.or_else(|_| async { Ok::<_, wax::Rejection>((44u32,)) });

    assert_eq!(
        wax::test::request().path("/33").filter(&f).await.unwrap(),
        33,
    );
    assert_eq!(wax::test::request().filter(&f).await.unwrap(), 44,);

    // OrElse can be combined with an infallible filter
    let a = wax::path::param::<u32>();
    let f = a.or_else(|_| async { Ok::<_, Infallible>((44u32,)) });

    let _: Result<_, Infallible> = wax::test::request().filter(&f).await;
}

#[tokio::test]
async fn recover() {
    let _ = pretty_env_logger::try_init();

    let a = wax::path::param::<String>();
    let f = a.recover(|err| async move { Err::<String, _>(err) });

    // not rejected
    let resp = wax::test::request().path("/hi").reply(&f).await;
    assert_eq!(resp.status(), 200);
    assert_eq!(resp.body(), "hi");

    // rejected, recovered, re-rejected
    let resp = wax::test::request().reply(&f).await;
    assert_eq!(resp.status(), 404);

    // Recover can be infallible
    let f = a.recover(|_| async move { Ok::<_, Infallible>("shh") });

    let _: Result<_, Infallible> = wax::test::request().filter(&f).await;
}

#[tokio::test]
async fn unify() {
    let _ = pretty_env_logger::try_init();

    let a = wax::path::param::<u32>();
    let b = wax::path::param::<u32>();
    let f = a.or(b).unify();

    let ex = wax::test::request().path("/1").filter(&f).await.unwrap();

    assert_eq!(ex, 1);
}

#[should_panic]
#[tokio::test]
async fn nested() {
    let f = wax::any().and_then(|| async {
        let p = wax::path::param::<u32>();
        wax::test::request().filter(&p).await
    });

    let _ = wax::test::request().filter(&f).await;
}
