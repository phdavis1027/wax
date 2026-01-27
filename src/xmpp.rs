pub mod iq {
    use tokio_xmpp::{jid::Jid, minidom::Element};
    use xmpp_parsers::iq::Iq;

    use crate::Rejection;

    #[derive(Debug)]
    pub struct Get {
        pub from: Option<Jid>,
        pub to: Option<Jid>,
        pub payload: Element,
        pub id: String,
        _sealed: (),
    }

    impl Get {
        pub(crate) fn try_from_iq(iq: Iq) -> Result<Self, Rejection> {
            match iq {
                Iq::Get {
                    from,
                    to,
                    id,
                    payload,
                } => Ok(Get {
                    from,
                    to,
                    id,
                    payload,
                    _sealed: (),
                }),
                _ => Err(crate::reject::item_not_found()),
            }
        }
    }

    #[derive(Debug)]
    pub struct Set {
        pub from: Option<Jid>,
        pub to: Option<Jid>,
        pub payload: Element,
        pub id: String,
        _sealed: (),
    }

    impl Set {
        pub(crate) fn try_from_iq(iq: Iq) -> Result<Self, Rejection> {
            match iq {
                Iq::Set {
                    from,
                    to,
                    id,
                    payload,
                } => Ok(Set {
                    from,
                    to,
                    id,
                    payload,
                    _sealed: (),
                }),
                _ => Err(crate::reject::item_not_found()),
            }
        }
    }
}
