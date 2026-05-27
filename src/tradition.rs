use crate::poincare::PoincareBall;

/// Named tradition with its hyperbolic embedding.
#[derive(Debug, Clone)]
pub struct TraditionEmbedding {
    pub name: String,
    pub coords: Vec<f64>,
}

impl TraditionEmbedding {
    pub fn new(name: &str, coords: Vec<f64>) -> Self {
        Self {
            name: name.to_string(),
            coords,
        }
    }

    /// Embed from a dial position (harmonic, rhythmic, spectral) into the Poincaré ball.
    /// Maps (h, r, s) -> normalized 3D vector scaled to stay inside the ball.
    pub fn from_dial(name: &str, harmonic: f64, rhythmic: f64, spectral: f64) -> Self {
        let raw = vec![harmonic / 5.0, rhythmic / 5.0, spectral / 5.0];
        let ball = PoincareBall::unit();
        let coords = ball.project(&raw);
        Self {
            name: name.to_string(),
            coords,
        }
    }

    /// Well-known traditions with their approximate dial positions.
    pub fn standard_traditions() -> Vec<Self> {
        let traditions = [
            ("Jazz", 3.2, 2.8, 2.5),
            ("Classical", 1.8, 1.2, 1.5),
            ("Rock", 3.5, 3.8, 3.0),
            ("Blues", 3.0, 2.5, 2.0),
            ("Electronic", 3.8, 4.0, 4.5),
            ("Hindustani", 2.5, 3.2, 1.8),
            ("Gamelan", 2.0, 3.5, 2.2),
            ("Gagaku", 1.5, 1.8, 1.0),
            ("WestAfrican", 2.8, 4.2, 2.8),
            ("FreeImprovisation", 4.0, 3.5, 3.8),
        ];
        traditions
            .iter()
            .map(|(n, h, r, s)| Self::from_dial(n, *h, *r, *s))
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_dial_inside_ball() {
        let t = TraditionEmbedding::from_dial("Test", 2.5, 2.5, 2.5);
        let norm = t.coords.iter().map(|v| v * v).sum::<f64>().sqrt();
        assert!(norm < 1.0);
    }

    #[test]
    fn test_standard_traditions() {
        let traditions = TraditionEmbedding::standard_traditions();
        assert_eq!(traditions.len(), 10);
        assert_eq!(traditions[0].name, "Jazz");
    }

    #[test]
    fn test_tradition_distances() {
        let ball = PoincareBall::unit();
        let traditions = TraditionEmbedding::standard_traditions();
        // Jazz should be closer to Blues than to Gagaku
        let jazz = &traditions[0];
        let blues = &traditions[3];
        let gagaku = &traditions[7];
        let d_jb = ball.distance(&jazz.coords, &blues.coords);
        let d_jg = ball.distance(&jazz.coords, &gagaku.coords);
        assert!(
            d_jb < d_jg,
            "Jazz-Blues ({}) should be closer than Jazz-Gagaku ({})",
            d_jb,
            d_jg
        );
    }
}
