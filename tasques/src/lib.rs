//! Crate for running concurrent tasks using an alternative approach.

#![allow(dead_code, unused_variables)]

mod manager;
mod signaler;
mod tasque;

pub mod lowlevel;

pub use manager::TasqueManager;
pub use tasque::Tasque;
