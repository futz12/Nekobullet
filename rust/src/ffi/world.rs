use super::types::{nkReal, nkRigidBodyHandle, nkWorldHandle, nkConstraintHandle, nkTransform, nkShapeHandle, nkGhostObjectHandle, nkSoftBodyHandle, nkVehicleHandle, nkCharacterHandle};

#[repr(C)]
pub struct nkRayTestResult {
    pub hit: i32,
    pub hit_point: [nkReal; 3],
    pub hit_normal: [nkReal; 3],
    pub hit_fraction: nkReal,
    pub body: nkRigidBodyHandle,
}

#[repr(C)]
pub struct nkContactPoint {
    pub position: [nkReal; 3],
    pub normal: [nkReal; 3],
    pub distance: nkReal,
    pub body_a: nkRigidBodyHandle,
    pub body_b: nkRigidBodyHandle,
}

pub type nkContactCallback = extern "C" fn(*mut nkContactPoint, *mut std::ffi::c_void);
pub type nkCollisionFilterCallback = extern "C" fn(nkRigidBodyHandle, nkRigidBodyHandle, *mut std::ffi::c_void) -> i32;

extern "C" {
    pub fn nk_world_create() -> nkWorldHandle;
    pub fn nk_world_destroy(world: nkWorldHandle);
    pub fn nk_world_set_gravity(world: nkWorldHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_world_get_gravity(world: nkWorldHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_world_step_simulation(world: nkWorldHandle, time_step: nkReal, max_sub_steps: i32, fixed_time_step: nkReal);
    pub fn nk_world_get_num_rigid_bodies(world: nkWorldHandle) -> i32;
    pub fn nk_world_get_num_constraints(world: nkWorldHandle) -> i32;
    pub fn nk_world_get_num_collision_objects(world: nkWorldHandle) -> i32;
    pub fn nk_world_clear_forces(world: nkWorldHandle);
    pub fn nk_world_add_rigid_body(world: nkWorldHandle, body: nkRigidBodyHandle);
    pub fn nk_world_remove_rigid_body(world: nkWorldHandle, body: nkRigidBodyHandle);
    pub fn nk_world_set_time_step(world: nkWorldHandle, time_step: nkReal);
    pub fn nk_world_get_time_step(world: nkWorldHandle) -> nkReal;
    pub fn nk_world_set_max_sub_steps(world: nkWorldHandle, max_sub_steps: i32);
    pub fn nk_world_get_max_sub_steps(world: nkWorldHandle) -> i32;
    pub fn nk_world_set_contact_breaking_threshold(world: nkWorldHandle, threshold: nkReal);
    pub fn nk_world_get_contact_breaking_threshold(world: nkWorldHandle) -> nkReal;
    
    pub fn nk_world_ray_test_closest(
        world: nkWorldHandle,
        from_x: nkReal, from_y: nkReal, from_z: nkReal,
        to_x: nkReal, to_y: nkReal, to_z: nkReal,
        out_result: *mut nkRayTestResult,
    );
    
    pub fn nk_world_ray_test_all(
        world: nkWorldHandle,
        from_x: nkReal, from_y: nkReal, from_z: nkReal,
        to_x: nkReal, to_y: nkReal, to_z: nkReal,
        out_results: *mut nkRayTestResult,
        max_results: i32,
        out_num_results: *mut i32,
    );
    
    pub fn nk_constraint_create_point2point(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        pivot_a_x: nkReal, pivot_a_y: nkReal, pivot_a_z: nkReal,
        pivot_b_x: nkReal, pivot_b_y: nkReal, pivot_b_z: nkReal,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_hinge(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        frame_a: *const nkTransform,
        frame_b: *const nkTransform,
        low_limit: nkReal,
        high_limit: nkReal,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_slider(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        frame_a: *const nkTransform,
        frame_b: *const nkTransform,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_fixed(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_generic_6dof(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        frame_a: *const nkTransform,
        frame_b: *const nkTransform,
        use_linear_reference_frame_a: i32,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_cone_twist(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        frame_a: *const nkTransform,
        frame_b: *const nkTransform,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_generic_6dof_spring(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        frame_a: *const nkTransform,
        frame_b: *const nkTransform,
        use_linear_reference_frame_a: i32,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_universal(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        anchor_x: nkReal, anchor_y: nkReal, anchor_z: nkReal,
        axis1_x: nkReal, axis1_y: nkReal, axis1_z: nkReal,
        axis2_x: nkReal, axis2_y: nkReal, axis2_z: nkReal,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_hinge2(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        anchor_x: nkReal, anchor_y: nkReal, anchor_z: nkReal,
        axis1_x: nkReal, axis1_y: nkReal, axis1_z: nkReal,
        axis2_x: nkReal, axis2_y: nkReal, axis2_z: nkReal,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_gear(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        axis_a_x: nkReal, axis_a_y: nkReal, axis_a_z: nkReal,
        axis_b_x: nkReal, axis_b_y: nkReal, axis_b_z: nkReal,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_create_generic_6dof_spring2(
        body_a: nkRigidBodyHandle,
        body_b: nkRigidBodyHandle,
        frame_a: *const nkTransform,
        frame_b: *const nkTransform,
        rotate_order: i32,
    ) -> nkConstraintHandle;
    
    pub fn nk_constraint_set_linear_lower_limit(constraint: nkConstraintHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_constraint_set_linear_upper_limit(constraint: nkConstraintHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_constraint_set_angular_lower_limit(constraint: nkConstraintHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_constraint_set_angular_upper_limit(constraint: nkConstraintHandle, x: nkReal, y: nkReal, z: nkReal);
    
    pub fn nk_constraint_get_linear_lower_limit(constraint: nkConstraintHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_constraint_get_linear_upper_limit(constraint: nkConstraintHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_constraint_get_angular_lower_limit(constraint: nkConstraintHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_constraint_get_angular_upper_limit(constraint: nkConstraintHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    
    pub fn nk_constraint_enable_spring_6dof(constraint: nkConstraintHandle, axis: i32, enable: i32);
    pub fn nk_constraint_set_stiffness_6dof(constraint: nkConstraintHandle, axis: i32, stiffness: nkReal);
    pub fn nk_constraint_set_damping_6dof(constraint: nkConstraintHandle, axis: i32, damping: nkReal);
    pub fn nk_constraint_set_equilibrium_point_6dof(constraint: nkConstraintHandle, axis: i32, val: nkReal);
    
    pub fn nk_constraint_set_limit_cone_twist(
        constraint: nkConstraintHandle,
        swing_span1: nkReal, swing_span2: nkReal, twist_span: nkReal,
        softness: nkReal, bias_factor: nkReal, relaxation_factor: nkReal);
    
    pub fn nk_constraint_set_motor_target_cone_twist(
        constraint: nkConstraintHandle,
        x: nkReal, y: nkReal, z: nkReal, w: nkReal);
    
    pub fn nk_constraint_set_param(constraint: nkConstraintHandle, num: i32, value: nkReal, axis: i32);
    pub fn nk_constraint_get_param(constraint: nkConstraintHandle, num: i32, axis: i32) -> nkReal;
    pub fn nk_constraint_set_breaking_impulse_threshold(constraint: nkConstraintHandle, threshold: nkReal);
    pub fn nk_constraint_get_breaking_impulse_threshold(constraint: nkConstraintHandle) -> nkReal;
    pub fn nk_constraint_set_enabled(constraint: nkConstraintHandle, enabled: i32);
    pub fn nk_constraint_is_enabled(constraint: nkConstraintHandle) -> i32;
    pub fn nk_constraint_get_constraint_type(constraint: nkConstraintHandle) -> i32;
    
    pub fn nk_constraint_destroy(world: nkWorldHandle, constraint: nkConstraintHandle);
    pub fn nk_world_add_constraint(world: nkWorldHandle, constraint: nkConstraintHandle, disable_collisions: i32);
    pub fn nk_world_remove_constraint(world: nkWorldHandle, constraint: nkConstraintHandle);
    
    pub fn nk_world_add_rigid_body_with_filter(world: nkWorldHandle, body: nkRigidBodyHandle, group: i32, mask: i32);
    
    pub fn nk_world_set_contact_callback(world: nkWorldHandle, callback: Option<nkContactCallback>, user_data: *mut std::ffi::c_void);
    pub fn nk_world_set_collision_filter(world: nkWorldHandle, callback: Option<nkCollisionFilterCallback>, user_data: *mut std::ffi::c_void);
    
    pub fn nk_world_set_solver_iterations(world: nkWorldHandle, iterations: i32);
    pub fn nk_world_get_solver_iterations(world: nkWorldHandle) -> i32;
    pub fn nk_world_set_erp(world: nkWorldHandle, erp: nkReal);
    pub fn nk_world_get_erp(world: nkWorldHandle) -> nkReal;
    pub fn nk_world_set_erp2(world: nkWorldHandle, erp2: nkReal);
    pub fn nk_world_get_erp2(world: nkWorldHandle) -> nkReal;
    
    pub fn nk_ghost_create() -> nkGhostObjectHandle;
    pub fn nk_ghost_destroy(ghost: nkGhostObjectHandle);
    pub fn nk_ghost_set_shape(ghost: nkGhostObjectHandle, shape: nkShapeHandle);
    pub fn nk_ghost_set_transform(ghost: nkGhostObjectHandle, transform: *const nkTransform);
    pub fn nk_ghost_get_transform(ghost: nkGhostObjectHandle, out_transform: *mut nkTransform);
    pub fn nk_world_add_ghost(world: nkWorldHandle, ghost: nkGhostObjectHandle);
    pub fn nk_world_remove_ghost(world: nkWorldHandle, ghost: nkGhostObjectHandle);
    pub fn nk_ghost_get_num_overlapping_objects(ghost: nkGhostObjectHandle) -> i32;
    pub fn nk_ghost_get_overlapping_object(ghost: nkGhostObjectHandle, index: i32) -> nkRigidBodyHandle;
    
    pub fn nk_world_add_softbody(world: nkWorldHandle, softbody: nkSoftBodyHandle);
    pub fn nk_world_remove_softbody(world: nkWorldHandle, softbody: nkSoftBodyHandle);
    pub fn nk_world_get_num_softbodies(world: nkWorldHandle) -> i32;
    
    pub fn nk_world_add_vehicle(world: nkWorldHandle, vehicle: nkVehicleHandle);
    pub fn nk_world_remove_vehicle(world: nkWorldHandle, vehicle: nkVehicleHandle);
    
    pub fn nk_world_add_character(world: nkWorldHandle, character: nkCharacterHandle);
    pub fn nk_world_remove_character(world: nkWorldHandle, character: nkCharacterHandle);
}
