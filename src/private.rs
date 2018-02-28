//! Not public API. Used as `$crate::export` by macros.

pub extern crate serde;
pub use lib::marker::{Send, Sync};
pub use lib::result::Result;
