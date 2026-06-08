# INTEGRATION.md — flux-hyperbolic-rs

## Role in the SuperInstance Ecosystem

flux-hyperbolic-rs embeds **musical traditions in hyperbolic space** (Poincaré ball and Lorentz models). It provides the geometric backbone for tradition-aware composition: hierarchical relationships (e.g., Jazz ⊂ Improvisation ⊂ Music) are naturally represented by tree-like distances in hyperbolic space, which Euclidean embeddings cannot capture efficiently.

## SuperInstance Integration Points

### 1. superinstance-live — Tradition-Aware Pipelines
- `FluxRoomPipeline` (superinstance-live) queries `TraditionEmbedding` to set default dial positions for each RoomMusician
- Example: a "Jazz" room gets dial positions `(harmonic=3.2, rhythmic=2.8, spectral=2.5)` from `TraditionEmbedding::from_dial()`
- `set_param("tradition", name)` remaps the room's embedding and cascades new dial positions

### 2. creative-engine-rust — Hyperbolic Creative Dynamics
- `CreativeSystem` states can be mapped into the Poincaré ball via `TraditionEmbedding::from_dial(state[0], state[1], state[2])`
- `CreativeNetwork` coupling strengths can be derived from hyperbolic distances between tradition embeddings:
  ```rust
  let dist = poincare.distance(&embed_a.coords, &embed_b.coords);
  let coupling = 1.0 / (1.0 + dist); // closer traditions = stronger coupling
  ```
- This links creative dynamics to cultural geometry: agents from similar traditions couple more strongly.

### 3. constraint-dynamics-rs — Tradition Distance Constraints
- `TraditionEmbedding` coordinates feed into `Constraint::binary` predicates
- A "genre consistency" constraint enforces that collaborating agents stay within a hyperbolic distance threshold:
  ```rust
  Constraint::binary(
      |a, b| poincare.distance(a, b) < 0.7,
      strength = 0.6,
  )
  ```
- constraint-dsl YAML can reference tradition names, which the compiler resolves to embeddings via `TraditionEmbedding::standard_traditions()`

### 4. si-runtime-python — Spectral Ranking of Traditions
- `AdjacencyMatrix` (si-runtime-python) can be built from tradition embedding distances
- `spectral_rank()` identifies the most "central" tradition in a fleet's current repertoire
- This informs `Fleet.spectral_rank()` when ranking agents by their declared tradition affiliations

### 5. plato-adapters — Embedding I/O Normalization
- `plato-adapters.transform` provides `normalize` and `encode` adapters for converting raw tradition descriptors (strings, tags, JSON) into 3D dial vectors
- `AdapterRegistry.chain(["parse_tags", "normalize_dials", "embed_poincare"])` feeds directly into `TraditionEmbedding::from_dial()`

### 6. si-cli — Registry & Audit
- `si-cli scan` detects `flux-hyperbolic` usage by looking for `PoincareBall`, `LorentzModel`, or `TraditionEmbedding` imports
- `si-cli check` validates that all tradition names in DSL files exist in `standard_traditions()`
- `si-cli audit` flags embeddings that fall outside the Poincaré ball (indicating bad input normalization)

## Dial / Room / Snap Compatibility

| Primitive | Mapping |
|-----------|---------|
| **Dial**  | `(harmonic, rhythmic, spectral)` triplet mapped to Poincaré ball coordinates; dial position = distance from origin |
| **Room**  | Each `TraditionEmbedding` is a Room; agents inside share the same embedding neighborhood |
| **Snap**  | `snap()` projects an embedding to the ball boundary (maximal distance = pure tradition identity, no blending) |
| **Cascade**| Child rooms inherit parent embedding + small Gaussian perturbation (cultural drift) |

## Energy Conservation

Hyperbolic embedding optimization via `RiemannianGD` consumes computational energy (iterations × dimension). The SuperInstance conservation framework treats this as an η expenditure:

```
η_embedding = iterations × lr × dimension
```

When `η_embedding` exceeds the agent's η budget, optimization is early-stopped and the current embedding is frozen (analogous to `FrozenContextWindow` in signal-chain).

## Quick Start

```rust
use flux_hyperbolic::{PoincareBall, TraditionEmbedding, RiemannianGD};

let ball = PoincareBall::unit();
let jazz = TraditionEmbedding::from_dial("Jazz", 3.2, 2.8, 2.5);
let classical = TraditionEmbedding::from_dial("Classical", 1.8, 1.2, 1.5);
let dist = ball.distance(&jazz.coords, &classical.coords);

// Optimize a new tradition toward Jazz
let opt = RiemannianGD::new(ball.clone(), 0.01);
let new_tradition = opt.optimize_to_targets(&[0.0, 0.0, 0.0], &[jazz.coords]);
```

## Tests

```bash
cargo test
```

Distance invariance, projection correctness, and optimization convergence tests must pass.
