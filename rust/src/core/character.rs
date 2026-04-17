use std::ffi::c_void;
use std::ptr::NonNull;

use super::types::{Real, Vec3};
use super::ghost::GhostObject;
use super::collision::ShapeHandle;
use crate::ffi;

pub struct CharacterController {
    handle: NonNull<c_void>,
}

impl CharacterController {
    pub fn new(ghost: &GhostObject, convex_shape: &ShapeHandle, step_height: Real, up: Vec3) -> Self {
        let handle = unsafe {
            ffi::nk_character_create(ghost.handle(), convex_shape.handle(), step_height, up.x, up.y, up.z)
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create character controller"),
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    pub fn set_walk_direction(&self, direction: Vec3) {
        unsafe {
            ffi::nk_character_set_walk_direction(self.handle.as_ptr(), direction.x, direction.y, direction.z);
        }
    }

    pub fn set_velocity_for_time(&self, velocity: Vec3, time_interval: Real) {
        unsafe {
            ffi::nk_character_set_velocity_for_time(
                self.handle.as_ptr(),
                velocity.x, velocity.y, velocity.z,
                time_interval,
            );
        }
    }

    pub fn set_linear_velocity(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_character_set_linear_velocity(self.handle.as_ptr(), velocity.x, velocity.y, velocity.z);
        }
    }

    pub fn linear_velocity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_character_get_linear_velocity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_angular_velocity(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_character_set_angular_velocity(self.handle.as_ptr(), velocity.x, velocity.y, velocity.z);
        }
    }

    pub fn angular_velocity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_character_get_angular_velocity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn jump(&self, velocity: Vec3) {
        unsafe {
            ffi::nk_character_jump(self.handle.as_ptr(), velocity.x, velocity.y, velocity.z);
        }
    }

    pub fn can_jump(&self) -> bool {
        unsafe { ffi::nk_character_can_jump(self.handle.as_ptr()) != 0 }
    }

    pub fn on_ground(&self) -> bool {
        unsafe { ffi::nk_character_on_ground(self.handle.as_ptr()) != 0 }
    }

    pub fn set_jump_speed(&self, speed: Real) {
        unsafe {
            ffi::nk_character_set_jump_speed(self.handle.as_ptr(), speed);
        }
    }

    pub fn jump_speed(&self) -> Real {
        unsafe { ffi::nk_character_get_jump_speed(self.handle.as_ptr()) }
    }

    pub fn set_fall_speed(&self, speed: Real) {
        unsafe {
            ffi::nk_character_set_fall_speed(self.handle.as_ptr(), speed);
        }
    }

    pub fn fall_speed(&self) -> Real {
        unsafe { ffi::nk_character_get_fall_speed(self.handle.as_ptr()) }
    }

    pub fn set_max_jump_height(&self, height: Real) {
        unsafe {
            ffi::nk_character_set_max_jump_height(self.handle.as_ptr(), height);
        }
    }

    pub fn set_max_slope(&self, slope_radians: Real) {
        unsafe {
            ffi::nk_character_set_max_slope(self.handle.as_ptr(), slope_radians);
        }
    }

    pub fn max_slope(&self) -> Real {
        unsafe { ffi::nk_character_get_max_slope(self.handle.as_ptr()) }
    }

    pub fn set_step_height(&self, height: Real) {
        unsafe {
            ffi::nk_character_set_step_height(self.handle.as_ptr(), height);
        }
    }

    pub fn step_height(&self) -> Real {
        unsafe { ffi::nk_character_get_step_height(self.handle.as_ptr()) }
    }

    pub fn set_gravity(&self, gravity: Vec3) {
        unsafe {
            ffi::nk_character_set_gravity(self.handle.as_ptr(), gravity.x, gravity.y, gravity.z);
        }
    }

    pub fn gravity(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_character_get_gravity(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_up(&self, up: Vec3) {
        unsafe {
            ffi::nk_character_set_up(self.handle.as_ptr(), up.x, up.y, up.z);
        }
    }

    pub fn up(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_character_get_up(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn warp(&self, position: Vec3) {
        unsafe {
            ffi::nk_character_warp(self.handle.as_ptr(), position.x, position.y, position.z);
        }
    }

    pub fn reset(&self) {
        unsafe {
            ffi::nk_character_reset(self.handle.as_ptr());
        }
    }

    pub fn set_linear_damping(&self, damping: Real) {
        unsafe {
            ffi::nk_character_set_linear_damping(self.handle.as_ptr(), damping);
        }
    }

    pub fn linear_damping(&self) -> Real {
        unsafe { ffi::nk_character_get_linear_damping(self.handle.as_ptr()) }
    }

    pub fn set_angular_damping(&self, damping: Real) {
        unsafe {
            ffi::nk_character_set_angular_damping(self.handle.as_ptr(), damping);
        }
    }

    pub fn angular_damping(&self) -> Real {
        unsafe { ffi::nk_character_get_angular_damping(self.handle.as_ptr()) }
    }

    pub fn set_max_penetration_depth(&self, depth: Real) {
        unsafe {
            ffi::nk_character_set_max_penetration_depth(self.handle.as_ptr(), depth);
        }
    }

    pub fn max_penetration_depth(&self) -> Real {
        unsafe { ffi::nk_character_get_max_penetration_depth(self.handle.as_ptr()) }
    }

    pub fn ghost_object(&self) -> *mut c_void {
        unsafe { ffi::nk_character_get_ghost_object(self.handle.as_ptr()) }
    }
}

impl Drop for CharacterController {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_character_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for CharacterController {}
unsafe impl Sync for CharacterController {}
