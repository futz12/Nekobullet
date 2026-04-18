use std::ffi::c_void;
use std::ptr::NonNull;

use super::rigidbody::RigidBody;
use super::types::{Real, Vec3};
use crate::ffi;

pub use crate::ffi::{
    MULTIBODY_CONSTRAINT_FIXED, MULTIBODY_CONSTRAINT_GEAR, MULTIBODY_CONSTRAINT_LIMIT,
    MULTIBODY_CONSTRAINT_POINT_TO_POINT, MULTIBODY_CONSTRAINT_SLIDER,
    MULTIBODY_CONSTRAINT_SPHERICAL_LIMIT, MULTIBODY_CONSTRAINT_SPHERICAL_MOTOR,
    MULTIBODY_CONSTRAINT_1DOF_JOINT_MOTOR,
};

pub struct MultiBodyConstraint {
    handle: NonNull<c_void>,
}

impl MultiBodyConstraint {
    pub fn new_point_to_point(
        rigid_body: &RigidBody,
        pivot_a: Vec3,
        pivot_b: Vec3,
    ) -> Option<Self> {
        let handle = unsafe {
            ffi::nk_multibody_constraint_create_p2p(
                rigid_body.handle(),
                pivot_a.x, pivot_a.y, pivot_a.z,
                pivot_b.x, pivot_b.y, pivot_b.z,
            )
        };
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn new_fixed(
        rigid_body: &RigidBody,
        pivot_a: Vec3,
        pivot_b: Vec3,
        frame_a: &[[Real; 3]; 3],
        frame_b: &[[Real; 3]; 3],
    ) -> Option<Self> {
        let handle = unsafe {
            ffi::nk_multibody_constraint_create_fixed(
                rigid_body.handle(),
                pivot_a.x, pivot_a.y, pivot_a.z,
                pivot_b.x, pivot_b.y, pivot_b.z,
                frame_a[0][0], frame_a[0][1], frame_a[0][2],
                frame_a[1][0], frame_a[1][1], frame_a[1][2],
                frame_a[2][0], frame_a[2][1], frame_a[2][2],
                frame_b[0][0], frame_b[0][1], frame_b[0][2],
                frame_b[1][0], frame_b[1][1], frame_b[1][2],
                frame_b[2][0], frame_b[2][1], frame_b[2][2],
            )
        };
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn new_slider(
        rigid_body: &RigidBody,
        pivot_a: Vec3,
        pivot_b: Vec3,
        frame_a: &[[Real; 3]; 3],
        frame_b: &[[Real; 3]; 3],
        axis: Vec3,
    ) -> Option<Self> {
        let handle = unsafe {
            ffi::nk_multibody_constraint_create_slider(
                rigid_body.handle(),
                pivot_a.x, pivot_a.y, pivot_a.z,
                pivot_b.x, pivot_b.y, pivot_b.z,
                frame_a[0][0], frame_a[0][1], frame_a[0][2],
                frame_a[1][0], frame_a[1][1], frame_a[1][2],
                frame_a[2][0], frame_a[2][1], frame_a[2][2],
                frame_b[0][0], frame_b[0][1], frame_b[0][2],
                frame_b[1][0], frame_b[1][1], frame_b[1][2],
                frame_b[2][0], frame_b[2][1], frame_b[2][2],
                axis.x, axis.y, axis.z,
            )
        };
        NonNull::new(handle).map(|h| Self { handle: h })
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn finalize(&self) {
        unsafe {
            ffi::nk_multibody_constraint_finalize(self.handle.as_ptr());
        }
    }

    pub fn constraint_type(&self) -> i32 {
        unsafe { ffi::nk_multibody_constraint_get_type(self.handle.as_ptr()) }
    }

    pub fn num_rows(&self) -> i32 {
        unsafe { ffi::nk_multibody_constraint_get_num_rows(self.handle.as_ptr()) }
    }

    pub fn set_max_applied_impulse(&self, max_impulse: Real) {
        unsafe {
            ffi::nk_multibody_constraint_set_max_applied_impulse(self.handle.as_ptr(), max_impulse);
        }
    }

    pub fn max_applied_impulse(&self) -> Real {
        unsafe { ffi::nk_multibody_constraint_get_max_applied_impulse(self.handle.as_ptr()) }
    }

    pub fn applied_impulse(&self, dof: i32) -> Real {
        unsafe { ffi::nk_multibody_constraint_get_applied_impulse(self.handle.as_ptr(), dof) }
    }

    pub fn set_pivot_in_b(&self, pivot: Vec3) {
        unsafe {
            ffi::nk_multibody_constraint_set_pivot_in_b(self.handle.as_ptr(), pivot.x, pivot.y, pivot.z);
        }
    }

    pub fn pivot_in_b(&self) -> Option<Vec3> {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_multibody_constraint_get_pivot_in_b(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Some(Vec3::new(x, y, z))
    }

    pub fn set_frame_in_b(&self, frame: &[[Real; 3]; 3]) {
        unsafe {
            ffi::nk_multibody_constraint_set_frame_in_b(
                self.handle.as_ptr(),
                frame[0][0], frame[0][1], frame[0][2],
                frame[1][0], frame[1][1], frame[1][2],
                frame[2][0], frame[2][1], frame[2][2],
            );
        }
    }
}

impl Drop for MultiBodyConstraint {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_multibody_constraint_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for MultiBodyConstraint {}
unsafe impl Sync for MultiBodyConstraint {}

#[cfg(test)]
mod tests_multibody_constraint {
    use super::*;
    use crate::core::world::PhysicsWorldBuilder;
    use crate::core::collision::CollisionShapeBuilder;
    use crate::core::rigidbody::RigidBodyBuilder;

    fn create_world_and_body() -> (crate::core::world::PhysicsWorld, RigidBody) {
        let world = PhysicsWorldBuilder::new().build();
        let shape = CollisionShapeBuilder::new()
            .sphere(1.0)
            .build()
            .expect("shape should be created");
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(1.0)
            .position(Vec3::new(0.0, 0.0, 0.0))
            .build()
            .expect("rigid body should be created");
        (world, body)
    }

    #[test]
    fn test_multibody_constraint_p2p() {
        let (_world, body) = create_world_and_body();
        
        let constraint = MultiBodyConstraint::new_point_to_point(
            &body,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        );
        
        assert!(constraint.is_some());
        let constraint = constraint.unwrap();
        
        assert_eq!(constraint.constraint_type(), MULTIBODY_CONSTRAINT_POINT_TO_POINT);
    }

    #[test]
    fn test_multibody_constraint_fixed() {
        let (_world, body) = create_world_and_body();
        
        let frame = [
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            [0.0, 0.0, 1.0],
        ];
        
        let constraint = MultiBodyConstraint::new_fixed(
            &body,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(0.0, 0.0, 0.0),
            &frame,
            &frame,
        );
        
        assert!(constraint.is_some());
        let constraint = constraint.unwrap();
        
        assert_eq!(constraint.constraint_type(), MULTIBODY_CONSTRAINT_FIXED);
    }

    #[test]
    fn test_multibody_constraint_max_impulse() {
        let (_world, body) = create_world_and_body();
        
        let constraint = MultiBodyConstraint::new_point_to_point(
            &body,
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
        ).unwrap();
        
        constraint.set_max_applied_impulse(100.0);
        assert!((constraint.max_applied_impulse() - 100.0).abs() < 0.001);
    }
}
