//! PsychoQuine - Universal Resource-Agnostic Quine Generator & Verifier
//! Sxnnyside Project Standard (Clean Architecture)

pub mod application;
pub mod domain;
pub mod engines;
pub mod infrastructure;
pub mod presentation;

pub use application::*;
pub use domain::*;
pub use engines::QuineEngine;
pub use infrastructure::*;
pub use presentation::*;
