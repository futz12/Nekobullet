#include "gimpact_decomp.h"
#include "../Extras/GIMPACTUtils/btGImpactConvexDecompositionShape.h"
#include "BulletCollision/CollisionShapes/btTriangleMeshShape.h"

nkGImpactDecompShapeHandle nk_gimpact_decomp_shape_create(
    nkShapeHandle trimesh,
    nkReal scale_x, nkReal scale_y, nkReal scale_z,
    nkReal margin,
    int transform_subshapes)
{
    if (!trimesh) return nullptr;
    
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(trimesh);
    
    btTriangleMeshShape* triangle_mesh_shape = dynamic_cast<btTriangleMeshShape*>(collision_shape);
    if (!triangle_mesh_shape) return nullptr;
    
    btStridingMeshInterface* mesh_interface = triangle_mesh_shape->getMeshInterface();
    if (!mesh_interface) return nullptr;
    
    btGImpactConvexDecompositionShape* decomp_shape = new btGImpactConvexDecompositionShape(
        mesh_interface,
        btVector3(scale_x, scale_y, scale_z),
        margin,
        transform_subshapes != 0
    );
    
    return static_cast<nkGImpactDecompShapeHandle>(decomp_shape);
}

void nk_gimpact_decomp_shape_destroy(nkGImpactDecompShapeHandle shape)
{
    if (!shape) return;
    btGImpactConvexDecompositionShape* decomp_shape = 
        static_cast<btGImpactConvexDecompositionShape*>(shape);
    delete decomp_shape;
}

int nk_gimpact_decomp_shape_get_num_child_shapes(nkGImpactDecompShapeHandle shape)
{
    if (!shape) return 0;
    btGImpactConvexDecompositionShape* decomp_shape = 
        static_cast<btGImpactConvexDecompositionShape*>(shape);
    return decomp_shape->getNumChildShapes();
}

void nk_gimpact_decomp_shape_update_bound(nkGImpactDecompShapeHandle shape)
{
    if (!shape) return;
    btGImpactConvexDecompositionShape* decomp_shape = 
        static_cast<btGImpactConvexDecompositionShape*>(shape);
    decomp_shape->updateBound();
}

void nk_gimpact_decomp_shape_set_margin(nkGImpactDecompShapeHandle shape, nkReal margin)
{
    if (!shape) return;
    btGImpactConvexDecompositionShape* decomp_shape = 
        static_cast<btGImpactConvexDecompositionShape*>(shape);
    decomp_shape->setMargin(margin);
}

nkReal nk_gimpact_decomp_shape_get_margin(nkGImpactDecompShapeHandle shape)
{
    if (!shape) return 0.0f;
    btGImpactConvexDecompositionShape* decomp_shape = 
        static_cast<btGImpactConvexDecompositionShape*>(shape);
    return decomp_shape->getMargin();
}

nkShapeHandle nk_gimpact_decomp_shape_get_child_shape(nkGImpactDecompShapeHandle shape, int index)
{
    if (!shape) return nullptr;
    btGImpactConvexDecompositionShape* decomp_shape = 
        static_cast<btGImpactConvexDecompositionShape*>(shape);
    
    if (index < 0 || index >= decomp_shape->getNumChildShapes()) return nullptr;
    
    return static_cast<nkShapeHandle>(decomp_shape->getChildShape(index));
}

void nk_gimpact_decomp_shape_get_child_transform(nkGImpactDecompShapeHandle shape, int index,
    nkReal* out_pos_x, nkReal* out_pos_y, nkReal* out_pos_z,
    nkReal* out_rot_x, nkReal* out_rot_y, nkReal* out_rot_z, nkReal* out_rot_w)
{
    if (!shape) return;
    btGImpactConvexDecompositionShape* decomp_shape = 
        static_cast<btGImpactConvexDecompositionShape*>(shape);
    
    if (index < 0 || index >= decomp_shape->getNumChildShapes()) return;
    
    btTransform transform = decomp_shape->getChildTransform(index);
    btVector3 pos = transform.getOrigin();
    btQuaternion rot = transform.getRotation();
    
    if (out_pos_x) *out_pos_x = pos.x();
    if (out_pos_y) *out_pos_y = pos.y();
    if (out_pos_z) *out_pos_z = pos.z();
    if (out_rot_x) *out_rot_x = rot.x();
    if (out_rot_y) *out_rot_y = rot.y();
    if (out_rot_z) *out_rot_z = rot.z();
    if (out_rot_w) *out_rot_w = rot.w();
}
