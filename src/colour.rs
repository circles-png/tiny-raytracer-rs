#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Colour {
    pub red: f32,
    pub green: f32,
    pub blue: f32,
}

impl Colour {
    pub fn new(red: f32, green: f32, blue: f32) -> Self {
        Self { red, green, blue }
    }

    pub fn from_hex(hex: u32) -> Self {
        Self {
            red: ((hex >> 16) & 0xff) as f32 / 255.,
            green: ((hex >> 8) & 0xff) as f32 / 255.,
            blue: (hex & 0xff) as f32 / 255.,
        }
    }

    pub fn as_hex(&self) -> u32 {
        ((self.red * 255.) as u32) << 16 | ((self.green * 255.) as u32) << 8 | (self.blue * 255.) as u32
    }

    pub fn as_rgb(self) -> [f32; 3] {
        [self.red, self.green, self.blue]
    }

    pub fn gray(value: f32) -> Self {
        Self::new(value, value, value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let colour = Colour::new(1., 1., 1.);
        assert_eq!(colour.red, 1.);
        assert_eq!(colour.green, 1.);
        assert_eq!(colour.blue, 1.);
    }

    #[test]
    fn test_from_hex() {
        let colour = Colour::from_hex(0xffffff);
        assert_eq!(colour.red, 1.);
        assert_eq!(colour.green, 1.);
        assert_eq!(colour.blue, 1.);
    }

    #[test]
    fn test_as_hex() {
        let colour = Colour::new(1., 1., 1.);
        assert_eq!(colour.as_hex(), 0xffffff);
    }

    #[test]
    fn test_as_rgb() {
        let colour = Colour::new(1., 1., 1.);
        assert_eq!(colour.as_rgb(), [1., 1., 1.]);
    }
}
