//! XMPP Stanza filters.
//!
//! These filters extract and match incoming XMPP stanzas, and provide
//! combinators for sending stanzas to other XMPP entities.

use std::convert::Infallible;
use std::marker::PhantomData;

use futures_util::future;
use tokio_xmpp::Stanza;
use xmpp_parsers::jid::Jid;
use xmpp_parsers::message::{Lang, Message};

use crate::filter::{filter_fn, filter_fn_one, Filter};
use crate::generic::One;
use crate::reject::Rejection;
use crate::Reply;

pub mod message;
pub mod presence;
pub mod query;

use query::Query;

/// Match incoming message stanzas without extracting.
pub fn message() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(|stanza: &mut Stanza| match stanza {
        Stanza::Message(_) => future::ok(()),
        _ => future::err(crate::reject::item_not_found()),
    })
}

/// Match incoming IQ stanzas, returning a [`Query`] that supports
/// type-state narrowing with `.get()` and `.set()`.
pub fn iq() -> Query<query::state::IqAny, impl Filter<Extract = (), Error = Rejection> + Copy> {
    Query {
        filter: filter_fn(|stanza: &mut Stanza| match stanza {
            Stanza::Iq(_) => future::ok(()),
            _ => future::err(crate::reject::item_not_found()),
        }),
        _state: PhantomData,
    }
}

/// Match incoming presence stanzas without extracting.
pub fn presence() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(|stanza: &mut Stanza| match stanza {
        Stanza::Presence(_) => future::ok(()),
        _ => future::err(crate::reject::item_not_found()),
    })
}

/// Extract the `from` JID attribute from the incoming stanza.
pub fn from() -> impl Filter<Extract = One<Option<Jid>>, Error = Infallible> + Copy {
    filter_fn_one(|stanza: &mut Stanza| {
        let from = match stanza {
            Stanza::Message(msg) => msg.from.clone(),
            Stanza::Iq(iq) => match iq {
                xmpp_parsers::iq::Iq::Get { from, .. }
                | xmpp_parsers::iq::Iq::Set { from, .. }
                | xmpp_parsers::iq::Iq::Result { from, .. }
                | xmpp_parsers::iq::Iq::Error { from, .. } => from.clone(),
            },
            Stanza::Presence(pres) => pres.from.clone(),
        };
        future::ok::<_, Infallible>(from)
    })
}

/// Extract the `to` JID attribute from the incoming stanza.
pub fn to() -> impl Filter<Extract = One<Option<Jid>>, Error = Infallible> + Copy {
    filter_fn_one(|stanza: &mut Stanza| {
        let to = match stanza {
            Stanza::Message(msg) => msg.to.clone(),
            Stanza::Iq(iq) => match iq {
                xmpp_parsers::iq::Iq::Get { to, .. }
                | xmpp_parsers::iq::Iq::Set { to, .. }
                | xmpp_parsers::iq::Iq::Result { to, .. }
                | xmpp_parsers::iq::Iq::Error { to, .. } => to.clone(),
            },
            Stanza::Presence(pres) => pres.to.clone(),
        };
        future::ok::<_, Infallible>(to)
    })
}

/// Extract the `from` JID attribute, rejecting if absent.
pub fn require_from() -> impl Filter<Extract = One<Jid>, Error = Rejection> + Copy {
    filter_fn_one(|stanza: &mut Stanza| {
        let from = match stanza {
            Stanza::Message(msg) => msg.from.clone(),
            Stanza::Iq(iq) => match iq {
                xmpp_parsers::iq::Iq::Get { from, .. }
                | xmpp_parsers::iq::Iq::Set { from, .. }
                | xmpp_parsers::iq::Iq::Result { from, .. }
                | xmpp_parsers::iq::Iq::Error { from, .. } => from.clone(),
            },
            Stanza::Presence(pres) => pres.from.clone(),
        };
        match from {
            Some(jid) => future::ok(jid),
            None => future::err(crate::reject::item_not_found()),
        }
    })
}

/// Extract the `to` JID attribute, rejecting if absent.
pub fn require_to() -> impl Filter<Extract = One<Jid>, Error = Rejection> + Copy {
    filter_fn_one(|stanza: &mut Stanza| {
        let to = match stanza {
            Stanza::Message(msg) => msg.to.clone(),
            Stanza::Iq(iq) => match iq {
                xmpp_parsers::iq::Iq::Get { to, .. }
                | xmpp_parsers::iq::Iq::Set { to, .. }
                | xmpp_parsers::iq::Iq::Result { to, .. }
                | xmpp_parsers::iq::Iq::Error { to, .. } => to.clone(),
            },
            Stanza::Presence(pres) => pres.to.clone(),
        };
        match to {
            Some(jid) => future::ok(jid),
            None => future::err(crate::reject::item_not_found()),
        }
    })
}

/// Create a message reply with the given body.
///
/// The reply's `to` is the incoming stanza's `from`, and the reply's `from`
/// is the incoming stanza's `to`.
pub fn reply(
    body: impl Into<String>,
) -> impl Filter<Extract = One<Message>, Error = Infallible> + Clone {
    let body = body.into();
    from()
        .and(to())
        .map(move |sender: Option<Jid>, recipient: Option<Jid>| {
            let mut msg = Message::new(sender);
            msg.from = recipient;
            msg.with_body(Lang::default(), body.clone())
        })
}

/// Extract the message body and echo it back as a reply.
pub fn echo() -> impl Filter<Extract = One<Message>, Error = Rejection> + Copy {
    message::body::param().and(from()).and(to()).map(
        |body: String, sender: Option<Jid>, recipient: Option<Jid>| {
            let mut msg = Message::new(sender);
            msg.from = recipient;
            msg.with_body(Lang::default(), body)
        },
    )
}

pub fn sink() -> impl Reply {
    None::<Stanza>
}
