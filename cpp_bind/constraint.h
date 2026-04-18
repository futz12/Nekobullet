#ifndef NEKOBULLET_CONSTRAINT_HPP
#define NEKOBULLET_CONSTRAINT_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkConstraintHandle nk_constraint_create_point2point(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal pivot_a_x, nkReal pivot_a_y, nkReal pivot_a_z,
    nkReal pivot_b_x, nkReal pivot_b_y, nkReal pivot_b_z);

nkConstraintHandle nk_constraint_create_hinge(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    nkReal low_limit, nkReal high_limit);

nkConstraintHandle nk_constraint_create_slider(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b);

nkConstraintHandle nk_constraint_create_fixed(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b);

nkConstraintHandle nk_constraint_create_generic_6dof(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    int use_linearReferenceFrameA);

nkConstraintHandle nk_constraint_create_cone_twist(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b);

nkConstraintHandle nk_constraint_create_generic_6dof_spring(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    int use_linearReferenceFrameA);

nkConstraintHandle nk_constraint_create_universal(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal anchor_x, nkReal anchor_y, nkReal anchor_z,
    nkReal axis1_x, nkReal axis1_y, nkReal axis1_z,
    nkReal axis2_x, nkReal axis2_y, nkReal axis2_z);

nkConstraintHandle nk_constraint_create_hinge2(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal anchor_x, nkReal anchor_y, nkReal anchor_z,
    nkReal axis1_x, nkReal axis1_y, nkReal axis1_z,
    nkReal axis2_x, nkReal axis2_y, nkReal axis2_z);

nkConstraintHandle nk_constraint_create_gear(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal axis_a_x, nkReal axis_a_y, nkReal axis_a_z,
    nkReal axis_b_x, nkReal axis_b_y, nkReal axis_b_z);

nkConstraintHandle nk_constraint_create_generic_6dof_spring2(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    int rotate_order);

void nk_constraint_set_linear_lower_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z);
void nk_constraint_set_linear_upper_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z);
void nk_constraint_set_angular_lower_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z);
void nk_constraint_set_angular_upper_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z);

void nk_constraint_get_linear_lower_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_constraint_get_linear_upper_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_constraint_get_angular_lower_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_constraint_get_angular_upper_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z);

void nk_constraint_enable_spring_6dof(nkConstraintHandle constraint, int axis, int enable);
void nk_constraint_set_stiffness_6dof(nkConstraintHandle constraint, int axis, nkReal stiffness);
void nk_constraint_set_damping_6dof(nkConstraintHandle constraint, int axis, nkReal damping);
void nk_constraint_set_equilibrium_point_6dof(nkConstraintHandle constraint, int axis, nkReal val);

void nk_constraint_set_limit_cone_twist(
    nkConstraintHandle constraint,
    nkReal swing_span1, nkReal swing_span2, nkReal twist_span,
    nkReal softness, nkReal bias_factor, nkReal relaxation_factor);

void nk_constraint_set_motor_target_cone_twist(
    nkConstraintHandle constraint,
    nkReal x, nkReal y, nkReal z, nkReal w);

void nk_constraint_set_param(nkConstraintHandle constraint, int num, nkReal value, int axis);
nkReal nk_constraint_get_param(nkConstraintHandle constraint, int num, int axis);
void nk_constraint_set_breaking_impulse_threshold(nkConstraintHandle constraint, nkReal threshold);
nkReal nk_constraint_get_breaking_impulse_threshold(nkConstraintHandle constraint);
void nk_constraint_set_enabled(nkConstraintHandle constraint, int enabled);
int nk_constraint_is_enabled(nkConstraintHandle constraint);

int nk_constraint_get_constraint_type(nkConstraintHandle constraint);

void nk_constraint_destroy(nkWorldHandle world, nkConstraintHandle constraint);
void nk_world_add_constraint(nkWorldHandle world, nkConstraintHandle constraint, int disable_collisions);
void nk_world_remove_constraint(nkWorldHandle world, nkConstraintHandle constraint);

#ifdef __cplusplus
}
#endif

#endif
