use std::ffi::c_void;
use std::ptr::NonNull;

use super::types::{Real, Transform, Vec3};
use super::rigidbody::RigidBody;
use super::world::PhysicsWorld;
use crate::ffi::{self, nkTransform};

#[derive(Debug, Clone, Copy)]
pub struct VehicleTuning {
    pub suspension_stiffness: Real,
    pub suspension_compression: Real,
    pub suspension_damping: Real,
    pub max_suspension_travel_cm: Real,
    pub friction_slip: Real,
    pub max_suspension_force: Real,
}

impl Default for VehicleTuning {
    fn default() -> Self {
        Self {
            suspension_stiffness: 5.88,
            suspension_compression: 0.83,
            suspension_damping: 0.88,
            max_suspension_travel_cm: 500.0,
            friction_slip: 10.5,
            max_suspension_force: 6000.0,
        }
    }
}

impl VehicleTuning {
    pub fn to_ffi(&self) -> ffi::nkVehicleTuning {
        ffi::nkVehicleTuning {
            suspension_stiffness: self.suspension_stiffness,
            suspension_compression: self.suspension_compression,
            suspension_damping: self.suspension_damping,
            max_suspension_travel_cm: self.max_suspension_travel_cm,
            friction_slip: self.friction_slip,
            max_suspension_force: self.max_suspension_force,
        }
    }
}

pub struct VehicleRaycaster {
    handle: NonNull<c_void>,
}

impl VehicleRaycaster {
    pub fn new(world: &PhysicsWorld) -> Self {
        let handle = unsafe { ffi::nk_vehicle_raycaster_create(world.handle()) };
        Self {
            handle: NonNull::new(handle).expect("Failed to create vehicle raycaster"),
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }
}

impl Drop for VehicleRaycaster {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_vehicle_raycaster_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for VehicleRaycaster {}
unsafe impl Sync for VehicleRaycaster {}

pub struct Vehicle {
    handle: NonNull<c_void>,
}

impl Vehicle {
    pub fn new(chassis: &RigidBody, raycaster: &VehicleRaycaster, tuning: &VehicleTuning) -> Self {
        let ffi_tuning = tuning.to_ffi();
        let handle = unsafe {
            ffi::nk_vehicle_create(chassis.handle(), raycaster.handle(), &ffi_tuning)
        };
        Self {
            handle: NonNull::new(handle).expect("Failed to create vehicle"),
        }
    }

    pub fn handle(&self) -> *mut c_void {
        self.handle.as_ptr()
    }

    #[allow(clippy::too_many_arguments)]
    pub fn add_wheel(&mut self, connection: Vec3, direction: Vec3, axle: Vec3,
        suspension_rest_length: Real, wheel_radius: Real, is_front_wheel: bool, tuning: &VehicleTuning) -> i32 {
        let ffi_tuning = tuning.to_ffi();
        unsafe {
            ffi::nk_vehicle_add_wheel(
                self.handle.as_ptr(),
                connection.x, connection.y, connection.z,
                direction.x, direction.y, direction.z,
                axle.x, axle.y, axle.z,
                suspension_rest_length, wheel_radius,
                if is_front_wheel { 1 } else { 0 },
                &ffi_tuning,
            )
        }
    }

    pub fn num_wheels(&self) -> i32 {
        unsafe { ffi::nk_vehicle_get_num_wheels(self.handle.as_ptr()) }
    }

    pub fn set_steering(&self, steering: Real, wheel: i32) {
        unsafe {
            ffi::nk_vehicle_set_steering_value(self.handle.as_ptr(), steering, wheel);
        }
    }

    pub fn steering(&self, wheel: i32) -> Real {
        unsafe { ffi::nk_vehicle_get_steering_value(self.handle.as_ptr(), wheel) }
    }

    pub fn apply_engine_force(&self, force: Real, wheel: i32) {
        unsafe {
            ffi::nk_vehicle_apply_engine_force(self.handle.as_ptr(), force, wheel);
        }
    }

    pub fn set_brake(&self, brake: Real, wheel: i32) {
        unsafe {
            ffi::nk_vehicle_set_brake(self.handle.as_ptr(), brake, wheel);
        }
    }

    pub fn current_speed_km_hour(&self) -> Real {
        unsafe { ffi::nk_vehicle_get_current_speed_km_hour(self.handle.as_ptr()) }
    }

    pub fn forward_vector(&self) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_vehicle_get_forward_vector(self.handle.as_ptr(), &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn set_coordinate_system(&self, right: i32, up: i32, forward: i32) {
        unsafe {
            ffi::nk_vehicle_set_coordinate_system(self.handle.as_ptr(), right, up, forward);
        }
    }

    pub fn reset_suspension(&self) {
        unsafe {
            ffi::nk_vehicle_reset_suspension(self.handle.as_ptr());
        }
    }

    pub fn wheel_transform(&self, wheel_index: i32) -> Transform {
        let mut nk_transform: nkTransform = unsafe { std::mem::zeroed() };
        unsafe {
            ffi::nk_vehicle_get_wheel_transform(self.handle.as_ptr(), wheel_index, &mut nk_transform);
        }
        nk_transform.to_core_transform()
    }

    pub fn update_wheel_transform(&self, wheel_index: i32, interpolated: bool) {
        unsafe {
            ffi::nk_vehicle_update_wheel_transform(self.handle.as_ptr(), wheel_index, if interpolated { 1 } else { 0 });
        }
    }

    pub fn is_wheel_in_contact(&self, wheel_index: i32) -> bool {
        unsafe { ffi::nk_vehicle_is_wheel_in_contact(self.handle.as_ptr(), wheel_index) != 0 }
    }

    pub fn wheel_contact_normal(&self, wheel_index: i32) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_vehicle_get_wheel_contact_normal(self.handle.as_ptr(), wheel_index, &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn wheel_contact_point(&self, wheel_index: i32) -> Vec3 {
        let mut x: Real = 0.0;
        let mut y: Real = 0.0;
        let mut z: Real = 0.0;
        unsafe {
            ffi::nk_vehicle_get_wheel_contact_point(self.handle.as_ptr(), wheel_index, &mut x, &mut y, &mut z);
        }
        Vec3::new(x, y, z)
    }

    pub fn wheel_suspension_length(&self, wheel_index: i32) -> Real {
        unsafe { ffi::nk_vehicle_get_wheel_suspension_length(self.handle.as_ptr(), wheel_index) }
    }

    pub fn wheel_radius(&self, wheel_index: i32) -> Real {
        unsafe { ffi::nk_vehicle_get_wheel_radius(self.handle.as_ptr(), wheel_index) }
    }
}

impl Drop for Vehicle {
    fn drop(&mut self) {
        unsafe {
            ffi::nk_vehicle_destroy(self.handle.as_ptr());
        }
    }
}

unsafe impl Send for Vehicle {}
unsafe impl Sync for Vehicle {}
