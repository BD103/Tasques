//! Crate for running concurrent tasks using an alternative approach.

mod manager;
mod tasque;

pub mod error;
pub mod lowlevel;

pub use manager::TasqueManager;
pub use tasque::Tasque;
