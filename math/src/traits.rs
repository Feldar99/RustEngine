use std::f32;

pub trait SquareRoot {
    type Output: Copy;

    fn sqrt(self) -> Self::Output;
}


impl SquareRoot for f32 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        self.sqrt()
    }
}
impl SquareRoot for f64 {
    type Output = f64;

    fn sqrt(self) -> f64 {
        self.sqrt()
    }
}
impl SquareRoot for i8 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        (self as f32).sqrt()
    }
}
impl SquareRoot for i16 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        (self as f32).sqrt()
    }
}
impl SquareRoot for i32 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        (self as f32).sqrt()
    }
}
impl SquareRoot for i64 {
    type Output = f64;

    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}
impl SquareRoot for u8 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        (self as f32).sqrt()
    }
}
impl SquareRoot for u16 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        (self as f32).sqrt()
    }
}
impl SquareRoot for u32 {
    type Output = f32;

    fn sqrt(self) -> f32 {
        (self as f32).sqrt()
    }
}
impl SquareRoot for u64 {
    type Output = f64;

    fn sqrt(self) -> f64 {
        (self as f64).sqrt()
    }
}