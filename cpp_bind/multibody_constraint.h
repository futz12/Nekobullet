#ifndef NEKOBULLET_MULTIBODY_CONSTRAINT_HPP
#define NEKOBULLET_MULTIBODY_CONSTRAINT_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkMultiBodyConstraintHandle;

nkMultiBodyConstraintHandle nk_multibody_constraint_create_p2p(
    nkRigidBodyHandle rigid_body,
    nkReal pivot_a_x, nkReal pivot_a_y, nkReal pivot_a_z,
    nkReal pivot_b_x, nkReal pivot_b_y, nkReal pivot_b_z);

nkMultiBodyConstraintHandle nk_multibody_constraint_create_fixed(
    nkRigidBodyHandle rigid_body,
    nkReal pivot_a_x, nkReal pivot_a_y, nkReal pivot_a_z,
    nkReal pivot_b_x, nkReal pivot_b_y, nkReal pivot_b_z,
    nkReal frame_a_00, nkReal frame_a_01, nkReal frame_a_02,
    nkReal frame_a_10, nkReal frame_a_11, nkReal frame_a_12,
    nkReal frame_a_20, nkReal frame_a_21, nkReal frame_a_22,
    nkReal frame_b_00, nkReal frame_b_01, nkReal frame_b_02,
    nkReal frame_b_10, nkReal frame_b_11, nkReal frame_b_12,
    nkReal frame_b_20, nkReal frame_b_21, nkReal frame_b_22);

nkMultiBodyConstraintHandle nk_multibody_constraint_create_slider(
    nkRigidBodyHandle rigid_body,
    nkReal pivot_a_x, nkReal pivot_a_y, nkReal pivot_a_z,
    nkReal pivot_b_x, nkReal pivot_b_y, nkReal pivot_b_z,
    nkReal frame_a_00, nkReal frame_a_01, nkReal frame_a_02,
    nkReal frame_a_10, nkReal frame_a_11, nkReal frame_a_12,
    nkReal frame_a_20, nkReal frame_a_21, nkReal frame_a_22,
    nkReal frame_b_00, nkReal frame_b_01, nkReal frame_b_02,
    nkReal frame_b_10, nkReal frame_b_11, nkReal frame_b_12,
    nkReal frame_b_20, nkReal frame_b_21, nkReal frame_b_22,
    nkReal axis_x, nkReal axis_y, nkReal axis_z);

void nk_multibody_constraint_destroy(nkMultiBodyConstraintHandle constraint);

void nk_multibody_constraint_finalize(nkMultiBodyConstraintHandle constraint);
int nk_multibody_constraint_get_type(nkMultiBodyConstraintHandle constraint);
int nk_multibody_constraint_get_num_rows(nkMultiBodyConstraintHandle constraint);

void nk_multibody_constraint_set_max_applied_impulse(nkMultiBodyConstraintHandle constraint, nkReal max_impulse);
nkReal nk_multibody_constraint_get_max_applied_impulse(nkMultiBodyConstraintHandle constraint);

nkReal nk_multibody_constraint_get_applied_impulse(nkMultiBodyConstraintHandle constraint, int dof);

void nk_multibody_constraint_set_pivot_in_b(nkMultiBodyConstraintHandle constraint, nkReal x, nkReal y, nkReal z);
void nk_multibody_constraint_get_pivot_in_b(nkMultiBodyConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z);

void nk_multibody_constraint_set_frame_in_b(nkMultiBodyConstraintHandle constraint,
    nkReal m00, nkReal m01, nkReal m02,
    nkReal m10, nkReal m11, nkReal m12,
    nkReal m20, nkReal m21, nkReal m22);

#ifdef __cplusplus
}
#endif

#endif
