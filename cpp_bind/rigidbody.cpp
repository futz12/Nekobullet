#include "rigidbody.h"
#include "btBulletDynamicsCommon.h"

nkRigidBodyHandle nk_rigidbody_create(nkWorldHandle world, nkShapeHandle shape, nkReal mass, nkTransform* start_transform)
{
    if (!shape || !start_transform) return nullptr;
    
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    
    btVector3 local_inertia(0, 0, 0);
    if (mass > 0.0f)
    {
        collision_shape->calculateLocalInertia(mass, local_inertia);
    }
    
    btTransform transform;
    transform.setOrigin(btVector3(
        start_transform->origin[0],
        start_transform->origin[1],
        start_transform->origin[2]
    ));
    transform.setRotation(btQuaternion(
        start_transform->rotation[0],
        start_transform->rotation[1],
        start_transform->rotation[2],
        start_transform->rotation[3]
    ));
    
    btDefaultMotionState* motion_state = new btDefaultMotionState(transform);
    btRigidBody::btRigidBodyConstructionInfo rb_info(mass, motion_state, collision_shape, local_inertia);
    btRigidBody* body = new btRigidBody(rb_info);
    
    if (world)
    {
        btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
        dynamics_world->addRigidBody(body);
    }
    
    return static_cast<nkRigidBodyHandle>(body);
}

void nk_rigidbody_destroy(nkWorldHandle world, nkRigidBodyHandle body)
{
    if (!world || !body) return;
    
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    
    dynamics_world->removeRigidBody(rigid_body);
    
    if (rigid_body->getMotionState())
    {
        delete rigid_body->getMotionState();
    }
    delete rigid_body;
}

void nk_rigidbody_get_transform(nkRigidBodyHandle body, nkTransform* out_transform)
{
    if (!body || !out_transform) return;
    
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btTransform transform = rigid_body->getWorldTransform();
    
    out_transform->origin[0] = transform.getOrigin().x();
    out_transform->origin[1] = transform.getOrigin().y();
    out_transform->origin[2] = transform.getOrigin().z();
    
    out_transform->rotation[0] = transform.getRotation().x();
    out_transform->rotation[1] = transform.getRotation().y();
    out_transform->rotation[2] = transform.getRotation().z();
    out_transform->rotation[3] = transform.getRotation().w();
}

void nk_rigidbody_set_transform(nkRigidBodyHandle body, nkTransform* transform)
{
    if (!body || !transform) return;
    
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btTransform bt_transform;
    bt_transform.setOrigin(btVector3(
        transform->origin[0],
        transform->origin[1],
        transform->origin[2]
    ));
    bt_transform.setRotation(btQuaternion(
        transform->rotation[0],
        transform->rotation[1],
        transform->rotation[2],
        transform->rotation[3]
    ));
    
    rigid_body->setWorldTransform(bt_transform);
    if (rigid_body->getMotionState())
    {
        rigid_body->getMotionState()->setWorldTransform(bt_transform);
    }
}

void nk_rigidbody_get_position(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btVector3 pos = rigid_body->getWorldTransform().getOrigin();
    *out_x = pos.x();
    *out_y = pos.y();
    *out_z = pos.z();
}

void nk_rigidbody_set_position(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btTransform transform = rigid_body->getWorldTransform();
    transform.setOrigin(btVector3(x, y, z));
    rigid_body->setWorldTransform(transform);
    if (rigid_body->getMotionState())
    {
        rigid_body->getMotionState()->setWorldTransform(transform);
    }
}

void nk_rigidbody_get_rotation(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z, nkReal* out_w)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btQuaternion rot = rigid_body->getWorldTransform().getRotation();
    *out_x = rot.x();
    *out_y = rot.y();
    *out_z = rot.z();
    *out_w = rot.w();
}

void nk_rigidbody_set_rotation(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z, nkReal w)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btTransform transform = rigid_body->getWorldTransform();
    transform.setRotation(btQuaternion(x, y, z, w));
    rigid_body->setWorldTransform(transform);
    if (rigid_body->getMotionState())
    {
        rigid_body->getMotionState()->setWorldTransform(transform);
    }
}

void nk_rigidbody_set_linear_velocity(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setLinearVelocity(btVector3(x, y, z));
}

void nk_rigidbody_get_linear_velocity(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btVector3 vel = rigid_body->getLinearVelocity();
    *out_x = vel.x();
    *out_y = vel.y();
    *out_z = vel.z();
}

void nk_rigidbody_set_angular_velocity(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setAngularVelocity(btVector3(x, y, z));
}

void nk_rigidbody_get_angular_velocity(nkRigidBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btVector3 vel = rigid_body->getAngularVelocity();
    *out_x = vel.x();
    *out_y = vel.y();
    *out_z = vel.z();
}

void nk_rigidbody_apply_force(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->applyForce(btVector3(x, y, z), btVector3(0, 0, 0));
}

void nk_rigidbody_apply_impulse(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->applyImpulse(btVector3(x, y, z), btVector3(0, 0, 0));
}

void nk_rigidbody_apply_torque(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->applyTorque(btVector3(x, y, z));
}

void nk_rigidbody_apply_torque_impulse(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->applyTorqueImpulse(btVector3(x, y, z));
}

void nk_rigidbody_apply_central_force(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->applyCentralForce(btVector3(x, y, z));
}

void nk_rigidbody_apply_central_impulse(nkRigidBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->applyCentralImpulse(btVector3(x, y, z));
}

void nk_rigidbody_clear_forces(nkRigidBodyHandle body)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->clearForces();
}

nkReal nk_rigidbody_get_mass(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    nkReal inv_mass = rigid_body->getInvMass();
    if (inv_mass > 0.0f)
        return 1.0f / inv_mass;
    return 0.0f;
}

void nk_rigidbody_set_mass(nkRigidBodyHandle body, nkReal mass)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btVector3 inertia(0, 0, 0);
    if (mass > 0.0f && rigid_body->getCollisionShape())
    {
        rigid_body->getCollisionShape()->calculateLocalInertia(mass, inertia);
    }
    rigid_body->setMassProps(mass, inertia);
}

nkReal nk_rigidbody_get_inverse_mass(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getInvMass();
}

void nk_rigidbody_set_damping(nkRigidBodyHandle body, nkReal linear, nkReal angular)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setDamping(linear, angular);
}

nkReal nk_rigidbody_get_linear_damping(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getLinearDamping();
}

nkReal nk_rigidbody_get_angular_damping(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getAngularDamping();
}

void nk_rigidbody_set_restitution(nkRigidBodyHandle body, nkReal restitution)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setRestitution(restitution);
}

nkReal nk_rigidbody_get_restitution(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getRestitution();
}

void nk_rigidbody_set_friction(nkRigidBodyHandle body, nkReal friction)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setFriction(friction);
}

nkReal nk_rigidbody_get_friction(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getFriction();
}

void nk_rigidbody_set_rolling_friction(nkRigidBodyHandle body, nkReal friction)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setRollingFriction(friction);
}

nkReal nk_rigidbody_get_rolling_friction(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getRollingFriction();
}

void nk_rigidbody_set_spinning_friction(nkRigidBodyHandle body, nkReal friction)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setSpinningFriction(friction);
}

nkReal nk_rigidbody_get_spinning_friction(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getSpinningFriction();
}

void nk_rigidbody_set_activation_state(nkRigidBodyHandle body, int state)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setActivationState(state);
}

int nk_rigidbody_get_activation_state(nkRigidBodyHandle body)
{
    if (!body) return 0;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getActivationState();
}

void nk_rigidbody_activate(nkRigidBodyHandle body)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->activate();
}

int nk_rigidbody_is_active(nkRigidBodyHandle body)
{
    if (!body) return 0;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->isActive() ? 1 : 0;
}

void nk_rigidbody_set_sleeping_thresholds(nkRigidBodyHandle body, nkReal linear, nkReal angular)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->setSleepingThresholds(linear, angular);
}

nkReal nk_rigidbody_get_linear_sleeping_threshold(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getLinearSleepingThreshold();
}

nkReal nk_rigidbody_get_angular_sleeping_threshold(nkRigidBodyHandle body)
{
    if (!body) return 0.0f;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getAngularSleepingThreshold();
}

void nk_rigidbody_set_kinematic(nkRigidBodyHandle body, int kinematic)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    if (kinematic)
    {
        rigid_body->setCollisionFlags(rigid_body->getCollisionFlags() | btCollisionObject::CF_KINEMATIC_OBJECT);
    }
    else
    {
        rigid_body->setCollisionFlags(rigid_body->getCollisionFlags() & ~btCollisionObject::CF_KINEMATIC_OBJECT);
    }
}

int nk_rigidbody_is_kinematic(nkRigidBodyHandle body)
{
    if (!body) return 0;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return (rigid_body->getCollisionFlags() & btCollisionObject::CF_KINEMATIC_OBJECT) ? 1 : 0;
}

void nk_rigidbody_set_static(nkRigidBodyHandle body, int is_static)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    if (is_static)
    {
        rigid_body->setCollisionFlags(rigid_body->getCollisionFlags() | btCollisionObject::CF_STATIC_OBJECT);
    }
    else
    {
        rigid_body->setCollisionFlags(rigid_body->getCollisionFlags() & ~btCollisionObject::CF_STATIC_OBJECT);
    }
}

int nk_rigidbody_is_static(nkRigidBodyHandle body)
{
    if (!body) return 0;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->isStaticObject() ? 1 : 0;
}

int nk_rigidbody_is_dynamic(nkRigidBodyHandle body)
{
    if (!body) return 0;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->isStaticOrKinematicObject() ? 0 : 1;
}

void nk_rigidbody_set_collision_shape(nkRigidBodyHandle body, nkShapeHandle shape)
{
    if (!body || !shape) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    rigid_body->setCollisionShape(collision_shape);
}

void nk_rigidbody_get_aabb(
    nkRigidBodyHandle body,
    nkReal* out_min_x, nkReal* out_min_y, nkReal* out_min_z,
    nkReal* out_max_x, nkReal* out_max_y, nkReal* out_max_z)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    btVector3 min_aabb, max_aabb;
    rigid_body->getAabb(min_aabb, max_aabb);
    *out_min_x = min_aabb.x();
    *out_min_y = min_aabb.y();
    *out_min_z = min_aabb.z();
    *out_max_x = max_aabb.x();
    *out_max_y = max_aabb.y();
    *out_max_z = max_aabb.z();
}

void nk_rigidbody_set_collision_group(nkRigidBodyHandle body, int group)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->getBroadphaseHandle()->m_collisionFilterGroup = group;
}

int nk_rigidbody_get_collision_group(nkRigidBodyHandle body)
{
    if (!body) return 0;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getBroadphaseHandle()->m_collisionFilterGroup;
}

void nk_rigidbody_set_collision_mask(nkRigidBodyHandle body, int mask)
{
    if (!body) return;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    rigid_body->getBroadphaseHandle()->m_collisionFilterMask = mask;
}

int nk_rigidbody_get_collision_mask(nkRigidBodyHandle body)
{
    if (!body) return 0;
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    return rigid_body->getBroadphaseHandle()->m_collisionFilterMask;
}
