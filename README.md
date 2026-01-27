# WIP: DO NOT USE THIS FOR ANYTHING REAL
# wax

A super-easy, composable framework for building XMPP server component servers. Forked from [warp](https://crates.io/crates/warp) and built on [xmpp-rs](https://xmpp.rs/). An attempt to bring the nice stanza-handler API of [Blather](https://github.com/adhearsion/blather) into the typesafe world of Rust.

The fundamental building block of `wax` is the `Filter`: they can be combined
and composed to express rich requirements on requests.
