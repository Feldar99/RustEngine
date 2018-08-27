extern crate math;
extern crate num;

#[cfg(test)]
mod tests {

    use math::matrix::Mat4;
    use math::vector::Vec4;

    use num::Zero;

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

    #[test]
    fn can_create_matrices () {
        let m4 = test_mat4_1();
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
    }

    #[test]
    fn has_zero_matrix () {
        let m4 = Mat4::<i32>::zero();
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
    }

    fn can_test_for_zero_matrix () {
        let z4 = Mat4::<i32>::zero();
        assert!(z4.is_zero());

        let m4 = test_mat4_1();
        assert!(!m4.is_zero());
    }

    #[test]
    fn can_add_matrices () {
        let m4 = test_mat4_1() + test_mat4_2();
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
    }
}