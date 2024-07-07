pub const SCREEN_WIDTH: usize = 160;
pub const SCREEN_HEIGHT: usize = 144;

/// GameBoy screen 160x144 pixels
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Screen([ScreenPixel; SCREEN_WIDTH * SCREEN_HEIGHT]);

impl Screen {
    pub fn new() -> Self {
        Self([ScreenPixel(0); SCREEN_WIDTH * SCREEN_HEIGHT])
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> ScreenPixel {
        assert!(x < SCREEN_WIDTH, "x out of bounds: {}", x);
        assert!(y < SCREEN_HEIGHT, "y out of bounds: {}", y);

        self.0[y * SCREEN_WIDTH + x]
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: ScreenPixel) {
        assert!(x < SCREEN_WIDTH, "x out of bounds: {}", x);
        assert!(y < SCREEN_HEIGHT, "y out of bounds: {}", y);

        self.0[y * SCREEN_WIDTH + x] = pixel;
    }
}

impl Default for Screen {
    fn default() -> Self {
        Self::new()
    }
}

/// Represents a single pixel on the screen.
///
/// 15 bit color depth.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenPixel(pub u16);

impl ScreenPixel {
    pub fn r(self) -> u8 {
        ((self.0 >> 10) & 0b11111) as u8
    }

    pub fn g(self) -> u8 {
        ((self.0 >> 5) & 0b11111) as u8
    }

    pub fn b(self) -> u8 {
        (self.0 & 0b11111) as u8
    }

    pub fn set_r(&mut self, value: u8) {
        self.0 = (self.0 & !(0b11111 << 10)) | ((value as u16) << 10);
    }

    pub fn set_g(&mut self, value: u8) {
        self.0 = (self.0 & !(0b11111 << 5)) | ((value as u16) << 5);
    }

    pub fn set_b(&mut self, value: u8) {
        self.0 = (self.0 & !0b11111) | value as u16;
    }
}

#[test]
fn test_pixel_color_functions() {
    let mut pixel = ScreenPixel(0);

    assert_eq!(pixel.r(), 0);
    assert_eq!(pixel.g(), 0);
    assert_eq!(pixel.b(), 0);

    pixel.set_r(31);

    assert_eq!(pixel.r(), 31);
    assert_eq!(pixel.g(), 0);
    assert_eq!(pixel.b(), 0);

    pixel.set_g(12);

    assert_eq!(pixel.r(), 31);
    assert_eq!(pixel.g(), 12);
    assert_eq!(pixel.b(), 0);

    pixel.set_b(3);

    assert_eq!(pixel.r(), 31);
    assert_eq!(pixel.g(), 12);
    assert_eq!(pixel.b(), 3);

    pixel.set_g(0);

    assert_eq!(pixel.r(), 31);
    assert_eq!(pixel.g(), 0);
    assert_eq!(pixel.b(), 3);
}
