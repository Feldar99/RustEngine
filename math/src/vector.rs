extern crate std;
extern crate num;

use std::ops::Add;
use std::ops::Mul;
use std::ops::AddAssign;
use std::ops::MulAssign;
use traits::SquareRoot;

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

    }
}
def_vector!(Vec4, {x, y, z, w});
def_vector!(Vec3, {x, y, z});
def_vector!(Vec2, {x, y});


//Implementing dot product outside macro, because + can't be used as a separator in a macro
impl<T: num::Num + Copy> Vec4<T>{

    pub fn dot<S: num::Num + Copy, Out: num::Num>(self, rhs: Vec4<S>) -> Out
        where T:Mul<S, Output = Out>
    {
        self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z +
            self.w * rhs.w
    }
}
impl<T: num::Num + Copy> Vec3<T>{

    pub fn dot<S: num::Num + Copy, Out: num::Num>(self, rhs: Vec3<S>) -> Out
        where T:Mul<S, Output = Out>
    {
        self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z
    }
}
impl<T: num::Num + Copy> Vec2<T>{

    pub fn dot<S: num::Num + Copy, Out: num::Num>(self, rhs: Vec2<S>) -> Out
        where T:Mul<S, Output = Out>
    {
        self.x * rhs.x +
            self.y * rhs.y
    }
}

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