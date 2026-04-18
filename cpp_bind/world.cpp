#include "world.h"
#include "btBulletDynamicsCommon.h"
#include "BulletSoftBody/btSoftBody.h"
#include "BulletSoftBody/btSoftRigidDynamicsWorld.h"
#include "BulletDynamics/Vehicle/btRaycastVehicle.h"
#include "BulletDynamics/Character/btKinematicCharacterController.h"

class NkOverlapFilterCallback : public btOverlapFilterCallback
{
public:
    nkCollisionFilterCallback m_callback;
    void* m_userData;

    NkOverlapFilterCallback(nkCollisionFilterCallback callback, void* userData)
        : m_callback(callback), m_userData(userData)
    {
    }

    virtual bool needBroadphaseCollision(btBroadphaseProxy* proxy0, btBroadphaseProxy* proxy1) const override
    {
        if (!m_callback)
            return true;

        btCollisionObject* obj0 = static_cast<btCollisionObject*>(proxy0->m_clientObject);
        btCollisionObject* obj1 = static_cast<btCollisionObject*>(proxy1->m_clientObject);

        btRigidBody* body0 = btRigidBody::upcast(obj0);
        btRigidBody* body1 = btRigidBody::upcast(obj1);

        if (!body0 || !body1)
            return true;

        return m_callback(static_cast<nkRigidBodyHandle>(body0), static_cast<nkRigidBodyHandle>(body1), m_userData) != 0;
    }
};

nkWorldHandle nk_world_create()
{
    btDefaultCollisionConfiguration* collision_config = new btDefaultCollisionConfiguration();
    btCollisionDispatcher* dispatcher = new btCollisionDispatcher(collision_config);
    btBroadphaseInterface* broadphase = new btDbvtBroadphase();
    btSequentialImpulseConstraintSolver* solver = new btSequentialImpulseConstraintSolver();
    btDiscreteDynamicsWorld* world = new btDiscreteDynamicsWorld(dispatcher, broadphase, solver, collision_config);
    return static_cast<nkWorldHandle>(world);
}

void nk_world_destroy(nkWorldHandle world)
{
    if (!world) return;
    
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    
    int num_constraints = dynamics_world->getNumConstraints();
    for (int i = num_constraints - 1; i >= 0; i--)
    {
        btTypedConstraint* constraint = dynamics_world->getConstraint(i);
        dynamics_world->removeConstraint(constraint);
        delete constraint;
    }
    
    int num_collision_objects = dynamics_world->getNumCollisionObjects();
    for (int i = num_collision_objects - 1; i >= 0; i--)
    {
        btCollisionObject* obj = dynamics_world->getCollisionObjectArray()[i];
        btRigidBody* body = btRigidBody::upcast(obj);
        if (body && body->getMotionState())
        {
            delete body->getMotionState();
        }
        dynamics_world->removeCollisionObject(obj);
        delete obj;
    }
    
    btCollisionDispatcher* dispatcher = static_cast<btCollisionDispatcher*>(dynamics_world->getDispatcher());
    btBroadphaseInterface* broadphase = dynamics_world->getBroadphase();
    btConstraintSolver* solver = dynamics_world->getConstraintSolver();
    btCollisionConfiguration* config = dispatcher->getCollisionConfiguration();
    
    delete dynamics_world;
    delete solver;
    delete broadphase;
    delete dispatcher;
    delete config;
}

void nk_world_set_gravity(nkWorldHandle world, nkReal x, nkReal y, nkReal z)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    dynamics_world->setGravity(btVector3(x, y, z));
}

void nk_world_get_gravity(nkWorldHandle world, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btVector3 gravity = dynamics_world->getGravity();
    *out_x = gravity.x();
    *out_y = gravity.y();
    *out_z = gravity.z();
}

void nk_world_step_simulation(nkWorldHandle world, nkReal time_step, int max_sub_steps, nkReal fixed_time_step)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    dynamics_world->stepSimulation(time_step, max_sub_steps, fixed_time_step);
}

int nk_world_get_num_rigid_bodies(nkWorldHandle world)
{
    if (!world) return 0;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    return dynamics_world->getNumCollisionObjects();
}

int nk_world_get_num_constraints(nkWorldHandle world)
{
    if (!world) return 0;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    return dynamics_world->getNumConstraints();
}

int nk_world_get_num_collision_objects(nkWorldHandle world)
{
    if (!world) return 0;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    return dynamics_world->getNumCollisionObjects();
}

void nk_world_clear_forces(nkWorldHandle world)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    dynamics_world->clearForces();
}

void nk_world_add_rigid_body(nkWorldHandle world, nkRigidBodyHandle body)
{
    if (!world || !body) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    dynamics_world->addRigidBody(rigid_body);
}

void nk_world_remove_rigid_body(nkWorldHandle world, nkRigidBodyHandle body)
{
    if (!world || !body) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    dynamics_world->removeRigidBody(rigid_body);
}

void nk_world_set_time_step(nkWorldHandle world, nkReal time_step)
{
    (void)world;
    (void)time_step;
}

nkReal nk_world_get_time_step(nkWorldHandle world)
{
    (void)world;
    return 1.0f / 60.0f;
}

void nk_world_set_max_sub_steps(nkWorldHandle world, int max_sub_steps)
{
    (void)world;
    (void)max_sub_steps;
}

int nk_world_get_max_sub_steps(nkWorldHandle world)
{
    (void)world;
    return 1;
}

void nk_world_set_contact_breaking_threshold(nkWorldHandle world, nkReal threshold)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    dynamics_world->getSolverInfo().m_linearSlop = threshold;
}

nkReal nk_world_get_contact_breaking_threshold(nkWorldHandle world)
{
    if (!world) return 0.0f;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    return dynamics_world->getSolverInfo().m_linearSlop;
}

void nk_world_add_rigid_body_with_filter(nkWorldHandle world, nkRigidBodyHandle body, int group, int mask)
{
    if (!world || !body) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btRigidBody* rigid_body = static_cast<btRigidBody*>(body);
    dynamics_world->addRigidBody(rigid_body, group, mask);
}

void nk_world_set_contact_callback(nkWorldHandle world, nkContactCallback callback, void* user_data)
{
    (void)world;
    (void)callback;
    (void)user_data;
}

void nk_world_set_collision_filter(nkWorldHandle world, nkCollisionFilterCallback callback, void* user_data)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btOverlappingPairCache* pair_cache = dynamics_world->getBroadphase()->getOverlappingPairCache();
    btOverlapFilterCallback* old_callback = pair_cache->getOverlapFilterCallback();
    if (old_callback)
    {
        delete old_callback;
    }
    if (callback)
    {
        NkOverlapFilterCallback* filter = new NkOverlapFilterCallback(callback, user_data);
        pair_cache->setOverlapFilterCallback(filter);
    }
    else
    {
        pair_cache->setOverlapFilterCallback(nullptr);
    }
}

void nk_world_set_solver_iterations(nkWorldHandle world, int iterations)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    dynamics_world->getSolverInfo().m_numIterations = iterations;
}

int nk_world_get_solver_iterations(nkWorldHandle world)
{
    if (!world) return 10;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    return dynamics_world->getSolverInfo().m_numIterations;
}

void nk_world_set_erp(nkWorldHandle world, nkReal erp)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    dynamics_world->getSolverInfo().m_erp = erp;
}

nkReal nk_world_get_erp(nkWorldHandle world)
{
    if (!world) return 0.2f;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    return dynamics_world->getSolverInfo().m_erp;
}

void nk_world_set_erp2(nkWorldHandle world, nkReal erp2)
{
    if (!world) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    dynamics_world->getSolverInfo().m_erp2 = erp2;
}

nkReal nk_world_get_erp2(nkWorldHandle world)
{
    if (!world) return 0.2f;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    return dynamics_world->getSolverInfo().m_erp2;
}

void nk_world_add_softbody(nkWorldHandle world, nkSoftBodyHandle softbody)
{
    if (!world || !softbody) return;
    btSoftRigidDynamicsWorld* dynamics_world = static_cast<btSoftRigidDynamicsWorld*>(world);
    btSoftBody* sb = static_cast<btSoftBody*>(softbody);
    dynamics_world->addSoftBody(sb);
}

void nk_world_remove_softbody(nkWorldHandle world, nkSoftBodyHandle softbody)
{
    if (!world || !softbody) return;
    btSoftRigidDynamicsWorld* dynamics_world = static_cast<btSoftRigidDynamicsWorld*>(world);
    btSoftBody* sb = static_cast<btSoftBody*>(softbody);
    dynamics_world->removeSoftBody(sb);
}

int nk_world_get_num_softbodies(nkWorldHandle world)
{
    if (!world) return 0;
    btSoftRigidDynamicsWorld* dynamics_world = static_cast<btSoftRigidDynamicsWorld*>(world);
    return dynamics_world->getSoftBodyArray().size();
}

void nk_world_add_vehicle(nkWorldHandle world, nkVehicleHandle vehicle)
{
    if (!world || !vehicle) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btRaycastVehicle* v = static_cast<btRaycastVehicle*>(vehicle);
    dynamics_world->addVehicle(v);
}

void nk_world_remove_vehicle(nkWorldHandle world, nkVehicleHandle vehicle)
{
    if (!world || !vehicle) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btRaycastVehicle* v = static_cast<btRaycastVehicle*>(vehicle);
    dynamics_world->removeVehicle(v);
}

void nk_world_add_character(nkWorldHandle world, nkCharacterHandle character)
{
    if (!world || !character) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btKinematicCharacterController* c = static_cast<btKinematicCharacterController*>(character);
    dynamics_world->addCharacter(c);
}

void nk_world_remove_character(nkWorldHandle world, nkCharacterHandle character)
{
    if (!world || !character) return;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btKinematicCharacterController* c = static_cast<btKinematicCharacterController*>(character);
    dynamics_world->removeCharacter(c);
}
