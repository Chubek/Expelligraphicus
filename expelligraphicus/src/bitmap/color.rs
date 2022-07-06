pub mod color {
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub enum ColorType {
        RGB,
        RGBA,
    }


    
    #[derive(Clone, Copy, PartialEq, Eq, Debug)]
    pub struct Color {
        r: u8,
        g: u8,
        b: u8,
        a: Option<u8>,
        pub color_type: ColorType,
    }

    impl Color {
        pub fn new_rgb(r: u8, g: u8, b: u8) -> Self {
            let a = None;
            let color_type = ColorType::RGB;

            Self { r, g, b, a, color_type }
        }

        pub fn new_rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
            let a_some = Some(a);
            let color_type = ColorType::RGBA;

            Self { r, g, b, a: a_some, color_type }
        }

        pub fn new_black() -> Self {
            Self::new_rgb(0, 0, 0)
        }

        pub fn new_white() -> Self {
            Self::new_rgb(255, 255, 255)
        }

        pub fn unravel_rgb(&self) -> (u8, u8, u8) {
            (self.r, self.g, self.b)
        }

        pub fn unravel_rgba(&self) -> (u8, u8, u8, u8) {
            if self.a.is_none() {
                panic!("Color is not RGBA!");
            }

            let a = self.a.unwrap();

            (self.r, self.g, self.b, a)
        }
    }
}
