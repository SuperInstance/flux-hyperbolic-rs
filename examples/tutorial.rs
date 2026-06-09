//! # flux-hyperbolic Tutorial
//!
//! A progressive guide to hyperbolic geometry for agent state spaces.
//! Run with: `cargo run --example tutorial`
//!
//! ## Lessons
//! 1. The Poincaré Ball — your geometric playground
//! 2. Hyperbolic distance — measuring similarity in curved space
//! 3. Möbius addition — hyperbolic vector math
//! 4. Exponential & logarithmic maps — navigating the manifold
//! 5. The Lorentz model — numerically stable hyperbolic space
//! 6. Tradition embeddings — representing concepts as hyperbolic points
//! 7. Riemannian optimization — finding centroids in curved space
//! 8. Putting it all together — a mini embedding search engine

use flux_hyperbolic::{LorentzModel, PoincareBall, RiemannianGD, TraditionEmbedding};

fn separator(title: &str) {
    println!();
    println!("═══════════════════════════════════════════════════════════");
    println!("  Lesson: {}", title);
    println!("═══════════════════════════════════════════════════════════");
    println!();
}

fn main() {
    println!("╔═══════════════════════════════════════════════════════════╗");
    println!("║         flux-hyperbolic Tutorial                         ║");
    println!("║   Hyperbolic Geometry for Agent State Spaces             ║");
    println!("╚═══════════════════════════════════════════════════════════╝");

    lesson_1_poincare_ball();
    lesson_2_hyperbolic_distance();
    lesson_3_mobius_addition();
    lesson_4_exp_log_maps();
    lesson_5_lorentz_model();
    lesson_6_tradition_embeddings();
    lesson_7_riemannian_optimization();
    lesson_8_embedding_search();

    println!();
    println!("✅ Tutorial complete! You've mastered hyperbolic geometry in Rust.");
}

// ── Lesson 1 ────────────────────────────────────────────────────────
fn lesson_1_poincare_ball() {
    separator("1. The Poincaré Ball — Your Geometric Playground");

    println!("The Poincaré ball is an open unit ball where points represent");
    println!("positions in hyperbolic space. It's perfect for hierarchical");
    println!("embeddings because distance grows exponentially near the boundary.\n");

    // Create a unit-curvature ball (curvature c = 1)
    let ball = PoincareBall::unit();
    println!("📦 Created unit Poincaré ball (curvature = 1.0)");
    println!("   Radius: {:.4}", ball.radius());

    // Try a different curvature
    let ball_c2 = PoincareBall::new(2.0).unwrap();
    println!("\n📦 Created ball with curvature = 2.0");
    println!("   Radius: {:.4} (= 1/√c)", ball_c2.radius());

    // Invalid curvature is rejected
    let bad = PoincareBall::new(-1.0);
    println!("\n❌ Curvature = -1.0 rejected: {}", bad.unwrap_err());

    // The conformal factor measures local stretching
    let origin = vec![0.0, 0.0];
    let near_boundary = vec![0.9, 0.0];
    println!("\n📏 Conformal factor (local stretching):");
    println!("   At origin [0, 0]:     λ = {:.4}", ball.conformal_factor(&origin));
    println!("   Near edge [0.9, 0]:   λ = {:.4}", ball.conformal_factor(&near_boundary));
    println!("   → Space stretches as you approach the boundary!");
}

// ── Lesson 2 ────────────────────────────────────────────────────────
fn lesson_2_hyperbolic_distance() {
    separator("2. Hyperbolic Distance — Measuring Similarity");

    println!("Hyperbolic distance is fundamentally different from Euclidean.");
    println!("Points near the boundary are infinitely far from the center.\n");

    let ball = PoincareBall::unit();

    // Distance from origin to various points
    let origin = vec![0.0, 0.0];
    let points = [
        (vec![0.1, 0.0], "0.1"),
        (vec![0.5, 0.0], "0.5"),
        (vec![0.9, 0.0], "0.9"),
        (vec![0.99, 0.0], "0.99"),
    ];

    println!("📏 Distance from origin to points along x-axis:");
    for (p, label) in &points {
        let d = ball.distance(&origin, p);
        println!("   dist([0,0], [{}, 0]) = {:.6}", label, d);
    }
    println!("   → Distances grow rapidly near the boundary!");

    // Distance is symmetric
    let u = vec![0.1, 0.2];
    let v = vec![0.3, -0.1];
    println!("\n📐 Distance is symmetric:");
    println!("   dist(u, v) = {:.6}", ball.distance(&u, &v));
    println!("   dist(v, u) = {:.6}", ball.distance(&v, &u));

    // Mobility: a similarity metric in (0, 1]
    println!("\n🤝 Mobility (similarity): exp(-distance)");
    println!("   Self:       mobility = {:.4}", ball.mobility(&u, &u));
    println!("   Near point: mobility = {:.4}", ball.mobility(&origin, &vec![0.1, 0.0]));
    println!("   Far point:  mobility = {:.4}", ball.mobility(&origin, &vec![0.8, 0.0]));
}

// ── Lesson 3 ────────────────────────────────────────────────────────
fn lesson_3_mobius_addition() {
    separator("3. Möbius Addition — Hyperbolic Vector Math");

    println!("Möbius addition (⊕) is the hyperbolic analog of vector addition.");
    println!("It's non-commutative: x ⊕ y ≠ y ⊕ x!\n");

    let ball = PoincareBall::unit();

    let x = vec![0.3, 0.1];
    let y = vec![0.1, 0.2];

    // Identity: x ⊕ 0 = x
    let origin = vec![0.0, 0.0];
    let x_plus_zero = ball.mobius_add(&x, &origin);
    println!("🔄 Identity law: x ⊕ origin = x");
    println!("   x            = [{:.4}, {:.4}]", x[0], x[1]);
    println!("   x ⊕ origin   = [{:.4}, {:.4}]", x_plus_zero[0], x_plus_zero[1]);

    // Non-commutativity
    let x_plus_y = ball.mobius_add(&x, &y);
    let y_plus_x = ball.mobius_add(&y, &x);
    println!("\n🔀 Non-commutativity:");
    println!("   x ⊕ y = [{:.6}, {:.6}]", x_plus_y[0], x_plus_y[1]);
    println!("   y ⊕ x = [{:.6}, {:.6}]", y_plus_x[0], y_plus_x[1]);
    println!("   Difference shows curved geometry at work!");

    // Geodesic midpoint
    let mid = ball.midpoint(&x, &y);
    println!("\n🎯 Geodesic midpoint:");
    println!("   x   = [{:.4}, {:.4}]", x[0], x[1]);
    println!("   mid = [{:.4}, {:.4}]", mid[0], mid[1]);
    println!("   y   = [{:.4}, {:.4}]", y[0], y[1]);
    println!("   d(x, mid) = {:.6}", ball.distance(&x, &mid));
    println!("   d(mid, y) = {:.6}", ball.distance(&mid, &y));
    println!("   → Equal distances confirm the midpoint!");
}

// ── Lesson 4 ────────────────────────────────────────────────────────
fn lesson_4_exp_log_maps() {
    separator("4. Exponential & Logarithmic Maps");

    println!("Maps let you convert between tangent vectors (Euclidean) and");
    println!("points on the manifold (hyperbolic). They're inverses!\n");

    let ball = PoincareBall::unit();
    let base = vec![0.2, 0.1];

    // Exponential map: tangent → manifold
    let tangent = vec![0.3, -0.2];
    let point = ball.exp_map(&base, &tangent);
    println!("📤 Exponential map (tangent → manifold):");
    println!("   Base:     [{:.4}, {:.4}]", base[0], base[1]);
    println!("   Tangent:  [{:.4}, {:.4}]", tangent[0], tangent[1]);
    println!("   Point:    [{:.4}, {:.4}]", point[0], point[1]);

    // Logarithmic map: manifold → tangent (round-trip)
    let recovered = ball.log_map(&base, &point);
    println!("\n📥 Logarithmic map (manifold → tangent):");
    println!("   Recovered: [{:.6}, {:.6}]", recovered[0], recovered[1]);
    println!("   Original:  [{:.6}, {:.6}]", tangent[0], tangent[1]);
    println!("   → Round-trip preserves the tangent vector!");

    // Projection keeps points inside the ball
    let outside = vec![2.0, 3.0];
    let projected = ball.project(&outside);
    let proj_norm = projected.iter().map(|v| v * v).sum::<f64>().sqrt();
    println!("\n🛡️  Projection (keeps points inside the ball):");
    println!("   Outside: [{:.1}, {:.1}] (norm > 1)", outside[0], outside[1]);
    println!("   Projected: [{:.4}, {:.4}] (norm = {:.6})", projected[0], projected[1], proj_norm);
}

// ── Lesson 5 ────────────────────────────────────────────────────────
fn lesson_5_lorentz_model() {
    separator("5. The Lorentz Model — Numerically Stable Hyperbolic Space");

    println!("The Lorentz (hyperboloid) model lives in Minkowski space.");
    println!("Points satisfy <x,x>_L = -1/c on the upper hyperboloid sheet.\n");

    let model = LorentzModel::unit();

    // Embed Euclidean points onto the hyperboloid
    let z1 = vec![0.0, 0.0];
    let z2 = vec![0.3, -0.2];
    let h1 = model.embed(&z1);
    let h2 = model.embed(&z2);

    println!("🔮 Embedding Euclidean → Hyperboloid:");
    println!("   [0.0, 0.0] → [{:.4}, {:.4}, {:.4}]", h1[0], h1[1], h1[2]);
    println!("   [0.3, -0.2] → [{:.4}, {:.4}, {:.4}]", h2[0], h2[1], h2[2]);
    println!("   (x₀ = √(1/c + ||z||²) is the time coordinate)");

    // Verify on hyperboloid: <x,x>_L = -1
    let mdot = model.minkowski_dot(&h2, &h2);
    println!("\n✓ Minkowski inner product <x,x>_L = {:.6} (should be -1.0)", mdot);

    // Distance
    let dist = model.distance(&h1, &h2);
    println!("\n📏 Distance between embedded points: {:.6}", dist);

    // Extract Euclidean coordinates back
    let eucl = model.to_euclidean(&h2);
    println!("\n🔙 Euclidean extraction: [{:.4}, {:.4}]", eucl[0], eucl[1]);

    // Projection normalizes back onto the hyperboloid
    let scaled: Vec<f64> = h2.iter().map(|v| v * 3.0).collect();
    let proj = model.project(&scaled);
    let proj_mdot = model.minkowski_dot(&proj, &proj);
    println!("\n🛡️  Projection after scaling 3×: <proj,proj>_L = {:.6}", proj_mdot);
}

// ── Lesson 6 ────────────────────────────────────────────────────────
fn lesson_6_tradition_embeddings() {
    separator("6. Tradition Embeddings — Concepts as Hyperbolic Points");

    println!("Tradition embeddings map musical traditions into hyperbolic space");
    println!("using dial positions (harmonic, rhythmic, spectral) as coordinates.\n");

    // Create custom embeddings
    let jazz = TraditionEmbedding::from_dial("Jazz", 3.2, 2.8, 2.5);
    let blues = TraditionEmbedding::from_dial("Blues", 3.0, 2.5, 2.0);
    let gagaku = TraditionEmbedding::from_dial("Gagaku", 1.5, 1.8, 1.0);

    println!("🎵 Custom tradition embeddings:");
    for t in [&jazz, &blues, &gagaku] {
        let norm: f64 = t.coords.iter().map(|v| v * v).sum::<f64>().sqrt();
        println!("   {} → [{:.3}, {:.3}, {:.3}] (norm = {:.4})",
            t.name, t.coords[0], t.coords[1], t.coords[2], norm);
    }

    // Compare distances
    let ball = PoincareBall::unit();
    println!("\n📏 Pairwise distances:");
    println!("   Jazz ↔ Blues:  {:.4}  (close!)", ball.distance(&jazz.coords, &blues.coords));
    println!("   Jazz ↔ Gagaku: {:.4}  (far!)", ball.distance(&jazz.coords, &gagaku.coords));

    // Standard traditions
    let traditions = TraditionEmbedding::standard_traditions();
    println!("\n📚 {} built-in traditions:", traditions.len());
    for t in &traditions {
        print!("   {}", t.name);
    }
    println!();

    // Find nearest neighbor
    let query = &jazz;
    let mut best_name = "";
    let mut best_dist = f64::INFINITY;
    for t in &traditions {
        let d = ball.distance(&query.coords, &t.coords);
        if d < best_dist {
            best_dist = d;
            best_name = &t.name;
        }
    }
    println!("\n🔍 Nearest standard tradition to custom Jazz: {} (dist = {:.4})", best_name, best_dist);
}

// ── Lesson 7 ────────────────────────────────────────────────────────
fn lesson_7_riemannian_optimization() {
    separator("7. Riemannian Optimization — Centroids in Curved Space");

    println!("Riemannian gradient descent finds the Fréchet mean (centroid)");
    println!("in hyperbolic space — the point minimizing total distance.\n");

    let ball = PoincareBall::unit();
    let gd = RiemannianGD::new(ball, 0.01);

    // Create some tradition embeddings
    let traditions = vec![
        TraditionEmbedding::new("Alpha", vec![0.3, 0.1]),
        TraditionEmbedding::new("Beta", vec![0.1, 0.3]),
        TraditionEmbedding::new("Gamma", vec![0.2, 0.2]),
    ];

    println!("📍 Traditions:");
    for t in &traditions {
        println!("   {} at [{:.1}, {:.1}]", t.name, t.coords[0], t.coords[1]);
    }

    // Find the centroid
    let centroid = gd.centroid(&traditions);
    println!("\n🎯 Hyperbolic centroid: [{:.6}, {:.6}]", centroid[0], centroid[1]);

    // Distances from centroid to each tradition
    println!("\n📏 Distances from centroid:");
    for t in &traditions {
        let d = gd.ball.distance(&centroid, &t.coords);
        println!("   → {} : {:.6}", t.name, d);
    }

    // Compare with Euclidean average
    let eucl_avg: Vec<f64> = vec![
        traditions.iter().map(|t| t.coords[0]).sum::<f64>() / traditions.len() as f64,
        traditions.iter().map(|t| t.coords[1]).sum::<f64>() / traditions.len() as f64,
    ];
    println!("\n⚖️  Euclidean average: [{:.6}, {:.6}]", eucl_avg[0], eucl_avg[1]);
    println!("   Hyperbolic centroid accounts for curvature!");
}

// ── Lesson 8 ────────────────────────────────────────────────────────
fn lesson_8_embedding_search() {
    separator("8. Mini Embedding Search Engine");

    println!("Let's build a tiny search engine that finds the most similar");
    println!("traditions using hyperbolic distance and mobility.\n");

    let ball = PoincareBall::unit();
    let traditions = TraditionEmbedding::standard_traditions();

    // Query: something close to Electronic
    let query = TraditionEmbedding::from_dial("Query", 3.6, 3.9, 4.3);
    println!("🔍 Query: {} at [{:.3}, {:.3}, {:.3}]",
        query.name, query.coords[0], query.coords[1], query.coords[2]);

    // Rank by mobility (descending)
    let mut ranked: Vec<(&str, f64, f64)> = traditions.iter()
        .map(|t| {
            let d = ball.distance(&query.coords, &t.coords);
            let m = ball.mobility(&query.coords, &t.coords);
            (t.name.as_str(), d, m)
        })
        .collect();
    ranked.sort_by(|a, b| b.2.partial_cmp(&a.2).unwrap());

    println!("\n🏆 Search results (ranked by mobility):\n");
    println!("   {:<22} {:>12} {:>12}", "Tradition", "Distance", "Mobility");
    println!("   {}", "-".repeat(48));
    for (name, dist, mob) in &ranked {
        println!("   {:<22} {:>12.4} {:>12.4}", name, dist, mob);
    }

    // Geodesic path: show the midpoint between query and top result
    let top = ranked[0].0;
    let top_trad = traditions.iter().find(|t| t.name == top).unwrap();
    let midpoint = ball.midpoint(&query.coords, &top_trad.coords);
    println!("\n🛤️  Geodesic midpoint between Query and {}:", top);
    println!("   [{:.4}, {:.4}, {:.4}]", midpoint[0], midpoint[1], midpoint[2]);
    println!("\n✨ This midpoint could represent a 'blended' tradition!");
}
