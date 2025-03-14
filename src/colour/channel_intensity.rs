use crispii_digits::Hex;

/// A struct representing the intensity of a colour channel (RGBA) as a two digit hex value - from 00 to FF (inclusive)
#[derive(Clone, Copy, Debug)]
pub struct ChannelIntensity {
    digit2: Hex,
    digit1: Hex,
}

impl ChannelIntensity {
    pub fn new(digit2: Hex, digit1: Hex) -> Self {
        Self {
            digit2,
            digit1,
        }
    }
}

impl From<ChannelIntensity> for u8 {
    fn from(value: ChannelIntensity) -> Self {
        (u8::from(value.digit2) << 4) + u8::from(value.digit1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_from_legal_min() {
        let result = u8::from(ChannelIntensity::new(Hex::Zero, Hex::Zero));
        assert_eq!(result, 0);
    }

    #[test]
    fn u8_from_legal_max() {
        let result = u8::from(ChannelIntensity::new(Hex::F, Hex::F));
        assert_eq!(result, 255);
    }
}
