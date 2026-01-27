//! Stanza ID filters.
//!
//! - `wax::id("expected-id")` - Predicate filter that matches stanzas with the given ID
//! - `wax::id::param()` - Extraction filter that yields the stanza ID

use futures_util::future;

use crate::correlation::GetStanzaId;
use crate::filter::{filter_fn_one, Filter};
use crate::generic::One;
use crate::reject::Rejection;

/// Extract the stanza ID from the incoming stanza.
///
/// Rejects stanzas that have no ID attribute.
///
/// # Example
///
/// ```ignore
/// use wax::Filter;
///
/// let route = wax::iq()
///     .and(wax::id::param())
///     .map(|id: String| {
///         format!("Received IQ with id: {}", id)
///     });
/// ```
pub fn param() -> impl Filter<Extract = One<String>, Error = Rejection> + Copy {
    filter_fn_one(|stanza| match stanza.get_stanza_id() {
        Some(id) => future::ok(id.as_str().to_owned()),
        None => future::err(crate::reject::item_not_found()),
    })
}

/// Filter that matches stanzas with a specific ID.
///
/// # Example
///
/// ```ignore
/// use wax::Filter;
///
/// let route = wax::id("request-123")
///     .and(wax::iq::param())
///     .map(|iq| { /* handle */ });
/// ```
pub fn id(expected: &'static str) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    param()
        .and_then(move |id: String| {
            if id == expected {
                future::ok(())
            } else {
                future::err(crate::reject::item_not_found())
            }
        })
        .untuple_one()
}
