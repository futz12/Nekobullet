use super::types::{nkReal, nkWorldHandle, nkRigidBodyHandle, nkVehicleHandle, nkVehicleRaycasterHandle, nkTransform};

#[repr(C)]
pub struct nkVehicleTuning {
    pub suspension_stiffness: nkReal,
    pub suspension_compression: nkReal,
    pub suspension_damping: nkReal,
    pub max_suspension_travel_cm: nkReal,
    pub friction_slip: nkReal,
    pub max_suspension_force: nkReal,
}

extern "C" {
    pub fn nk_vehicle_raycaster_create(world: nkWorldHandle) -> nkVehicleRaycasterHandle;
    pub fn nk_vehicle_raycaster_destroy(raycaster: nkVehicleRaycasterHandle);
    
    pub fn nk_vehicle_tuning_init_default(tuning: *mut nkVehicleTuning);
    
    pub fn nk_vehicle_create(chassis: nkRigidBodyHandle, raycaster: nkVehicleRaycasterHandle, tuning: *const nkVehicleTuning) -> nkVehicleHandle;
    pub fn nk_vehicle_destroy(vehicle: nkVehicleHandle);
    
    pub fn nk_vehicle_add_wheel(vehicle: nkVehicleHandle,
        connection_x: nkReal, connection_y: nkReal, connection_z: nkReal,
        direction_x: nkReal, direction_y: nkReal, direction_z: nkReal,
        axle_x: nkReal, axle_y: nkReal, axle_z: nkReal,
        suspension_rest_length: nkReal, wheel_radius: nkReal,
        is_front_wheel: i32, tuning: *const nkVehicleTuning) -> i32;
    pub fn nk_vehicle_get_num_wheels(vehicle: nkVehicleHandle) -> i32;
    
    pub fn nk_vehicle_set_steering_value(vehicle: nkVehicleHandle, steering: nkReal, wheel: i32);
    pub fn nk_vehicle_get_steering_value(vehicle: nkVehicleHandle, wheel: i32) -> nkReal;
    pub fn nk_vehicle_apply_engine_force(vehicle: nkVehicleHandle, force: nkReal, wheel: i32);
    pub fn nk_vehicle_set_brake(vehicle: nkVehicleHandle, brake: nkReal, wheel: i32);
    pub fn nk_vehicle_get_current_speed_km_hour(vehicle: nkVehicleHandle) -> nkReal;
    pub fn nk_vehicle_get_forward_vector(vehicle: nkVehicleHandle, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_vehicle_set_coordinate_system(vehicle: nkVehicleHandle, right_index: i32, up_index: i32, forward_index: i32);
    pub fn nk_vehicle_reset_suspension(vehicle: nkVehicleHandle);
    
    pub fn nk_vehicle_get_wheel_transform(vehicle: nkVehicleHandle, wheel_index: i32, out_transform: *mut nkTransform);
    pub fn nk_vehicle_update_wheel_transform(vehicle: nkVehicleHandle, wheel_index: i32, interpolated: i32);
    pub fn nk_vehicle_is_wheel_in_contact(vehicle: nkVehicleHandle, wheel_index: i32) -> i32;
    pub fn nk_vehicle_get_wheel_contact_normal(vehicle: nkVehicleHandle, wheel_index: i32, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_vehicle_get_wheel_contact_point(vehicle: nkVehicleHandle, wheel_index: i32, out_x: *mut nkReal, out_y: *mut nkReal, out_z: *mut nkReal);
    pub fn nk_vehicle_get_wheel_suspension_length(vehicle: nkVehicleHandle, wheel_index: i32) -> nkReal;
    pub fn nk_vehicle_get_wheel_radius(vehicle: nkVehicleHandle, wheel_index: i32) -> nkReal;
}
