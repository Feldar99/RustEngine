extern crate math;
extern crate num;

#[cfg(test)]
mod tests {

    use math::matrix::Mat4;
    use math::matrix::Mat3;
    use math::vector::Vec4;
    use math::vector::Vec3;
    use math::vector::Vec2;

    use num::Zero;
    use num::One;

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

    fn test_mat4_1() -> Mat4<i32> {
        Mat4{values: [Vec4{x: 1,  y: 2,  z: 3,  w: 4 },
                      Vec4{x: 5,  y: 6,  z: 7,  w: 8 },
                      Vec4{x: 9,  y: 10, z: 11, w: 12},
                      Vec4{x: 13, y: 14, z: 15, w: 16}]}
    }

    fn test_mat4_2() -> Mat4<i32> {
        Mat4{values: [Vec4{x: 17, y: 18, z: 19, w: 20},
                      Vec4{x: 21, y: 22, z: 23, w: 24},
                      Vec4{x: 25, y: 26, z: 27, w: 28},
                      Vec4{x: 29, y: 30, z: 31, w: 32}]}
    }

    fn test_mat3_1() -> Mat3<i32> {
        Mat3{values: [Vec3{x: 33, y: 34, z: 35},
                      Vec3{x: 36, y: 37, z: 38},
                      Vec3{x: 39, y: 40, z: 41}]}
    }

    fn test_mat3_2() -> Mat3<i32> {
        Mat3{values: [Vec3{x: 42, y: 43, z: 44},
                      Vec3{x: 45, y: 46, z: 47},
                      Vec3{x: 48, y: 49, z: 50}]}
    }

    fn test_vec4() -> Vec4<i32> {
        Vec4{x: 1, y: 2, z: 3, w: 1}
    }

    fn test_vec4f() -> Vec4<f32> {
        Vec4{x: 1.0, y: 2.0, z: 3.0, w: 0.0}
    }

    fn test_vec3() -> Vec3<i32> {
        Vec3{x: 5, y: 6, z: 1}
    }

    fn test_vec3f_1() -> Vec3<f32> {
        Vec3{x: 5.0, y: 6.0, z: 1.0}
    }

    fn test_vec3f_2() -> Vec3<f32> {
        Vec3{x: 10.0, y: 11.0, z: 1.0}
    }

    fn test_vec2() -> Vec2<i32> {
        Vec2{x: 8, y: 9}
    }

    #[test]
    fn can_create_matrices () {
        let m4 = test_mat4_1();
        let m3 = test_mat3_1();
        assert_eq!(m4.values[0].x, 1);
        assert_eq!(m4.values[0].y, 2);
        assert_eq!(m4.values[0].z, 3);
        assert_eq!(m4.values[0].w, 4);
        assert_eq!(m4.values[1].x, 5);
        assert_eq!(m4.values[1].y, 6);
        assert_eq!(m4.values[1].z, 7);
        assert_eq!(m4.values[1].w, 8);
        assert_eq!(m4.values[2].x, 9);
        assert_eq!(m4.values[2].y, 10);
        assert_eq!(m4.values[2].z, 11);
        assert_eq!(m4.values[2].w, 12);
        assert_eq!(m4.values[3].x, 13);
        assert_eq!(m4.values[3].y, 14);
        assert_eq!(m4.values[3].z, 15);
        assert_eq!(m4.values[3].w, 16);

        assert_eq!(m3.values[0].x, 33);
        assert_eq!(m3.values[0].y, 34);
        assert_eq!(m3.values[0].z, 35);
        assert_eq!(m3.values[1].x, 36);
        assert_eq!(m3.values[1].y, 37);
        assert_eq!(m3.values[1].z, 38);
        assert_eq!(m3.values[2].x, 39);
        assert_eq!(m3.values[2].y, 40);
        assert_eq!(m3.values[2].z, 41);
    }

    #[test]
    fn has_zero_matrix () {
        let m4 = Mat4::<i32>::zero();
        let m3 = Mat3::<i32>::zero();
        assert_eq!(m4.values[0].x, 0);
        assert_eq!(m4.values[0].y, 0);
        assert_eq!(m4.values[0].z, 0);
        assert_eq!(m4.values[0].w, 0);
        assert_eq!(m4.values[1].x, 0);
        assert_eq!(m4.values[1].y, 0);
        assert_eq!(m4.values[1].z, 0);
        assert_eq!(m4.values[1].w, 0);
        assert_eq!(m4.values[2].x, 0);
        assert_eq!(m4.values[2].y, 0);
        assert_eq!(m4.values[2].z, 0);
        assert_eq!(m4.values[2].w, 0);
        assert_eq!(m4.values[3].x, 0);
        assert_eq!(m4.values[3].y, 0);
        assert_eq!(m4.values[3].z, 0);
        assert_eq!(m4.values[3].w, 0);

        assert_eq!(m3.values[0].x, 0);
        assert_eq!(m3.values[0].y, 0);
        assert_eq!(m3.values[0].z, 0);
        assert_eq!(m3.values[1].x, 0);
        assert_eq!(m3.values[1].y, 0);
        assert_eq!(m3.values[1].z, 0);
        assert_eq!(m3.values[2].x, 0);
        assert_eq!(m3.values[2].y, 0);
        assert_eq!(m3.values[2].z, 0);
    }

    #[test]
    fn can_test_for_zero_matrix () {
        let z4 = Mat4::<i32>::zero();
        let z3 = Mat3::<i32>::zero();
        assert!(z4.is_zero());
        assert!(z3.is_zero());

        let m4 = test_mat4_1();
        let m3 = test_mat3_1();
        assert!(!m4.is_zero());
        assert!(!m3.is_zero());
    }

    #[test]
    fn has_identity_matrix () {
        let m4 = Mat4::<i32>::one();
        assert_eq!(m4.values[0].x, 1);
        assert_eq!(m4.values[0].y, 0);
        assert_eq!(m4.values[0].z, 0);
        assert_eq!(m4.values[0].w, 0);
        assert_eq!(m4.values[1].x, 0);
        assert_eq!(m4.values[1].y, 1);
        assert_eq!(m4.values[1].z, 0);
        assert_eq!(m4.values[1].w, 0);
        assert_eq!(m4.values[2].x, 0);
        assert_eq!(m4.values[2].y, 0);
        assert_eq!(m4.values[2].z, 1);
        assert_eq!(m4.values[2].w, 0);
        assert_eq!(m4.values[3].x, 0);
        assert_eq!(m4.values[3].y, 0);
        assert_eq!(m4.values[3].z, 0);
        assert_eq!(m4.values[3].w, 1);

        let m3 = Mat3::<i32>::one();
        assert_eq!(m3.values[0].x, 1);
        assert_eq!(m3.values[0].y, 0);
        assert_eq!(m3.values[0].z, 0);
        assert_eq!(m3.values[1].x, 0);
        assert_eq!(m3.values[1].y, 1);
        assert_eq!(m3.values[1].z, 0);
        assert_eq!(m3.values[2].x, 0);
        assert_eq!(m3.values[2].y, 0);
        assert_eq!(m3.values[2].z, 1);
    }

    #[test]
    fn can_add_matrices () {
        let m4 = test_mat4_1() + test_mat4_2();
        let m3 = test_mat3_1() + test_mat3_2();
        assert_eq!(m4.values[0].x, 18);
        assert_eq!(m4.values[0].y, 20);
        assert_eq!(m4.values[0].z, 22);
        assert_eq!(m4.values[0].w, 24);
        assert_eq!(m4.values[1].x, 26);
        assert_eq!(m4.values[1].y, 28);
        assert_eq!(m4.values[1].z, 30);
        assert_eq!(m4.values[1].w, 32);
        assert_eq!(m4.values[2].x, 34);
        assert_eq!(m4.values[2].y, 36);
        assert_eq!(m4.values[2].z, 38);
        assert_eq!(m4.values[2].w, 40);
        assert_eq!(m4.values[3].x, 42);
        assert_eq!(m4.values[3].y, 44);
        assert_eq!(m4.values[3].z, 46);
        assert_eq!(m4.values[3].w, 48);

        assert_eq!(m3.values[0].x, 75);
        assert_eq!(m3.values[0].y, 77);
        assert_eq!(m3.values[0].z, 79);
        assert_eq!(m3.values[1].x, 81);
        assert_eq!(m3.values[1].y, 83);
        assert_eq!(m3.values[1].z, 85);
        assert_eq!(m3.values[2].x, 87);
        assert_eq!(m3.values[2].y, 89);
        assert_eq!(m3.values[2].z, 91);
    }

    #[test]
    fn can_add_assign_matrices () {
        let mut m4 = test_mat4_1();
        let mut m3 = test_mat3_1();
        m4 += test_mat4_2();
        m3 += test_mat3_2();
        assert_eq!(m4.values[0].x, 18);
        assert_eq!(m4.values[0].y, 20);
        assert_eq!(m4.values[0].z, 22);
        assert_eq!(m4.values[0].w, 24);
        assert_eq!(m4.values[1].x, 26);
        assert_eq!(m4.values[1].y, 28);
        assert_eq!(m4.values[1].z, 30);
        assert_eq!(m4.values[1].w, 32);
        assert_eq!(m4.values[2].x, 34);
        assert_eq!(m4.values[2].y, 36);
        assert_eq!(m4.values[2].z, 38);
        assert_eq!(m4.values[2].w, 40);
        assert_eq!(m4.values[3].x, 42);
        assert_eq!(m4.values[3].y, 44);
        assert_eq!(m4.values[3].z, 46);
        assert_eq!(m4.values[3].w, 48);

        assert_eq!(m3.values[0].x, 75);
        assert_eq!(m3.values[0].y, 77);
        assert_eq!(m3.values[0].z, 79);
        assert_eq!(m3.values[1].x, 81);
        assert_eq!(m3.values[1].y, 83);
        assert_eq!(m3.values[1].z, 85);
        assert_eq!(m3.values[2].x, 87);
        assert_eq!(m3.values[2].y, 89);
        assert_eq!(m3.values[2].z, 91);
    }

    #[test]
    fn can_negate_matrices () {
        let m4 = -test_mat4_1();
        let m3 = -test_mat3_1();
        assert_eq!(m4.values[0].x, -1);
        assert_eq!(m4.values[0].y, -2);
        assert_eq!(m4.values[0].z, -3);
        assert_eq!(m4.values[0].w, -4);
        assert_eq!(m4.values[1].x, -5);
        assert_eq!(m4.values[1].y, -6);
        assert_eq!(m4.values[1].z, -7);
        assert_eq!(m4.values[1].w, -8);
        assert_eq!(m4.values[2].x, -9);
        assert_eq!(m4.values[2].y, -10);
        assert_eq!(m4.values[2].z, -11);
        assert_eq!(m4.values[2].w, -12);
        assert_eq!(m4.values[3].x, -13);
        assert_eq!(m4.values[3].y, -14);
        assert_eq!(m4.values[3].z, -15);
        assert_eq!(m4.values[3].w, -16);

        assert_eq!(m3.values[0].x, -33);
        assert_eq!(m3.values[0].y, -34);
        assert_eq!(m3.values[0].z, -35);
        assert_eq!(m3.values[1].x, -36);
        assert_eq!(m3.values[1].y, -37);
        assert_eq!(m3.values[1].z, -38);
        assert_eq!(m3.values[2].x, -39);
        assert_eq!(m3.values[2].y, -40);
        assert_eq!(m3.values[2].z, -41);
    }

    #[test]
    fn can_subtract_matrices () {
        let m4 = test_mat4_1() - test_mat4_2();
        let m3 = test_mat3_1() - test_mat3_2();

        assert_eq!(m4.values[0].x, -16);
        assert_eq!(m4.values[0].y, -16);
        assert_eq!(m4.values[0].z, -16);
        assert_eq!(m4.values[0].w, -16);
        assert_eq!(m4.values[1].x, -16);
        assert_eq!(m4.values[1].y, -16);
        assert_eq!(m4.values[1].z, -16);
        assert_eq!(m4.values[1].w, -16);
        assert_eq!(m4.values[2].x, -16);
        assert_eq!(m4.values[2].y, -16);
        assert_eq!(m4.values[2].z, -16);
        assert_eq!(m4.values[2].w, -16);
        assert_eq!(m4.values[3].x, -16);
        assert_eq!(m4.values[3].y, -16);
        assert_eq!(m4.values[3].z, -16);
        assert_eq!(m4.values[3].w, -16);

        assert_eq!(m3.values[0].x, -9);
        assert_eq!(m3.values[0].y, -9);
        assert_eq!(m3.values[0].z, -9);
        assert_eq!(m3.values[1].x, -9);
        assert_eq!(m3.values[1].y, -9);
        assert_eq!(m3.values[1].z, -9);
        assert_eq!(m3.values[2].x, -9);
        assert_eq!(m3.values[2].y, -9);
        assert_eq!(m3.values[2].z, -9);
    }

    #[test]
    fn can_subtract_assign_matrices () {
        let mut m4 = test_mat4_1();
        m4 -= test_mat4_2();
        let mut m3 = test_mat3_1();
        m3 -= test_mat3_2();

        assert_eq!(m4.values[0].x, -16);
        assert_eq!(m4.values[0].y, -16);
        assert_eq!(m4.values[0].z, -16);
        assert_eq!(m4.values[0].w, -16);
        assert_eq!(m4.values[1].x, -16);
        assert_eq!(m4.values[1].y, -16);
        assert_eq!(m4.values[1].z, -16);
        assert_eq!(m4.values[1].w, -16);
        assert_eq!(m4.values[2].x, -16);
        assert_eq!(m4.values[2].y, -16);
        assert_eq!(m4.values[2].z, -16);
        assert_eq!(m4.values[2].w, -16);
        assert_eq!(m4.values[3].x, -16);
        assert_eq!(m4.values[3].y, -16);
        assert_eq!(m4.values[3].z, -16);
        assert_eq!(m4.values[3].w, -16);

        assert_eq!(m3.values[0].x, -9);
        assert_eq!(m3.values[0].y, -9);
        assert_eq!(m3.values[0].z, -9);
        assert_eq!(m3.values[1].x, -9);
        assert_eq!(m3.values[1].y, -9);
        assert_eq!(m3.values[1].z, -9);
        assert_eq!(m3.values[2].x, -9);
        assert_eq!(m3.values[2].y, -9);
        assert_eq!(m3.values[2].z, -9);
    }

    #[test]
    fn can_multiply_matrices_by_scalars () {
        let m4 = test_mat4_1() * 2;
        let m3 = test_mat3_1() * 3;
        assert_eq!(m4.values[0].x, 2);
        assert_eq!(m4.values[0].y, 4);
        assert_eq!(m4.values[0].z, 6);
        assert_eq!(m4.values[0].w, 8);
        assert_eq!(m4.values[1].x, 10);
        assert_eq!(m4.values[1].y, 12);
        assert_eq!(m4.values[1].z, 14);
        assert_eq!(m4.values[1].w, 16);
        assert_eq!(m4.values[2].x, 18);
        assert_eq!(m4.values[2].y, 20);
        assert_eq!(m4.values[2].z, 22);
        assert_eq!(m4.values[2].w, 24);
        assert_eq!(m4.values[3].x, 26);
        assert_eq!(m4.values[3].y, 28);
        assert_eq!(m4.values[3].z, 30);
        assert_eq!(m4.values[3].w, 32);

        assert_eq!(m3.values[0].x, 99);
        assert_eq!(m3.values[0].y, 102);
        assert_eq!(m3.values[0].z, 105);
        assert_eq!(m3.values[1].x, 108);
        assert_eq!(m3.values[1].y, 111);
        assert_eq!(m3.values[1].z, 114);
        assert_eq!(m3.values[2].x, 117);
        assert_eq!(m3.values[2].y, 120);
        assert_eq!(m3.values[2].z, 123);
    }

    #[test]
    fn can_multiply_matrices_by_vectors () {
        let v4 = test_mat4_1() * test_vec4();
        let v3 = test_mat3_1() * test_vec3();
        assert_eq!(v4.x, 18);
        assert_eq!(v4.y, 46);
        assert_eq!(v4.z, 74);
        assert_eq!(v4.w, 102);

        assert_eq!(v3.x, 404);
        assert_eq!(v3.y, 440);
        assert_eq!(v3.z, 476);
    }

    #[test]
    fn can_multiply_matrices () {
        let m4 = test_mat4_1() * test_mat4_2();
        assert_eq!(m4.values[0][0], 250);
        assert_eq!(m4.values[0][1], 260);
        assert_eq!(m4.values[0][2], 270);
        assert_eq!(m4.values[0][3], 280);
        assert_eq!(m4.values[1][0], 618);
        assert_eq!(m4.values[1][1], 644);
        assert_eq!(m4.values[1][2], 670);
        assert_eq!(m4.values[1][3], 696);
        assert_eq!(m4.values[2][0], 986);
        assert_eq!(m4.values[2][1], 1028);
        assert_eq!(m4.values[2][2], 1070);
        assert_eq!(m4.values[2][3], 1112);
        assert_eq!(m4.values[3][0], 1354);
        assert_eq!(m4.values[3][1], 1412);
        assert_eq!(m4.values[3][2], 1470);
        assert_eq!(m4.values[3][3], 1528);

        let m3 = test_mat3_1() * test_mat3_2();
        assert_eq!(m3.values[0].x, 4596);
        assert_eq!(m3.values[0].y, 4698);
        assert_eq!(m3.values[0].z, 4800);
        assert_eq!(m3.values[1].x, 5001);
        assert_eq!(m3.values[1].y, 5112);
        assert_eq!(m3.values[1].z, 5223);
        assert_eq!(m3.values[2].x, 5406);
        assert_eq!(m3.values[2].y, 5526);
        assert_eq!(m3.values[2].z, 5646);
    }

    #[test]
    fn can_multiply_assign_matrices () {
        let mut m4 = test_mat4_1();
        m4 *= test_mat4_2();
        assert_eq!(m4.values[0][0], 250);
        assert_eq!(m4.values[0][1], 260);
        assert_eq!(m4.values[0][2], 270);
        assert_eq!(m4.values[0][3], 280);
        assert_eq!(m4.values[1][0], 618);
        assert_eq!(m4.values[1][1], 644);
        assert_eq!(m4.values[1][2], 670);
        assert_eq!(m4.values[1][3], 696);
        assert_eq!(m4.values[2][0], 986);
        assert_eq!(m4.values[2][1], 1028);
        assert_eq!(m4.values[2][2], 1070);
        assert_eq!(m4.values[2][3], 1112);
        assert_eq!(m4.values[3][0], 1354);
        assert_eq!(m4.values[3][1], 1412);
        assert_eq!(m4.values[3][2], 1470);
        assert_eq!(m4.values[3][3], 1528);

        let mut m3 = test_mat3_1();
        m3 *= test_mat3_2();
        assert_eq!(m3.values[0].x, 4596);
        assert_eq!(m3.values[0].y, 4698);
        assert_eq!(m3.values[0].z, 4800);
        assert_eq!(m3.values[1].x, 5001);
        assert_eq!(m3.values[1].y, 5112);
        assert_eq!(m3.values[1].z, 5223);
        assert_eq!(m3.values[2].x, 5406);
        assert_eq!(m3.values[2].y, 5526);
        assert_eq!(m3.values[2].z, 5646);
    }

    #[test]
    fn can_divide_matrices_by_scalars () {
        let m4 = test_mat4_1() / 2;
        let m3 = test_mat3_1() / 3;
        assert_eq!(m4.values[0].x, 0);
        assert_eq!(m4.values[0].y, 1);
        assert_eq!(m4.values[0].z, 1);
        assert_eq!(m4.values[0].w, 2);
        assert_eq!(m4.values[1].x, 2);
        assert_eq!(m4.values[1].y, 3);
        assert_eq!(m4.values[1].z, 3);
        assert_eq!(m4.values[1].w, 4);
        assert_eq!(m4.values[2].x, 4);
        assert_eq!(m4.values[2].y, 5);
        assert_eq!(m4.values[2].z, 5);
        assert_eq!(m4.values[2].w, 6);
        assert_eq!(m4.values[3].x, 6);
        assert_eq!(m4.values[3].y, 7);
        assert_eq!(m4.values[3].z, 7);
        assert_eq!(m4.values[3].w, 8);

        assert_eq!(m3.values[0].x, 11);
        assert_eq!(m3.values[0].y, 11);
        assert_eq!(m3.values[0].z, 11);
        assert_eq!(m3.values[1].x, 12);
        assert_eq!(m3.values[1].y, 12);
        assert_eq!(m3.values[1].z, 12);
        assert_eq!(m3.values[2].x, 13);
        assert_eq!(m3.values[2].y, 13);
        assert_eq!(m3.values[2].z, 13);
    }

    #[test]
    fn can_divide_assign_matrices_by_scalars () {
        let mut m4 = test_mat4_1();
        m4 /= 2;
        let mut m3 = test_mat3_1();
        m3 /= 3;
        assert_eq!(m4.values[0].x, 0);
        assert_eq!(m4.values[0].y, 1);
        assert_eq!(m4.values[0].z, 1);
        assert_eq!(m4.values[0].w, 2);
        assert_eq!(m4.values[1].x, 2);
        assert_eq!(m4.values[1].y, 3);
        assert_eq!(m4.values[1].z, 3);
        assert_eq!(m4.values[1].w, 4);
        assert_eq!(m4.values[2].x, 4);
        assert_eq!(m4.values[2].y, 5);
        assert_eq!(m4.values[2].z, 5);
        assert_eq!(m4.values[2].w, 6);
        assert_eq!(m4.values[3].x, 6);
        assert_eq!(m4.values[3].y, 7);
        assert_eq!(m4.values[3].z, 7);
        assert_eq!(m4.values[3].w, 8);

        assert_eq!(m3.values[0].x, 11);
        assert_eq!(m3.values[0].y, 11);
        assert_eq!(m3.values[0].z, 11);
        assert_eq!(m3.values[1].x, 12);
        assert_eq!(m3.values[1].y, 12);
        assert_eq!(m3.values[1].z, 12);
        assert_eq!(m3.values[2].x, 13);
        assert_eq!(m3.values[2].y, 13);
        assert_eq!(m3.values[2].z, 13);
    }

    #[test]
    fn can_index_matrices () {
        let m4 = test_mat4_1();
        let m3 = test_mat3_1();
        assert_eq!(m4[0].x, 1);
        assert_eq!(m4[0].y, 2);
        assert_eq!(m4[0].z, 3);
        assert_eq!(m4[0].w, 4);
        assert_eq!(m4[1].x, 5);
        assert_eq!(m4[1].y, 6);
        assert_eq!(m4[1].z, 7);
        assert_eq!(m4[1].w, 8);
        assert_eq!(m4[2].x, 9);
        assert_eq!(m4[2].y, 10);
        assert_eq!(m4[2].z, 11);
        assert_eq!(m4[2].w, 12);
        assert_eq!(m4[3].x, 13);
        assert_eq!(m4[3].y, 14);
        assert_eq!(m4[3].z, 15);
        assert_eq!(m4[3].w, 16);

        assert_eq!(m3.values[0].x, 33);
        assert_eq!(m3.values[0].y, 34);
        assert_eq!(m3.values[0].z, 35);
        assert_eq!(m3.values[1].x, 36);
        assert_eq!(m3.values[1].y, 37);
        assert_eq!(m3.values[1].z, 38);
        assert_eq!(m3.values[2].x, 39);
        assert_eq!(m3.values[2].y, 40);
        assert_eq!(m3.values[2].z, 41);
    }

    #[test]
    #[should_panic]
    fn cannot_index_matrices_out_of_bounds () {
        let m4 = test_mat4_1();
        let _ = m4[4];
    }

    #[test]
    fn can_assign_to_matrices_by_index () {
        let mut m4 = test_mat4_1();
        m4[0].x = 17;
        m4[0].y = 18;
        m4[0].z = 19;
        m4[0].w = 20;
        m4[1].x = 21;
        m4[1].y = 22;
        m4[1].z = 23;
        m4[1].w = 24;
        m4[2].x = 25;
        m4[2].y = 26;
        m4[2].z = 27;
        m4[2].w = 28;
        m4[3].x = 29;
        m4[3].y = 30;
        m4[3].z = 31;
        m4[3].w = 32;
        assert_eq!(m4[0].x, 17);
        assert_eq!(m4[0].y, 18);
        assert_eq!(m4[0].z, 19);
        assert_eq!(m4[0].w, 20);
        assert_eq!(m4[1].x, 21);
        assert_eq!(m4[1].y, 22);
        assert_eq!(m4[1].z, 23);
        assert_eq!(m4[1].w, 24);
        assert_eq!(m4[2].x, 25);
        assert_eq!(m4[2].y, 26);
        assert_eq!(m4[2].z, 27);
        assert_eq!(m4[2].w, 28);
        assert_eq!(m4[3].x, 29);
        assert_eq!(m4[3].y, 30);
        assert_eq!(m4[3].z, 31);
        assert_eq!(m4[3].w, 32);

        let mut m3 = test_mat3_1();
        m3[0].x = 42;
        m3[0].y = 43;
        m3[0].z = 44;
        m3[1].x = 45;
        m3[1].y = 46;
        m3[1].z = 47;
        m3[2].x = 48;
        m3[2].y = 49;
        m3[2].z = 50;
        assert_eq!(m3[0].x, 42);
        assert_eq!(m3[0].y, 43);
        assert_eq!(m3[0].z, 44);
        assert_eq!(m3[1].x, 45);
        assert_eq!(m3[1].y, 46);
        assert_eq!(m3[1].z, 47);
        assert_eq!(m3[2].x, 48);
        assert_eq!(m3[2].y, 49);
        assert_eq!(m3[2].z, 50);
    }

    #[test]
    #[should_panic]
    fn cannot_assign_to_matrix_out_of_bounds () {
        let mut m4 = test_mat4_1();
        m4[4] = test_vec4();
    }

    #[test]
    fn can_create_scale_matrix () {
        let v4 = test_vec4();
        let v3 = test_vec3();
        let v2 = test_vec2();
        let m4 = Mat4::<i32>::scale(&v3);
        assert_eq!(m4[0].x, 5);
        assert_eq!(m4[0].y, 0);
        assert_eq!(m4[0].z, 0);
        assert_eq!(m4[0].w, 0);
        assert_eq!(m4[1].x, 0);
        assert_eq!(m4[1].y, 6);
        assert_eq!(m4[1].z, 0);
        assert_eq!(m4[1].w, 0);
        assert_eq!(m4[2].x, 0);
        assert_eq!(m4[2].y, 0);
        assert_eq!(m4[2].z, 1);
        assert_eq!(m4[2].w, 0);
        assert_eq!(m4[3].x, 0);
        assert_eq!(m4[3].y, 0);
        assert_eq!(m4[3].z, 0);
        assert_eq!(m4[3].w, 1);

        let scaled4 = m4 * v4;

        assert_eq!(scaled4.x, v4.x * v3.x);
        assert_eq!(scaled4.y, v4.y * v3.y);
        assert_eq!(scaled4.z, v4.z * v3.z);
        assert_eq!(scaled4.w, v4.w);

        let m3 = Mat3::<i32>::scale(&v2);
        assert_eq!(m3[0].x, 8);
        assert_eq!(m3[0].y, 0);
        assert_eq!(m3[0].z, 0);
        assert_eq!(m3[1].x, 0);
        assert_eq!(m3[1].y, 9);
        assert_eq!(m3[1].z, 0);
        assert_eq!(m3[2].x, 0);
        assert_eq!(m3[2].y, 0);
        assert_eq!(m3[2].z, 1);

        let scaled3 = m3 * v3;

        assert_eq!(scaled3.x, v3.x * v2.x);
        assert_eq!(scaled3.y, v3.y * v2.y);

    }

    #[test]
    fn can_create_translation_matrix () {
        let v4 = test_vec4();
        let v3 = test_vec3();
        let v2 = test_vec2();
        let m4 = Mat4::<i32>::translate(&v3);
        assert_eq!(m4[0].x, 1);
        assert_eq!(m4[0].y, 0);
        assert_eq!(m4[0].z, 0);
        assert_eq!(m4[0].w, 5);
        assert_eq!(m4[1].x, 0);
        assert_eq!(m4[1].y, 1);
        assert_eq!(m4[1].z, 0);
        assert_eq!(m4[1].w, 6);
        assert_eq!(m4[2].x, 0);
        assert_eq!(m4[2].y, 0);
        assert_eq!(m4[2].z, 1);
        assert_eq!(m4[2].w, 1);
        assert_eq!(m4[3].x, 0);
        assert_eq!(m4[3].y, 0);
        assert_eq!(m4[3].z, 0);
        assert_eq!(m4[3].w, 1);

        let translated4 = m4 * v4;
        let added4 = Vec3 {x: v4.x, y: v4.y, z: v4.z} + v3;
        assert_eq!(translated4.x, added4.x);
        assert_eq!(translated4.y, added4.y);
        assert_eq!(translated4.z, added4.z);

        let m3 = Mat3::<i32>::translate(&v2);
        assert_eq!(m3[0].x, 1);
        assert_eq!(m3[0].y, 0);
        assert_eq!(m3[0].z, 8);
        assert_eq!(m3[1].x, 0);
        assert_eq!(m3[1].y, 1);
        assert_eq!(m3[1].z, 9);
        assert_eq!(m3[2].x, 0);
        assert_eq!(m3[2].y, 0);
        assert_eq!(m3[2].z, 1);

        let translated3 = m3 * v3;
        let added3 = Vec2 {x: v3.x, y: v3.y} + v2;
        assert_eq!(translated3.x, added3.x);
        assert_eq!(translated3.y, added3.y);
    }

    #[test]
    fn can_create_cross_product_matrix () {
        let v3 = test_vec3f_1();
        let m3 = Mat3::<f32>::cross_product(&v3);
        assert_approx_eq!(m3[0].x,  0.0, EPSILON);
        assert_approx_eq!(m3[0].y, -1.0, EPSILON);
        assert_approx_eq!(m3[0].z,  6.0, EPSILON);
        assert_approx_eq!(m3[1].x,  1.0, EPSILON);
        assert_approx_eq!(m3[1].y,  0.0, EPSILON);
        assert_approx_eq!(m3[1].z, -5.0, EPSILON);
        assert_approx_eq!(m3[2].x, -6.0, EPSILON);
        assert_approx_eq!(m3[2].y,  5.0, EPSILON);
        assert_approx_eq!(m3[2].z,  0.0, EPSILON);

        let transformed = m3 * test_vec3f_2();
        let cross_product = v3.cross(test_vec3f_2());
        assert_approx_eq!(transformed.x, cross_product.x, EPSILON);
        assert_approx_eq!(transformed.y, cross_product.y, EPSILON);
        assert_approx_eq!(transformed.z, cross_product.z, EPSILON);
    }

    #[test]
    fn can_create_tensor_matrix () {
        let v3_1 = test_vec3f_1();
        let v3_2 = test_vec3f_2();
        let tensor = Mat3::<f32>::tensor(&v3_1, &v3_2);
        assert_approx_eq!(tensor[0].x, v3_1.x * v3_2.x, EPSILON);
        assert_approx_eq!(tensor[0].y, v3_1.x * v3_2.y, EPSILON);
        assert_approx_eq!(tensor[0].z, v3_1.x * v3_2.z, EPSILON);
        assert_approx_eq!(tensor[1].x, v3_1.y * v3_2.x, EPSILON);
        assert_approx_eq!(tensor[1].y, v3_1.y * v3_2.y, EPSILON);
        assert_approx_eq!(tensor[1].z, v3_1.y * v3_2.z, EPSILON);
        assert_approx_eq!(tensor[2].x, v3_1.z * v3_2.x, EPSILON);
        assert_approx_eq!(tensor[2].y, v3_1.z * v3_2.y, EPSILON);
        assert_approx_eq!(tensor[2].z, v3_1.z * v3_2.z, EPSILON);
    }

    #[test]
    fn can_transpose_matrix () {
        let mut m4 = test_mat4_1();
        let mut m3 = test_mat3_1();
        let transposed4 = m4.transposed();
        let transposed3 = m3.transposed();
        assert_eq!(transposed4[0].x, m4[0].x);
        assert_eq!(transposed4[0].y, m4[1].x);
        assert_eq!(transposed4[0].z, m4[2].x);
        assert_eq!(transposed4[0].w, m4[3].x);
        assert_eq!(transposed4[1].x, m4[0].y);
        assert_eq!(transposed4[1].y, m4[1].y);
        assert_eq!(transposed4[1].z, m4[2].y);
        assert_eq!(transposed4[1].w, m4[3].y);
        assert_eq!(transposed4[2].x, m4[0].z);
        assert_eq!(transposed4[2].y, m4[1].z);
        assert_eq!(transposed4[2].z, m4[2].z);
        assert_eq!(transposed4[2].w, m4[3].z);
        assert_eq!(transposed4[3].x, m4[0].w);
        assert_eq!(transposed4[3].y, m4[1].w);
        assert_eq!(transposed4[3].z, m4[2].w);
        assert_eq!(transposed4[3].w, m4[3].w);

        assert_eq!(transposed3[0].x, m3[0].x);
        assert_eq!(transposed3[0].y, m3[1].x);
        assert_eq!(transposed3[0].z, m3[2].x);
        assert_eq!(transposed3[1].x, m3[0].y);
        assert_eq!(transposed3[1].y, m3[1].y);
        assert_eq!(transposed3[1].z, m3[2].y);
        assert_eq!(transposed3[2].x, m3[0].z);
        assert_eq!(transposed3[2].y, m3[1].z);
        assert_eq!(transposed3[2].z, m3[2].z);

        m4.transpose();
        m3.transpose();
        assert_eq!(transposed4[0].x, m4[0].x);
        assert_eq!(transposed4[0].y, m4[0].y);
        assert_eq!(transposed4[0].z, m4[0].z);
        assert_eq!(transposed4[0].w, m4[0].w);
        assert_eq!(transposed4[1].x, m4[1].x);
        assert_eq!(transposed4[1].y, m4[1].y);
        assert_eq!(transposed4[1].z, m4[1].z);
        assert_eq!(transposed4[1].w, m4[1].w);
        assert_eq!(transposed4[2].x, m4[2].x);
        assert_eq!(transposed4[2].y, m4[2].y);
        assert_eq!(transposed4[2].z, m4[2].z);
        assert_eq!(transposed4[2].w, m4[2].w);
        assert_eq!(transposed4[3].x, m4[3].x);
        assert_eq!(transposed4[3].y, m4[3].y);
        assert_eq!(transposed4[3].z, m4[3].z);
        assert_eq!(transposed4[3].w, m4[3].w);

        assert_eq!(transposed3[0].x, m3[0].x);
        assert_eq!(transposed3[0].y, m3[0].y);
        assert_eq!(transposed3[0].z, m3[0].z);
        assert_eq!(transposed3[1].x, m3[1].x);
        assert_eq!(transposed3[1].y, m3[1].y);
        assert_eq!(transposed3[1].z, m3[1].z);
        assert_eq!(transposed3[2].x, m3[2].x);
        assert_eq!(transposed3[2].y, m3[2].y);
        assert_eq!(transposed3[2].z, m3[2].z);

    }

    #[test]
    fn can_create_rotation_matrix () {
        let axis = test_vec3f_1().normalized();
        let v4 = Vec4 {x: axis.y, y: -axis.x, z: 0f32, w: 1f32} * test_vec4f().length();
        let m4 = Mat4::<f32>::rotate(&axis, 45.0);
        assert_approx_eq!(m4[0].x,  0.825, EPSILON);
        assert_approx_eq!(m4[0].y,  0.052, EPSILON);
        assert_approx_eq!(m4[0].z,  0.562, EPSILON);
        assert_approx_eq!(m4[0].w,  0.000, EPSILON);
        assert_approx_eq!(m4[1].x,  0.232, EPSILON);
        assert_approx_eq!(m4[1].y,  0.877, EPSILON);
        assert_approx_eq!(m4[1].z, -0.421, EPSILON);
        assert_approx_eq!(m4[1].w,  0.000, EPSILON);
        assert_approx_eq!(m4[2].x, -0.515, EPSILON);
        assert_approx_eq!(m4[2].y,  0.477, EPSILON);
        assert_approx_eq!(m4[2].z,  0.712, EPSILON);
        assert_approx_eq!(m4[2].w,  0.000, EPSILON);
        assert_approx_eq!(m4[3].x,  0.000, EPSILON);
        assert_approx_eq!(m4[3].y,  0.000, EPSILON);
        assert_approx_eq!(m4[3].z,  0.000, EPSILON);
        assert_approx_eq!(m4[3].w,  1.000, EPSILON);

        let rotated = m4 * v4;
        assert_approx_eq!(v4.length(), rotated.length(), EPSILON);

        let v4_trimmed  = Vec3 {x: v4.x,      y: v4.y,      z: v4.z     };
        let rot_trimmed = Vec3 {x: rotated.x, y: rotated.y, z: rotated.z};
        assert_approx_eq!(v4_trimmed.angle(rot_trimmed), 45.0, EPSILON);
    }

//    fn test_mat4_1() -> Mat4<i32> {
//        Mat4{values: [Vec4{x: 1,  y: 2,  z: 3,  w: 4 },
//            Vec4{x: 5,  y: 6,  z: 7,  w: 8 },
//            Vec4{x: 9,  y: 10, z: 11, w: 12},
//            Vec4{x: 13, y: 14, z: 15, w: 16}]}
//    }
//
//    fn test_mat4_2() -> Mat4<i32> {
//        Mat4{values: [Vec4{x: 17, y: 18, z: 19, w: 20},
//            Vec4{x: 21, y: 22, z: 23, w: 24},
//            Vec4{x: 25, y: 26, z: 27, w: 28},
//            Vec4{x: 29, y: 30, z: 31, w: 32}]}
//    }
}