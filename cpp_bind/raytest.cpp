#include "raytest.h"
#include "btBulletDynamicsCommon.h"

struct nkClosestRayResultCallback : public btCollisionWorld::ClosestRayResultCallback
{
    nkClosestRayResultCallback(const btVector3& rayFromWorld, const btVector3& rayToWorld)
        : btCollisionWorld::ClosestRayResultCallback(rayFromWorld, rayToWorld)
    {
    }
};

void nk_world_ray_test_closest(
    nkWorldHandle world,
    nkReal from_x, nkReal from_y, nkReal from_z,
    nkReal to_x, nkReal to_y, nkReal to_z,
    nkRayTestResult* out_result)
{
    if (!world || !out_result) return;
    
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    
    btVector3 from(from_x, from_y, from_z);
    btVector3 to(to_x, to_y, to_z);
    
    nkClosestRayResultCallback callback(from, to);
    dynamics_world->rayTest(from, to, callback);
    
    if (callback.hasHit())
    {
        out_result->hit = 1;
        out_result->hit_point[0] = callback.m_hitPointWorld.x();
        out_result->hit_point[1] = callback.m_hitPointWorld.y();
        out_result->hit_point[2] = callback.m_hitPointWorld.z();
        out_result->hit_normal[0] = callback.m_hitNormalWorld.x();
        out_result->hit_normal[1] = callback.m_hitNormalWorld.y();
        out_result->hit_normal[2] = callback.m_hitNormalWorld.z();
        out_result->hit_fraction = callback.m_closestHitFraction;
        out_result->body = static_cast<nkRigidBodyHandle>(const_cast<btCollisionObject*>(callback.m_collisionObject));
    }
    else
    {
        out_result->hit = 0;
        out_result->hit_point[0] = 0;
        out_result->hit_point[1] = 0;
        out_result->hit_point[2] = 0;
        out_result->hit_normal[0] = 0;
        out_result->hit_normal[1] = 0;
        out_result->hit_normal[2] = 0;
        out_result->hit_fraction = 1.0f;
        out_result->body = nullptr;
    }
}

struct nkAllHitsRayResultCallback : public btCollisionWorld::AllHitsRayResultCallback
{
    nkAllHitsRayResultCallback(const btVector3& rayFromWorld, const btVector3& rayToWorld)
        : btCollisionWorld::AllHitsRayResultCallback(rayFromWorld, rayToWorld)
    {
    }
};

void nk_world_ray_test_all(
    nkWorldHandle world,
    nkReal from_x, nkReal from_y, nkReal from_z,
    nkReal to_x, nkReal to_y, nkReal to_z,
    nkRayTestResult* out_results, int max_results, int* out_num_results)
{
    if (!world || !out_results || !out_num_results) return;
    
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    
    btVector3 from(from_x, from_y, from_z);
    btVector3 to(to_x, to_y, to_z);
    
    nkAllHitsRayResultCallback callback(from, to);
    dynamics_world->rayTest(from, to, callback);
    
    int num_hits = 0;
    int count = callback.m_collisionObjects.size();
    
    for (int i = 0; i < count && i < max_results; i++)
    {
        out_results[i].hit = 1;
        out_results[i].hit_point[0] = callback.m_hitPointWorld[i].x();
        out_results[i].hit_point[1] = callback.m_hitPointWorld[i].y();
        out_results[i].hit_point[2] = callback.m_hitPointWorld[i].z();
        out_results[i].hit_normal[0] = callback.m_hitNormalWorld[i].x();
        out_results[i].hit_normal[1] = callback.m_hitNormalWorld[i].y();
        out_results[i].hit_normal[2] = callback.m_hitNormalWorld[i].z();
        out_results[i].hit_fraction = callback.m_hitFractions[i];
        out_results[i].body = static_cast<nkRigidBodyHandle>(const_cast<btCollisionObject*>(callback.m_collisionObjects[i]));
        num_hits++;
    }
    
    *out_num_results = num_hits;
}
