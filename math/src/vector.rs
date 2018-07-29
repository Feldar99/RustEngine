extern crate std;
extern crate num;

use std::ops::Add;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;
use traits::SquareRoot;

#[derive(Copy, Clone)]
pub struct Vec4<T: Copy>{
    x:T,
    y:T,
    z:T,
    w:T,
}

impl<T: Copy, S: Copy, Out: Copy> Add<Vec4<S>> for Vec4<T> where T:Add<S, Output=Out> {
    type Output = Vec4<Out>;

    fn add(self, other: Vec4<S>) -> Vec4<Out> {
        Vec4 { x: self.x + other.x,
               y: self.y + other.y,
               z: self.z + other.z,
               w: self.w + other.w}
    }
}

impl<T: Copy, S: Copy> AddAssign<Vec4<S>> for Vec4<T> where T:AddAssign<S> {
    fn add_assign(&mut self, rhs: Vec4<S>) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl<T: Copy> Vec4<T>{

    pub fn dot<S: Copy, Out>(self, rhs: Vec4<S>) -> Out
        where T:Mul<S, Output = Out>, Out:Add<Output = Out>
    {
        self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z +
            self.w * rhs.w
    }
}

impl<T: Copy, S: Copy, Out: Copy> Mul<S> for Vec4<T> where T:Mul<S, Output = Out> {
    type Output = Vec4<Out>;

    fn mul(self, rhs: S) -> Vec4<Out> {
        Vec4 { x: self.x * rhs,
               y: self.y * rhs,
               z: self.z * rhs,
               w: self.w * rhs}
    }
}

//macro_rules! scalar_times_vector {
//    ($scalar_type:ident, $vector_type:ident, {$($component:ident),+}) => {
//        impl<T:Copy, Out: Copy> Mul<$vector_type<T>> for $scalar_type
//            where $scalar_type:Mul<T, Output = Out>, T:num::Num
//        {
//            type Output = $vector_type<Out>;
//
//            fn mul(self, rhs: Vec4<T>) -> $vector_type<Out> {
//                $vector_type {
//                    $($component: self * rhs.$component),+
//                }
//            }
//        }
//    }
//}
//scalar_times_vector!(f32, Vec4, {x, y, z, w});

impl<T:Copy, Out: Copy> Mul<Vec4<T>> for f32 where f32:Mul<T, Output = Out>, T:num::Num {
    type Output = Vec4<Out>;

    fn mul(self, rhs: Vec4<T>) -> Vec4<Out> {
        Vec4 { x: self * rhs.x,
               y: self * rhs.y,
               z: self * rhs.z,
               w: self * rhs.w
        }
    }
}

impl<T: Copy, S: Copy> MulAssign<S> for Vec4<T> where T:MulAssign<S> {
    fn mul_assign(&mut self, rhs: S) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl<T: Copy, Out> Vec4<T> where T:Mul<Output = Out>, Out:Add<Output = Out>{

    pub fn length_sq(self) -> Out {
        self.dot(self)
    }
}

impl<T: SquareRoot + Copy> Vec4<T> where T:Mul<Output=T>, T:Add<Output=T>{

    pub fn length(self) -> <T as SquareRoot>::Output {
        self.length_sq().sqrt()
    }
}

