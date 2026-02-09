use std::convert::Infallible;
use std::marker::PhantomData;

use futures_util::future;
use tokio_xmpp::Stanza;
use xmpp_parsers::jid::Jid;

use crate::filter::{filter_fn, Filter, FilterBase, Internal};
use crate::generic::{self, Combine, CombinedTuples, HListProduct, One, Tuple};
use crate::reject::{CombineRejection, Rejection};

pub mod state {
    pub trait Iq {}

    #[derive(Clone, Copy)]
    pub struct IqAny;
    #[derive(Clone, Copy)]
    pub struct Get;
    #[derive(Clone, Copy)]
    pub struct Set;

    impl Iq for IqAny {}
    impl Iq for Get {}
    impl Iq for Set {}
}

#[derive(Clone, Copy)]
pub struct Query<S, F> {
    pub(crate) filter: F,
    pub(crate) _state: PhantomData<S>,
}

impl<S, F: FilterBase> FilterBase for Query<S, F> {
    type Extract = F::Extract;
    type Error = F::Error;
    type Future = F::Future;

    fn filter(&self, internal: Internal) -> Self::Future {
        self.filter.filter(internal)
    }
}

// === IQ type narrowing (only before narrowing to get/set) ===

impl<F> Query<state::IqAny, F>
where
    F: Filter<Extract = (), Error = Rejection> + Copy,
{
    pub fn get(self) -> Query<state::Get, impl Filter<Extract = (), Error = Rejection> + Copy> {
        Query {
            filter: self
                .filter
                .and(filter_fn(|stanza: &mut Stanza| match stanza {
                    Stanza::Iq(xmpp_parsers::iq::Iq::Get { .. }) => future::ok(()),
                    _ => future::err(crate::reject::item_not_found()),
                })),
            _state: PhantomData,
        }
    }

    pub fn set(self) -> Query<state::Set, impl Filter<Extract = (), Error = Rejection> + Copy> {
        Query {
            filter: self
                .filter
                .and(filter_fn(|stanza: &mut Stanza| match stanza {
                    Stanza::Iq(xmpp_parsers::iq::Iq::Set { .. }) => future::ok(()),
                    _ => future::err(crate::reject::item_not_found()),
                })),
            _state: PhantomData,
        }
    }
}

// === JID extraction (available on all Query states) ===

impl<S, F> Query<S, F> {
    pub fn from(
        self,
    ) -> Query<
        S,
        impl Filter<
                Extract = CombinedTuples<F::Extract, One<Option<Jid>>>,
                Error = <Infallible as CombineRejection<F::Error>>::One,
            > + Copy,
    >
    where
        F: Filter + Copy,
        F::Extract: Send,
        <<F as FilterBase>::Extract as Tuple>::HList: Combine<HListProduct!(Option<Jid>)> + Send,
        CombinedTuples<F::Extract, One<Option<Jid>>>: Send,
        Infallible: CombineRejection<F::Error>,
    {
        Query {
            filter: self.filter.and(super::from()),
            _state: PhantomData,
        }
    }

    pub fn to(
        self,
    ) -> Query<
        S,
        impl Filter<
                Extract = CombinedTuples<F::Extract, One<Option<Jid>>>,
                Error = <Infallible as CombineRejection<F::Error>>::One,
            > + Copy,
    >
    where
        F: Filter + Copy,
        F::Extract: Send,
        <F::Extract as Tuple>::HList: Combine<HListProduct!(Option<Jid>)> + Send,
        CombinedTuples<F::Extract, One<Option<Jid>>>: Send,
        Infallible: CombineRejection<F::Error>,
    {
        Query {
            filter: self.filter.and(super::to()),
            _state: PhantomData,
        }
    }

    pub fn require_from(
        self,
    ) -> Query<
        S,
        impl Filter<
                Extract = CombinedTuples<F::Extract, One<Jid>>,
                Error = <Rejection as CombineRejection<F::Error>>::One,
            > + Copy,
    >
    where
        F: Filter + Copy,
        F::Extract: Send,
        <F::Extract as Tuple>::HList: Combine<HListProduct!(Jid)> + Send,
        CombinedTuples<F::Extract, One<Jid>>: Send,
        Rejection: CombineRejection<F::Error>,
    {
        Query {
            filter: self.filter.and(super::require_from()),
            _state: PhantomData,
        }
    }

    pub fn require_to(
        self,
    ) -> Query<
        S,
        impl Filter<
                Extract = CombinedTuples<F::Extract, One<Jid>>,
                Error = <Rejection as CombineRejection<F::Error>>::One,
            > + Copy,
    >
    where
        F: Filter + Copy,
        F::Extract: Send,
        <F::Extract as Tuple>::HList: Combine<HListProduct!(Jid)> + Send,
        CombinedTuples<F::Extract, One<Jid>>: Send,
        Rejection: CombineRejection<F::Error>,
    {
        Query {
            filter: self.filter.and(super::require_to()),
            _state: PhantomData,
        }
    }
}
