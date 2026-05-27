# flux-hyperbolic-rs

Rust port of [flux-hyperbolic](https://github.com/SuperInstance/flux-hyperbolic) — hyperbolic geometry for musical tradition embeddings.

## Overview

Provides Poincaré ball and Lorentz (hyperboloid) models of hyperbolic space, with:
- Hyperbolic distance, exponential/logarithmic maps, Möbius addition
- Riemannian gradient descent for tradition embedding optimization
- Tradition embeddings mapped from dial space coordinates

## Usage

```rust
use flux_hyperbolic::{PoincareBall, TraditionEmbedding, RiemannianGD};

let ball = PoincareBall::unit();

// Distance between two points
let u = vec![0.1, 0.2];
let v = vec![0.3, -0.1];
println!("Distance: {}", ball.distance(&u, &v));

// Embed traditions
let traditions = TraditionEmbedding::standard_traditions();

// Optimize to find centroid
let gd = RiemannianGD::new(ball, 0.01);
let centroid = gd.centroid(&traditions[..3]);
```

## Relation to Python version

Pure Rust port with no Python FFI. Same mathematical formulas, idiomatic Rust API.

## License

MIT
