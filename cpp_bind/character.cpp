#include "character.h"
#include "BulletDynamics/Character/btKinematicCharacterController.h"
#include "BulletCollision/CollisionDispatch/btGhostObject.h"
#include "BulletCollision/CollisionShapes/btConvexShape.h"

nkCharacterHandle nk_character_create(nkGhostObjectHandle ghost, nkShapeHandle convex_shape, 
    nkReal step_height, nkReal up_x, nkReal up_y, nkReal up_z)
{
    if (!ghost || !convex_shape) return nullptr;
    
    btPairCachingGhostObject* ghost_object = static_cast<btPairCachingGhostObject*>(ghost);
    btConvexShape* convex = static_cast<btConvexShape*>(convex_shape);
    btVector3 up(up_x, up_y, up_z);
    
    btKinematicCharacterController* character = new btKinematicCharacterController(ghost_object, convex, step_height, up);
    return static_cast<nkCharacterHandle>(character);
}

void nk_character_destroy(nkCharacterHandle character)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    delete controller;
}

void nk_character_set_walk_direction(nkCharacterHandle character, nkReal x, nkReal y, nkReal z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setWalkDirection(btVector3(x, y, z));
}

void nk_character_set_velocity_for_time(nkCharacterHandle character, nkReal vx, nkReal vy, nkReal vz, nkReal time_interval)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setVelocityForTimeInterval(btVector3(vx, vy, vz), time_interval);
}

void nk_character_set_linear_velocity(nkCharacterHandle character, nkReal x, nkReal y, nkReal z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setLinearVelocity(btVector3(x, y, z));
}

void nk_character_get_linear_velocity(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    btVector3 velocity = controller->getLinearVelocity();
    *out_x = velocity.x();
    *out_y = velocity.y();
    *out_z = velocity.z();
}

void nk_character_set_angular_velocity(nkCharacterHandle character, nkReal x, nkReal y, nkReal z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setAngularVelocity(btVector3(x, y, z));
}

void nk_character_get_angular_velocity(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    btVector3 velocity = controller->getAngularVelocity();
    *out_x = velocity.x();
    *out_y = velocity.y();
    *out_z = velocity.z();
}

void nk_character_jump(nkCharacterHandle character, nkReal vx, nkReal vy, nkReal vz)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->jump(btVector3(vx, vy, vz));
}

int nk_character_can_jump(nkCharacterHandle character)
{
    if (!character) return 0;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->canJump() ? 1 : 0;
}

int nk_character_on_ground(nkCharacterHandle character)
{
    if (!character) return 0;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->onGround() ? 1 : 0;
}

void nk_character_set_jump_speed(nkCharacterHandle character, nkReal speed)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setJumpSpeed(speed);
}

nkReal nk_character_get_jump_speed(nkCharacterHandle character)
{
    if (!character) return 0.0f;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->getJumpSpeed();
}

void nk_character_set_fall_speed(nkCharacterHandle character, nkReal speed)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setFallSpeed(speed);
}

nkReal nk_character_get_fall_speed(nkCharacterHandle character)
{
    if (!character) return 0.0f;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->getFallSpeed();
}

void nk_character_set_max_jump_height(nkCharacterHandle character, nkReal height)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setMaxJumpHeight(height);
}

void nk_character_set_max_slope(nkCharacterHandle character, nkReal slope_radians)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setMaxSlope(slope_radians);
}

nkReal nk_character_get_max_slope(nkCharacterHandle character)
{
    if (!character) return 0.0f;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->getMaxSlope();
}

void nk_character_set_step_height(nkCharacterHandle character, nkReal height)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setStepHeight(height);
}

nkReal nk_character_get_step_height(nkCharacterHandle character)
{
    if (!character) return 0.0f;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->getStepHeight();
}

void nk_character_set_gravity(nkCharacterHandle character, nkReal x, nkReal y, nkReal z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setGravity(btVector3(x, y, z));
}

void nk_character_get_gravity(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    btVector3 gravity = controller->getGravity();
    *out_x = gravity.x();
    *out_y = gravity.y();
    *out_z = gravity.z();
}

void nk_character_set_up(nkCharacterHandle character, nkReal x, nkReal y, nkReal z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setUp(btVector3(x, y, z));
}

void nk_character_get_up(nkCharacterHandle character, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    btVector3 up = controller->getUp();
    *out_x = up.x();
    *out_y = up.y();
    *out_z = up.z();
}

void nk_character_warp(nkCharacterHandle character, nkReal x, nkReal y, nkReal z)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->warp(btVector3(x, y, z));
}

void nk_character_reset(nkCharacterHandle character)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->reset(nullptr);
}

void nk_character_set_linear_damping(nkCharacterHandle character, nkReal damping)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setLinearDamping(damping);
}

nkReal nk_character_get_linear_damping(nkCharacterHandle character)
{
    if (!character) return 0.0f;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->getLinearDamping();
}

void nk_character_set_angular_damping(nkCharacterHandle character, nkReal damping)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setAngularDamping(damping);
}

nkReal nk_character_get_angular_damping(nkCharacterHandle character)
{
    if (!character) return 0.0f;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->getAngularDamping();
}

void nk_character_set_max_penetration_depth(nkCharacterHandle character, nkReal depth)
{
    if (!character) return;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    controller->setMaxPenetrationDepth(depth);
}

nkReal nk_character_get_max_penetration_depth(nkCharacterHandle character)
{
    if (!character) return 0.0f;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return controller->getMaxPenetrationDepth();
}

nkGhostObjectHandle nk_character_get_ghost_object(nkCharacterHandle character)
{
    if (!character) return nullptr;
    btKinematicCharacterController* controller = static_cast<btKinematicCharacterController*>(character);
    return static_cast<nkGhostObjectHandle>(controller->getGhostObject());
}
