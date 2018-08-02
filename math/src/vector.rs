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
        pub struct $vector_type<T: Copy>{
            $(pub $component:T),+
        }

//        scalar_times_vector!(f32, $vector_type, {$($component), +});
        impl<T:Copy, S:Copy, Out:Copy> Add<$vector_type<S>> for $vector_type<T> where T:Add<S, Output=Out> {
            type Output = $vector_type<Out>;

            fn add(self, other: $vector_type<S>) -> $vector_type<Out> {
                $vector_type {
                    $($component: self.$component + other.$component),+
                }
            }
        }

        impl<T: Copy, S: Copy> AddAssign<$vector_type<S>> for $vector_type<T> where T:AddAssign<S> {
            fn add_assign(&mut self, rhs: $vector_type<S>) {
                $(self.$component += rhs.$component;)+
            }
        }

        impl<T: Copy, S: Copy, Out: Copy> Mul<S> for $vector_type<T> where T:Mul<S, Output = Out> {
            type Output = $vector_type<Out>;

            fn mul(self, rhs: S) -> $vector_type<Out> {
                $vector_type {
                    $($component: self.$component * rhs),+
                }
            }
        }

        impl<T: Copy, S: Copy> MulAssign<S> for $vector_type<T> where T:MulAssign<S> {
            fn mul_assign(&mut self, rhs: S) {
                $(self.$component *= rhs);+
            }
        }

        impl<T: Copy, Out> $vector_type<T> where T:Mul<Output = Out>, Out:Add<Output = Out>{

            pub fn length_sq(self) -> Out {
                self.dot(self)
            }
        }

        impl<T: SquareRoot + Copy> $vector_type<T> where T:Mul<Output=T>, T:Add<Output=T>{

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
impl<T: Copy> Vec3<T>{

    pub fn dot<S: Copy, Out>(self, rhs: Vec3<S>) -> Out
        where T:Mul<S, Output = Out>, Out:Add<Output = Out>
    {
        self.x * rhs.x +
            self.y * rhs.y +
            self.z * rhs.z
    }
}
impl<T: Copy> Vec2<T>{

    pub fn dot<S: Copy, Out>(self, rhs: Vec2<S>) -> Out
        where T:Mul<S, Output = Out>, Out:Add<Output = Out>
    {
        self.x * rhs.x +
            self.y * rhs.y
    }
}