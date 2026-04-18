#include "constraint.h"
#include "btBulletDynamicsCommon.h"
#include "BulletDynamics/ConstraintSolver/btUniversalConstraint.h"
#include "BulletDynamics/ConstraintSolver/btHinge2Constraint.h"
#include "BulletDynamics/ConstraintSolver/btGearConstraint.h"
#include "BulletDynamics/ConstraintSolver/btGeneric6DofSpring2Constraint.h"

nkConstraintHandle nk_constraint_create_point2point(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal pivot_a_x, nkReal pivot_a_y, nkReal pivot_a_z,
    nkReal pivot_b_x, nkReal pivot_b_y, nkReal pivot_b_z)
{
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btPoint2PointConstraint* constraint = new btPoint2PointConstraint(
        *rb_a, *rb_b,
        btVector3(pivot_a_x, pivot_a_y, pivot_a_z),
        btVector3(pivot_b_x, pivot_b_y, pivot_b_z)
    );
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_hinge(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    nkReal low_limit, nkReal high_limit)
{
    if (!frame_a || !frame_b) return nullptr;
    
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btTransform bt_frame_a, bt_frame_b;
    bt_frame_a.setOrigin(btVector3(frame_a->origin[0], frame_a->origin[1], frame_a->origin[2]));
    bt_frame_a.setRotation(btQuaternion(frame_a->rotation[0], frame_a->rotation[1], frame_a->rotation[2], frame_a->rotation[3]));
    bt_frame_b.setOrigin(btVector3(frame_b->origin[0], frame_b->origin[1], frame_b->origin[2]));
    bt_frame_b.setRotation(btQuaternion(frame_b->rotation[0], frame_b->rotation[1], frame_b->rotation[2], frame_b->rotation[3]));
    
    btHingeConstraint* constraint = new btHingeConstraint(*rb_a, *rb_b, bt_frame_a, bt_frame_b);
    constraint->setLimit(low_limit, high_limit);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_slider(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b)
{
    if (!frame_a || !frame_b) return nullptr;
    
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btTransform bt_frame_a, bt_frame_b;
    bt_frame_a.setOrigin(btVector3(frame_a->origin[0], frame_a->origin[1], frame_a->origin[2]));
    bt_frame_a.setRotation(btQuaternion(frame_a->rotation[0], frame_a->rotation[1], frame_a->rotation[2], frame_a->rotation[3]));
    bt_frame_b.setOrigin(btVector3(frame_b->origin[0], frame_b->origin[1], frame_b->origin[2]));
    bt_frame_b.setRotation(btQuaternion(frame_b->rotation[0], frame_b->rotation[1], frame_b->rotation[2], frame_b->rotation[3]));
    
    btSliderConstraint* constraint = new btSliderConstraint(*rb_a, *rb_b, bt_frame_a, bt_frame_b, true);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_fixed(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b)
{
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btFixedConstraint* constraint = new btFixedConstraint(*rb_a, *rb_b, btTransform::getIdentity(), btTransform::getIdentity());
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_generic_6dof(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    int use_linearReferenceFrameA)
{
    if (!frame_a || !frame_b) return nullptr;
    
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btTransform bt_frame_a, bt_frame_b;
    bt_frame_a.setOrigin(btVector3(frame_a->origin[0], frame_a->origin[1], frame_a->origin[2]));
    bt_frame_a.setRotation(btQuaternion(frame_a->rotation[0], frame_a->rotation[1], frame_a->rotation[2], frame_a->rotation[3]));
    bt_frame_b.setOrigin(btVector3(frame_b->origin[0], frame_b->origin[1], frame_b->origin[2]));
    bt_frame_b.setRotation(btQuaternion(frame_b->rotation[0], frame_b->rotation[1], frame_b->rotation[2], frame_b->rotation[3]));
    
    btGeneric6DofConstraint* constraint = new btGeneric6DofConstraint(
        *rb_a, *rb_b, bt_frame_a, bt_frame_b, use_linearReferenceFrameA != 0);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_cone_twist(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b)
{
    if (!frame_a || !frame_b) return nullptr;
    
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btTransform bt_frame_a, bt_frame_b;
    bt_frame_a.setOrigin(btVector3(frame_a->origin[0], frame_a->origin[1], frame_a->origin[2]));
    bt_frame_a.setRotation(btQuaternion(frame_a->rotation[0], frame_a->rotation[1], frame_a->rotation[2], frame_a->rotation[3]));
    bt_frame_b.setOrigin(btVector3(frame_b->origin[0], frame_b->origin[1], frame_b->origin[2]));
    bt_frame_b.setRotation(btQuaternion(frame_b->rotation[0], frame_b->rotation[1], frame_b->rotation[2], frame_b->rotation[3]));
    
    btConeTwistConstraint* constraint = new btConeTwistConstraint(*rb_a, *rb_b, bt_frame_a, bt_frame_b);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_generic_6dof_spring(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    int use_linearReferenceFrameA)
{
    if (!frame_a || !frame_b) return nullptr;
    
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btTransform bt_frame_a, bt_frame_b;
    bt_frame_a.setOrigin(btVector3(frame_a->origin[0], frame_a->origin[1], frame_a->origin[2]));
    bt_frame_a.setRotation(btQuaternion(frame_a->rotation[0], frame_a->rotation[1], frame_a->rotation[2], frame_a->rotation[3]));
    bt_frame_b.setOrigin(btVector3(frame_b->origin[0], frame_b->origin[1], frame_b->origin[2]));
    bt_frame_b.setRotation(btQuaternion(frame_b->rotation[0], frame_b->rotation[1], frame_b->rotation[2], frame_b->rotation[3]));
    
    btGeneric6DofSpringConstraint* constraint = new btGeneric6DofSpringConstraint(
        *rb_a, *rb_b, bt_frame_a, bt_frame_b, use_linearReferenceFrameA != 0);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_universal(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal anchor_x, nkReal anchor_y, nkReal anchor_z,
    nkReal axis1_x, nkReal axis1_y, nkReal axis1_z,
    nkReal axis2_x, nkReal axis2_y, nkReal axis2_z)
{
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btVector3 anchor(anchor_x, anchor_y, anchor_z);
    btVector3 axis1(axis1_x, axis1_y, axis1_z);
    btVector3 axis2(axis2_x, axis2_y, axis2_z);
    
    btUniversalConstraint* constraint = new btUniversalConstraint(*rb_a, *rb_b, anchor, axis1, axis2);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_hinge2(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal anchor_x, nkReal anchor_y, nkReal anchor_z,
    nkReal axis1_x, nkReal axis1_y, nkReal axis1_z,
    nkReal axis2_x, nkReal axis2_y, nkReal axis2_z)
{
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btVector3 anchor(anchor_x, anchor_y, anchor_z);
    btVector3 axis1(axis1_x, axis1_y, axis1_z);
    btVector3 axis2(axis2_x, axis2_y, axis2_z);
    
    btHinge2Constraint* constraint = new btHinge2Constraint(*rb_a, *rb_b, anchor, axis1, axis2);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_gear(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkReal axis_a_x, nkReal axis_a_y, nkReal axis_a_z,
    nkReal axis_b_x, nkReal axis_b_y, nkReal axis_b_z)
{
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btVector3 axis_a(axis_a_x, axis_a_y, axis_a_z);
    btVector3 axis_b(axis_b_x, axis_b_y, axis_b_z);
    
    btGearConstraint* constraint = new btGearConstraint(*rb_a, *rb_b, axis_a, axis_b);
    
    return static_cast<nkConstraintHandle>(constraint);
}

nkConstraintHandle nk_constraint_create_generic_6dof_spring2(
    nkRigidBodyHandle body_a, nkRigidBodyHandle body_b,
    nkTransform* frame_a, nkTransform* frame_b,
    int rotate_order)
{
    if (!frame_a || !frame_b) return nullptr;
    
    btRigidBody* rb_a = static_cast<btRigidBody*>(body_a);
    btRigidBody* rb_b = static_cast<btRigidBody*>(body_b);
    
    btTransform bt_frame_a, bt_frame_b;
    bt_frame_a.setOrigin(btVector3(frame_a->origin[0], frame_a->origin[1], frame_a->origin[2]));
    bt_frame_a.setRotation(btQuaternion(frame_a->rotation[0], frame_a->rotation[1], frame_a->rotation[2], frame_a->rotation[3]));
    bt_frame_b.setOrigin(btVector3(frame_b->origin[0], frame_b->origin[1], frame_b->origin[2]));
    bt_frame_b.setRotation(btQuaternion(frame_b->rotation[0], frame_b->rotation[1], frame_b->rotation[2], frame_b->rotation[3]));
    
    RotateOrder ro = RO_XYZ;
    switch (rotate_order) {
        case 0: ro = RO_XYZ; break;
        case 1: ro = RO_XZY; break;
        case 2: ro = RO_YXZ; break;
        case 3: ro = RO_YZX; break;
        case 4: ro = RO_ZXY; break;
        case 5: ro = RO_ZYX; break;
        default: ro = RO_XYZ; break;
    }
    
    btGeneric6DofSpring2Constraint* constraint = new btGeneric6DofSpring2Constraint(
        *rb_a, *rb_b, bt_frame_a, bt_frame_b, ro);
    
    return static_cast<nkConstraintHandle>(constraint);
}

void nk_constraint_set_linear_lower_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    dof_constraint->setLinearLowerLimit(btVector3(x, y, z));
}

void nk_constraint_set_linear_upper_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    dof_constraint->setLinearUpperLimit(btVector3(x, y, z));
}

void nk_constraint_set_angular_lower_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    dof_constraint->setAngularLowerLimit(btVector3(x, y, z));
}

void nk_constraint_set_angular_upper_limit(nkConstraintHandle constraint, nkReal x, nkReal y, nkReal z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    dof_constraint->setAngularUpperLimit(btVector3(x, y, z));
}

void nk_constraint_get_linear_lower_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    btVector3 limit;
    dof_constraint->getLinearLowerLimit(limit);
    *out_x = limit.x();
    *out_y = limit.y();
    *out_z = limit.z();
}

void nk_constraint_get_linear_upper_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    btVector3 limit;
    dof_constraint->getLinearUpperLimit(limit);
    *out_x = limit.x();
    *out_y = limit.y();
    *out_z = limit.z();
}

void nk_constraint_get_angular_lower_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    btVector3 limit;
    dof_constraint->getAngularLowerLimit(limit);
    *out_x = limit.x();
    *out_y = limit.y();
    *out_z = limit.z();
}

void nk_constraint_get_angular_upper_limit(nkConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!constraint) return;
    btGeneric6DofConstraint* dof_constraint = static_cast<btGeneric6DofConstraint*>(constraint);
    btVector3 limit;
    dof_constraint->getAngularUpperLimit(limit);
    *out_x = limit.x();
    *out_y = limit.y();
    *out_z = limit.z();
}

void nk_constraint_enable_spring_6dof(nkConstraintHandle constraint, int axis, int enable)
{
    if (!constraint) return;
    btGeneric6DofSpringConstraint* spring_constraint = static_cast<btGeneric6DofSpringConstraint*>(constraint);
    spring_constraint->enableSpring(axis, enable != 0);
}

void nk_constraint_set_stiffness_6dof(nkConstraintHandle constraint, int axis, nkReal stiffness)
{
    if (!constraint) return;
    btGeneric6DofSpringConstraint* spring_constraint = static_cast<btGeneric6DofSpringConstraint*>(constraint);
    spring_constraint->setStiffness(axis, stiffness);
}

void nk_constraint_set_damping_6dof(nkConstraintHandle constraint, int axis, nkReal damping)
{
    if (!constraint) return;
    btGeneric6DofSpringConstraint* spring_constraint = static_cast<btGeneric6DofSpringConstraint*>(constraint);
    spring_constraint->setDamping(axis, damping);
}

void nk_constraint_set_equilibrium_point_6dof(nkConstraintHandle constraint, int axis, nkReal val)
{
    if (!constraint) return;
    btGeneric6DofSpringConstraint* spring_constraint = static_cast<btGeneric6DofSpringConstraint*>(constraint);
    spring_constraint->setEquilibriumPoint(axis, val);
}

void nk_constraint_set_limit_cone_twist(
    nkConstraintHandle constraint,
    nkReal swing_span1, nkReal swing_span2, nkReal twist_span,
    nkReal softness, nkReal bias_factor, nkReal relaxation_factor)
{
    if (!constraint) return;
    btConeTwistConstraint* cone_constraint = static_cast<btConeTwistConstraint*>(constraint);
    cone_constraint->setLimit(swing_span1, swing_span2, twist_span, softness, bias_factor, relaxation_factor);
}

void nk_constraint_set_motor_target_cone_twist(
    nkConstraintHandle constraint,
    nkReal x, nkReal y, nkReal z, nkReal w)
{
    if (!constraint) return;
    btConeTwistConstraint* cone_constraint = static_cast<btConeTwistConstraint*>(constraint);
    cone_constraint->setMotorTarget(btQuaternion(x, y, z, w));
}

void nk_constraint_set_param(nkConstraintHandle constraint, int num, nkReal value, int axis)
{
    if (!constraint) return;
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    typed_constraint->setParam(num, value, axis);
}

nkReal nk_constraint_get_param(nkConstraintHandle constraint, int num, int axis)
{
    if (!constraint) return 0.0f;
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    return typed_constraint->getParam(num, axis);
}

void nk_constraint_set_breaking_impulse_threshold(nkConstraintHandle constraint, nkReal threshold)
{
    if (!constraint) return;
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    typed_constraint->setBreakingImpulseThreshold(threshold);
}

nkReal nk_constraint_get_breaking_impulse_threshold(nkConstraintHandle constraint)
{
    if (!constraint) return 0.0f;
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    return typed_constraint->getBreakingImpulseThreshold();
}

void nk_constraint_set_enabled(nkConstraintHandle constraint, int enabled)
{
    if (!constraint) return;
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    typed_constraint->setEnabled(enabled != 0);
}

int nk_constraint_is_enabled(nkConstraintHandle constraint)
{
    if (!constraint) return 0;
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    return typed_constraint->isEnabled() ? 1 : 0;
}

int nk_constraint_get_constraint_type(nkConstraintHandle constraint)
{
    if (!constraint) return -1;
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    
    switch (typed_constraint->getConstraintType())
    {
        case POINT2POINT_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_POINT2POINT;
        case HINGE_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_HINGE;
        case SLIDER_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_SLIDER;
        case D6_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_GENERIC_6DOF;
        case CONETWIST_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_CONE_TWIST;
        case D6_SPRING_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_GENERIC_6DOF_SPRING;
        case FIXED_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_FIXED;
        case GEAR_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_GEAR;
        case D6_SPRING_2_CONSTRAINT_TYPE: return NK_CONSTRAINT_TYPE_GENERIC_6DOF_SPRING2;
        default: return -1;
    }
}

void nk_constraint_destroy(nkWorldHandle world, nkConstraintHandle constraint)
{
    if (!world || !constraint) return;
    
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    
    dynamics_world->removeConstraint(typed_constraint);
    delete typed_constraint;
}

void nk_world_add_constraint(nkWorldHandle world, nkConstraintHandle constraint, int disable_collisions)
{
    if (!world || !constraint) return;
    
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    
    dynamics_world->addConstraint(typed_constraint, disable_collisions != 0);
}

void nk_world_remove_constraint(nkWorldHandle world, nkConstraintHandle constraint)
{
    if (!world || !constraint) return;
    
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btTypedConstraint* typed_constraint = static_cast<btTypedConstraint*>(constraint);
    
    dynamics_world->removeConstraint(typed_constraint);
}
