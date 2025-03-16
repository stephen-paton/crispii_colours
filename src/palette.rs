use std::collections::HashMap;

use crate::Colour;

/// A Colour Palette consisting of { name: Colour } pairs
#[derive(Debug, Clone)]
pub struct Palette {
    colours: HashMap<String, Colour>
}

impl Palette {
    pub fn new() -> Self {
        Self {
            colours: HashMap::new(),
        }
    }

    /// Adds a new Colour to the Palette
    /// 
    /// If the provided name is already in use by another Colour, returns the Palette as is
    pub fn add_colour(mut self, name: &str, colour: Colour) -> Self {
        self.colours.entry(name.to_owned()).or_insert(colour);

        self
    }

    /// Updates an existing Colour in the Palette to a new one
    /// 
    /// If no such Colour can be found, returns the Palette as is 
    pub fn update_colour(mut self, name: &str, colour: Colour) -> Self {
        if let Some(stored_colour) = self.colours.get_mut(name) {
            *stored_colour = colour;
        }

        self
    }

    /// Removes an existing Colour from the Palette
    /// 
    /// If no such Colour can be found, returns the Palette as is
    pub fn remove_colour(mut self, name: &str) -> Self {
        self.colours.remove(name);

        self
    }

    /// Returns the length of the Palette i.e. the total number of Colours that it contains
    pub fn len(&self) -> usize {
        self.colours.len()
    }

    /// Retrieves the rgb hex value of the Colour (0xRR_GG_BB), or None if no such Colour exists 
    pub fn get_colour_rgb_hex(&self, name: &str) -> Option<u32> {
        Some(self.colours.get(name)?.as_rgb_hex())
    }

    /// Retrieves the rgba hex value of the Colour (0xRR_GG_BB_AA), or None of no such Colour exists
    pub fn get_colour_rgba_hex(&self, name: &str) -> Option<u32> {
        Some(self.colours.get(name)?.as_rgba_hex())
    }

    /// Retrieves the names of all existing Colours within the Palette
    pub fn get_colour_names(&self) -> Vec<&str> {
        self.colours.keys().map(|s| s.as_str()).collect()
    }
}

#[cfg(test)]
mod tests {
    use crate::ColourChannelIntensity;

    use super::*;

    #[test]
    fn add_colour() {
        let palette = Palette::new()
            .add_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFF),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            );

        let result = palette.get_colour_names();
        
        assert_eq!(result.len(), 1);
        assert_eq!(*result.get(0).unwrap(), "red");
    }

    #[test]
    fn try_add_duplicate_colour() {
        let palette = Palette::new()
            .add_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFF),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            )
            .add_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFF),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            );

        let result = palette.get_colour_names();

        assert_eq!(result.len(), 1);
        assert_eq!(*result.get(0).unwrap(), "red");
    }

    #[test]
    fn update_colour() {
        let result = Palette::new()
            .add_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFF),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            )
            .update_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFA),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            )
            .get_colour_rgb_hex("red").unwrap();

        assert_eq!(result, 0xFA_00_00);
    }

    #[test]
    fn try_update_non_existent_colour() {
        let palette = Palette::new()
            .add_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFF),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            )
            .update_colour(
                "pink",
                Colour::rgb(
                    ColourChannelIntensity::new(0xE0),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            );

        let result = palette.get_colour_names();

        assert_eq!(result.len(), 1);
        assert_eq!(*result.get(0).unwrap(), "red");
    }

    #[test]
    fn remove_colour() {
        let palette = Palette::new()
            .add_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFF),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            )
            .remove_colour("red");

        let result = palette.get_colour_names();

        assert_eq!(result.len(), 0);
    }

    #[test]
    fn try_remove_non_existent_colour() {
        let palette = Palette::new()
            .add_colour(
                "red",
                Colour::rgb(
                    ColourChannelIntensity::new(0xFF),
                    ColourChannelIntensity::new(0x00),
                    ColourChannelIntensity::new(0x00)
                )
            )
            .remove_colour("pink");

        let result = palette.get_colour_names();

        assert_eq!(result.len(), 1);
        assert_eq!(*result.get(0).unwrap(), "red");
    }
}
