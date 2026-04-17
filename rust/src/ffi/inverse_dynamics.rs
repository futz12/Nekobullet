use super::types::{nkMultiBodyHandle, nkReal};

extern "C" {
    pub fn nk_multibody_create() -> nkMultiBodyHandle;
    pub fn nk_multibody_destroy(multibody: nkMultiBodyHandle);

    pub fn nk_multibody_add_body(
        multibody: nkMultiBodyHandle,
        body_index: i32,
        parent_index: i32,
        joint_type: i32,
        parent_r_x: nkReal,
        parent_r_y: nkReal,
        parent_r_z: nkReal,
        body_T_parent_00: nkReal,
        body_T_parent_01: nkReal,
        body_T_parent_02: nkReal,
        body_T_parent_10: nkReal,
        body_T_parent_11: nkReal,
        body_T_parent_12: nkReal,
        body_T_parent_20: nkReal,
        body_T_parent_21: nkReal,
        body_T_parent_22: nkReal,
        axis_x: nkReal,
        axis_y: nkReal,
        axis_z: nkReal,
        mass: nkReal,
        com_x: nkReal,
        com_y: nkReal,
        com_z: nkReal,
        inertia_xx: nkReal,
        inertia_xy: nkReal,
        inertia_xz: nkReal,
        inertia_yy: nkReal,
        inertia_yz: nkReal,
        inertia_zz: nkReal,
    ) -> i32;
    pub fn nk_multibody_finalize(multibody: nkMultiBodyHandle) -> i32;

    pub fn nk_multibody_calculate_inverse_dynamics(
        multibody: nkMultiBodyHandle,
        q: *const nkReal,
        u: *const nkReal,
        dot_u: *const nkReal,
        joint_forces: *mut nkReal,
        num_dofs: i32,
    ) -> i32;

    pub fn nk_multibody_calculate_mass_matrix(
        multibody: nkMultiBodyHandle,
        q: *const nkReal,
        num_q: i32,
        mass_matrix: *mut nkReal,
        initialize_matrix: i32,
        set_lower_triangular: i32,
    ) -> i32;

    pub fn nk_multibody_get_num_bodies(multibody: nkMultiBodyHandle) -> i32;
    pub fn nk_multibody_get_num_dofs(multibody: nkMultiBodyHandle) -> i32;

    pub fn nk_multibody_set_gravity(multibody: nkMultiBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_multibody_set_accept_invalid_mass(multibody: nkMultiBodyHandle, accept: i32);

    pub fn nk_multibody_print_tree(multibody: nkMultiBodyHandle);
}
