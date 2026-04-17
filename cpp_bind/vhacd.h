#ifndef NEKOBULLET_VHACD_HPP
#define NEKOBULLET_VHACD_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkVHACDHandle;

typedef struct nkVHACDParams
{
    unsigned int resolution;
    int depth;
    double concavity;
    double alpha;
    double beta;
    double gamma;
    unsigned int maxVerticesPerHull;
    double minVolumePerCH;
    int planeDownsampling;
    int convexhullDownsampling;
    int pca;
    int mode;
    int convexhullApproximation;
} nkVHACDParams;

nkVHACDHandle nk_vhacd_create();
void nk_vhacd_destroy(nkVHACDHandle vhacd);

int nk_vhacd_compute(nkVHACDHandle vhacd, 
    const double* points, int numPoints,
    const int* triangles, int numTriangles,
    const nkVHACDParams* params);

void nk_vhacd_cancel(nkVHACDHandle vhacd);

int nk_vhacd_get_num_hulls(nkVHACDHandle vhacd);
int nk_vhacd_get_hull(nkVHACDHandle vhacd, int hullIndex, 
    double* outPoints, int* outNumPoints, 
    int* outTriangles, int* outNumTriangles);

void nk_vhacd_clean(nkVHACDHandle vhacd);

#ifdef __cplusplus
}
#endif

#endif
