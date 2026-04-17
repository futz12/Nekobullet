#ifndef NEKOBULLET_RIGIDBODY_HPP
#define NEKOBULLET_RIGIDBODY_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkRigidBodyHandle nk_rigidbody_create(nkWorldHandle world, nkShapeHandle shape, nkReal mass, nkTransform* start_transform);
void nk_rigidbody_destroy(nkWorldHandle world, nkRigidBodyHandle body);

void nk_rigidbody_get_transform(nkRigidBodyHandle body, nkTransform* out_transform);
void nk_rigidbody_set_transform(nkRigidBodyHandle body, nkTransform* transform);
void nk_rigidbody_get_position(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_rigidbody_set_position(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_get_rotation(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z, nkReal* out_w);
void nk_rigidbody_set_rotation(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z, nkReal w);

void nk_rigidbody_set_linear_velocity(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_get_linear_velocity(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_rigidbody_set_angular_velocity(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_get_angular_velocity(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z);

void nk_rigidbody_apply_force(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_apply_impulse(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_apply_torque(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_apply_torque_impulse(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_apply_central_force(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_apply_central_impulse(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_rigidbody_clear_forces(nkRigidBodyHandle body);

nkReal nk_rigidbody_get_mass(nkRigidBodyHandle body);
void nk_rigidbody_set_mass(nkRigidBodyHandle body, nkReal mass);
nkReal nk_rigidbody_get_inverse_mass(nkRigidBodyHandle body);

void nk_rigidbody_set_damping(nkRigidBodyHandle body, nkReal linear, nkReal angular);
nkReal nk_rigidbody_get_linear_damping(nkRigidBodyHandle body);
nkReal nk_rigidbody_get_angular_damping(nkRigidBodyHandle body);

void nk_rigidbody_set_restitution(nkRigidBodyHandle body, nkReal restitution);
nkReal nk_rigidbody_get_restitution(nkRigidBodyHandle body);
void nk_rigidbody_set_friction(nkRigidBodyHandle body, nkReal friction);
nkReal nk_rigidbody_get_friction(nkRigidBodyHandle body);
void nk_rigidbody_set_rolling_friction(nkRigidBodyHandle body, nkReal friction);
nkReal nk_rigidbody_get_rolling_friction(nkRigidBodyHandle body);
void nk_rigidbody_set_spinning_friction(nkRigidBodyHandle body, nkReal friction);
nkReal nk_rigidbody_get_spinning_friction(nkRigidBodyHandle body);

void nk_rigidbody_set_activation_state(nkRigidBodyHandle body, int state);
int nk_rigidbody_get_activation_state(nkRigidBodyHandle body);
void nk_rigidbody_activate(nkRigidBodyHandle body);
int nk_rigidbody_is_active(nkRigidBodyHandle body);
void nk_rigidbody_set_sleeping_thresholds(nkRigidBodyHandle body, nkReal linear, nkReal angular);
nkReal nk_rigidbody_get_linear_sleeping_threshold(nkRigidBodyHandle body);
nkReal nk_rigidbody_get_angular_sleeping_threshold(nkRigidBodyHandle body);

void nk_rigidbody_set_kinematic(nkRigidBodyHandle body, int kinematic);
int nk_rigidbody_is_kinematic(nkRigidBodyHandle body);
void nk_rigidbody_set_static(nkRigidBodyHandle body, int is_static);
int nk_rigidbody_is_static(nkRigidBodyHandle body);
int nk_rigidbody_is_dynamic(nkRigidBodyHandle body);

void nk_rigidbody_set_collision_shape(nkRigidBodyHandle body, nkShapeHandle shape);
void nk_rigidbody_get_aabb(
    nkRigidBodyHandle body,
    nkReal* out_min_x, nkReal* out_min_y, nkReal* out_min_z,
    nkReal* out_max_x, nkReal* out_max_y, nkReal* out_max_z);

void nk_rigidbody_set_collision_group(nkRigidBodyHandle body, int group);
int nk_rigidbody_get_collision_group(nkRigidBodyHandle body);
void nk_rigidbody_set_collision_mask(nkRigidBodyHandle body, int mask);
int nk_rigidbody_get_collision_mask(nkRigidBodyHandle body);

#ifdef __cplusplus
}
#endif

#endif
