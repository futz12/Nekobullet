#include "hacd.h"
#include "../Extras/HACD/hacdHACD.h"
#include <vector>

nkHACDHandle nk_hacd_create()
{
    HACD::HACD* hacd = new HACD::HACD();
    return static_cast<nkHACDHandle>(hacd);
}

void nk_hacd_destroy(nkHACDHandle hacd)
{
    if (!hacd) return;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    delete hacdInst;
}

void nk_hacd_set_params(nkHACDHandle hacd, const nkHACDParams* params)
{
    if (!hacd || !params) return;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    
    hacdInst->SetNClusters(params->maxHullCount);
    hacdInst->SetNVerticesPerCH(params->maxVerticesPerHull);
    hacdInst->SetConcavity(params->concavity);
    hacdInst->SetCompacityWeight(params->alpha);
    hacdInst->SetVolumeWeight(params->beta);
    hacdInst->SetConnectDist(params->ccConnectDist);
    hacdInst->SetAddFacesPoints(params->addFacesPoints != 0);
    hacdInst->SetAddExtraDistPoints(params->addExtraDistPoints != 0);
    hacdInst->SetAddNeighboursDistPoints(params->addNeighboursDistPoints != 0);
}

void nk_hacd_get_params(nkHACDHandle hacd, nkHACDParams* outParams)
{
    if (!hacd || !outParams) return;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    
    outParams->maxHullCount = static_cast<int>(hacdInst->GetNClusters());
    outParams->maxVerticesPerHull = static_cast<int>(hacdInst->GetNVerticesPerCH());
    outParams->concavity = hacdInst->GetConcavity();
    outParams->alpha = hacdInst->GetCompacityWeight();
    outParams->beta = hacdInst->GetVolumeWeight();
    outParams->ccConnectDist = hacdInst->GetConnectDist();
    outParams->addFacesPoints = hacdInst->GetAddFacesPoints() ? 1 : 0;
    outParams->addExtraDistPoints = hacdInst->GetAddExtraDistPoints() ? 1 : 0;
    outParams->addNeighboursDistPoints = hacdInst->GetAddNeighboursDistPoints() ? 1 : 0;
}

int nk_hacd_set_points(nkHACDHandle hacd, const double* points, int numPoints)
{
    if (!hacd || !points || numPoints <= 0) return 0;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    
    std::vector<HACD::Vec3<HACD::Real>>* vecPoints = new std::vector<HACD::Vec3<HACD::Real>>(numPoints);
    for (int i = 0; i < numPoints; i++)
    {
        (*vecPoints)[i] = HACD::Vec3<HACD::Real>(points[i * 3], points[i * 3 + 1], points[i * 3 + 2]);
    }
    
    hacdInst->SetPoints(&(*vecPoints)[0]);
    hacdInst->SetNPoints(numPoints);
    
    return 1;
}

int nk_hacd_set_triangles(nkHACDHandle hacd, const int* triangles, int numTriangles)
{
    if (!hacd || !triangles || numTriangles <= 0) return 0;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    
    std::vector<HACD::Vec3<long>>* vecTriangles = new std::vector<HACD::Vec3<long>>(numTriangles);
    for (int i = 0; i < numTriangles; i++)
    {
        (*vecTriangles)[i] = HACD::Vec3<long>(triangles[i * 3], triangles[i * 3 + 1], triangles[i * 3 + 2]);
    }
    
    hacdInst->SetTriangles(&(*vecTriangles)[0]);
    hacdInst->SetNTriangles(numTriangles);
    
    return 1;
}

int nk_hacd_compute(nkHACDHandle hacd)
{
    if (!hacd) return 0;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    
    hacdInst->NormalizeData();
    bool result = hacdInst->Compute(false, false);
    
    return result ? 1 : 0;
}

int nk_hacd_get_num_hulls(nkHACDHandle hacd)
{
    if (!hacd) return 0;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    return static_cast<int>(hacdInst->GetNClusters());
}

int nk_hacd_get_hull_points_count(nkHACDHandle hacd, int hullIndex)
{
    if (!hacd) return 0;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    return static_cast<int>(hacdInst->GetNPointsCH(hullIndex));
}

int nk_hacd_get_hull_triangles_count(nkHACDHandle hacd, int hullIndex)
{
    if (!hacd) return 0;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    return static_cast<int>(hacdInst->GetNTrianglesCH(hullIndex));
}

int nk_hacd_get_hull(nkHACDHandle hacd, int hullIndex, double* outPoints, int* outNumPoints, int* outTriangles, int* outNumTriangles)
{
    if (!hacd || hullIndex < 0) return 0;
    HACD::HACD* hacdInst = static_cast<HACD::HACD*>(hacd);
    
    size_t numPoints = hacdInst->GetNPointsCH(hullIndex);
    size_t numTriangles = hacdInst->GetNTrianglesCH(hullIndex);
    
    if (numPoints == 0 || numTriangles == 0) return 0;
    
    std::vector<HACD::Vec3<HACD::Real>> points(numPoints);
    std::vector<HACD::Vec3<long>> triangles(numTriangles);
    
    if (!hacdInst->GetCH(hullIndex, &points[0], &triangles[0])) return 0;
    
    if (outPoints && outNumPoints)
    {
        for (size_t i = 0; i < numPoints; i++)
        {
            outPoints[i * 3] = points[i].X();
            outPoints[i * 3 + 1] = points[i].Y();
            outPoints[i * 3 + 2] = points[i].Z();
        }
        *outNumPoints = static_cast<int>(numPoints);
    }
    
    if (outTriangles && outNumTriangles)
    {
        for (size_t i = 0; i < numTriangles; i++)
        {
            outTriangles[i * 3] = static_cast<int>(triangles[i].X());
            outTriangles[i * 3 + 1] = static_cast<int>(triangles[i].Y());
            outTriangles[i * 3 + 2] = static_cast<int>(triangles[i].Z());
        }
        *outNumTriangles = static_cast<int>(numTriangles);
    }
    
    return 1;
}
