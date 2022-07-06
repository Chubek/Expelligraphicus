pub mod thick_line {
    use crate::{linear_algebra::types::types::Point2I64, bitmap::{bitmap::bitmap::Canvas, color::color::Color}};

    fn x_perpendicular
        (canvas: &mut Canvas,
        color: Color,
        pt: Point2I64,
        dp: Point2I64,
        step: Point2I64,
        eint: i64,
        w_left: i64,
        w_right: i64,
        winit: i64,
    ) 
    {
        let threshold  = dp.x - 2 * dp.y;
        let e_diag = -2*dp.x;
        let e_square = 2 * dp.y;
        let (mut p, mut q) = (0, 0);

        let mut y = pt.y;
        let mut x = pt.x;
        let mut error = eint.clone();
        let mut tk = dp.x - dp.y - winit;

        while tk <= w_left {
            canvas.set_pixel(x, y, color);

            if error >= threshold {
                x += step.x;
                error += e_diag;
                tk += 2*dp.y;
            
            }

            error += e_square;
            y += step.y;
            tk += 2 * dp.x;
            q += 1;
        }

        let mut y = pt.y;
        let mut x = pt.x;
        let mut error = eint.clone();
        let mut tk = dp.x - dp.y - winit;


        while tk <= w_right {
            if p > 0 {
                canvas.set_pixel(x, y, color);
            }   
    
            if error > threshold {         
                if error >= threshold {
                    x -= step.x;
                    error += e_diag;
                    tk += 2*dp.y;
                    
                }
        
                    error += e_square;
                    y -= step.y;
                    tk += 2 * dp.x;
                    p += 1;
            }
    
        }
        

        if (q == 0 && p < 2) { canvas.set_pixel(pt.x, pt.y, color); };

    }


    fn x_varthick_line(
        canvas: &mut Canvas,
        color: Color,
        pt: Point2I64,
        dp: Point2I64, 
        step: Point2I64,
        double_left: &dyn Fn(i64, i64) -> f64,
        arg_left: &dyn Fn(),
        double_right: &dyn Fn(i64, i64) -> f64,
        arg_right: &dyn Fn(),
        pxstep: i64,
        pystep: i64)
        {
            
        }

        
}