extern crate math;
extern crate assert_approx_eq;
extern crate num;

#[cfg(test)]
mod tests {


    use math::vector::Vec4;
    use math::vector::Vec3;
    use math::vector::Vec2;

    use num::Zero;

    const EPSILON:f32 = 0.0005;

    macro_rules! assert_approx_eq {
        ($left:expr, $right:expr, $epsilon:expr) => {
            assert!(($left) < ($right) + ($epsilon),
                "\nleft: `{}`,\n right: `{}`,\n epsilon: {}\n\n",
                $left,
                $right,
                $epsilon
            );
            assert!(($left) > ($right) - ($epsilon),
                "\nleft: `{}`,\n right: `{}`,\n epsilon: {}\n\n",
                $left,
                $right,
                $epsilon
            );
        }
    }

    #[test]
    fn can_create_vectors() {
        let v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let v3 = Vec3{x: 5, y: 6, z: 7};
        let v2 = Vec2{x: 8, y: 9};
        assert_eq!(v4.x, 1);
        assert_eq!(v4.y, 2);
        assert_eq!(v4.z, 3);
        assert_eq!(v4.w, 4);
        assert_eq!(v3.x, 5);
        assert_eq!(v3.y, 6);
        assert_eq!(v3.z, 7);
        assert_eq!(v2.x, 8);
        assert_eq!(v2.y, 9);
    }

    #[test]
    fn can_add_vectors() {

        let v4 = Vec4{x: 1, y: 2, z: 3, w: 4} + Vec4{x:10, y: 11, z: 12, w: 13};
        let v3 = Vec3{x: 5, y: 6, z: 7}       + Vec3{x: 14, y: 15, z: 16};
        let v2 = Vec2{x: 8, y: 9}             + Vec2{x: 17, y: 18};
        assert_eq!(v4.x, 11);
        assert_eq!(v4.y, 13);
        assert_eq!(v4.z, 15);
        assert_eq!(v4.w, 17);
        assert_eq!(v3.x, 19);
        assert_eq!(v3.y, 21);
        assert_eq!(v3.z, 23);
        assert_eq!(v2.x, 25);
        assert_eq!(v2.y, 27);
    }

    #[test]
    fn can_add_assign_vectors() {

        let mut v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let mut v3 = Vec3{x: 5, y: 6, z: 7};
        let mut v2 = Vec2{x: 8, y: 9};
        v4 += Vec4{x:10, y: 11, z: 12, w: 13};
        v3 += Vec3{x: 14, y: 15, z: 16};
        v2 += Vec2{x: 17, y: 18};
        assert_eq!(v4.x, 11);
        assert_eq!(v4.y, 13);
        assert_eq!(v4.z, 15);
        assert_eq!(v4.w, 17);
        assert_eq!(v3.x, 19);
        assert_eq!(v3.y, 21);
        assert_eq!(v3.z, 23);
        assert_eq!(v2.x, 25);
        assert_eq!(v2.y, 27);
    }
    #[test]
    fn can_negate_vectors() {
        let v4 = -Vec4{x: 18, y: 17, z: 16, w: 15};
        let v3 = -Vec3{x: 14, y: 13, z: 12};
        let v2 = -Vec2{x: 11, y: 10};
        assert_eq!(v4.x, -18);
        assert_eq!(v4.y, -17);
        assert_eq!(v4.z, -16);
        assert_eq!(v4.w, -15);
        assert_eq!(v3.x, -14);
        assert_eq!(v3.y, -13);
        assert_eq!(v3.z, -12);
        assert_eq!(v2.x, -11);
        assert_eq!(v2.y, -10);
    }

    #[test]
    fn can_subtract_vectors() {

        let v4 = Vec4{x: 1, y: 2, z: 3, w: 4} - Vec4{x: 18, y: 17, z: 16, w: 15};
        let v3 = Vec3{x: 5, y: 6, z: 7}       - Vec3{x: 14, y: 13, z: 12};
        let v2 = Vec2{x: 8, y: 9}             - Vec2{x: 11, y: 10};
        assert_eq!(v4.x, -17);
        assert_eq!(v4.y, -15);
        assert_eq!(v4.z, -13);
        assert_eq!(v4.w, -11);
        assert_eq!(v3.x, -9);
        assert_eq!(v3.y, -7);
        assert_eq!(v3.z, -5);
        assert_eq!(v2.x, -3);
        assert_eq!(v2.y, -1);
    }

    #[test]
    fn can_subtract_assign_vectors() {

        let mut v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let mut v3 = Vec3{x: 5, y: 6, z: 7};
        let mut v2 = Vec2{x: 8, y: 9};
        v4 -= Vec4{x: 18, y: 17, z: 16, w: 15};
        v3 -= Vec3{x: 14, y: 13, z: 12};
        v2 -= Vec2{x: 11, y: 10};
        assert_eq!(v4.x, -17);
        assert_eq!(v4.y, -15);
        assert_eq!(v4.z, -13);
        assert_eq!(v4.w, -11);
        assert_eq!(v3.x, -9);
        assert_eq!(v3.y, -7);
        assert_eq!(v3.z, -5);
        assert_eq!(v2.x, -3);
        assert_eq!(v2.y, -1);
    }

    #[test]
    fn can_dot_product_vectors() {

        let p4 = Vec4{x: 1, y: 2, z: 3, w: 4} .dot(Vec4{x:10, y: 11, z: 12, w: 13});
        let p3 = Vec3{x: 5, y: 6, z: 7}       .dot(Vec3{x: 14, y: 15, z: 16});
        let p2 = Vec2{x: 8, y: 9}             .dot(Vec2{x: 17, y: 18});
        assert_eq!(p4, 120);
        assert_eq!(p3, 272);
        assert_eq!(p2, 298);
    }

    #[test]
    fn can_multiply_vectors_by_scalars() {
        let v4 = Vec4{x: 1, y: 2, z: 3, w: 4} * 2;
        let v3 = Vec3{x: 5, y: 6, z: 7} * 3;
        let v2 = Vec2{x: 8, y: 9} * 4;
        assert_eq!(v4.x, 2);
        assert_eq!(v4.y, 4);
        assert_eq!(v4.z, 6);
        assert_eq!(v4.w, 8);
        assert_eq!(v3.x, 15);
        assert_eq!(v3.y, 18);
        assert_eq!(v3.z, 21);
        assert_eq!(v2.x, 32);
        assert_eq!(v2.y, 36);
    }
    #[test]
    fn can_multiply_assign_vectors_by_scalars() {
        let mut v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let mut v3 = Vec3{x: 5, y: 6, z: 7};
        let mut v2 = Vec2{x: 8, y: 9};
        v4 *= 2;
        v3 *= 3;
        v2 *= 4;
        assert_eq!(v4.x, 2);
        assert_eq!(v4.y, 4);
        assert_eq!(v4.z, 6);
        assert_eq!(v4.w, 8);
        assert_eq!(v3.x, 15);
        assert_eq!(v3.y, 18);
        assert_eq!(v3.z, 21);
        assert_eq!(v2.x, 32);
        assert_eq!(v2.y, 36);
    }

    #[test]
    fn can_divide_vectors_by_scalars() {
        let v4 = Vec4{x: 1, y: 2, z: 3, w: 4} / 2;
        let v3 = Vec3{x: 5, y: 6, z: 7} / 3;
        let v2 = Vec2{x: 8, y: 12} / 4;
        assert_eq!(v4.x, 0);
        assert_eq!(v4.y, 1);
        assert_eq!(v4.z, 1);
        assert_eq!(v4.w, 2);
        assert_eq!(v3.x, 1);
        assert_eq!(v3.y, 2);
        assert_eq!(v3.z, 2);
        assert_eq!(v2.x, 2);
        assert_eq!(v2.y, 3);
    }
    #[test]
    fn can_divide_assign_vectors_by_scalars() {
        let mut v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let mut v3 = Vec3{x: 5, y: 6, z: 7};
        let mut v2 = Vec2{x: 8, y: 12};
        v4 /= 2;
        v3 /= 3;
        v2 /= 4;
        assert_eq!(v4.x, 0);
        assert_eq!(v4.y, 1);
        assert_eq!(v4.z, 1);
        assert_eq!(v4.w, 2);
        assert_eq!(v3.x, 1);
        assert_eq!(v3.y, 2);
        assert_eq!(v3.z, 2);
        assert_eq!(v2.x, 2);
        assert_eq!(v2.y, 3);
    }

    #[test]
    fn can_find_length_squared_of_vectors() {
        let len_sq4 = Vec4{x: 1, y: 2, z: 3, w: 4}.length_sq();
        let len_sq3 = Vec3{x: 5, y: 6, z: 7}.length_sq();
        let len_sq2 = Vec2{x: 8, y: 9}.length_sq();
        assert_eq!(len_sq4, 30);
        assert_eq!(len_sq3, 110);
        assert_eq!(len_sq2, 145);
    }

    #[test]
    fn can_find_length_of_vectors() {
        let len4 = Vec4{x: 1, y: 2, z: 3, w: 4}.length();
        let len3 = Vec3{x: 5, y: 6, z: 7}.length();
        let len2 = Vec2{x: 8, y: 9}.length();
        assert_approx_eq!(len4, 5.477, EPSILON);
        assert_approx_eq!(len3, 10.488, EPSILON);
        assert_approx_eq!(len2, 12.042, EPSILON);

    }

    #[test]
    fn can_cross_vec3s() {
        let v1 = Vec3{x: 1, y: 2, z: 3};
        let v2 = Vec3{x: 4, y: 5, z: 6};
        let cross = v1.cross(v2);
        assert_eq!(cross.x, -3);
        assert_eq!(cross.y, 6);
        assert_eq!(cross.z, -3);
        assert_eq!(v1.dot(cross), 0);
        assert_eq!(v2.dot(cross), 0);
    }

    #[test]
    fn can_normalize_vectors() {
        let mut v4 = Vec4::<f32>{x: 1.0, y: 2.0, z: 3.0, w: 4.0};
        let mut v3 = Vec3::<f32>{x: 5.0, y: 6.0, z: 7.0};
        let mut v2 = Vec2::<f32>{x: 8.0, y: 9.0};
        let norm4 = v4.normalized();
        let norm3 = v3.normalized();
        let norm2 = v2.normalized();
        assert_approx_eq!(norm4.x, 0.183, EPSILON);
        assert_approx_eq!(norm4.y, 0.365, EPSILON);
        assert_approx_eq!(norm4.z, 0.548, EPSILON);
        assert_approx_eq!(norm4.w, 0.730, EPSILON);
        assert_approx_eq!(norm3.x, 0.477, EPSILON);
        assert_approx_eq!(norm3.y, 0.572, EPSILON);
        assert_approx_eq!(norm3.z, 0.667, EPSILON);
        assert_approx_eq!(norm2.x, 0.664, EPSILON);
        assert_approx_eq!(norm2.y, 0.747, EPSILON);
        assert_approx_eq!(norm4.length(), 1.0, EPSILON);
        assert_approx_eq!(norm3.length(), 1.0, EPSILON);
        assert_approx_eq!(norm2.length(), 1.0, EPSILON);

        v4.normalize();
        v3.normalize();
        v2.normalize();
        assert_eq!(v4.x, norm4.x);
        assert_eq!(v4.y, norm4.y);
        assert_eq!(v4.z, norm4.z);
        assert_eq!(v4.w, norm4.w);
        assert_eq!(v3.x, norm3.x);
        assert_eq!(v3.y, norm3.y);
        assert_eq!(v3.z, norm3.z);
        assert_eq!(v2.x, norm2.x);
        assert_eq!(v2.y, norm2.y);
    }

    #[test]
    fn has_zero_vector () {
        let v4 = Vec4::<i32>::zero();
        let v3 = Vec3::<i32>::zero();
        let v2 = Vec2::<i32>::zero();

        assert_eq!(v4.x, 0);
        assert_eq!(v4.y, 0);
        assert_eq!(v4.z, 0);
        assert_eq!(v4.w, 0);
        assert_eq!(v3.x, 0);
        assert_eq!(v3.y, 0);
        assert_eq!(v3.z, 0);
        assert_eq!(v2.x, 0);
        assert_eq!(v2.y, 0);
    }

    #[test]
    fn can_test_for_zero () {
        let z4 = Vec4::<i32>::zero();
        let z3 = Vec3::<i32>::zero();
        let z2 = Vec2::<i32>::zero();

        assert!(z4.is_zero());
        assert!(z3.is_zero());
        assert!(z2.is_zero());

        let v4 = Vec4::<f32>{x: 1.0, y: 2.0, z: 3.0, w: 4.0};
        let v3 = Vec3::<f32>{x: 5.0, y: 6.0, z: 7.0};
        let v2 = Vec2::<f32>{x: 8.0, y: 9.0};

        assert!(!v4.is_zero());
        assert!(!v3.is_zero());
        assert!(!v2.is_zero());
    }

    #[test]
    fn can_index_vectors () {
        let v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let v3 = Vec3{x: 5, y: 6, z: 7};
        let v2 = Vec2{x: 8, y: 9};
        assert_eq!(v4[0], 1);
        assert_eq!(v4[1], 2);
        assert_eq!(v4[2], 3);
        assert_eq!(v4[3], 4);
        assert_eq!(v3[0], 5);
        assert_eq!(v3[1], 6);
        assert_eq!(v3[2], 7);
        assert_eq!(v2[0], 8);
        assert_eq!(v2[1], 9);
    }

    #[test]
    #[should_panic]
    fn cannot_index_vectors_out_of_bounds () {
        let v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let _ = v4[4];
    }

    #[test]
    fn can_assign_to_vectors_by_index () {
        let mut v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        let mut v3 = Vec3{x: 5, y: 6, z: 7};
        let mut v2 = Vec2{x: 8, y: 9};
        v4[0] = 10;
        v4[1] = 11;
        v4[2] = 12;
        v4[3] = 13;
        v3[0] = 14;
        v3[1] = 15;
        v3[2] = 16;
        v2[0] = 17;
        v2[1] = 18;
        assert_eq!(v4.x, 10);
        assert_eq!(v4.y, 11);
        assert_eq!(v4.z, 12);
        assert_eq!(v4.w, 13);
        assert_eq!(v3.x, 14);
        assert_eq!(v3.y, 15);
        assert_eq!(v3.z, 16);
        assert_eq!(v2.x, 17);
        assert_eq!(v2.y, 18);
    }

    #[test]
    #[should_panic]
    fn cannot_assign_to_vector_out_of_bounds () {
        let mut v4 = Vec4{x: 1, y: 2, z: 3, w: 4};
        v4[4] = 5;;
    }
}