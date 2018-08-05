extern crate math;
extern crate assert_approx_eq;

#[cfg(test)]
mod tests {


    use math::vector::Vec4;
    use math::vector::Vec3;
    use math::vector::Vec2;

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
}