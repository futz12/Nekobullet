use super::types::{nkReal, nkSoftBodyHandle, nkSoftBodyWorldInfoHandle, nkRigidBodyHandle};

extern "C" {
    pub fn nk_softbody_world_info_create() -> nkSoftBodyWorldInfoHandle;
    pub fn nk_softbody_world_info_destroy(info: nkSoftBodyWorldInfoHandle);
    pub fn nk_softbody_world_info_set_gravity(info: nkSoftBodyWorldInfoHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_world_info_set_air_density(info: nkSoftBodyWorldInfoHandle, density: nkReal);
    pub fn nk_softbody_world_info_set_water_density(info: nkSoftBodyWorldInfoHandle, density: nkReal);
    pub fn nk_softbody_world_info_set_water_offset(info: nkSoftBodyWorldInfoHandle, offset: nkReal);
    pub fn nk_softbody_world_info_set_water_normal(info: nkSoftBodyWorldInfoHandle, x: nkReal, y: nkReal, z: nkReal);
    
    pub fn nk_softbody_create_rope(info: nkSoftBodyWorldInfoHandle,
        from_x: nkReal, from_y: nkReal, from_z: nkReal,
        to_x: nkReal, to_y: nkReal, to_z: nkReal,
        res: i32, fixeds: i32) -> nkSoftBodyHandle;
    pub fn nk_softbody_create_patch(info: nkSoftBodyWorldInfoHandle,
        corner00_x: nkReal, corner00_y: nkReal, corner00_z: nkReal,
        corner10_x: nkReal, corner10_y: nkReal, corner10_z: nkReal,
        corner01_x: nkReal, corner01_y: nkReal, corner01_z: nkReal,
        corner11_x: nkReal, corner11_y: nkReal, corner11_z: nkReal,
        resx: i32, resy: i32, fixeds: i32, gendiags: i32) -> nkSoftBodyHandle;
    pub fn nk_softbody_create_ellipsoid(info: nkSoftBodyWorldInfoHandle,
        center_x: nkReal, center_y: nkReal, center_z: nkReal,
        radius_x: nkReal, radius_y: nkReal, radius_z: nkReal,
        res: i32) -> nkSoftBodyHandle;
    pub fn nk_softbody_create_from_trimesh(info: nkSoftBodyWorldInfoHandle,
        vertices: *const nkReal, triangles: *const i32, ntriangles: i32) -> nkSoftBodyHandle;
    pub fn nk_softbody_create_from_convex_hull(info: nkSoftBodyWorldInfoHandle,
        vertices: *const nkReal, nvertices: i32) -> nkSoftBodyHandle;
    pub fn nk_softbody_destroy(softbody: nkSoftBodyHandle);
    
    pub fn nk_softbody_get_num_nodes(softbody: nkSoftBodyHandle) -> i32;
    pub fn nk_softbody_get_node_position(softbody: nkSoftBodyHandle, index: i32, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_softbody_set_node_position(softbody: nkSoftBodyHandle, index: i32, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_get_node_velocity(softbody: nkSoftBodyHandle, index: i32, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_softbody_set_node_velocity(softbody: nkSoftBodyHandle, index: i32, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_get_node_mass(softbody: nkSoftBodyHandle, index: i32) -> nkReal;
    pub fn nk_softbody_set_node_mass(softbody: nkSoftBodyHandle, index: i32, mass: nkReal);
    
    pub fn nk_softbody_get_total_mass(softbody: nkSoftBodyHandle) -> nkReal;
    pub fn nk_softbody_set_total_mass(softbody: nkSoftBodyHandle, mass: nkReal);
    pub fn nk_softbody_set_velocity(softbody: nkSoftBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_add_velocity(softbody: nkSoftBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_apply_force(softbody: nkSoftBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_apply_impulse(softbody: nkSoftBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_clear_forces(softbody: nkSoftBodyHandle);
    
    pub fn nk_softbody_append_anchor(softbody: nkSoftBodyHandle, node_index: i32, body: nkRigidBodyHandle,
        local_x: nkReal, local_y: nkReal, local_z: nkReal, disable_collision: i32);
    pub fn nk_softbody_remove_anchor(softbody: nkSoftBodyHandle, node_index: i32);
    
    pub fn nk_softbody_set_material_stiffness(softbody: nkSoftBodyHandle, kLST: nkReal, kAST: nkReal, kVST: nkReal);
    pub fn nk_softbody_set_wind_velocity(softbody: nkSoftBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_softbody_get_wind_velocity(softbody: nkSoftBodyHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    
    pub fn nk_softbody_set_config_damping(softbody: nkSoftBodyHandle, damping: nkReal);
    pub fn nk_softbody_set_config_drag(softbody: nkSoftBodyHandle, drag: nkReal);
    pub fn nk_softbody_set_config_lift(softbody: nkSoftBodyHandle, lift: nkReal);
    pub fn nk_softbody_set_config_pressure(softbody: nkSoftBodyHandle, pressure: nkReal);
    pub fn nk_softbody_set_config_volume_conversation(softbody: nkSoftBodyHandle, volume: nkReal);
    pub fn nk_softbody_set_config_time_scale(softbody: nkSoftBodyHandle, scale: nkReal);
}
