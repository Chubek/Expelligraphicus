#![allow(unused)]

pub mod bitmap;
pub mod drawing_algos;
pub mod linear_algebra;

#[cfg(test)]
mod tests {
    use crate::{linear_algebra::types::types::Point2I64, bitmap::{bitmap::bitmap::Canvas, color::color::Color, context::context::render_to_screen}, drawing_algos::draw_line::draw_line::line_straight};

    #[test]
    fn test_draw_line() {
        let p1 = Point2I64::new(10, 20);
        let p2 = Point2I64::new(50, 90);
            
        let mut canvas = Canvas::new(
            400, 400, 
            Color::new_rgb(250, 23, 140));
    
        let line_color = Color::new_black();
    
        line_straight(p1, p2, &mut canvas, line_color);
    
        render_to_screen(canvas);
    }
}
