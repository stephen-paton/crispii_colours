mod channel_intensity;
pub use channel_intensity::ChannelIntensity;

pub use crispii_digits::Hex;

/// A struct representing an RGBA colour
#[derive(Clone, Copy, Debug)]
pub struct Colour {
    r: ChannelIntensity,
    g: ChannelIntensity,
    b: ChannelIntensity,
    a: ChannelIntensity,
}

impl Colour {
    pub fn rgba(r: ChannelIntensity, g: ChannelIntensity, b: ChannelIntensity, a: ChannelIntensity) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    pub fn rgb(r: ChannelIntensity, g: ChannelIntensity, b: ChannelIntensity) -> Self {
        Self {
            r,
            g,
            b,
            a: ChannelIntensity::new(Hex::F, Hex::F),
        }
    }
}

impl From<Colour> for u32 {
    fn from(value: Colour) -> Self {
        ((u8::from(value.r) as u32) << 24)
        + ((u8::from(value.g) as u32) << 16)
        + ((u8::from(value.b) as u32) << 8)
        + (u8::from(value.a) as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_black() {
        let result = u32::from(Colour::rgb(
            ChannelIntensity::new(Hex::Zero, Hex::Zero),
            ChannelIntensity::new(Hex::Zero, Hex::Zero),
            ChannelIntensity::new(Hex::Zero, Hex::Zero)
        ));
        assert_eq!(result, 0x00_00_00_FF);
    }

    #[test]
    fn is_red() {
        let result = u32::from(Colour::rgb(
            ChannelIntensity::new(Hex::F, Hex::F),
            ChannelIntensity::new(Hex::Zero, Hex::Zero),
            ChannelIntensity::new(Hex::Zero, Hex::Zero)
        ));
        assert_eq!(result, 0xFF_00_00_FF);
    }

    #[test]
    fn is_green() {
        let result = u32::from(Colour::rgb(
            ChannelIntensity::new(Hex::Zero, Hex::Zero),
            ChannelIntensity::new(Hex::F, Hex::F),
            ChannelIntensity::new(Hex::Zero, Hex::Zero)
        ));
        assert_eq!(result, 0x00_FF_00_FF);
    }

    #[test]
    fn is_blue() {
        let result = u32::from(Colour::rgb(
            ChannelIntensity::new(Hex::Zero, Hex::Zero),
            ChannelIntensity::new(Hex::Zero, Hex::Zero),
            ChannelIntensity::new(Hex::F, Hex::F)
        ));
        assert_eq!(result, 0x00_00_FF_FF);
    }

    #[test]
    fn is_white() {
        let result = u32::from(Colour::rgb(
            ChannelIntensity::new(Hex::F, Hex::F),
            ChannelIntensity::new(Hex::F, Hex::F),
            ChannelIntensity::new(Hex::F, Hex::F)
        ));
        assert_eq!(result, 0xFF_FF_FF_FF);
    }

    #[test]
    fn is_white_and_invisible() {
        let result = u32::from(Colour::rgba(
            ChannelIntensity::new(Hex::F, Hex::F),
            ChannelIntensity::new(Hex::F, Hex::F),
            ChannelIntensity::new(Hex::F, Hex::F),
            ChannelIntensity::new(Hex::Zero, Hex::Zero)
        ));
        assert_eq!(result, 0xFF_FF_FF_00);
    }
}
