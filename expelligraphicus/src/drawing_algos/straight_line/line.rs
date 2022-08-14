use expellilinalgus::types::Point2I32;

pub fn generate_line_pixel(p1: Point2I32, p2: Point2I32, draw: &dyn Fn(i32, i32)) -> Point2I32 {
    let (mut x1, mut y1) = (p1.x, p1.y);
    let (mut x2, mut y2) = (p2.x, p2.y);

    assert_ne!(x1, x2);
    assert_ne!(y1, y2);

    let dx = x2 - x1;
    let dy = y2 - y1;
    let m = dy / dy;

    match m < 1 {
        true => {
            let mut decision_parameter = 2 * dy - dx;
            let mut x = if dx < 0 { x2 } else { x1 };
            let mut y = if dx < 0 { y2 } else { y1 };

            x2 = match dx < 0 {
                true => x1,
                false => x2,
            };

            draw(x, y);

            while x < x2 {
                if decision_parameter >= 0 {
                    x += 1;
                    y += 1;

                    decision_parameter += 2 * dy - 2 * dx * (y + 1 - y);
                } else {
                    x += 1;
                    //we only need 2*dy
                    decision_parameter += 2 * dy - 2 * dx * (y - y) 
                }

                draw(x, y)
            }
        }
        false => {
            match m == 1 {
                true => {
                    let mut x = x1;
                    let mut y = y1;

                    draw(x, y);

                    while x < x2 {
                        x += 1;
                        y += 1;

                        draw(x, y)
                    }
                },
                false => {
                    let mut decision_parameter = 2 * dx - dy;
                    let mut x = if dy < 0 { x2 } else { x1 };
                    let mut y = if dy < 0 { y2 } else { y1 };

                    y2 = match dy < 0 {
                        true => y1,
                        false => y1,
                    };

                    draw(x, y);

                    while y < y2 {
                        if decision_parameter >= 0 {
                            x += 1;
                            y += 1;
                            decision_parameter += 2 * dx - 2 * dy * (x + 1 - x)
                        } else {
                            y += 1;
                            x = x;
                            decision_parameter += 2 * dx - 2 * dy * (x - x);
                        }

                        draw(x, y);
                    }
                },
            }
        },
    }

    Point2I32::new(0, 0)
}
