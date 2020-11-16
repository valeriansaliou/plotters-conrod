// plotters-conrod
//
// Conrod backend for Plotters
// Copyright: 2020, Valerian Saliou <valerian@valeriansaliou.name>
// License: MIT

/*!
   The Plotters Conrod backend.

   This is an implementation of a Conrod backend for Plotters. This is more efficient than using the default Bitmap backend when plotting in Conrod, as it has been observed that Conrod was quite inefficient at re-rendering images at high FPS (eg. for real-time plotting).

   This backend has been optimized as for speed, and as to render plots that look very similar to the default Bitmap backend, if not indistinguishable.

   See the documentation for [ConrodBackend](struct.ConrodBackend.html) for more details.
*/

mod backend;
mod error;
mod graph;
mod triangulate;
mod utils;

pub use backend::ConrodBackend;
pub use error::ConrodBackendError;
pub use graph::ConrodBackendReusableGraph;
