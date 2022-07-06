pub mod bitmap {
    use crate::bitmap::color::color::Color;

    pub type CanvasType = Vec<Vec<Color>>;

    #[derive(Debug)]
    pub enum ImageError {
        ErrorGettingPixel,
        ErrorGettingRow,
    }

    pub type ResultSet = std::result::Result<(), ImageError>;

    #[derive(Clone)]
    pub struct Canvas(CanvasType);

    impl Canvas {
        pub fn new(width: usize, height: usize, init_value: Color) -> Self {
            let ys = vec![init_value; height];
            let xs = vec![ys; width];

            Canvas(xs)
        }

        pub fn set_pixel(&mut self, xi32: i64, yi32: i64, value: Color) -> ResultSet {
            let (xi, yi) = (xi32 as usize, yi32 as usize);

            let Canvas(canvas) = self;

            let y = canvas.get_mut(yi);

            match y {
                Some(xs) => {
                    let x = xs.get_mut(xi);

                    match x {
                        Some(pixel) => {
                            *pixel = value;

                            Ok(())
                        }
                        None => Err(ImageError::ErrorGettingPixel),
                    }
                }
                None => Err(ImageError::ErrorGettingRow),
            }
        }

        pub fn get_pixel_impl(&self, x: usize, y: usize) -> Color {
            let Canvas(canvas) = self;

            canvas[x][y].clone()
        }

        pub fn get_size(&self) -> (usize, usize) {
            let Canvas(canvas) = self;

            let h = canvas.len();
            let w = canvas[0].len();

            (w, h)
        }

        pub fn get_map(&self) -> CanvasType {
            let Canvas(canvas) = self;

            canvas.to_vec()
        }
    }
}
