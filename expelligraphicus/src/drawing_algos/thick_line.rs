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
        double_left: &dyn Fn(&dyn Fn(), i64, i64) -> f64,
        arg_left: &dyn Fn(),
        double_right: &dyn Fn(&dyn Fn(), i64, i64) -> f64,
        arg_right: &dyn Fn(),
        pstep: Point2I64)
        {
            let mut p_error = 0i64;
            let mut error = 0i64;
            let mut y = pt.y;
            let mut x = pt.x;
            let threshold = dp.x - 2 * dp.y;
            let e_diag = -2 * dp.x;
            let e_square = 2* dp.y;
            let length = dp.x + 1;
            let d = ((dp.x * dp.x + dp.y * dp.y) as f64).sqrt();
            
            
            for p in 0..length {
                let w_left = (double_left(arg_left, p, length) * 2.0 * d) as i64;
                let w_right = (double_right(arg_right, p, length) * 2.0 * d) as i64;
                x_perpendicular(
                    canvas, 
                    color, 
                    Point2I64::new(x, y),
                    Point2I64::new(dp.x, dp.y), 
                    pstep,
                    p_error, w_left, w_right, error
                );

                if error >= threshold {
                    y += step.y;
                    error += e_diag;
    
                    if p_error >= threshold {
                        x_perpendicular(
                            canvas, 
                            color, 
                            pt, 
                            dp, 
                            pstep, 
                            (p_error + e_diag + e_square), 
                            w_left, 
                            w_right,
                            error
                        );

                        p_error += e_diag;
                    }
                    p_error += e_square;
                }

                error += e_square;
                x += step.x;

            };

            
        
        }
        
        
}