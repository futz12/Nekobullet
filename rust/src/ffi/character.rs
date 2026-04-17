use super::types::{nkCharacterHandle, nkGhostObjectHandle, nkReal, nkShapeHandle};

extern "C" {
    pub fn nk_character_create(
        ghost: nkGhostObjectHandle,
        convex_shape: nkShapeHandle,
        step_height: nkReal,
        up_x: nkReal,
        up_y: nkReal,
        up_z: nkReal,
    ) -> nkCharacterHandle;
    pub fn nk_character_destroy(character: nkCharacterHandle);

    pub fn nk_character_set_walk_direction(
        character: nkCharacterHandle,
        x: nkReal,
        y: nkReal,
        z: nkReal,
    );
    pub fn nk_character_set_velocity_for_time(
        character: nkCharacterHandle,
        vx: nkReal,
        vy: nkReal,
        vz: nkReal,
        time_interval: nkReal,
    );
    pub fn nk_character_set_linear_velocity(
        character: nkCharacterHandle,
        x: nkReal,
        y: nkReal,
        z: nkReal,
    );
    pub fn nk_character_get_linear_velocity(
        character: nkCharacterHandle,
        out_x: *mut nkReal,
        out_y: *mut nkReal,
        out_z: *mut nkReal,
    );
    pub fn nk_character_set_angular_velocity(
        character: nkCharacterHandle,
        x: nkReal,
        y: nkReal,
        z: nkReal,
    );
    pub fn nk_character_get_angular_velocity(
        character: nkCharacterHandle,
        out_x: *mut nkReal,
        out_y: *mut nkReal,
        out_z: *mut nkReal,
    );

    pub fn nk_character_jump(character: nkCharacterHandle, vx: nkReal, vy: nkReal, vz: nkReal);
    pub fn nk_character_can_jump(character: nkCharacterHandle) -> i32;
    pub fn nk_character_on_ground(character: nkCharacterHandle) -> i32;

    pub fn nk_character_set_jump_speed(character: nkCharacterHandle, speed: nkReal);
    pub fn nk_character_get_jump_speed(character: nkCharacterHandle) -> nkReal;
    pub fn nk_character_set_fall_speed(character: nkCharacterHandle, speed: nkReal);
    pub fn nk_character_get_fall_speed(character: nkCharacterHandle) -> nkReal;
    pub fn nk_character_set_max_jump_height(character: nkCharacterHandle, height: nkReal);
    pub fn nk_character_set_max_slope(character: nkCharacterHandle, slope_radians: nkReal);
    pub fn nk_character_get_max_slope(character: nkCharacterHandle) -> nkReal;
    pub fn nk_character_set_step_height(character: nkCharacterHandle, height: nkReal);
    pub fn nk_character_get_step_height(character: nkCharacterHandle) -> nkReal;
    pub fn nk_character_set_gravity(
        character: nkCharacterHandle,
        x: nkReal,
        y: nkReal,
        z: nkReal,
    );
    pub fn nk_character_get_gravity(
        character: nkCharacterHandle,
        out_x: *mut nkReal,
        out_y: *mut nkReal,
        out_z: *mut nkReal,
    );
    pub fn nk_character_set_up(character: nkCharacterHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_character_get_up(
        character: nkCharacterHandle,
        out_x: *mut nkReal,
        out_y: *mut nkReal,
        out_z: *mut nkReal,
    );

    pub fn nk_character_warp(character: nkCharacterHandle, x: nkReal, y: nkReal, z: nkReal);
    pub fn nk_character_reset(character: nkCharacterHandle);
    pub fn nk_character_set_linear_damping(character: nkCharacterHandle, damping: nkReal);
    pub fn nk_character_get_linear_damping(character: nkCharacterHandle) -> nkReal;
    pub fn nk_character_set_angular_damping(character: nkCharacterHandle, damping: nkReal);
    pub fn nk_character_get_angular_damping(character: nkCharacterHandle) -> nkReal;
    pub fn nk_character_set_max_penetration_depth(character: nkCharacterHandle, depth: nkReal);
    pub fn nk_character_get_max_penetration_depth(character: nkCharacterHandle) -> nkReal;
    pub fn nk_character_get_ghost_object(character: nkCharacterHandle) -> nkGhostObjectHandle;
}
