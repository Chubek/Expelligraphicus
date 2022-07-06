pub mod draw_line {
    use crate::{linear_algebra::types::types::Point2I64, bitmap::{bitmap::bitmap::Canvas, color::color::Color}};

    pub fn line_straight(
            p1: Point2I64, 
            p2: Point2I64,
            canvas: &mut Canvas,
            color: Color
        )
    {
        fn plot_line_low(
            p1: Point2I64, 
            p2: Point2I64,
            canvas: &mut Canvas,
            color: Color
        ) {
            let mut dx = p2.x - p1.x;
            let mut dy = p2.y - p1.y;
    
            let mut yi = 1i64;
    
            if dy < 0 {
                yi = -1;
                dy = -dy;
            }
    
            let mut d = (2 * dy) - dx;
            let mut y = p1.y.clone();
    
            for x in p1.x..p2.x {
                canvas.set_pixel(x, y, color);
    
                if d > 0 {
                    y += yi;
                    d += (2 * (dy - dx));
                } else {
                    d += 2 *- dy;
                }
            }
        }
    
        fn plot_line_high(
            p1: Point2I64, 
            p2: Point2I64,
            canvas: &mut Canvas,
            color: Color
        ) {
            let mut dx = p2.x - p1.x;
            let mut dy = p2.y - p1.y;
    
            let mut xi = 1i64;
    
            if dy < 0 {
                xi = -1;
                dx = -dx;
            }
    
            let mut d = (2 * dx) - dy;
            let mut x = p1.x.clone();
    
            for y in p1.y..p2.y {
                canvas.set_pixel(x, y, color);
    
                if d > 0 {
                    x += xi;
                    d += (2 * (dx - dy));
                } else {
                    d += 2 *- dx;
                }
            }
        }
    
        fn plot_line(
            p1: Point2I64, 
            p2: Point2I64,
            canvas: &mut Canvas,
            color: Color
        ) {
            if (p2.y - p1.y).abs() < (p2.x - p1.x).abs() {
                if p1.x > p2.x {
                    plot_line_low(p2, p1, canvas, color);
                } else {
                    plot_line_low(p1, p2, canvas, color);
                }
            }
            else {
                if p1.y > p2.y {
                    plot_line_high(p2, p1, canvas, color)
                } else {
                    plot_line_high(p1, p2, canvas, color)
                }
            }
        }

        plot_line(p1, p2, canvas, color);

    }

   
    
}