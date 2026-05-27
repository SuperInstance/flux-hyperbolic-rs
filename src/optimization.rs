use crate::poincare::PoincareBall;
use crate::tradition::TraditionEmbedding;

/// Riemannian gradient descent optimizer for hyperbolic embeddings.
///
/// Uses the Riemannian gradient: ∇_R = (1/λ_x²) * ∇_E
/// where λ_x is the conformal factor and ∇_E is the Euclidean gradient.
#[derive(Debug, Clone)]
pub struct RiemannianGD {
    pub ball: PoincareBall,
    pub learning_rate: f64,
    pub max_iterations: usize,
    pub tolerance: f64,
}

impl RiemannianGD {
    pub fn new(ball: PoincareBall, learning_rate: f64) -> Self {
        Self {
            ball,
            learning_rate,
            max_iterations: 1000,
            tolerance: 1e-6,
        }
    }

    /// Optimize a point to be close to a set of target points (minimize sum of distances).
    pub fn optimize_to_targets(&self, start: &[f64], targets: &[Vec<f64>]) -> Vec<f64> {
        let mut point = self.ball.project(start);
        let _c = self.ball.curvature;

        for _ in 0..self.max_iterations {
            let mut gradient = vec![0.0; point.len()];
            let lam_sq = self.ball.conformal_factor(&point).powi(2);

            for target in targets {
                // Numerical gradient of distance
                let eps = 1e-5;
                let base_dist = self.ball.distance(&point, target);
                for i in 0..point.len() {
                    let mut point_plus = point.clone();
                    point_plus[i] += eps;
                    let dist_plus = self.ball.distance(&point_plus, target);
                    gradient[i] += (dist_plus - base_dist) / eps;
                }
            }

            // Riemannian gradient
            let riem_grad: Vec<f64> = gradient.iter().map(|g| g / lam_sq).collect();

            // Update and project
            let step: Vec<f64> = riem_grad.iter().map(|g| self.learning_rate * g).collect();
            let new_point = self
                .ball
                .exp_map(&point, &step.iter().map(|s| -s).collect::<Vec<_>>());

            // Check convergence
            let movement: f64 = new_point
                .iter()
                .zip(point.iter())
                .map(|(a, b)| (a - b).powi(2))
                .sum::<f64>()
                .sqrt();
            point = new_point;
            if movement < self.tolerance {
                break;
            }
        }

        self.ball.project(&point)
    }

    /// Find the hyperbolic centroid (Frechet mean) of a set of traditions.
    pub fn centroid(&self, traditions: &[TraditionEmbedding]) -> Vec<f64> {
        let targets: Vec<Vec<f64>> = traditions.iter().map(|t| t.coords.clone()).collect();
        let start = &targets[0];
        self.optimize_to_targets(start, &targets)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_to_single_target() {
        let ball = PoincareBall::unit();
        let gd = RiemannianGD::new(ball, 0.01);
        let target = vec![0.5, 0.0];
        let start = vec![0.0, 0.0];
        let result = gd.optimize_to_targets(&start, &[target.clone()]);
        // Should move toward the target
        let dist_start = gd.ball.distance(&start, &target);
        let dist_result = gd.ball.distance(&result, &target);
        assert!(
            dist_result < dist_start,
            "Should move closer: {} -> {}",
            dist_start,
            dist_result
        );
    }

    #[test]
    fn test_centroid() {
        let ball = PoincareBall::unit();
        let gd = RiemannianGD::new(ball, 0.01);
        let traditions = vec![
            TraditionEmbedding::new("A", vec![0.3, 0.1]),
            TraditionEmbedding::new("B", vec![0.1, 0.3]),
        ];
        let centroid = gd.centroid(&traditions);
        // Centroid should be somewhere between the two
        let dist_a = gd.ball.distance(&centroid, &traditions[0].coords);
        let dist_b = gd.ball.distance(&centroid, &traditions[1].coords);
        let dist_ab = gd
            .ball
            .distance(&traditions[0].coords, &traditions[1].coords);
        assert!(dist_a < dist_ab);
        assert!(dist_b < dist_ab);
    }
}
