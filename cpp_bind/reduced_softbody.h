#ifndef NEKOBULLET_REDUCED_SOFTBODY_HPP
#define NEKOBULLET_REDUCED_SOFTBODY_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkReducedDeformableBodyHandle;

nkReducedDeformableBodyHandle nk_reduced_softbody_create(
    nkSoftBodyWorldInfoHandle world_info,
    int node_count,
    const nkReal* positions,
    const nkReal* masses);

void nk_reduced_softbody_destroy(nkReducedDeformableBodyHandle body);

void nk_reduced_softbody_set_reduced_modes(nkReducedDeformableBodyHandle body, int num_modes, int full_size);
int nk_reduced_softbody_get_num_reduced_modes(nkReducedDeformableBodyHandle body);
int nk_reduced_softbody_get_num_full_dofs(nkReducedDeformableBodyHandle body);

void nk_reduced_softbody_set_stiffness_scale(nkReducedDeformableBodyHandle body, nkReal ks);
void nk_reduced_softbody_set_mass_scale(nkReducedDeformableBodyHandle body, nkReal rho);
void nk_reduced_softbody_set_damping(nkReducedDeformableBodyHandle body, nkReal alpha, nkReal beta);
void nk_reduced_softbody_set_fixed_node(nkReducedDeformableBodyHandle body, int node_index);
void nk_reduced_softbody_disable_reduced_modes(nkReducedDeformableBodyHandle body, int rigid_only);

void nk_reduced_softbody_set_rigid_velocity(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_reduced_softbody_set_rigid_angular_velocity(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_reduced_softbody_get_rigid_velocity(nkReducedDeformableBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_reduced_softbody_get_rigid_angular_velocity(nkReducedDeformableBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z);

nkReal nk_reduced_softbody_get_total_mass(nkReducedDeformableBodyHandle body);
void nk_reduced_softbody_set_total_mass(nkReducedDeformableBodyHandle body, nkReal mass);

void nk_reduced_softbody_get_rigid_transform(nkReducedDeformableBodyHandle body, nkTransform* out_transform);
void nk_reduced_softbody_set_rigid_transform(nkReducedDeformableBodyHandle body, nkTransform* transform);

void nk_reduced_softbody_apply_central_impulse(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z);
void nk_reduced_softbody_apply_torque_impulse(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z);

int nk_reduced_softbody_get_num_nodes(nkReducedDeformableBodyHandle body);
void nk_reduced_softbody_get_node_position(nkReducedDeformableBodyHandle body, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_reduced_softbody_get_node_rest_position(nkReducedDeformableBodyHandle body, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z);

nkReducedDeformableBodyHandle nk_reduced_softbody_create_from_vtk(
    nkSoftBodyWorldInfoHandle world_info,
    const char* vtk_file_path);

void nk_reduced_softbody_read_reduced_info(nkReducedDeformableBodyHandle body, const char* file_path);

#ifdef __cplusplus
}
#endif

#endif
