#include "ghost.h"
#include "btBulletDynamicsCommon.h"
#include "BulletCollision/CollisionDispatch/btGhostObject.h"

nkGhostObjectHandle nk_ghost_create()
{
    btGhostObject* ghost = new btGhostObject();
    return static_cast<nkGhostObjectHandle>(ghost);
}

void nk_ghost_destroy(nkGhostObjectHandle ghost)
{
    if (!ghost) return;
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    delete ghost_obj;
}

void nk_ghost_set_shape(nkGhostObjectHandle ghost, nkShapeHandle shape)
{
    if (!ghost || !shape) return;
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    ghost_obj->setCollisionShape(collision_shape);
}

void nk_ghost_set_transform(nkGhostObjectHandle ghost, nkTransform* transform)
{
    if (!ghost || !transform) return;
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    btTransform bt_transform;
    bt_transform.setOrigin(btVector3(transform->origin[0], transform->origin[1], transform->origin[2]));
    bt_transform.setRotation(btQuaternion(transform->rotation[0], transform->rotation[1], transform->rotation[2], transform->rotation[3]));
    ghost_obj->setWorldTransform(bt_transform);
}

void nk_ghost_get_transform(nkGhostObjectHandle ghost, nkTransform* out_transform)
{
    if (!ghost || !out_transform) return;
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    btTransform transform = ghost_obj->getWorldTransform();
    out_transform->origin[0] = transform.getOrigin().x();
    out_transform->origin[1] = transform.getOrigin().y();
    out_transform->origin[2] = transform.getOrigin().z();
    out_transform->rotation[0] = transform.getRotation().x();
    out_transform->rotation[1] = transform.getRotation().y();
    out_transform->rotation[2] = transform.getRotation().z();
    out_transform->rotation[3] = transform.getRotation().w();
}

void nk_world_add_ghost(nkWorldHandle world, nkGhostObjectHandle ghost)
{
    if (!world || !ghost) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    dynamics_world->addCollisionObject(ghost_obj);
}

void nk_world_remove_ghost(nkWorldHandle world, nkGhostObjectHandle ghost)
{
    if (!world || !ghost) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    dynamics_world->removeCollisionObject(ghost_obj);
}

int nk_ghost_get_num_overlapping_objects(nkGhostObjectHandle ghost)
{
    if (!ghost) return 0;
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    return ghost_obj->getNumOverlappingObjects();
}

nkRigidBodyHandle nk_ghost_get_overlapping_object(nkGhostObjectHandle ghost, int index)
{
    if (!ghost) return nullptr;
    btGhostObject* ghost_obj = static_cast<btGhostObject*>(ghost);
    btCollisionObject* obj = ghost_obj->getOverlappingObject(index);
    return static_cast<nkRigidBodyHandle>(obj);
}
