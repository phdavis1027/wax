//! Minimal example demonstrating the wax component server API.
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(dead_code)]

mod catapult_cred;
mod customer_id;
mod redis;
mod tel;

use bb8_redis::RedisConnectionManager;
use tokio_xmpp::Component;
use xmpp_parsers::jid::Jid;

use wax::{Filter, ServeComponent};

use crate::{
    catapult_cred::CatapultCred,
    redis::{with_redis, FindInRedis, RedisPool},
};

#[tokio::main]
async fn main() {
    let manager = RedisConnectionManager::new("redis://127.0.0.1/").unwrap();
    let redis_pool = bb8_redis::bb8::Pool::builder()
        .build(manager)
        .await
        .unwrap();

    let ibr = wax::iq()
        .get()
        .require_from()
        .and(with_redis(redis_pool))
        .and_then(
            async |from: Jid, pool: RedisPool| {
                let mut con = pool.get().await.map_err(|_| wax::reject::reject())?;
                let catapult_cred = from
                    .find::<CatapultCred, _>(&mut *con)
                    .await
                    .map_err(|_| wax::reject::reject())?;
                Ok::<_, wax::Rejection>(wax::sink())
            },
        );

    Component::new("sgxbwmsgsv2.localhost", "secret")
        .await
        .expect("Failed to connect")
        .serve(ibr)
        .run()
        .await;
}
