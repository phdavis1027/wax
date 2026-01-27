use redis::{FromRedisValue, ParsingError, ToRedisArgs};
use tokio_xmpp::jid::Jid;

use crate::redis::{ByJid, RedisKey};

#[derive(Debug)]
pub struct CustomerId(pub(crate) String);

impl AsRef<str> for &CustomerId {
    fn as_ref(&self) -> &str {
        self.0.as_ref()
    }
}

impl FromRedisValue for CustomerId {
    fn from_redis_value(v: redis::Value) -> Result<Self, ParsingError> {
        match v {
            redis::Value::SimpleString(s) => Ok(CustomerId(s)),
            _ => Err(ParsingError::from("Wrong type for customer_id")),
        }
    }
}

impl ByJid for CustomerId {
    fn by_jid(jid: &Jid) -> impl ToRedisArgs {
        format!("jmp_customer_id-{0}", jid.as_str())
    }
}

impl RedisKey for CustomerId {
    type Key = Jid;

    fn find_cmd(key: &Self::Key) -> redis::Cmd {
        redis::cmd("GET").arg(Self::by_jid(key)).take()
    }
}
