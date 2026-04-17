#ifndef NEKOBULLET_VEHICLE_HPP
#define NEKOBULLET_VEHICLE_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct nkVehicleTuning
{
    nkReal suspension_stiffness;
    nkReal suspension_compression;
    nkReal suspension_damping;
    nkReal max_suspension_travel_cm;
    nkReal friction_slip;
    nkReal max_suspension_force;
} nkVehicleTuning;

nkVehicleRaycasterHandle nk_vehicle_raycaster_create(nkWorldHandle world);
void nk_vehicle_raycaster_destroy(nkVehicleRaycasterHandle raycaster);

void nk_vehicle_tuning_init_default(nkVehicleTuning* tuning);

nkVehicleHandle nk_vehicle_create(nkRigidBodyHandle chassis, nkVehicleRaycasterHandle raycaster, nkVehicleTuning* tuning);
void nk_vehicle_destroy(nkVehicleHandle vehicle);

int nk_vehicle_add_wheel(nkVehicleHandle vehicle,
    nkReal connection_x, nkReal connection_y, nkReal connection_z,
    nkReal direction_x, nkReal direction_y, nkReal direction_z,
    nkReal axle_x, nkReal axle_y, nkReal axle_z,
    nkReal suspension_rest_length, nkReal wheel_radius,
    int is_front_wheel, nkVehicleTuning* tuning);
int nk_vehicle_get_num_wheels(nkVehicleHandle vehicle);

void nk_vehicle_set_steering_value(nkVehicleHandle vehicle, nkReal steering, int wheel);
nkReal nk_vehicle_get_steering_value(nkVehicleHandle vehicle, int wheel);
void nk_vehicle_apply_engine_force(nkVehicleHandle vehicle, nkReal force, int wheel);
void nk_vehicle_set_brake(nkVehicleHandle vehicle, nkReal brake, int wheel);
nkReal nk_vehicle_get_current_speed_km_hour(nkVehicleHandle vehicle);
void nk_vehicle_get_forward_vector(nkVehicleHandle vehicle, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_vehicle_set_coordinate_system(nkVehicleHandle vehicle, int right_index, int up_index, int forward_index);
void nk_vehicle_reset_suspension(nkVehicleHandle vehicle);

void nk_vehicle_get_wheel_transform(nkVehicleHandle vehicle, int wheel_index, nkTransform* out_transform);
void nk_vehicle_update_wheel_transform(nkVehicleHandle vehicle, int wheel_index, int interpolated);
int nk_vehicle_is_wheel_in_contact(nkVehicleHandle vehicle, int wheel_index);
void nk_vehicle_get_wheel_contact_normal(nkVehicleHandle vehicle, int wheel_index, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_vehicle_get_wheel_contact_point(nkVehicleHandle vehicle, int wheel_index, nkReal* out_x, nkReal* out_y, nkReal* out_z);
nkReal nk_vehicle_get_wheel_suspension_length(nkVehicleHandle vehicle, int wheel_index);
nkReal nk_vehicle_get_wheel_radius(nkVehicleHandle vehicle, int wheel_index);

#ifdef __cplusplus
}
#endif

#endif
