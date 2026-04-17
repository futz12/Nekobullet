#ifndef NEKOBULLET_INVERSE_DYNAMICS_HPP
#define NEKOBULLET_INVERSE_DYNAMICS_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkMultiBodyHandle nk_multibody_create();
void nk_multibody_destroy(nkMultiBodyHandle multibody);

int nk_multibody_add_body(nkMultiBodyHandle multibody,
    int body_index, int parent_index, int joint_type,
    nkReal parent_r_x, nkReal parent_r_y, nkReal parent_r_z,
    nkReal body_T_parent_00, nkReal body_T_parent_01, nkReal body_T_parent_02,
    nkReal body_T_parent_10, nkReal body_T_parent_11, nkReal body_T_parent_12,
    nkReal body_T_parent_20, nkReal body_T_parent_21, nkReal body_T_parent_22,
    nkReal axis_x, nkReal axis_y, nkReal axis_z,
    nkReal mass,
    nkReal com_x, nkReal com_y, nkReal com_z,
    nkReal inertia_xx, nkReal inertia_xy, nkReal inertia_xz,
    nkReal inertia_yy, nkReal inertia_yz, nkReal inertia_zz);
int nk_multibody_finalize(nkMultiBodyHandle multibody);

int nk_multibody_calculate_inverse_dynamics(nkMultiBodyHandle multibody,
    const nkReal* q, const nkReal* u, const nkReal* dot_u,
    nkReal* joint_forces, int num_dofs);

int nk_multibody_calculate_mass_matrix(nkMultiBodyHandle multibody,
    const nkReal* q, int num_q,
    nkReal* mass_matrix, int initialize_matrix, int set_lower_triangular);

int nk_multibody_get_num_bodies(nkMultiBodyHandle multibody);
int nk_multibody_get_num_dofs(nkMultiBodyHandle multibody);

void nk_multibody_set_gravity(nkMultiBodyHandle multibody, nkReal x, nkReal y, nkReal z);
void nk_multibody_set_accept_invalid_mass(nkMultiBodyHandle multibody, int accept);

void nk_multibody_print_tree(nkMultiBodyHandle multibody);

#ifdef __cplusplus
}
#endif

#endif
