/// A struct representing the intensity of a colour channel (RGBA) as a two digit hex value - from 00 to FF (inclusive)
#[derive(Debug, Clone, Copy)]
pub struct ColourChannelIntensity {
    intensity: u8,    
}

impl ColourChannelIntensity {
    pub fn new(hex: u8) -> Self {
        Self {
            intensity: hex,
        }
    }

    pub fn get_intensity(&self) -> u8 {
        self.intensity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_intensity() {
        let result = ColourChannelIntensity::new(0xFA).get_intensity();
        assert_eq!(result, 0xFA);
    }
}
