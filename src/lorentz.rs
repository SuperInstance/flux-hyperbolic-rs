/// Lorentz (hyperboloid) model of hyperbolic space.
///
/// Points live on the upper sheet of the hyperboloid:
///   L^n = {x ∈ R^{n+1} : <x,x>_L = -1/c, x_0 > 0}
/// where <x,y>_L = -x_0*y_0 + sum(x_i*y_i) is the Minkowski inner product.

#[derive(Debug, Clone)]
pub struct LorentzModel {
    pub curvature: f64,
}

impl LorentzModel {
    pub fn new(curvature: f64) -> Result<Self, crate::error::HyperbolicError> {
        if curvature <= 0.0 {
            return Err(crate::error::HyperbolicError::InvalidCurvature(curvature));
        }
        Ok(Self { curvature })
    }

    pub fn unit() -> Self {
        Self { curvature: 1.0 }
    }

    /// Minkowski inner product: -x[0]*y[0] + sum(x[i]*y[i])
    pub fn minkowski_dot(&self, x: &[f64], y: &[f64]) -> f64 {
        -x[0] * y[0]
            + x[1..]
                .iter()
                .zip(y[1..].iter())
                .map(|(a, b)| a * b)
                .sum::<f64>()
    }

    /// Distance in the Lorentz model.
    /// d(x, y) = (1/sqrt(c)) * arccosh(-c * <x,y>_L)
    pub fn distance(&self, x: &[f64], y: &[f64]) -> f64 {
        let mink = self.minkowski_dot(x, y);
        let arg = (-self.curvature * mink).max(1.0);
        arg.acosh() / self.curvature.sqrt()
    }

    /// Embed a Euclidean point onto the hyperboloid.
    /// Given z ∈ R^n, construct x = [sqrt(1/c + ||z||^2), z_1, ..., z_n]
    pub fn embed(&self, z: &[f64]) -> Vec<f64> {
        let norm_sq: f64 = z.iter().map(|v| v * v).sum();
        let x0 = (1.0 / self.curvature + norm_sq).sqrt();
        let mut result = Vec::with_capacity(z.len() + 1);
        result.push(x0);
        result.extend_from_slice(z);
        result
    }

    /// Project back to the hyperboloid (normalize).
    pub fn project(&self, x: &[f64]) -> Vec<f64> {
        let norm = (-self.minkowski_dot(x, x) * self.curvature).abs().sqrt();
        if norm < 1e-15 {
            let mut origin = vec![0.0; x.len()];
            origin[0] = 1.0 / self.curvature.sqrt();
            return origin;
        }
        x.iter().map(|v| v / norm).collect()
    }

    /// Lorentzian exponential map at base point.
    pub fn exp_map(&self, base: &[f64], tangent: &[f64]) -> Vec<f64> {
        let c = self.curvature;
        let t_norm_sq = -c * self.minkowski_dot(tangent, tangent);
        if t_norm_sq < 1e-15 {
            return base.to_vec();
        }
        let t_norm = t_norm_sq.sqrt();
        let scale1 = (t_norm * c.sqrt()).cosh() / t_norm;
        let scale2 = (t_norm * c.sqrt()).sinh() / (t_norm * c.sqrt());
        base.iter()
            .zip(tangent.iter())
            .map(|(b, t)| scale1 * b + scale2 * t)
            .collect()
    }

    /// Extract the Euclidean coordinates (drop x_0).
    pub fn to_euclidean<'a>(&self, x: &'a [f64]) -> &'a [f64] {
        &x[1..]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use approx::*;

    #[test]
    fn test_embed_origin() {
        let model = LorentzModel::unit();
        let z = vec![0.0, 0.0];
        let x = model.embed(&z);
        assert_abs_diff_eq!(x[0], 1.0, epsilon = 1e-10);
        assert_abs_diff_eq!(x[1], 0.0, epsilon = 1e-10);
    }

    #[test]
    fn test_minkowski_dot_on_hyperboloid() {
        let model = LorentzModel::unit();
        let z = vec![0.3, -0.2];
        let x = model.embed(&z);
        let mdot = model.minkowski_dot(&x, &x);
        assert_abs_diff_eq!(mdot, -1.0, epsilon = 1e-10);
    }

    #[test]
    fn test_distance_zero() {
        let model = LorentzModel::unit();
        let z = vec![0.3, -0.2];
        let x = model.embed(&z);
        assert_abs_diff_eq!(model.distance(&x, &x), 0.0, epsilon = 1e-8);
    }

    #[test]
    fn test_distance_positive() {
        let model = LorentzModel::unit();
        let a = model.embed(&[0.1, 0.0]);
        let b = model.embed(&[0.5, 0.0]);
        assert!(model.distance(&a, &b) > 0.0);
    }

    #[test]
    fn test_project() {
        let model = LorentzModel::unit();
        let z = vec![0.3, 0.4];
        let x = model.embed(&z);
        let scaled: Vec<f64> = x.iter().map(|v| v * 2.0).collect();
        let proj = model.project(&scaled);
        let mdot = model.minkowski_dot(&proj, &proj);
        assert_abs_diff_eq!(mdot, -1.0, epsilon = 1e-8);
    }

    #[test]
    fn test_to_euclidean() {
        let model = LorentzModel::unit();
        let z = vec![0.3, 0.4];
        let x = model.embed(&z);
        let eucl = model.to_euclidean(&x);
        assert_eq!(eucl.len(), 2);
        assert_abs_diff_eq!(eucl[0], 0.3, epsilon = 1e-10);
    }
}
