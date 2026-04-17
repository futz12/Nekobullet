#include "multibody_constraint.h"
#include "BulletDynamics/Featherstone/btMultiBody.h"
#include "BulletDynamics/Featherstone/btMultiBodyPoint2Point.h"
#include "BulletDynamics/Featherstone/btMultiBodyFixedConstraint.h"
#include "BulletDynamics/Featherstone/btMultiBodySliderConstraint.h"
#include "BulletDynamics/Dynamics/btRigidBody.h"

nkMultiBodyConstraintHandle nk_multibody_constraint_create_p2p(
    nkRigidBodyHandle rigid_body,
    nkReal pivot_a_x, nkReal pivot_a_y, nkReal pivot_a_z,
    nkReal pivot_b_x, nkReal pivot_b_y, nkReal pivot_b_z)
{
    if (!rigid_body) return nullptr;
    
    btRigidBody* rb = static_cast<btRigidBody*>(rigid_body);
    btVector3 pivotA(pivot_a_x, pivot_a_y, pivot_a_z);
    btVector3 pivotB(pivot_b_x, pivot_b_y, pivot_b_z);
    
    btMultiBodyPoint2Point* constraint = new btMultiBodyPoint2Point(
        nullptr, -1, rb, pivotA, pivotB);
    
    return static_cast<nkMultiBodyConstraintHandle>(constraint);
}

nkMultiBodyConstraintHandle nk_multibody_constraint_create_fixed(
    nkRigidBodyHandle rigid_body,
    nkReal pivot_a_x, nkReal pivot_a_y, nkReal pivot_a_z,
    nkReal pivot_b_x, nkReal pivot_b_y, nkReal pivot_b_z,
    nkReal frame_a_00, nkReal frame_a_01, nkReal frame_a_02,
    nkReal frame_a_10, nkReal frame_a_11, nkReal frame_a_12,
    nkReal frame_a_20, nkReal frame_a_21, nkReal frame_a_22,
    nkReal frame_b_00, nkReal frame_b_01, nkReal frame_b_02,
    nkReal frame_b_10, nkReal frame_b_11, nkReal frame_b_12,
    nkReal frame_b_20, nkReal frame_b_21, nkReal frame_b_22)
{
    if (!rigid_body) return nullptr;
    
    btRigidBody* rb = static_cast<btRigidBody*>(rigid_body);
    btVector3 pivotA(pivot_a_x, pivot_a_y, pivot_a_z);
    btVector3 pivotB(pivot_b_x, pivot_b_y, pivot_b_z);
    btMatrix3x3 frameA(
        frame_a_00, frame_a_01, frame_a_02,
        frame_a_10, frame_a_11, frame_a_12,
        frame_a_20, frame_a_21, frame_a_22);
    btMatrix3x3 frameB(
        frame_b_00, frame_b_01, frame_b_02,
        frame_b_10, frame_b_11, frame_b_12,
        frame_b_20, frame_b_21, frame_b_22);
    
    btMultiBodyFixedConstraint* constraint = new btMultiBodyFixedConstraint(
        nullptr, -1, rb, pivotA, pivotB, frameA, frameB);
    
    return static_cast<nkMultiBodyConstraintHandle>(constraint);
}

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
    nkReal axis_x, nkReal axis_y, nkReal axis_z)
{
    if (!rigid_body) return nullptr;
    
    btRigidBody* rb = static_cast<btRigidBody*>(rigid_body);
    btVector3 pivotA(pivot_a_x, pivot_a_y, pivot_a_z);
    btVector3 pivotB(pivot_b_x, pivot_b_y, pivot_b_z);
    btMatrix3x3 frameA(
        frame_a_00, frame_a_01, frame_a_02,
        frame_a_10, frame_a_11, frame_a_12,
        frame_a_20, frame_a_21, frame_a_22);
    btMatrix3x3 frameB(
        frame_b_00, frame_b_01, frame_b_02,
        frame_b_10, frame_b_11, frame_b_12,
        frame_b_20, frame_b_21, frame_b_22);
    btVector3 jointAxis(axis_x, axis_y, axis_z);
    
    btMultiBodySliderConstraint* constraint = new btMultiBodySliderConstraint(
        nullptr, -1, rb, pivotA, pivotB, frameA, frameB, jointAxis);
    
    return static_cast<nkMultiBodyConstraintHandle>(constraint);
}

void nk_multibody_constraint_destroy(nkMultiBodyConstraintHandle constraint)
{
    if (!constraint) return;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    delete c;
}

void nk_multibody_constraint_finalize(nkMultiBodyConstraintHandle constraint)
{
    if (!constraint) return;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    c->finalizeMultiDof();
}

int nk_multibody_constraint_get_type(nkMultiBodyConstraintHandle constraint)
{
    if (!constraint) return -1;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    return c->getConstraintType();
}

int nk_multibody_constraint_get_num_rows(nkMultiBodyConstraintHandle constraint)
{
    if (!constraint) return 0;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    return c->getNumRows();
}

void nk_multibody_constraint_set_max_applied_impulse(nkMultiBodyConstraintHandle constraint, nkReal max_impulse)
{
    if (!constraint) return;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    c->setMaxAppliedImpulse(max_impulse);
}

nkReal nk_multibody_constraint_get_max_applied_impulse(nkMultiBodyConstraintHandle constraint)
{
    if (!constraint) return 0.0f;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    return c->getMaxAppliedImpulse();
}

nkReal nk_multibody_constraint_get_applied_impulse(nkMultiBodyConstraintHandle constraint, int dof)
{
    if (!constraint) return 0.0f;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    return c->getAppliedImpulse(dof);
}

void nk_multibody_constraint_set_pivot_in_b(nkMultiBodyConstraintHandle constraint, nkReal x, nkReal y, nkReal z)
{
    if (!constraint) return;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    c->setPivotInB(btVector3(x, y, z));
}

void nk_multibody_constraint_get_pivot_in_b(nkMultiBodyConstraintHandle constraint, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!constraint) return;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    
    int type = c->getConstraintType();
    if (type == MULTIBODY_CONSTRAINT_POINT_TO_POINT) {
        btMultiBodyPoint2Point* p2p = static_cast<btMultiBodyPoint2Point*>(c);
        const btVector3& pivot = p2p->getPivotInB();
        if (out_x) *out_x = pivot.x();
        if (out_y) *out_y = pivot.y();
        if (out_z) *out_z = pivot.z();
    }
}

void nk_multibody_constraint_set_frame_in_b(nkMultiBodyConstraintHandle constraint,
    nkReal m00, nkReal m01, nkReal m02,
    nkReal m10, nkReal m11, nkReal m12,
    nkReal m20, nkReal m21, nkReal m22)
{
    if (!constraint) return;
    btMultiBodyConstraint* c = static_cast<btMultiBodyConstraint*>(constraint);
    btMatrix3x3 frame(
        m00, m01, m02,
        m10, m11, m12,
        m20, m21, m22);
    c->setFrameInB(frame);
}
