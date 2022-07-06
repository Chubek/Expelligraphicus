pub mod points {
    use num_traits::NumOps;
    
    pub struct Point2d<T: NumOps> {
        pub x: T,
        pub y: T,
    }

    impl<T: NumOps>  Point2d<T> {
        pub fn new(x: T, y: T) -> Self {
            Self { x, y }
        }
    }

}

