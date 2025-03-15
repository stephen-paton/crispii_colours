pub mod colour_channel_intensity;
pub use colour_channel_intensity::ColourChannelIntensity;

pub mod opacity_channel_intensity;
pub use opacity_channel_intensity::OpacityChannelIntensity;

/// A struct representing an RGBA colour
#[derive(Debug, Clone, Copy)]
pub struct Colour {
    r: ColourChannelIntensity,
    g: ColourChannelIntensity,
    b: ColourChannelIntensity,
    a: OpacityChannelIntensity,
}

impl Colour {
    pub fn rgba(r: ColourChannelIntensity, g: ColourChannelIntensity, b: ColourChannelIntensity, a: OpacityChannelIntensity) -> Self {
        Self {
            r,
            g,
            b,
            a,
        }
    }

    pub fn rgb(r: ColourChannelIntensity, g: ColourChannelIntensity, b: ColourChannelIntensity) -> Self {
        Self {
            r,
            g,
            b,
            a: OpacityChannelIntensity::new(100).expect("legal value"),
        }
    }

    pub fn as_rgb_hex(&self) -> u32 {
        ((self.r.get_intensity() as u32) << 16)
        + ((self.g.get_intensity() as u32) << 8)
        + (self.b.get_intensity() as u32)
    }

    pub fn as_rgba_hex(&self) -> u32 {
        ((self.r.get_intensity() as u32) << 24)
        + ((self.g.get_intensity() as u32) << 16)
        + ((self.b.get_intensity() as u32) << 8)
        + (((self.a.get_intensity() as f32) * 0.01 * (255 as f32)).floor() as u32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_rgb_black() {
        let result = Colour::rgb(
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0x00)
        ).as_rgb_hex();
        assert_eq!(result, 0x00_00_00);
    }

    #[test]
    fn is_rgb_red() {
        let result = Colour::rgb(
            ColourChannelIntensity::new(0xFF),
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0x00)
        ).as_rgb_hex();
        assert_eq!(result, 0xFF_00_00);
    }

    #[test]
    fn is_rgb_green() {
        let result = Colour::rgb(
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0xFF),
            ColourChannelIntensity::new(0x00)
        ).as_rgb_hex();
        assert_eq!(result, 0x00_FF_00);
    }

    #[test]
    fn is_rgb_blue() {
        let result = Colour::rgb(
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0xFF)
        ).as_rgb_hex();
        assert_eq!(result, 0x00_00_FF);
    }

    #[test]
    fn is_rgb_white() {
        let result = Colour::rgb(
            ColourChannelIntensity::new(0xFF),
            ColourChannelIntensity::new(0xFF),
            ColourChannelIntensity::new(0xFF)
        ).as_rgb_hex();
        assert_eq!(result, 0xFF_FF_FF);
    }

    #[test]
    fn is_rgba_black_and_50_percent_opaque() {
        let result = Colour::rgba(
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0x00),
            ColourChannelIntensity::new(0x00),
            OpacityChannelIntensity::new(50).expect("legal value")
        ).as_rgba_hex();
        assert_eq!(result, 0x00_00_00_7F);
    }
}
