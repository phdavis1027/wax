use redis::{FromRedisValue, ParsingError, ToRedisArgs};
use tokio_xmpp::jid::Jid;

use crate::redis::{ByJid, RedisKey};
use crate::tel::Tel;

#[derive(Debug)]
pub struct CatapultCred {
    user_id: String,
    token: String,
    secret: String,
    tel: Tel,
}

impl FromRedisValue for CatapultCred {
    fn from_redis_value(v: redis::Value) -> Result<Self, ParsingError> {
        let [user_id, token, secret, tel]: [String; 4] = v
            .into_sequence()
            .map_err(|_| ParsingError::from("Invalid type for catapult_cred"))?
            .into_iter()
            .map_while(|v| String::from_redis_value(v).ok())
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| ParsingError::from("Wrong size for catapult_cred"))?;

        let tel: Tel = tel
            .try_into()
            .map_err(|_| ParsingError::from("Invalid tel in catapult_cred"))?;

        Ok(CatapultCred {
            user_id,
            token,
            secret,
            tel,
        })
    }
}

impl ByJid for CatapultCred {
    fn by_jid(jid: &Jid) -> impl ToRedisArgs {
        format!("catapult_cred-{}", jid.to_bare())
    }
}

impl RedisKey for CatapultCred {
    type Key = Jid;

    fn find_cmd(key: &Self::Key) -> redis::Cmd {
        redis::cmd("LRANGE")
            .arg(Self::by_jid(key))
            .arg(0)
            .arg(4)
            .take()
    }
}
