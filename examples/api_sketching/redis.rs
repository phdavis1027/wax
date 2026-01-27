use std::convert::Infallible;

use bb8_redis::{bb8::Pool, RedisConnectionManager};
use redis::FromRedisValue;
use tokio_xmpp::jid::Jid;
use wax::Filter;

use crate::customer_id::CustomerId;

pub trait RedisKey: FromRedisValue {
    type Key;

    fn find_cmd(key: &Self::Key) -> redis::Cmd;
}

pub trait FindInRedis: Sized {
    async fn find<T, C>(&self, con: &mut C) -> Result<T, redis::RedisError>
    where
        T: RedisKey<Key = Self>,
        C: redis::aio::ConnectionLike,
    {
        T::find_cmd(self).query_async(con).await
    }
}

impl FindInRedis for CustomerId {}
impl FindInRedis for Jid {}

pub trait ByJid: RedisKey {
    fn by_jid(jid: &Jid) -> impl redis::ToRedisArgs;
}

pub trait ByCustomerId: RedisKey {
    fn by_cid(cid: &CustomerId) -> impl redis::ToRedisArgs;
}

pub type RedisPool = Pool<RedisConnectionManager>;

pub fn with_redis(pool: RedisPool) -> impl Filter<Extract = (RedisPool,), Error = Infallible> + Clone {
    wax::any().map(move || pool.clone())
}
