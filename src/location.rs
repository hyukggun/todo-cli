#[derive(Debug)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
}

impl Location {
    pub fn new(latitude: f64, longitude: f64) -> Self {
        Self {
            latitude,
            longitude,
        }
    }

    pub fn from_strings(latitude: &str, longitude: &str) -> Result<Self, std::num::ParseFloatError> {
        Ok(Self::new(
            latitude.trim().parse()?,
            longitude.trim().parse()?,
        ))
    }
}
