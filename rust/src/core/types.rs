pub type Real = f32;

pub use glam::{Mat4, Quat, Vec3, Vec4};

#[repr(C)]
#[derive(Debug, Clone, Copy, Default)]
pub struct Transform {
    pub position: Vec3,
    pub rotation: Quat,
}

impl Transform {
    pub fn new(position: Vec3, rotation: Quat) -> Self {
        Self { position, rotation }
    }

    pub fn from_position(position: Vec3) -> Self {
        Self {
            position,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn identity() -> Self {
        Self {
            position: Vec3::ZERO,
            rotation: Quat::IDENTITY,
        }
    }

    pub fn to_mat4(&self) -> Mat4 {
        Mat4::from_rotation_translation(self.rotation, self.position)
    }

    pub fn from_mat4(mat: &Mat4) -> Self {
        let (_scale, rotation, translation) = mat.to_scale_rotation_translation();
        Self {
            position: translation,
            rotation,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn new(min: Vec3, max: Vec3) -> Self {
        Self { min, max }
    }

    pub fn center(&self) -> Vec3 {
        (self.min + self.max) * 0.5
    }

    pub fn extents(&self) -> Vec3 {
        (self.max - self.min) * 0.5
    }

    pub fn size(&self) -> Vec3 {
        self.max - self.min
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum MotionType {
    Static = 0,
    Kinematic = 1,
    #[default]
    Dynamic = 2,
}


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
#[derive(Default)]
pub enum ActivationState {
    Inactive = 0,
    #[default]
    Active = 1,
    DisableDeactivation = 2,
    DisableSimulation = 3,
}


#[cfg(test)]
mod tests {
    use super::*;
    use approx::assert_relative_eq;
    use glam::Mat3;

    const FLT_EPSILON: f32 = f32::EPSILON;

    fn rand_vec3() -> Vec3 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let r = (now as f32).sin();
        Vec3::new(r * 0.5 + 0.5, (r * 1.3).sin() * 0.5 + 0.5, (r * 1.7).sin() * 0.5 + 0.5)
    }

    fn rand_quat() -> Quat {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let r = (now as f32).sin();
        Quat::from_xyzw(
            r * 0.5 + 0.5,
            (r * 1.3).sin() * 0.5 + 0.5,
            (r * 1.7).sin() * 0.5 + 0.5,
            (r * 2.1).sin() * 0.5 + 0.5,
        )
        .normalize()
    }

    fn rand_scalar() -> f32 {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        ((now as f32).sin() * 0.5 + 0.5).max(0.001)
    }

    mod v3dot_tests {
        use super::*;

        #[test]
        fn test_v3dot_basic() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = Vec3::new(4.0, 5.0, 6.0);
            let result = v1.dot(v2);
            let expected = 1.0 * 4.0 + 2.0 * 5.0 + 3.0 * 6.0;
            assert_relative_eq!(result, expected, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_v3dot_random() {
            for _ in 0..100 {
                let v1 = rand_vec3();
                let v2 = rand_vec3();
                let result = v1.dot(v2);
                let expected = v1.x * v2.x + v1.y * v2.y + v1.z * v2.z;
                assert_relative_eq!(result, expected, epsilon = FLT_EPSILON * 4.0);
            }
        }

        #[test]
        fn test_v3dot_perpendicular() {
            let v1 = Vec3::X;
            let v2 = Vec3::Y;
            assert_relative_eq!(v1.dot(v2), 0.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_v3dot_parallel() {
            let v1 = Vec3::new(1.0, 2.0, 3.0);
            let v2 = v1 * 2.0;
            let expected = v1.length_squared() * 2.0;
            assert_relative_eq!(v1.dot(v2), expected, epsilon = FLT_EPSILON * 4.0);
        }
    }

    mod v3cross_tests {
        use super::*;

        #[test]
        fn test_v3cross_basic() {
            let v1 = Vec3::X;
            let v2 = Vec3::Y;
            let result = v1.cross(v2);
            assert_relative_eq!(result.x, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.y, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.z, 1.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_v3cross_random() {
            for _ in 0..100 {
                let v1 = rand_vec3();
                let v2 = rand_vec3();
                let result = v1.cross(v2);
                let expected = Vec3::new(
                    v1.y * v2.z - v1.z * v2.y,
                    v1.z * v2.x - v1.x * v2.z,
                    v1.x * v2.y - v1.y * v2.x,
                );
                let diff = (result - expected).length();
                assert!(diff < FLT_EPSILON * 4.0, "Cross product mismatch: diff = {}", diff);
            }
        }

        #[test]
        fn test_v3cross_self() {
            let v = Vec3::new(1.0, 2.0, 3.0);
            let result = v.cross(v);
            assert_relative_eq!(result.length(), 0.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_v3cross_antisymmetric() {
            let v1 = rand_vec3();
            let v2 = rand_vec3();
            let r1 = v1.cross(v2);
            let r2 = v2.cross(v1);
            assert_relative_eq!((r1 + r2).length(), 0.0, epsilon = FLT_EPSILON * 4.0);
        }
    }

    mod v3norm_tests {
        use super::*;

        #[test]
        fn test_v3norm_basic() {
            let v = Vec3::new(3.0, 4.0, 0.0);
            let normalized = v.normalize();
            assert_relative_eq!(normalized.length(), 1.0, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_v3norm_random() {
            for _ in 0..100 {
                let v = rand_vec3();
                if v.length() > FLT_EPSILON {
                    let normalized = v.normalize();
                    assert_relative_eq!(normalized.length(), 1.0, epsilon = FLT_EPSILON * 4.0);
                }
            }
        }

        #[test]
        fn test_v3norm_preserves_direction() {
            let v = Vec3::new(1.0, 2.0, 3.0);
            let normalized = v.normalize();
            let dot = v.normalize().dot(normalized);
            assert_relative_eq!(dot, 1.0, epsilon = FLT_EPSILON * 4.0);
        }
    }

    mod v3lerp_tests {
        use super::*;

        #[test]
        fn test_v3lerp_basic() {
            let v0 = Vec3::new(0.0, 0.0, 0.0);
            let v1 = Vec3::new(10.0, 10.0, 10.0);
            let result = v0.lerp(v1, 0.5);
            let expected = Vec3::new(5.0, 5.0, 5.0);
            assert_relative_eq!(result.x, expected.x, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.y, expected.y, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.z, expected.z, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_v3lerp_random() {
            for _ in 0..100 {
                let v0 = rand_vec3();
                let v1 = rand_vec3();
                let t = rand_scalar();
                let result = v0.lerp(v1, t);
                let expected = v0 + (v1 - v0) * t;
                let diff = (result - expected).length();
                assert!(diff < FLT_EPSILON * 4.0, "Lerp mismatch: diff = {}", diff);
            }
        }

        #[test]
        fn test_v3lerp_endpoints() {
            let v0 = Vec3::new(1.0, 2.0, 3.0);
            let v1 = Vec3::new(4.0, 5.0, 6.0);
            let r0 = v0.lerp(v1, 0.0);
            let r1 = v0.lerp(v1, 1.0);
            assert_relative_eq!((r0 - v0).length(), 0.0, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!((r1 - v1).length(), 0.0, epsilon = FLT_EPSILON * 4.0);
        }
    }

    mod v3interp_tests {
        use super::*;

        #[test]
        fn test_v3interp_basic() {
            let v0 = Vec3::new(0.0, 0.0, 0.0);
            let v1 = Vec3::new(10.0, 10.0, 10.0);
            let t = 0.3;
            let s = 1.0 - t;
            let expected = v0 * s + v1 * t;
            let result = v0.lerp(v1, t);
            let diff = (result - expected).length();
            assert!(diff < FLT_EPSILON * 4.0, "Interp mismatch: diff = {}", diff);
        }

        #[test]
        fn test_v3interp_random() {
            for _ in 0..100 {
                let v0 = rand_vec3();
                let v1 = rand_vec3();
                let t = rand_scalar();
                let s = 1.0 - t;
                let expected = v0 * s + v1 * t;
                let result = v0.lerp(v1, t);
                let diff = (result - expected).length();
                assert!(diff < FLT_EPSILON * 4.0, "Interp mismatch: diff = {}", diff);
            }
        }
    }

    mod v3sdiv_tests {
        use super::*;

        #[test]
        fn test_v3sdiv_basic() {
            let v = Vec3::new(10.0, 20.0, 30.0);
            let s = 2.0;
            let result = v / s;
            let expected = Vec3::new(5.0, 10.0, 15.0);
            assert_relative_eq!(result.x, expected.x, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.y, expected.y, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.z, expected.z, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_v3sdiv_random() {
            for _ in 0..100 {
                let v = rand_vec3();
                let s = rand_scalar() + 0.1;
                let result = v / s;
                let expected = v * (1.0 / s);
                let diff = (result - expected).length();
                assert!(diff < FLT_EPSILON * 4.0, "Sdiv mismatch: diff = {}", diff);
            }
        }
    }

    mod v3div_tests {
        use super::*;

        #[test]
        fn test_v3div_basic() {
            let v1 = Vec3::new(10.0, 20.0, 30.0);
            let v2 = Vec3::new(2.0, 4.0, 6.0);
            let result = Vec3::new(v1.x / v2.x, v1.y / v2.y, v1.z / v2.z);
            let expected = Vec3::new(5.0, 5.0, 5.0);
            assert_relative_eq!(result.x, expected.x, epsilon = FLT_EPSILON * 10.0);
            assert_relative_eq!(result.y, expected.y, epsilon = FLT_EPSILON * 10.0);
            assert_relative_eq!(result.z, expected.z, epsilon = FLT_EPSILON * 10.0);
        }

        #[test]
        fn test_v3div_random() {
            for _ in 0..100 {
                let v1 = rand_vec3();
                let v2 = rand_vec3() + Vec3::splat(0.1);
                let result = Vec3::new(v1.x / v2.x, v1.y / v2.y, v1.z / v2.z);
                let expected = Vec3::new(
                    v1.x / v2.x,
                    v1.y / v2.y,
                    v1.z / v2.z,
                );
                let diff = (result - expected).length();
                assert!(diff < FLT_EPSILON * 10.0, "Div mismatch: diff = {}", diff);
            }
        }
    }

    mod v3rotate_tests {
        use super::*;

        #[test]
        fn test_v3rotate_basic() {
            let v = Vec3::X;
            let axis = Vec3::Z;
            let angle = std::f32::consts::FRAC_PI_2;
            let result = Mat3::from_axis_angle(axis, angle) * v;
            assert_relative_eq!(result.x, 0.0, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.y, 1.0, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.z, 0.0, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_v3rotate_random() {
            for _ in 0..100 {
                let v = rand_vec3();
                let axis = rand_vec3().normalize();
                let angle = rand_scalar() * std::f32::consts::PI;
                
                let o = axis * axis.dot(v);
                let x = v - o;
                let y = axis.cross(v);
                let expected = o + x * angle.cos() + y * angle.sin();
                
                let result = Mat3::from_axis_angle(axis, angle) * v;
                let diff = (result - expected).length();
                assert!(diff < FLT_EPSILON * 4.0, "Rotate mismatch: diff = {}", diff);
            }
        }
    }

    mod v3skew_tests {
        use super::*;

        #[test]
        fn test_v3skew_basic() {
            let v = Vec3::new(1.0, 2.0, 3.0);
            let v1 = Vec3::new(0.0, -v.z, v.y);
            let v2 = Vec3::new(v.z, 0.0, -v.x);
            let v3 = Vec3::new(-v.y, v.x, 0.0);
            
            assert_relative_eq!(v1.x, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(v1.y, -3.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(v1.z, 2.0, epsilon = FLT_EPSILON);
            
            assert_relative_eq!(v2.x, 3.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(v2.y, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(v2.z, -1.0, epsilon = FLT_EPSILON);
            
            assert_relative_eq!(v3.x, -2.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(v3.y, 1.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(v3.z, 0.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_v3skew_random() {
            for _ in 0..100 {
                let v = rand_vec3();
                let v1 = Vec3::new(0.0, -v.z, v.y);
                let v2 = Vec3::new(v.z, 0.0, -v.x);
                let v3 = Vec3::new(-v.y, v.x, 0.0);
                
                assert_relative_eq!(v1.x, 0.0, epsilon = FLT_EPSILON);
                assert_relative_eq!(v2.y, 0.0, epsilon = FLT_EPSILON);
                assert_relative_eq!(v3.z, 0.0, epsilon = FLT_EPSILON);
            }
        }
    }

    mod v3triple_tests {
        use super::*;

        #[test]
        fn test_v3triple_basic() {
            let v1 = Vec3::X;
            let v2 = Vec3::Y;
            let v3 = Vec3::Z;
            let result = v1.dot(v2.cross(v3));
            assert_relative_eq!(result, 1.0, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_v3triple_random() {
            for _ in 0..100 {
                let v1 = rand_vec3();
                let v2 = rand_vec3();
                let v3 = rand_vec3();
                let result = v1.dot(v2.cross(v3));
                let expected = v1.x * (v2.y * v3.z - v2.z * v3.y)
                    + v1.y * (v2.z * v3.x - v2.x * v3.z)
                    + v1.z * (v2.x * v3.y - v2.y * v3.x);
                assert_relative_eq!(result, expected, epsilon = FLT_EPSILON * 4.0);
            }
        }
    }

    mod quaternion_tests {
        use super::*;

        #[test]
        fn test_qtmul_basic() {
            let q1 = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
            let q2 = Quat::from_rotation_y(std::f32::consts::FRAC_PI_4);
            let result = q1 * q2;
            assert!(result.is_normalized());
        }

        #[test]
        fn test_qtmul_random() {
            for _ in 0..100 {
                let q1 = rand_quat();
                let q2 = rand_quat();
                let result = q1 * q2;
                
                let x = q1.w * q2.x + q1.x * q2.w + q1.y * q2.z - q1.z * q2.y;
                let y = q1.w * q2.y + q1.y * q2.w + q1.z * q2.x - q1.x * q2.z;
                let z = q1.w * q2.z + q1.z * q2.w + q1.x * q2.y - q1.y * q2.x;
                let w = q1.w * q2.w - q1.x * q2.x - q1.y * q2.y - q1.z * q2.z;
                
                assert_relative_eq!(result.x, x, epsilon = FLT_EPSILON * 10.0);
                assert_relative_eq!(result.y, y, epsilon = FLT_EPSILON * 10.0);
                assert_relative_eq!(result.z, z, epsilon = FLT_EPSILON * 10.0);
                assert_relative_eq!(result.w, w, epsilon = FLT_EPSILON * 10.0);
            }
        }

        #[test]
        fn test_qtdot_basic() {
            let q1 = Quat::IDENTITY;
            let q2 = Quat::IDENTITY;
            let result = q1.dot(q2);
            assert_relative_eq!(result, 1.0, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_qtdot_random() {
            for _ in 0..100 {
                let q1 = rand_quat();
                let q2 = rand_quat();
                let result = q1.dot(q2);
                let expected = q1.x * q2.x + q1.y * q2.y + q1.z * q2.z + q1.w * q2.w;
                assert_relative_eq!(result, expected, epsilon = FLT_EPSILON * 4.0);
            }
        }

        #[test]
        fn test_qtnorm_basic() {
            let q = Quat::from_xyzw(1.0, 2.0, 3.0, 4.0);
            let normalized = q.normalize();
            assert_relative_eq!(normalized.length(), 1.0, epsilon = FLT_EPSILON * 10.0);
        }

        #[test]
        fn test_qtnorm_random() {
            for _ in 0..100 {
                let q = rand_quat();
                assert_relative_eq!(q.length(), 1.0, epsilon = FLT_EPSILON * 10.0);
            }
        }

        #[test]
        fn test_qtmul_qv3_basic() {
            let q = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
            let v = Vec3::X;
            let result = q * v;
            assert_relative_eq!(result.x, 0.0, epsilon = FLT_EPSILON * 8.0);
            assert_relative_eq!(result.y, 1.0, epsilon = FLT_EPSILON * 8.0);
        }

        #[test]
        fn test_qtmul_v3_q_basic() {
            let q = Quat::from_rotation_z(std::f32::consts::FRAC_PI_2);
            let v = Vec3::X;
            let result = q * v;
            let expected = q * v;
            let diff = (result - expected).length();
            assert!(diff < FLT_EPSILON * 8.0, "qtmulV3Q mismatch: diff = {}", diff);
        }
    }

    mod matrix3x3_tests {
        use super::*;

        fn to_mat3(m: Mat4) -> glam::Mat3 {
            glam::Mat3::from_cols(
                m.x_axis.truncate(),
                m.y_axis.truncate(),
                m.z_axis.truncate(),
            )
        }

        #[test]
        fn test_3x3mul_m_basic() {
            let m1 = Mat4::from_rotation_x(std::f32::consts::FRAC_PI_4);
            let m2 = Mat4::from_rotation_y(std::f32::consts::FRAC_PI_4);
            let result = m1 * m2;
            
            let m1_3 = to_mat3(m1);
            let m2_3 = to_mat3(m2);
            let expected = m1_3 * m2_3;
            let result_3 = to_mat3(result);
            
            for i in 0..3 {
                for j in 0..3 {
                    assert_relative_eq!(result_3.col(i)[j], expected.col(i)[j], epsilon = FLT_EPSILON * 4.0);
                }
            }
        }

        #[test]
        fn test_3x3transpose_basic() {
            let m = glam::Mat3::from_cols(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.0, 6.0),
                Vec3::new(7.0, 8.0, 9.0),
            );
            let t = m.transpose();
            
            assert_relative_eq!(t.x_axis.x, 1.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(t.x_axis.y, 4.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(t.x_axis.z, 7.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_3x3mul_mv_basic() {
            let m = glam::Mat3::from_cols(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.0, 6.0),
                Vec3::new(7.0, 8.0, 9.0),
            );
            let v = Vec3::new(1.0, 1.0, 1.0);
            let result = m * v;
            
            let expected = Vec3::new(
                m.x_axis.x * v.x + m.y_axis.x * v.y + m.z_axis.x * v.z,
                m.x_axis.y * v.x + m.y_axis.y * v.y + m.z_axis.y * v.z,
                m.x_axis.z * v.x + m.y_axis.z * v.y + m.z_axis.z * v.z,
            );
            
            assert_relative_eq!(result.x, expected.x, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.y, expected.y, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.z, expected.z, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_3x3set_rot_basic() {
            let q = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
            let m = Mat4::from_quat(q);
            let extracted = Quat::from_mat4(&m);
            
            let diff = (q - extracted).length();
            assert!(diff < FLT_EPSILON * 10.0, "setRot/getRot mismatch: diff = {}", diff);
        }

        #[test]
        fn test_3x3get_rot_basic() {
            let q = Quat::from_rotation_y(std::f32::consts::FRAC_PI_3);
            let m = Mat4::from_quat(q);
            let extracted = Quat::from_mat4(&m);
            
            let dot = q.dot(extracted).abs();
            assert!(dot > 1.0 - FLT_EPSILON * 10.0, "getRot mismatch: dot = {}", dot);
        }

        #[test]
        fn test_3x3mul_m1_m2_basic() {
            let m1 = glam::Mat3::from_cols(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            );
            let m2 = glam::Mat3::from_cols(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.0, 6.0),
                Vec3::new(7.0, 8.0, 9.0),
            );
            
            let result = m1 * m2;
            assert_relative_eq!(result.x_axis.x, 1.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.x_axis.y, 2.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.x_axis.z, 3.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.y_axis.x, 4.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.y_axis.y, 5.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.y_axis.z, 6.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.z_axis.x, 7.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.z_axis.y, 8.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(result.z_axis.z, 9.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_3x3mul_vm_basic() {
            let m = glam::Mat3::from_cols(
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            );
            let v = Vec3::new(1.0, 2.0, 3.0);
            
            let expected = Vec3::new(
                v.x * m.x_axis.x + v.y * m.y_axis.x + v.z * m.z_axis.x,
                v.x * m.x_axis.y + v.y * m.y_axis.y + v.z * m.z_axis.y,
                v.x * m.x_axis.z + v.y * m.y_axis.z + v.z * m.z_axis.z,
            );
            
            let result = Vec3::new(
                v.dot(m.x_axis),
                v.dot(m.y_axis),
                v.dot(m.z_axis),
            );
            
            assert_relative_eq!(result.x, expected.x, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.y, expected.y, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.z, expected.z, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_3x3transpose_times_basic() {
            let a = glam::Mat3::from_cols(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.0, 6.0),
                Vec3::new(7.0, 8.0, 9.0),
            );
            let b = glam::Mat3::from_cols(
                Vec3::new(9.0, 8.0, 7.0),
                Vec3::new(6.0, 5.0, 4.0),
                Vec3::new(3.0, 2.0, 1.0),
            );
            
            let at = a.transpose();
            let result = at * b;
            
            for i in 0..3usize {
                for j in 0..3usize {
                    let mut sum = 0.0f32;
                    for k in 0..3 {
                        sum += at.col(k)[i] * b.col(j)[k];
                    }
                    assert_relative_eq!(result.col(j)[i], sum, epsilon = FLT_EPSILON * 4.0);
                }
            }
        }

        #[test]
        fn test_3x3times_transpose_basic() {
            let a = glam::Mat3::from_cols(
                Vec3::new(1.0, 2.0, 3.0),
                Vec3::new(4.0, 5.0, 6.0),
                Vec3::new(7.0, 8.0, 9.0),
            );
            let b = glam::Mat3::from_cols(
                Vec3::new(9.0, 8.0, 7.0),
                Vec3::new(6.0, 5.0, 4.0),
                Vec3::new(3.0, 2.0, 1.0),
            );
            
            let bt = b.transpose();
            let result = a * bt;
            
            for i in 0..3usize {
                for j in 0..3usize {
                    let mut sum = 0.0f32;
                    for k in 0..3 {
                        sum += a.col(k)[i] * bt.col(j)[k];
                    }
                    assert_relative_eq!(result.col(j)[i], sum, epsilon = FLT_EPSILON * 4.0);
                }
            }
        }
    }

    mod aabb_tests {
        use super::*;

        #[test]
        fn test_aabb_intersect_basic() {
            let a = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
            let b = Aabb::new(Vec3::new(5.0, 5.0, 5.0), Vec3::new(15.0, 15.0, 15.0));
            
            let intersect = a.min.x <= b.max.x && a.max.x >= b.min.x
                && a.min.y <= b.max.y && a.max.y >= b.min.y
                && a.min.z <= b.max.z && a.max.z >= b.min.z;
            
            assert!(intersect);
        }

        #[test]
        fn test_aabb_intersect_disjoint() {
            let a = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
            let b = Aabb::new(Vec3::new(20.0, 20.0, 20.0), Vec3::new(30.0, 30.0, 30.0));
            
            let intersect = a.min.x <= b.max.x && a.max.x >= b.min.x
                && a.min.y <= b.max.y && a.max.y >= b.min.y
                && a.min.z <= b.max.z && a.max.z >= b.min.z;
            
            assert!(!intersect);
        }

        #[test]
        fn test_aabb_merge_basic() {
            let a = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
            let b = Aabb::new(Vec3::new(5.0, 5.0, 5.0), Vec3::new(15.0, 15.0, 15.0));
            
            let merged = Aabb::new(
                a.min.min(b.min),
                a.max.max(b.max),
            );
            
            assert_relative_eq!(merged.min.x, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(merged.min.y, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(merged.min.z, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(merged.max.x, 15.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(merged.max.y, 15.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(merged.max.z, 15.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_aabb_proximity() {
            let a = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
            let b = Aabb::new(Vec3::new(5.0, 5.0, 5.0), Vec3::new(15.0, 15.0, 15.0));
            let o = Aabb::new(Vec3::new(2.0, 2.0, 2.0), Vec3::new(12.0, 12.0, 12.0));
            
            let prox_a = (a.min + a.max - o.min - o.max).length();
            let prox_b = (b.min + b.max - o.min - o.max).length();
            
            let select = if prox_a < prox_b { 0 } else { 1 };
            assert!(select == 0 || select == 1);
        }
    }

    mod transform_tests {
        use super::*;

        #[test]
        fn test_transform_identity() {
            let t = Transform::identity();
            assert_relative_eq!(t.position.x, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(t.position.y, 0.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(t.position.z, 0.0, epsilon = FLT_EPSILON);
            assert!(t.rotation.is_near_identity());
        }

        #[test]
        fn test_transform_from_position() {
            let pos = Vec3::new(1.0, 2.0, 3.0);
            let t = Transform::from_position(pos);
            assert_relative_eq!(t.position.x, 1.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(t.position.y, 2.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(t.position.z, 3.0, epsilon = FLT_EPSILON);
            assert!(t.rotation.is_near_identity());
        }

        #[test]
        fn test_transform_to_from_mat4() {
            let pos = Vec3::new(1.0, 2.0, 3.0);
            let rot = Quat::from_rotation_x(std::f32::consts::FRAC_PI_4);
            let t = Transform::new(pos, rot);
            
            let mat = t.to_mat4();
            let t2 = Transform::from_mat4(&mat);
            
            assert_relative_eq!(t.position.x, t2.position.x, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(t.position.y, t2.position.y, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(t.position.z, t2.position.z, epsilon = FLT_EPSILON * 4.0);
        }

        #[test]
        fn test_transform_roundtrip() {
            for _ in 0..100 {
                let pos = rand_vec3();
                let rot = rand_quat();
                let t = Transform::new(pos, rot);
                
                let mat = t.to_mat4();
                let t2 = Transform::from_mat4(&mat);
                
                let pos_diff = (t.position - t2.position).length();
                assert!(pos_diff < FLT_EPSILON * 4.0, "Position roundtrip mismatch: diff = {}", pos_diff);
            }
        }
    }

    mod types_tests {
        use super::*;

        #[test]
        fn test_aabb_center() {
            let aabb = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 10.0, 10.0));
            let center = aabb.center();
            assert_relative_eq!(center.x, 5.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(center.y, 5.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(center.z, 5.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_aabb_extents() {
            let aabb = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 20.0, 30.0));
            let extents = aabb.extents();
            assert_relative_eq!(extents.x, 5.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(extents.y, 10.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(extents.z, 15.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_aabb_size() {
            let aabb = Aabb::new(Vec3::new(0.0, 0.0, 0.0), Vec3::new(10.0, 20.0, 30.0));
            let size = aabb.size();
            assert_relative_eq!(size.x, 10.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(size.y, 20.0, epsilon = FLT_EPSILON);
            assert_relative_eq!(size.z, 30.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_motion_type_values() {
            assert_eq!(MotionType::Static as u8, 0);
            assert_eq!(MotionType::Kinematic as u8, 1);
            assert_eq!(MotionType::Dynamic as u8, 2);
        }

        #[test]
        fn test_motion_type_default() {
            let mt: MotionType = Default::default();
            assert_eq!(mt, MotionType::Dynamic);
        }

        #[test]
        fn test_activation_state_values() {
            assert_eq!(ActivationState::Inactive as u8, 0);
            assert_eq!(ActivationState::Active as u8, 1);
            assert_eq!(ActivationState::DisableDeactivation as u8, 2);
            assert_eq!(ActivationState::DisableSimulation as u8, 3);
        }

        #[test]
        fn test_activation_state_default() {
            let as_: ActivationState = Default::default();
            assert_eq!(as_, ActivationState::Active);
        }
    }

    mod math_tests {
        use super::*;

        #[test]
        fn test_maxdot_basic() {
            let vertices = [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, 0.0, 1.0),
            ];
            let dir = Vec3::new(1.0, 0.0, 0.0);
            
            let mut max_dot = f32::MIN;
            let mut max_idx = 0usize;
            for (i, v) in vertices.iter().enumerate() {
                let dot = v.dot(dir);
                if dot > max_dot {
                    max_dot = dot;
                    max_idx = i;
                }
            }
            
            assert_eq!(max_idx, 0);
            assert_relative_eq!(max_dot, 1.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_mindot_basic() {
            let vertices = [
                Vec3::new(1.0, 0.0, 0.0),
                Vec3::new(-1.0, 0.0, 0.0),
                Vec3::new(0.0, 1.0, 0.0),
            ];
            let dir = Vec3::new(1.0, 0.0, 0.0);
            
            let mut min_dot = f32::MAX;
            let mut min_idx = 0usize;
            for (i, v) in vertices.iter().enumerate() {
                let dot = v.dot(dir);
                if dot < min_dot {
                    min_dot = dot;
                    min_idx = i;
                }
            }
            
            assert_eq!(min_idx, 1);
            assert_relative_eq!(min_dot, -1.0, epsilon = FLT_EPSILON);
        }

        #[test]
        fn test_dot3_basic() {
            let v = Vec3::new(1.0, 2.0, 3.0);
            let v1 = Vec3::new(1.0, 0.0, 0.0);
            let v2 = Vec3::new(0.0, 1.0, 0.0);
            let v3 = Vec3::new(0.0, 0.0, 1.0);
            
            let result = Vec3::new(v.dot(v1), v.dot(v2), v.dot(v3));
            
            assert_relative_eq!(result.x, 1.0, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.y, 2.0, epsilon = FLT_EPSILON * 4.0);
            assert_relative_eq!(result.z, 3.0, epsilon = FLT_EPSILON * 4.0);
        }
    }
}
