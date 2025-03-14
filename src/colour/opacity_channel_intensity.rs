use crispii_errors::{CrispiiError, InvalidArgumentError};

#[derive(Clone, Copy, Debug)]
pub struct OpacityChannelIntensity {
    intensity: u8,
}

impl OpacityChannelIntensity {
    pub fn new(percentage: u8) -> Result<Self, Box<dyn CrispiiError>> {
        if percentage > 100 {
            return Err(Box::new(InvalidArgumentError::new("opacity", "Must be between 0 and 100 (inclusive)")));
        }

        Ok(Self {
            intensity: percentage,
        })
    }

    pub fn get_intensity(&self) -> u8 {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_intensity_valid() {
        let result = OpacityChannelIntensity::new(100).expect("within valid range").get_intensity();
        assert_eq!(result, 100);
    }

    #[test]
    #[should_panic]
    fn new_invalid() {
        let result = OpacityChannelIntensity::new(101).expect("outside of valid range").get_intensity();
        assert_eq!(result, 101);
    }
}
