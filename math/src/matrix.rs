extern crate num;

use vector::Vec4;
use vector::Vec3;
use vector::Vec2;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::SubAssign;
use std::ops::DivAssign;
use std::ops::Index;
use std::ops::IndexMut;
use std::fmt::Debug;
//use traits::SquareRoot;
use num::Zero;
use num::One;
use num::Float;

macro_rules! def_matrix {
    ($matrix_type:ident, $row_type:ident, $coord_vec_type:ident, $rows:expr) => {
        #[derive(Copy, Clone)]
        pub struct $matrix_type<T> where T: num::Num + Copy {
            pub values: [$row_type<T>; $rows]
        }

        impl<T> $matrix_type<T> where T: num::Num + Copy {
            pub fn scale(scale: &$coord_vec_type<T>) -> $matrix_type<T> {
                let mut result = $matrix_type::<T>::one();
                for i in 0..($rows-1) {
                    result[i][i] = scale[i];
                }
                result
            }

            pub fn translate(scale: &$coord_vec_type<T>) -> $matrix_type<T> {
                let mut result = $matrix_type::<T>::one();
                for i in 0..($rows-1) {
                    result[i][$rows - 1] = scale[i];
                }
                result
            }
        }

        impl<T> Zero for $matrix_type<T> where T: num::Num + Copy {
            fn zero() -> $matrix_type<T> {
                $matrix_type { values: [$row_type::zero(); $rows] }
            }

            fn is_zero(&self) -> bool {
                for i in 0..$rows {
                    if !self.values[i].is_zero() {
                        return false;
                    }
                }
                true
            }
        }

        impl<T> One for $matrix_type<T>
            where T: num::Num + Copy + One
        {
            fn one() -> $matrix_type<T> {
                let mut result = $matrix_type::<T>::zero();
                for i in 0..$rows {
                    result[i][i] = T::one();
                }
                result
            }
        }

        impl<T, S, Out> Add<$matrix_type<S>> for $matrix_type<T>
            where T: num::Num + Copy + Add<S, Output=Out>,
                  S: num::Num + Copy,
                  Out: num::Num + Copy
        {
            type Output = $matrix_type<Out>;

            fn add(self, rhs: $matrix_type<S>) -> $matrix_type<Out> {
                let mut result = $matrix_type::<Out>::zero();
                for i in 0..$rows {
                    result.values[i] = self.values[i] + rhs.values[i];
                }
                result
            }
        }

        impl<T, S> AddAssign<$matrix_type<S>> for $matrix_type<T>
            where T: num::Num + Copy + AddAssign<S>,
                  S: num::Num + Copy
        {
            fn add_assign(&mut self, rhs: $matrix_type<S>) {
                for i in 0..$rows {
                    self.values[i] += rhs.values[i];
                }
            }
        }

        impl<T, Out> Neg for $matrix_type<T>
            where T: num::Num + Copy + Neg<Output=Out>,
                  Out: num::Num + Copy
        {
            type Output = $matrix_type<Out>;

            fn neg(self) -> $matrix_type<Out> {
                let mut result = $matrix_type::<Out>::zero();
                for i in 0..$rows {
                    result.values[i] = -self.values[i];
                }
                result
            }
        }

        impl<T, S, Out> Sub<$matrix_type<S>> for $matrix_type<T>
            where T: num::Num + Copy + Sub<S, Output=Out>,
                  S: num::Num + Copy,
                  Out: num::Num + Copy
        {
            type Output = $matrix_type<Out>;

            fn sub(self, rhs: $matrix_type<S>) -> $matrix_type<Out> {
                let mut result = $matrix_type::<Out>::zero();
                for i in 0..$rows {
                    result.values[i] = self.values[i] - rhs.values[i];
                }
                result
            }
        }

        impl<T, S> SubAssign<$matrix_type<S>> for $matrix_type<T>
            where T: num::Num + Copy + SubAssign<S>,
                  S: num::Num + Copy
        {
            fn sub_assign(&mut self, rhs: $matrix_type<S>) {
                for i in 0..$rows {
                    self.values[i] -= rhs.values[i];
                }
            }
        }

        impl<T, S, Out> Mul<S> for $matrix_type<T>
            where T: num::Num + Copy + Mul<S, Output=Out>,
                  S: num::Num + Copy,
                  Out: num::Num + Copy
        {
            type Output = $matrix_type<Out>;

            fn mul(self, rhs: S) -> $matrix_type<Out> {
                let mut result = $matrix_type::<Out>::zero();
                for i in 0..$rows {
                    result.values[i] = self.values[i] * rhs;
                }
                result
            }
        }

        impl<T, S, Out> Mul<$row_type<S>> for $matrix_type<T>
            where T: num::Num + Copy + Mul<S, Output=Out>,
                  S: num::Num + Copy,
                  Out: num::Num + Copy
        {
            type Output = $row_type<Out>;

            fn mul(self, rhs: $row_type<S>) -> $row_type<Out> {
                let mut v = $row_type::<Out>::zero();
                for i in 0..$rows {
                    v[i] = self.values[i].dot(rhs);
                }
                v
            }
        }

        impl<T> Mul<$matrix_type<T>> for $matrix_type<T>
            where T: num::Num + Copy
        {
            type Output = $matrix_type<T>;

            fn mul(self, rhs: $matrix_type<T>) -> $matrix_type<T> {
                let mut result = $matrix_type::<T>::zero();
                for row in 0..$rows {
                    for col in 0..$rows {
                        for i in 0..$rows {
                            result.values[row][col] = result.values[row][col] +
                                (self.values[row][i] * rhs.values[i][col]);
                        }
                    }
                }
                result
            }
        }

        impl<T, S, MulOut> MulAssign<$matrix_type<S>> for $matrix_type<T>
            where T: num::Num + Copy + Mul<S, Output=MulOut> + AddAssign<MulOut>,
                  S: num::Num + Copy,
        {
            fn mul_assign(&mut self, rhs: $matrix_type<S>) {
                for row in 0..$rows {
                    let mut row_values = $row_type::<T>::zero();
                    for col in 0..$rows {
                        for i in 0..$rows {
                            row_values[col] += self.values[row][i] * rhs.values[i][col];
                        }
                    }
                    self.values[row] = row_values;
                }
            }
        }

        impl<T, S, Out> Div<S> for $matrix_type<T>
            where T: num::Num + Copy + Div<S, Output=Out>,
                  S: Copy,
                  Out: num::Num + Copy
        {
            type Output = $matrix_type<Out>;

            fn div(self, rhs: S) -> $matrix_type<Out> {
                let mut result = $matrix_type::<Out>::zero();
                for i in 0..$rows {
                    result.values[i] = self.values[i] / rhs;
                }
                result
            }
        }

        impl<T, S> DivAssign<S> for $matrix_type<T>
            where T: num::Num + Copy + DivAssign<S>,
                  S: Copy
        {
            fn div_assign(&mut self, rhs: S) {
                for i in 0..$rows {
                    self.values[i] /= rhs;
                }
            }
        }

        impl<T> Index<usize> for $matrix_type<T> where T: num::Num + Copy {
            type Output = $row_type<T>;

            fn index(&self, index: usize) -> &$row_type<T> {
                &self.values[index]
            }
        }

        impl<T> Index<(usize, usize)> for $matrix_type<T> where T: num::Num + Copy {
            type Output = T;

            fn index(&self, index: (usize, usize)) -> &T {
                &self.values[index.0][index.1]
            }
        }

        impl<T> IndexMut<usize> for $matrix_type<T> where T: num::Num + Copy {
            fn index_mut(&mut self, index: usize) -> &mut $row_type<T> {
                &mut self.values[index]
            }
        }

        impl<T> IndexMut<(usize, usize)> for $matrix_type<T> where T: num::Num + Copy {
            fn index_mut(&mut self, index: (usize, usize)) -> &mut T {
                &mut self.values[index.0][index.1]
            }
        }

        impl<T> $matrix_type<T> where T: num::Num + Copy {
            pub fn tensor(v: &$row_type<T>, w: &$row_type<T>) -> $matrix_type<T> {
                let mut result = $matrix_type::<T>::zero();
                for row in 0..$rows {
                    for col in 0..$rows {
                        result[row][col] = v[row] * w[col];
                    }
                }
                result
            }
        }

        impl<T> $matrix_type<T> where T: num::Num + Copy {
            pub fn transpose (&mut self) {
                for row in 0..$rows {
                    for col in 0..row + 1 {
                        let temp = self[row][col];
                        self[row][col] = self[col][row];
                        self[col][row] = temp;
                    }
                }
            }

            pub fn transposed (&self) -> $matrix_type<T> {
                let mut result = $matrix_type::<T>::zero();
                for row in 0..$rows {
                    for col in 0..row + 1 {
                        result[row][col] = self[col][row];
                        result[col][row] = self[row][col];
                    }
                }
                result
            }
        }
    }
}
def_matrix!(Mat4, Vec4, Vec3, 4);
def_matrix!(Mat3, Vec3, Vec2, 3);

impl<T> Mat3<T> where T:num::Num + Copy + Neg<Output=T> {

    pub fn cross_product(v : &Vec3<T>) -> Mat3<T> {
        Mat3::<T> { values: [Vec3::<T>{x: T::zero(), y: -v.z,      z:  v.y     },
                             Vec3::<T>{x:  v.z,      y: T::zero(), z: -v.x     },
                             Vec3::<T>{x: -v.y,      y:  v.x,      z: T::zero()}]
                  }
    }
}

impl<T> Mat4<T> where T: Float + Copy + AddAssign<T> + Debug{

    pub fn rotate(axis: &Vec3<T>, angle:T) -> Mat4<T> {

        debug_assert!(axis.is_unit());
        let cos = angle.to_radians().cos();
        let sin = angle.to_radians().sin();
        let mut result3 = Mat3::<T>::one() * cos;
        result3 += Mat3::<T>::cross_product(axis) * sin;
        result3 += Mat3::<T>::tensor(axis, axis) * (T::one() - cos);
        let mut result4 = Mat4::<T>::zero();
        for row in 0..3 {
            for col in 0..3 {
                result4[row][col] = result3[row][col];
            }
        }
        result4[3][3] = T::one();
        result4
    }
}