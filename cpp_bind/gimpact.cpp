#include "gimpact.h"
#include "btBulletDynamicsCommon.h"
#include "BulletCollision/Gimpact/btGImpactShape.h"

#include <cstdlib>

nkGImpactShapeHandle nk_gimpact_shape_create(nkShapeHandle trimesh, float scaleX, float scaleY, float scaleZ)
{
    if (!trimesh) return nullptr;
    
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(trimesh);
    
    btTriangleMeshShape* triangle_mesh_shape = dynamic_cast<btTriangleMeshShape*>(collision_shape);
    if (!triangle_mesh_shape) return nullptr;
    
    btStridingMeshInterface* mesh_interface = triangle_mesh_shape->getMeshInterface();
    if (!mesh_interface) return nullptr;
    
    btGImpactMeshShape* gimpact_shape = new btGImpactMeshShape(mesh_interface);
    gimpact_shape->setLocalScaling(btVector3(scaleX, scaleY, scaleZ));
    gimpact_shape->updateBound();
    
    return static_cast<nkGImpactShapeHandle>(gimpact_shape);
}

void nk_gimpact_shape_destroy(nkGImpactShapeHandle shape)
{
    if (!shape) return;
    btGImpactMeshShape* gimpact_shape = static_cast<btGImpactMeshShape*>(shape);
    delete gimpact_shape;
}

void nk_gimpact_shape_update_bound(nkGImpactShapeHandle shape)
{
    if (!shape) return;
    btGImpactMeshShape* gimpact_shape = static_cast<btGImpactMeshShape*>(shape);
    gimpact_shape->updateBound();
}

int nk_gimpact_shape_get_num_child_shapes(nkGImpactShapeHandle shape)
{
    if (!shape) return 0;
    btGImpactMeshShape* gimpact_shape = static_cast<btGImpactMeshShape*>(shape);
    return gimpact_shape->getMeshPartCount();
}

void nk_gimpact_shape_set_margin(nkGImpactShapeHandle shape, float margin)
{
    if (!shape) return;
    btGImpactMeshShape* gimpact_shape = static_cast<btGImpactMeshShape*>(shape);
    gimpact_shape->setMargin(margin);
}

float nk_gimpact_shape_get_margin(nkGImpactShapeHandle shape)
{
    if (!shape) return 0.0f;
    btGImpactMeshShape* gimpact_shape = static_cast<btGImpactMeshShape*>(shape);
    return gimpact_shape->getMargin();
}
