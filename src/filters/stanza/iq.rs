//! IQ stanza extraction.

use futures_util::future;
use tokio_xmpp::Stanza;
use xmpp_parsers::iq::Iq;

use crate::filter::{filter_fn_one, Filter, FilterBase, Internal};
use crate::generic::One;
use crate::reject::Rejection;
use crate::xmpp::iq::{Get, Set};

/// Extract the incoming stanza as an [`Iq`], rejecting non-IQ stanzas.
///
/// # Example
///
/// ```ignore
/// use wax::Filter;
/// use xmpp_parsers::iq::Iq;
///
/// let route = wax::iq::param()
///     .map(|iq: Iq| {
///         // Handle the IQ request
///     });
/// ```
pub fn param() -> impl Filter<Extract = One<Iq>, Error = Rejection> + Copy {
    filter_fn_one(|stanza: &mut Stanza| match stanza {
        Stanza::Iq(iq) => future::ok(iq.clone()),
        _ => future::err(crate::reject::item_not_found()),
    })
}

pub trait GetFilter {
    fn get(self) -> impl Filter<Extract = One<Get>, Error = Rejection> + Copy;
}

impl<F> GetFilter for F
where
    F: Filter<Extract = One<Iq>, Error = Rejection> + Copy,
{
    fn get(self) -> impl Filter<Extract = One<Get>, Error = Rejection> + Copy {
        self.and_then(async move |iq: Iq| Get::try_from_iq(iq))
    }
}

pub trait SetFilter {
    fn set(self) -> impl Filter<Extract = One<Set>, Error = Rejection> + Copy;
}

impl<F> SetFilter for F
where
    F: Filter<Extract = One<Iq>, Error = Rejection> + Copy,
{
    fn set(self) -> impl Filter<Extract = One<Set>, Error = Rejection> + Copy {
        self.and_then(async move |iq: Iq| Set::try_from_iq(iq))
    }
}
