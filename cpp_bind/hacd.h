#ifndef NEKOBULLET_HACD_HPP
#define NEKOBULLET_HACD_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkHACDHandle;

typedef struct nkHACDParams
{
    int maxHullCount;
    int maxVerticesPerHull;
    double concavity;
    double alpha;
    double beta;
    double ccConnectDist;
    int addFacesPoints;
    int addExtraDistPoints;
    int addNeighboursDistPoints;
} nkHACDParams;

nkHACDHandle nk_hacd_create();
void nk_hacd_destroy(nkHACDHandle hacd);

void nk_hacd_set_params(nkHACDHandle hacd, const nkHACDParams* params);
void nk_hacd_get_params(nkHACDHandle hacd, nkHACDParams* outParams);

int nk_hacd_set_points(nkHACDHandle hacd, const double* points, int numPoints);
int nk_hacd_set_triangles(nkHACDHandle hacd, const int* triangles, int numTriangles);

int nk_hacd_compute(nkHACDHandle hacd);

int nk_hacd_get_num_hulls(nkHACDHandle hacd);
int nk_hacd_get_hull_points_count(nkHACDHandle hacd, int hullIndex);
int nk_hacd_get_hull_triangles_count(nkHACDHandle hacd, int hullIndex);
int nk_hacd_get_hull(nkHACDHandle hacd, int hullIndex, double* outPoints, int* outNumPoints, int* outTriangles, int* outNumTriangles);

#ifdef __cplusplus
}
#endif

#endif
