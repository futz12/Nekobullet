use super::types::{nkReal, nkRigidBodyHandle};

pub type nkMultiBodyConstraintHandle = *mut std::ffi::c_void;

pub const MULTIBODY_CONSTRAINT_LIMIT: i32 = 3;
pub const MULTIBODY_CONSTRAINT_1DOF_JOINT_MOTOR: i32 = 4;
pub const MULTIBODY_CONSTRAINT_GEAR: i32 = 5;
pub const MULTIBODY_CONSTRAINT_POINT_TO_POINT: i32 = 6;
pub const MULTIBODY_CONSTRAINT_SLIDER: i32 = 7;
pub const MULTIBODY_CONSTRAINT_SPHERICAL_MOTOR: i32 = 8;
pub const MULTIBODY_CONSTRAINT_FIXED: i32 = 9;
pub const MULTIBODY_CONSTRAINT_SPHERICAL_LIMIT: i32 = 10;

extern "C" {
    pub fn nk_multibody_constraint_create_p2p(
        rigid_body: nkRigidBodyHandle,
        pivot_a_x: nkReal, pivot_a_y: nkReal, pivot_a_z: nkReal,
        pivot_b_x: nkReal, pivot_b_y: nkReal, pivot_b_z: nkReal,
    ) -> nkMultiBodyConstraintHandle;

    pub fn nk_multibody_constraint_create_fixed(
        rigid_body: nkRigidBodyHandle,
        pivot_a_x: nkReal, pivot_a_y: nkReal, pivot_a_z: nkReal,
        pivot_b_x: nkReal, pivot_b_y: nkReal, pivot_b_z: nkReal,
        frame_a_00: nkReal, frame_a_01: nkReal, frame_a_02: nkReal,
        frame_a_10: nkReal, frame_a_11: nkReal, frame_a_12: nkReal,
        frame_a_20: nkReal, frame_a_21: nkReal, frame_a_22: nkReal,
        frame_b_00: nkReal, frame_b_01: nkReal, frame_b_02: nkReal,
        frame_b_10: nkReal, frame_b_11: nkReal, frame_b_12: nkReal,
        frame_b_20: nkReal, frame_b_21: nkReal, frame_b_22: nkReal,
    ) -> nkMultiBodyConstraintHandle;

    pub fn nk_multibody_constraint_create_slider(
        rigid_body: nkRigidBodyHandle,
        pivot_a_x: nkReal, pivot_a_y: nkReal, pivot_a_z: nkReal,
        pivot_b_x: nkReal, pivot_b_y: nkReal, pivot_b_z: nkReal,
        frame_a_00: nkReal, frame_a_01: nkReal, frame_a_02: nkReal,
        frame_a_10: nkReal, frame_a_11: nkReal, frame_a_12: nkReal,
        frame_a_20: nkReal, frame_a_21: nkReal, frame_a_22: nkReal,
        frame_b_00: nkReal, frame_b_01: nkReal, frame_b_02: nkReal,
        frame_b_10: nkReal, frame_b_11: nkReal, frame_b_12: nkReal,
        frame_b_20: nkReal, frame_b_21: nkReal, frame_b_22: nkReal,
        axis_x: nkReal, axis_y: nkReal, axis_z: nkReal,
    ) -> nkMultiBodyConstraintHandle;

    pub fn nk_multibody_constraint_destroy(constraint: nkMultiBodyConstraintHandle);

    pub fn nk_multibody_constraint_finalize(constraint: nkMultiBodyConstraintHandle);
    pub fn nk_multibody_constraint_get_type(constraint: nkMultiBodyConstraintHandle) -> i32;
    pub fn nk_multibody_constraint_get_num_rows(constraint: nkMultiBodyConstraintHandle) -> i32;

    pub fn nk_multibody_constraint_set_max_applied_impulse(constraint: nkMultiBodyConstraintHandle, max_impulse: nkReal);
    pub fn nk_multibody_constraint_get_max_applied_impulse(constraint: nkMultiBodyConstraintHandle) -> nkReal;

    pub fn nk_multibody_constraint_get_applied_impulse(constraint: nkMultiBodyConstraintHandle, dof: i32) -> nkReal;

    pub fn nk_multibody_constraint_set_pivot_in_b(constraint: nkMultiBodyConstraintHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_multibody_constraint_get_pivot_in_b(constraint: nkMultiBodyConstraintHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);

    pub fn nk_multibody_constraint_set_frame_in_b(
        constraint: nkMultiBodyConstraintHandle,
        m00: nkReal, m01: nkReal, m02: nkReal,
        m10: nkReal, m11: nkReal, m12: nkReal,
        m20: nkReal, m21: nkReal, m22: nkReal,
    );
}
