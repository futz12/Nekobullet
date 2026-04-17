#ifndef NEKOBULLET_RAYTEST_HPP
#define NEKOBULLET_RAYTEST_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef struct nkRayTestResult
{
    int hit;
    nkVector3 hit_point;
    nkVector3 hit_normal;
    nkReal hit_fraction;
    nkRigidBodyHandle body;
} nkRayTestResult;

void nk_world_ray_test_closest(
    nkWorldHandle world,
    nkReal from_x, nkReal from_y, nkReal from_z,
    nkReal to_x, nkReal to_y, nkReal to_z,
    nkRayTestResult* out_result);

void nk_world_ray_test_all(
    nkWorldHandle world,
    nkReal from_x, nkReal from_y, nkReal from_z,
    nkReal to_x, nkReal to_y, nkReal to_z,
    nkRayTestResult* out_results, int max_results, int* out_num_results);

#ifdef __cplusplus
}
#endif

#endif
