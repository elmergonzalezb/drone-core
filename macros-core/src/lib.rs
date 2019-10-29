//! Procedural macros base for [Drone], an Embedded Operating System.
//!
//! This crate provides shared functionality for all Drone procedural macro
//! crates.
//!
//! [Drone]: https://github.com/drone-os/drone

#![deny(elided_lifetimes_in_paths)]
#![warn(missing_docs)]
#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions, clippy::must_use_candidate)]

mod cfg_cond;
mod macros;
mod unkeywordize;

pub use self::{
    cfg_cond::{CfgCond, CfgCondExt},
    unkeywordize::unkeywordize,
};
