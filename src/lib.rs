//! flux-hyperbolic: Hyperbolic geometry for musical tradition embeddings.
//!
//! Rust port of the Python [flux-hyperbolic](https://github.com/SuperInstance/flux-hyperbolic) library.
//!
//! # Models
//!
//! - **Poincaré ball**: open unit ball with conformal metric, ideal for hierarchical embeddings
//! - **Lorentz model**: hyperboloid in Minkowski space, numerically stable for optimization
//!
//! # Core Operations
//!
//! - Hyperbolic distance, exponential/logarithmic maps, Möbius addition
//! - Riemannian gradient descent for tradition embedding optimization

mod error;
mod lorentz;
mod optimization;
mod poincare;
mod tradition;

pub use error::HyperbolicError;
pub use lorentz::LorentzModel;
pub use optimization::RiemannianGD;
pub use poincare::PoincareBall;
pub use tradition::TraditionEmbedding;
