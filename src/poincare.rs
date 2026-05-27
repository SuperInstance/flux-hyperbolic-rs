use crate::error::HyperbolicError;

/// Poincaré ball model of hyperbolic space.
///
/// Points live inside the open ball B^n = {x : ||x|| < 1/sqrt(c)}.
/// Distance: d(u,v) = (1/sqrt(c)) * arccosh(1 + 2c||u-v||^2 / ((1-c||u||^2)(1-c||v||^2)))
#[derive(Debug, Clone)]
pub struct PoincareBall {
    pub curvature: f64,
}

impl PoincareBall {
    pub fn new(curvature: f64) -> Result<Self, HyperbolicError> {
        if curvature <= 0.0 {
            return Err(HyperbolicError::InvalidCurvature(curvature));
        }
        Ok(Self { curvature })
    }

    pub fn unit() -> Self {
        Self { curvature: 1.0 }
    }

    pub fn radius(&self) -> f64 {
        1.0 / self.curvature.sqrt()
    }

    /// Conformal factor λ_x = 2 / (1 - c||x||^2)
    pub fn conformal_factor(&self, x: &[f64]) -> f64 {
        let norm_sq = dot(x, x);
        2.0 / (1.0 - self.curvature * norm_sq)
    }

    /// Hyperbolic distance between two points.
    pub fn distance(&self, u: &[f64], v: &[f64]) -> f64 {
        let u_norm_sq = dot(u, u);
        let v_norm_sq = dot(v, v);
        let diff: Vec<f64> = u.iter().zip(v.iter()).map(|(a, b)| a - b).collect();
        let diff_norm_sq = dot(&diff, &diff);
        let denom = (1.0 - self.curvature * u_norm_sq) * (1.0 - self.curvature * v_norm_sq);
        if denom <= 0.0 {
            let u_proj = self.project(u);
            let v_proj = self.project(v);
            return self.distance(&u_proj, &v_proj);
        }
        let arg = 1.0 + 2.0 * self.curvature * diff_norm_sq / denom;
        (arg.max(1.0)).acosh() / self.curvature.sqrt()
    }

    /// Project a point back inside the ball.
    pub fn project(&self, x: &[f64]) -> Vec<f64> {
        let max_norm = self.radius() - 1e-5;
        let norm = x.iter().map(|v| v * v).sum::<f64>().sqrt();
        if norm >= max_norm {
            x.iter().map(|v| v * (max_norm / norm)).collect()
        } else {
            x.to_vec()
        }
    }

    /// Exponential map: tangent vector at `base` to a point on the manifold.
    pub fn exp_map(&self, base: &[f64], tangent: &[f64]) -> Vec<f64> {
        let base_norm_sq = dot(base, base);
        let lam = 2.0 / (1.0 - self.curvature * base_norm_sq);
        let t_norm = tangent.iter().map(|v| v * v).sum::<f64>().sqrt();
        if t_norm < 1e-15 {
            return base.to_vec();
        }
        let scale = (lam * t_norm / 2.0).tanh();
        let result: Vec<f64> = base
            .iter()
            .zip(tangent.iter())
            .map(|(b, t)| b + scale * t / t_norm)
            .collect();
        self.project(&result)
    }

    /// Logarithmic map: point on manifold to tangent vector at `base`.
    pub fn log_map(&self, base: &[f64], point: &[f64]) -> Vec<f64> {
        let diff: Vec<f64> = point.iter().zip(base.iter()).map(|(p, b)| p - b).collect();
        let diff_norm = diff.iter().map(|v| v * v).sum::<f64>().sqrt();
        if diff_norm < 1e-15 {
            return vec![0.0; base.len()];
        }
        let base_norm_sq = dot(base, base);
        let lam = 2.0 / (1.0 - self.curvature * base_norm_sq);
        let scale = (2.0 / lam) * diff_norm.atanh();
        diff.iter().map(|d| scale * d / diff_norm).collect()
    }

    /// Mobility: exp(-distance), in (0, 1]. Higher = closer.
    pub fn mobility(&self, u: &[f64], v: &[f64]) -> f64 {
        (-self.distance(u, v)).exp()
    }

    /// Möbius addition: x ⊕ y = ((1+2c<x,y>+c||y||^2)x + (1-c||x||^2)y) / (1+2c<x,y>+c^2||x||^2||y||^2)
    pub fn mobius_add(&self, x: &[f64], y: &[f64]) -> Vec<f64> {
        let x_norm_sq = dot(x, x);
        let y_norm_sq = dot(y, y);
        let xy = dot(x, y);
        let c = self.curvature;
        let denom = 1.0 + 2.0 * c * xy + c * c * x_norm_sq * y_norm_sq;
        let scale_x = (1.0 + 2.0 * c * xy + c * y_norm_sq) / denom;
        let scale_y = (1.0 - c * x_norm_sq) / denom;
        x.iter()
            .zip(y.iter())
            .map(|(xi, yi)| scale_x * xi + scale_y * yi)
            .collect()
    }

    /// Geodesic midpoint between two points.
    pub fn midpoint(&self, u: &[f64], v: &[f64]) -> Vec<f64> {
        let log_v = self.log_map(u, v);
        let half: Vec<f64> = log_v.iter().map(|v| v * 0.5).collect();
        self.exp_map(u, &half)
    }
}

fn dot(a: &[f64], b: &[f64]) -> f64 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_unit_ball() {
        let ball = PoincareBall::unit();
        assert_abs_diff_eq!(ball.radius(), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_invalid_curvature() {
        assert!(PoincareBall::new(0.0).is_err());
        assert!(PoincareBall::new(-1.0).is_err());
    }

    #[test]
    fn test_distance_zero() {
        let ball = PoincareBall::unit();
        let u = vec![0.1, 0.2];
        assert_abs_diff_eq!(ball.distance(&u, &u), 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_distance_symmetric() {
        let ball = PoincareBall::unit();
        let u = vec![0.1, 0.2];
        let v = vec![0.3, -0.1];
        assert_abs_diff_eq!(
            ball.distance(&u, &v),
            ball.distance(&v, &u),
            epsilon = 1e-10
        );
    }

    #[test]
    fn test_distance_positive() {
        let ball = PoincareBall::unit();
        let u = vec![0.0, 0.0];
        let v = vec![0.5, 0.0];
        assert!(ball.distance(&u, &v) > 0.0);
    }

    #[test]
    fn test_project_inside() {
        let ball = PoincareBall::unit();
        let x = vec![2.0, 2.0];
        let p = ball.project(&x);
        let norm = p.iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!(norm < 1.0);
    }

    #[test]
    fn test_exp_log_roundtrip() {
        let ball = PoincareBall::unit();
        let base = vec![0.1, 0.2];
        let tangent = vec![0.05, -0.03];
        let point = ball.exp_map(&base, &tangent);
        let recovered = ball.log_map(&base, &point);
        for (a, b) in tangent.iter().zip(recovered.iter()) {
            assert_abs_diff_eq!(a, b, epsilon = 1e-6);
        }
    }

    #[test]
    fn test_mobility_self() {
        let ball = PoincareBall::unit();
        let u = vec![0.1, 0.2];
        assert_abs_diff_eq!(ball.mobility(&u, &u), 1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_mobility_decreases() {
        let ball = PoincareBall::unit();
        let o = vec![0.0, 0.0];
        let near = vec![0.1, 0.0];
        let far = vec![0.5, 0.0];
        assert!(ball.mobility(&o, &near) > ball.mobility(&o, &far));
    }

    #[test]
    fn test_mobius_add_identity() {
        let ball = PoincareBall::unit();
        let x = vec![0.3, -0.2];
        let origin = vec![0.0, 0.0];
        let result = ball.mobius_add(&x, &origin);
        for (a, b) in x.iter().zip(result.iter()) {
            assert_abs_diff_eq!(a, b, epsilon = 1e-10);
        }
    }

    #[test]
    fn test_midpoint_between_origin_and_point() {
        let ball = PoincareBall::unit();
        let o = vec![0.0, 0.0];
        let p = vec![0.5, 0.0];
        let mid = ball.midpoint(&o, &p);
        assert!(mid[0] > 0.0 && mid[0] < 0.5);
    }

    #[test]
    fn test_conformal_factor_at_origin() {
        let ball = PoincareBall::unit();
        let o = vec![0.0, 0.0];
        assert_abs_diff_eq!(ball.conformal_factor(&o), 2.0, epsilon = 1e-10);
    }

    #[test]
    fn test_custom_curvature() {
        let ball = PoincareBall::new(2.0).unwrap();
        assert_abs_diff_eq!(ball.radius(), 1.0 / 2.0_f64.sqrt(), epsilon = 1e-10);
        let u = vec![0.0, 0.0];
        let v = vec![0.1, 0.0];
        assert!(ball.distance(&u, &v) > 0.0);
    }
}
