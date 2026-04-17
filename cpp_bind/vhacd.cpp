#include "vhacd.h"
#include "../Extras/VHACD/public/VHACD.h"

nkVHACDHandle nk_vhacd_create()
{
    VHACD::IVHACD* vhacd = VHACD::CreateVHACD();
    return static_cast<nkVHACDHandle>(vhacd);
}

void nk_vhacd_destroy(nkVHACDHandle vhacd)
{
    if (!vhacd) return;
    VHACD::IVHACD* impl = static_cast<VHACD::IVHACD*>(vhacd);
    impl->Release();
}

int nk_vhacd_compute(nkVHACDHandle vhacd, 
    const double* points, int numPoints,
    const int* triangles, int numTriangles,
    const nkVHACDParams* params)
{
    if (!vhacd || !points || !triangles || !params) return 0;
    if (numPoints <= 0 || numTriangles <= 0) return 0;
    
    VHACD::IVHACD* impl = static_cast<VHACD::IVHACD*>(vhacd);
    
    VHACD::IVHACD::Parameters vhacdParams;
    vhacdParams.m_resolution = params->resolution;
    vhacdParams.m_depth = params->depth;
    vhacdParams.m_concavity = params->concavity;
    vhacdParams.m_alpha = params->alpha;
    vhacdParams.m_beta = params->beta;
    vhacdParams.m_gamma = params->gamma;
    vhacdParams.m_maxNumVerticesPerCH = params->maxVerticesPerHull;
    vhacdParams.m_minVolumePerCH = params->minVolumePerCH;
    vhacdParams.m_planeDownsampling = params->planeDownsampling;
    vhacdParams.m_convexhullDownsampling = params->convexhullDownsampling;
    vhacdParams.m_pca = params->pca;
    vhacdParams.m_mode = params->mode;
    vhacdParams.m_convexhullApproximation = params->convexhullApproximation;
    
    return impl->Compute(
        points,
        sizeof(double) * 3,
        static_cast<unsigned int>(numPoints),
        triangles,
        sizeof(int) * 3,
        static_cast<unsigned int>(numTriangles),
        vhacdParams
    ) ? 1 : 0;
}

void nk_vhacd_cancel(nkVHACDHandle vhacd)
{
    if (!vhacd) return;
    VHACD::IVHACD* impl = static_cast<VHACD::IVHACD*>(vhacd);
    impl->Cancel();
}

int nk_vhacd_get_num_hulls(nkVHACDHandle vhacd)
{
    if (!vhacd) return 0;
    VHACD::IVHACD* impl = static_cast<VHACD::IVHACD*>(vhacd);
    return static_cast<int>(impl->GetNConvexHulls());
}

int nk_vhacd_get_hull(nkVHACDHandle vhacd, int hullIndex, 
    double* outPoints, int* outNumPoints, 
    int* outTriangles, int* outNumTriangles)
{
    if (!vhacd) return 0;
    VHACD::IVHACD* impl = static_cast<VHACD::IVHACD*>(vhacd);
    
    VHACD::IVHACD::ConvexHull hull;
    impl->GetConvexHull(static_cast<unsigned int>(hullIndex), hull);
    
    if (outNumPoints)
    {
        *outNumPoints = static_cast<int>(hull.m_nPoints);
    }
    
    if (outNumTriangles)
    {
        *outNumTriangles = static_cast<int>(hull.m_nTriangles);
    }
    
    if (outPoints && hull.m_points)
    {
        for (unsigned int i = 0; i < hull.m_nPoints * 3; i++)
        {
            outPoints[i] = hull.m_points[i];
        }
    }
    
    if (outTriangles && hull.m_triangles)
    {
        for (unsigned int i = 0; i < hull.m_nTriangles * 3; i++)
        {
            outTriangles[i] = hull.m_triangles[i];
        }
    }
    
    return 1;
}

void nk_vhacd_clean(nkVHACDHandle vhacd)
{
    if (!vhacd) return;
    VHACD::IVHACD* impl = static_cast<VHACD::IVHACD*>(vhacd);
    impl->Clean();
}
