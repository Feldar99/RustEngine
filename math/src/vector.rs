extern crate std;
extern crate num;

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
use traits::SquareRoot;
use num::Zero;

//    this seems to be blocked by https://github.com/rust-lang/rust/issues/34260
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

macro_rules! def_vector {
    ($vector_type:ident, {$($component:ident),+}) => {
        #[derive(Copy, Clone)]
        pub struct $vector_type<T: num::Num + Copy>{
            $(pub $component:T),+
        }

//        scalar_times_vector!(f32, $vector_type, {$($component), +});
        impl<T:num::Num + Copy, S:num::Num + Copy, Out:num::Num + Copy> Add<$vector_type<S>>
            for $vector_type<T>
            where T:Add<S, Output=Out>
        {
            type Output = $vector_type<Out>;

            fn add(self, other: $vector_type<S>) -> $vector_type<Out> {
                $vector_type {
                    $($component: self.$component + other.$component),+
                }
            }
        }

        impl<T: num::Num + Copy, S: num::Num + Copy> AddAssign<$vector_type<S>>
            for $vector_type<T>
            where T:AddAssign<S>
        {
            fn add_assign(&mut self, rhs: $vector_type<S>) {
                $(self.$component += rhs.$component;)+
            }
        }

        impl<T: num::Num + Copy, Out: num::Num + Copy> Neg for $vector_type<T>
            where T:Neg<Output = Out>
        {
            type Output = $vector_type<Out>;

            fn neg(self) -> $vector_type<Out> {
                $vector_type {
                    $($component: -self.$component),+
                }
            }
        }

        impl<T: num::Num + Copy, S: num::Num + Copy, Out: num::Num + Copy>
            Sub<$vector_type<S>> for $vector_type<T>
            where T:Sub<S, Output = Out>
        {
            type Output = $vector_type<Out>;

            fn sub(self, rhs: $vector_type<S>) -> $vector_type<Out> {
                $vector_type::<Out> {
                    $($component: self.$component - rhs.$component),+
                }
            }
        }

        impl<T: num::Num + Copy, S: num::Num + Copy> SubAssign<$vector_type<S>> for $vector_type<T>
            where T:SubAssign<S>
        {
            fn sub_assign(&mut self, rhs: $vector_type<S>) {
                $(self.$component -= rhs.$component;)+
            }
        }

        impl<T: num::Num + Copy, S: num::Num + Copy, Out: num::Num + Copy> Mul<S>
            for $vector_type<T>
            where T:Mul<S, Output = Out>
        {
            type Output = $vector_type<Out>;

            fn mul(self, rhs: S) -> $vector_type<Out> {
                $vector_type {
                    $($component: self.$component * rhs),+
                }
            }
        }

        impl<T: num::Num + Copy, S: num::Num + Copy> MulAssign<S> for $vector_type<T>
            where T:MulAssign<S>
        {
            fn mul_assign(&mut self, rhs: S) {
                $(self.$component *= rhs);+
            }
        }

        impl <T: num::Num + Copy, S: Copy, Out: num::Num + Copy> Div<S> for $vector_type<T>
            where T:Div<S, Output = Out>
        {
            type Output = $vector_type<Out>;

            fn div(self, rhs: S) -> $vector_type<Out> {
                $vector_type {
                    $($component: self.$component / rhs),+
                }
            }
        }

        impl <T: num::Num + Copy, S: Copy> DivAssign<S> for $vector_type<T> where T:DivAssign<S> {
            fn div_assign(&mut self, rhs: S) {
                $(self.$component /= rhs;)+
            }
        }

        impl<T: num::Num + Copy> $vector_type<T>{

            pub fn length_sq(self) -> T {
                self.dot(self)
            }
        }

        impl<T: SquareRoot + num::Num + Copy> $vector_type<T>{

            pub fn length(self) -> <T as SquareRoot>::Output {
                self.length_sq().sqrt()
            }
        }

        impl<T: SquareRoot + num::Num + Copy, Out: num::Num + Copy> $vector_type<T>
            where T:Div<<T as SquareRoot>::Output, Output = Out>
        {

            pub fn normalized(self) -> $vector_type<Out>{
                self / self.length()
            }
        }

        impl<T: SquareRoot + num::Num + Copy> $vector_type<T>
            where T:DivAssign<<T as SquareRoot>::Output>
        {

            pub fn normalize(&mut self) {
                *self /= self.length();
            }
        }

        impl<T: num::Num + Copy> Zero for $vector_type<T> {
            fn zero() -> $vector_type<T> {
                $vector_type {
                    $($component: T::zero()),+
                }
            }

            fn is_zero(&self) -> bool {
                $(self.$component.is_zero())&&+

            }
        }

        impl<T: num::Num + Copy> Index<usize> for $vector_type<T> {
            type Output = T;

            fn index(&self, index: usize) -> &T {
                let mut component_index:usize = 0;
                $(if index == component_index {
                    return &self.$component
                }
                component_index += 1;)+
                panic!("Index out of range for vector. size: {}, index: {}", component_index, index);
            }
        }

        impl<T: num::Num + Copy> IndexMut<usize> for $vector_type<T> {

            fn index_mut(&mut self, index: usize) -> &mut T {
                let mut component_index:usize = 0;
                $(if index == component_index {
                    return &mut self.$component
                }
                component_index += 1;)+
                panic!("Index out of range for vector. size: {}, index: {}", component_index, index);
            }
        }

        impl<T: num::Num + Copy> $vector_type<T>{

            pub fn dot<S: num::Num + Copy, Out: num::Num>(self, rhs: $vector_type<S>) -> Out
                where T:Mul<S, Output = Out>
            {
                let mut sum:Out = Out::zero();
                $(sum = sum + self.$component * rhs.$component;)+
                sum
            }
        }

    }
}
def_vector!(Vec4, {x, y, z, w});
def_vector!(Vec3, {x, y, z});
def_vector!(Vec2, {x, y});

impl<T: num::Num + Copy> Vec3<T> {

    pub fn cross<S: num::Num + Copy, Out: num::Num + Copy>(self, rhs: Vec3<S>) -> Vec3<Out>
        where T:Mul<S, Output = Out>
    {
        Vec3 {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x
        }
    }
}