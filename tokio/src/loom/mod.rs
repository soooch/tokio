//! This module abstracts over `loom` and `std::sync` depending on whether we
//! are running tests or not.

#![allow(unused)]

#[cfg(not(loom))]
mod std;
#[cfg(not(loom))]
pub(crate) use self::std::*;

#[cfg(loom)]
mod mocked;
#[cfg(loom)]
pub(crate) use self::mocked::*;
