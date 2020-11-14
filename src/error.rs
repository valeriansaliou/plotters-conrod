// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

#[derive(Debug)]
/// Indicates that some error occured within the Conrod backend
pub enum ConrodBackendError {
    /// The parent widget position could not be acquired, is the parent widget drawn in Conrod?
    NoParentPosition,
}

impl std::fmt::Display for ConrodBackendError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{:?}", self)
    }
}

impl std::error::Error for ConrodBackendError {}
