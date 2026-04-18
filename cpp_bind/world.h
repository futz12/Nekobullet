#ifndef NEKOBULLET_WORLD_HPP
#define NEKOBULLET_WORLD_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkWorldHandle nk_world_create();
void nk_world_destroy(nkWorldHandle world);
void nk_world_set_gravity(nkWorldHandle world, nkReal x, nkReal y, nkReal z);
void nk_world_get_gravity(nkWorldHandle world, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_world_step_simulation(nkWorldHandle world, nkReal time_step, int max_sub_steps, nkReal fixed_time_step);
int nk_world_get_num_rigid_bodies(nkWorldHandle world);
int nk_world_get_num_constraints(nkWorldHandle world);
int nk_world_get_num_collision_objects(nkWorldHandle world);
void nk_world_clear_forces(nkWorldHandle world);
void nk_world_add_rigid_body(nkWorldHandle world, nkRigidBodyHandle body);
void nk_world_remove_rigid_body(nkWorldHandle world, nkRigidBodyHandle body);
void nk_world_set_time_step(nkWorldHandle world, nkReal time_step);
nkReal nk_world_get_time_step(nkWorldHandle world);
void nk_world_set_max_sub_steps(nkWorldHandle world, int max_sub_steps);
int nk_world_get_max_sub_steps(nkWorldHandle world);
void nk_world_set_contact_breaking_threshold(nkWorldHandle world, nkReal threshold);
nkReal nk_world_get_contact_breaking_threshold(nkWorldHandle world);
void nk_world_add_rigid_body_with_filter(nkWorldHandle world, nkRigidBodyHandle body, int group, int mask);

void nk_world_set_solver_iterations(nkWorldHandle world, int iterations);
int nk_world_get_solver_iterations(nkWorldHandle world);
void nk_world_set_erp(nkWorldHandle world, nkReal erp);
nkReal nk_world_get_erp(nkWorldHandle world);
void nk_world_set_erp2(nkWorldHandle world, nkReal erp2);
nkReal nk_world_get_erp2(nkWorldHandle world);

typedef struct nkContactPoint
{
    nkVector3 position;
    nkVector3 normal;
    nkReal distance;
    nkRigidBodyHandle body_a;
    nkRigidBodyHandle body_b;
} nkContactPoint;

typedef void (*nkContactCallback)(nkContactPoint* contact, void* user_data);
void nk_world_set_contact_callback(nkWorldHandle world, nkContactCallback callback, void* user_data);

typedef int (*nkCollisionFilterCallback)(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    void* user_data);
void nk_world_set_collision_filter(nkWorldHandle world, nkCollisionFilterCallback callback, void* user_data);

void nk_world_add_softbody(nkWorldHandle world, nkSoftBodyHandle softbody);
void nk_world_remove_softbody(nkWorldHandle world, nkSoftBodyHandle softbody);
int nk_world_get_num_softbodies(nkWorldHandle world);

void nk_world_add_vehicle(nkWorldHandle world, nkVehicleHandle vehicle);
void nk_world_remove_vehicle(nkWorldHandle world, nkVehicleHandle vehicle);

void nk_world_add_character(nkWorldHandle world, nkCharacterHandle character);
void nk_world_remove_character(nkWorldHandle world, nkCharacterHandle character);

#ifdef __cplusplus
}
#endif

#endif
