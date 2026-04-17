use super::types::{nkReal, nkSoftBodyWorldInfoHandle, nkTransform};

pub type nkReducedDeformableBodyHandle = *mut std::ffi::c_void;

extern "C" {
    pub fn nk_reduced_softbody_create(
        world_info: nkSoftBodyWorldInfoHandle,
        node_count: i32,
        positions: *const nkReal,
        masses: *const nkReal,
    ) -> nkReducedDeformableBodyHandle;
    pub fn nk_reduced_softbody_destroy(body: nkReducedDeformableBodyHandle);

    pub fn nk_reduced_softbody_set_reduced_modes(body: nkReducedDeformableBodyHandle, num_modes: i32, full_size: i32);
    pub fn nk_reduced_softbody_get_num_reduced_modes(body: nkReducedDeformableBodyHandle) -> i32;
    pub fn nk_reduced_softbody_get_num_full_dofs(body: nkReducedDeformableBodyHandle) -> i32;

    pub fn nk_reduced_softbody_set_stiffness_scale(body: nkReducedDeformableBodyHandle, ks: nkReal);
    pub fn nk_reduced_softbody_set_mass_scale(body: nkReducedDeformableBodyHandle, rho: nkReal);
    pub fn nk_reduced_softbody_set_damping(body: nkReducedDeformableBodyHandle, alpha: nkReal, beta: nkReal);
    pub fn nk_reduced_softbody_set_fixed_node(body: nkReducedDeformableBodyHandle, node_index: i32);
    pub fn nk_reduced_softbody_disable_reduced_modes(body: nkReducedDeformableBodyHandle, rigid_only: i32);

    pub fn nk_reduced_softbody_set_rigid_velocity(body: nkReducedDeformableBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_reduced_softbody_set_rigid_angular_velocity(body: nkReducedDeformableBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_reduced_softbody_get_rigid_velocity(body: nkReducedDeformableBodyHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_reduced_softbody_get_rigid_angular_velocity(body: nkReducedDeformableBodyHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);

    pub fn nk_reduced_softbody_get_total_mass(body: nkReducedDeformableBodyHandle) -> nkReal;
    pub fn nk_reduced_softbody_set_total_mass(body: nkReducedDeformableBodyHandle, mass: nkReal);

    pub fn nk_reduced_softbody_get_rigid_transform(body: nkReducedDeformableBodyHandle, out_transform: *mut nkTransform);
    pub fn nk_reduced_softbody_set_rigid_transform(body: nkReducedDeformableBodyHandle, transform: *mut nkTransform);

    pub fn nk_reduced_softbody_apply_central_impulse(body: nkReducedDeformableBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_reduced_softbody_apply_torque_impulse(body: nkReducedDeformableBodyHandle, x: nkReal, y: nkReal, z: nkReal);

    pub fn nk_reduced_softbody_get_num_nodes(body: nkReducedDeformableBodyHandle) -> i32;
    pub fn nk_reduced_softbody_get_node_position(body: nkReducedDeformableBodyHandle, index: i32, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_reduced_softbody_get_node_rest_position(body: nkReducedDeformableBodyHandle, index: i32, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);

    pub fn nk_reduced_softbody_create_from_vtk(
        world_info: nkSoftBodyWorldInfoHandle,
        vtk_file_path: *const std::ffi::c_char,
    ) -> nkReducedDeformableBodyHandle;

    pub fn nk_reduced_softbody_read_reduced_info(body: nkReducedDeformableBodyHandle, file_path: *const std::ffi::c_char);
}
