extern crate num;

use vector::Vec4;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;
use std::ops::DivAssign;
//use std::ops::Index;
//use std::ops::IndexMut;
//use traits::SquareRoot;
use num::Zero;

#[derive(Copy, Clone)]
pub struct Mat4<T: num::Num + Copy> {
    pub values:[Vec4<T>; 4]
}

impl<T: num::Num + Copy> Mat4<T> {

//    const WIDTH:usize = 4;
//    const HEIGHT:usize = 4;
}

impl<T: num::Num + Copy> Zero for Mat4<T> {
    fn zero() -> Mat4<T> {
        Mat4 {values: [Vec4::zero(), Vec4::zero(), Vec4::zero(), Vec4::zero()]}
    }

    fn is_zero(&self) -> bool {
        for i in 0..4 {
            if !self.values[i].is_zero() {
                return false;
            }
        }
        true
    }
}

impl<T, S, Out> Add<Mat4<S>> for Mat4<T>
    where T: num::Num + Copy + Add<S, Output=Out>,
          S: num::Num + Copy,
          Out: num::Num + Copy
{
    type Output = Mat4<Out>;

    fn add(self, rhs: Mat4<S>) -> Mat4<Out> {
        let mut result = Mat4::<Out>::zero();
        for i in 0..4 {
            result.values[i] = self.values[i] + rhs.values[i];
        }
        result
    }
}

impl<T, S> AddAssign<Mat4<S>> for Mat4<T>
    where T: num::Num + Copy + AddAssign<S>,
          S: num::Num + Copy
{
    fn add_assign(&mut self, rhs: Mat4<S>) {
        for i in 0..4 {
            self.values[i] += rhs.values[i];
        }
    }
}

impl<T, Out> Neg for Mat4<T>
    where T: num::Num + Copy + Neg<Output = Out>,
          Out: num::Num + Copy
{
    type Output = Mat4<Out>;

    fn neg(self) -> Mat4<Out> {
        let mut result = Mat4::<Out>::zero();
        for i in 0..4 {
            result.values[i] = -self.values[i];
        }
        result
    }
}

impl<T, S, Out> Sub<Mat4<S>> for Mat4<T>
    where T: num::Num + Copy + Sub<S, Output = Out>,
          S: num::Num + Copy,
          Out: num::Num + Copy
{
    type Output = Mat4<Out>;

    fn sub(self, rhs: Mat4<S>) -> Mat4<Out> {
        let mut result = Mat4::<Out>::zero();
        for i in 0..4 {
            result.values[i] = self.values[i] - rhs.values[i];
        }
        result
    }
}

impl<T, S> SubAssign<Mat4<S>> for Mat4<T>
    where T: num::Num + Copy + SubAssign<S>,
          S: num::Num + Copy
{
    fn sub_assign(&mut self, rhs: Mat4<S>) {
        for i in 0..4 {
            self.values[i] -= rhs.values[i];
        }
    }
}

impl<T, S, Out> Mul<S> for Mat4<T>
    where T: num::Num + Copy + Mul<S, Output = Out>,
          S: num::Num + Copy,
          Out: num::Num + Copy
{
    type Output = Mat4<Out>;

    fn mul(self, rhs: S) -> Mat4<Out> {
        let mut result = Mat4::<Out>::zero();
        for i in 0..4 {
            result.values[i] = self.values[i] * rhs;
        }
        result
    }
}

impl<T, S, Out> Mul<Vec4<S>> for Mat4<T>
    where T: num::Num + Copy + Mul<S, Output = Out>,
          S: num::Num + Copy,
          Out: num::Num + Copy
{
    type Output = Vec4<Out>;

    fn mul(self, rhs: Vec4<S>) -> Vec4<Out> {
        let mut v4 = Vec4::<Out>::zero();
        for i in 0..4 {
            v4[i] = self.values[i].dot(rhs);
        }
        v4
    }
}

impl<T, S, Out> Mul<Mat4<S>> for Mat4<T>
    where T: num::Num + Copy + Mul<S, Output = Out>,
          S: num::Num + Copy,
          Out: num::Num + Copy + AddAssign
{
    type Output = Mat4<Out>;

    fn mul(self, rhs: Mat4<S>) -> Mat4<Out> {
        let mut result = Mat4::<Out>::zero();
        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    result.values[row][col] += self.values[row][i] * rhs.values[i][col];
                }
            }
        }
        result
    }
}

impl<T, S, MulOut> MulAssign<Mat4<S>> for Mat4<T>
    where T: num::Num + Copy + Mul<S, Output = MulOut> + AddAssign<MulOut>,
          S: num::Num + Copy,
{
    fn mul_assign(&mut self, rhs: Mat4<S>) {
        for row in 0..4 {
            let mut row_values = Vec4::<T>::zero();
            for col in 0..4 {
                for i in 0..4 {
                    row_values[col] += self.values[row][i] * rhs.values[i][col];
                }
            }
            self.values[row] = row_values;
        }
    }
}

impl<T, S, Out> Div<S> for Mat4<T>
    where T: num::Num + Copy + Div<S, Output = Out>,
          S: Copy,
          Out: num::Num + Copy
{
    type Output = Mat4<Out>;

    fn div(self, rhs: S) -> Mat4<Out> {
        let mut result = Mat4::<Out>::zero();
        for i in 0..4 {
            result.values[i] = self.values[i] / rhs;
        }
        result
    }
}

impl<T, S> DivAssign<S> for Mat4<T>
    where T: num::Num + Copy + DivAssign<S>,
          S: Copy
{
    fn div_assign(&mut self, rhs: S) {
        for i in 0..4 {
            self.values[i] /= rhs;
        }
    }
}

