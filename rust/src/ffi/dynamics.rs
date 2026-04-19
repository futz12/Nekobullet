use super::types::{nkReal, nkRigidBodyHandle, nkShapeHandle, nkTransform, nkWorldHandle};

extern "C" {
    pub fn nk_rigidbody_create(
        world: nkWorldHandle,
        shape: nkShapeHandle,
        mass: nkReal,
        start_transform: *const nkTransform,
        additional_damping: i32,
    ) -> nkRigidBodyHandle;
    pub fn nk_rigidbody_destroy(world: nkWorldHandle, body: nkRigidBodyHandle);
    pub fn nk_rigidbody_get_transform(body: nkRigidBodyHandle, out_transform: *mut nkTransform);
    pub fn nk_rigidbody_set_transform(body: nkRigidBodyHandle, transform: *const nkTransform);
    pub fn nk_rigidbody_get_position(body: nkRigidBodyHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_rigidbody_set_position(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_get_rotation(body: nkRigidBodyHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal, out_w: *mut nkReal);
    pub fn nk_rigidbody_set_rotation(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal, w: nkReal);
    pub fn nk_rigidbody_set_linear_velocity(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_get_linear_velocity(body: nkRigidBodyHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_rigidbody_set_angular_velocity(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_get_angular_velocity(body: nkRigidBodyHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_rigidbody_apply_force(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_apply_impulse(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_apply_torque(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_apply_torque_impulse(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_apply_central_force(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_apply_central_impulse(body: nkRigidBodyHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_rigidbody_clear_forces(body: nkRigidBodyHandle);
    pub fn nk_rigidbody_get_mass(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_mass(body: nkRigidBodyHandle, mass: nkReal);
    pub fn nk_rigidbody_get_inverse_mass(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_damping(body: nkRigidBodyHandle, linear: nkReal, angular: nkReal);
    pub fn nk_rigidbody_get_linear_damping(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_get_angular_damping(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_restitution(body: nkRigidBodyHandle, restitution: nkReal);
    pub fn nk_rigidbody_get_restitution(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_friction(body: nkRigidBodyHandle, friction: nkReal);
    pub fn nk_rigidbody_get_friction(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_rolling_friction(body: nkRigidBodyHandle, friction: nkReal);
    pub fn nk_rigidbody_get_rolling_friction(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_spinning_friction(body: nkRigidBodyHandle, friction: nkReal);
    pub fn nk_rigidbody_get_spinning_friction(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_activation_state(body: nkRigidBodyHandle, state: i32);
    pub fn nk_rigidbody_get_activation_state(body: nkRigidBodyHandle) -> i32;
    pub fn nk_rigidbody_activate(body: nkRigidBodyHandle);
    pub fn nk_rigidbody_is_active(body: nkRigidBodyHandle) -> i32;
    pub fn nk_rigidbody_set_sleeping_thresholds(body: nkRigidBodyHandle, linear: nkReal, angular: nkReal);
    pub fn nk_rigidbody_get_linear_sleeping_threshold(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_get_angular_sleeping_threshold(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_kinematic(body: nkRigidBodyHandle, kinematic: i32);
    pub fn nk_rigidbody_is_kinematic(body: nkRigidBodyHandle) -> i32;
    pub fn nk_rigidbody_set_static(body: nkRigidBodyHandle, is_static: i32);
    pub fn nk_rigidbody_is_static(body: nkRigidBodyHandle) -> i32;
    pub fn nk_rigidbody_is_dynamic(body: nkRigidBodyHandle) -> i32;
    pub fn nk_rigidbody_set_collision_shape(body: nkRigidBodyHandle, shape: nkShapeHandle);
    pub fn nk_rigidbody_get_aabb(
        body: nkRigidBodyHandle,
        out_min_x: *mut nkReal,
        out_min_y: *mut nkReal,
        out_min_z: *mut nkReal,
        out_max_x: *mut nkReal,
        out_max_y: *mut nkReal,
        out_max_z: *mut nkReal,
    );
    pub fn nk_rigidbody_set_collision_group(body: nkRigidBodyHandle, group: i32);
    pub fn nk_rigidbody_get_collision_group(body: nkRigidBodyHandle) -> i32;
    pub fn nk_rigidbody_set_collision_mask(body: nkRigidBodyHandle, mask: i32);
    pub fn nk_rigidbody_get_collision_mask(body: nkRigidBodyHandle) -> i32;
    pub fn nk_rigidbody_set_ccd_motion_threshold(body: nkRigidBodyHandle, threshold: nkReal);
    pub fn nk_rigidbody_get_ccd_motion_threshold(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_ccd_swept_sphere_radius(body: nkRigidBodyHandle, radius: nkReal);
    pub fn nk_rigidbody_get_ccd_swept_sphere_radius(body: nkRigidBodyHandle) -> nkReal;
    pub fn nk_rigidbody_set_no_contact_response(body: nkRigidBodyHandle, no_contact_response: i32);
}
