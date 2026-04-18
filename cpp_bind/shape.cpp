#include "shape.h"
#include "btBulletDynamicsCommon.h"
#include "BulletCollision/CollisionShapes/btHeightfieldTerrainShape.h"

nkShapeHandle nk_shape_create_box(nkReal half_extent_x, nkReal half_extent_y, nkReal half_extent_z)
{
    btCollisionShape* shape = new btBoxShape(btVector3(half_extent_x, half_extent_y, half_extent_z));
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_sphere(nkReal radius)
{
    btCollisionShape* shape = new btSphereShape(radius);
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_capsule(nkReal radius, nkReal height)
{
    btCollisionShape* shape = new btCapsuleShape(radius, height);
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_capsule_z(nkReal radius, nkReal height)
{
    btCollisionShape* shape = new btCapsuleShapeZ(radius, height);
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_cylinder(nkReal radius, nkReal height)
{
    btCollisionShape* shape = new btCylinderShape(btVector3(radius, height * 0.5f, radius));
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_cone(nkReal radius, nkReal height)
{
    btCollisionShape* shape = new btConeShape(radius, height);
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_plane(nkReal normal_x, nkReal normal_y, nkReal normal_z, nkReal constant)
{
    btCollisionShape* shape = new btStaticPlaneShape(btVector3(normal_x, normal_y, normal_z), constant);
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_convex_hull(const nkReal* points, int num_points)
{
    btConvexHullShape* shape = new btConvexHullShape();
    for (int i = 0; i < num_points; i++)
    {
        shape->addPoint(btVector3(points[i * 3], points[i * 3 + 1], points[i * 3 + 2]));
    }
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_compound()
{
    btCompoundShape* shape = new btCompoundShape();
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_triangle_mesh(
    const nkReal* vertices, int num_vertices,
    const int* indices, int num_indices)
{
    btTriangleMesh* mesh = new btTriangleMesh();
    
    for (int i = 0; i < num_indices; i += 3)
    {
        btVector3 v0(vertices[indices[i] * 3], vertices[indices[i] * 3 + 1], vertices[indices[i] * 3 + 2]);
        btVector3 v1(vertices[indices[i + 1] * 3], vertices[indices[i + 1] * 3 + 1], vertices[indices[i + 1] * 3 + 2]);
        btVector3 v2(vertices[indices[i + 2] * 3], vertices[indices[i + 2] * 3 + 1], vertices[indices[i + 2] * 3 + 2]);
        mesh->addTriangle(v0, v1, v2);
    }
    
    btBvhTriangleMeshShape* shape = new btBvhTriangleMeshShape(mesh, true);
    return static_cast<nkShapeHandle>(shape);
}

nkShapeHandle nk_shape_create_heightfield(
    int width, int length,
    const nkReal* height_data,
    nkReal min_height, nkReal max_height,
    int up_axis)
{
    btHeightfieldTerrainShape* shape = new btHeightfieldTerrainShape(
        width, length, height_data, min_height, max_height, up_axis, false
    );
    return static_cast<nkShapeHandle>(shape);
}

void nk_compound_add_child(nkShapeHandle compound, nkShapeHandle child_shape, nkTransform* local_transform)
{
    if (!compound || !child_shape || !local_transform) return;
    btCompoundShape* compound_shape = static_cast<btCompoundShape*>(compound);
    btCollisionShape* child = static_cast<btCollisionShape*>(child_shape);
    
    btTransform transform;
    transform.setOrigin(btVector3(
        local_transform->origin[0],
        local_transform->origin[1],
        local_transform->origin[2]
    ));
    transform.setRotation(btQuaternion(
        local_transform->rotation[0],
        local_transform->rotation[1],
        local_transform->rotation[2],
        local_transform->rotation[3]
    ));
    
    compound_shape->addChildShape(transform, child);
}

void nk_shape_destroy(nkShapeHandle shape)
{
    if (!shape) return;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    delete collision_shape;
}

int nk_shape_get_type(nkShapeHandle shape)
{
    if (!shape) return -1;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    
    switch (collision_shape->getShapeType())
    {
        case BOX_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_BOX;
        case SPHERE_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_SPHERE;
        case CAPSULE_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_CAPSULE;
        case CYLINDER_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_CYLINDER;
        case CONE_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_CONE;
        case COMPOUND_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_COMPOUND;
        case CONVEX_HULL_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_CONVEX_HULL;
        case TRIANGLE_MESH_SHAPE_PROXYTYPE: return NK_SHAPE_TYPE_TRIANGLE_MESH;
        case STATIC_PLANE_PROXYTYPE: return NK_SHAPE_TYPE_STATIC_PLANE;
        default: return -1;
    }
}

void nk_shape_set_local_scaling(nkShapeHandle shape, nkReal x, nkReal y, nkReal z)
{
    if (!shape) return;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    collision_shape->setLocalScaling(btVector3(x, y, z));
}

void nk_shape_get_local_scaling(nkShapeHandle shape, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!shape) return;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    btVector3 scaling = collision_shape->getLocalScaling();
    *out_x = scaling.x();
    *out_y = scaling.y();
    *out_z = scaling.z();
}

void nk_shape_calculate_local_inertia(nkShapeHandle shape, nkReal mass, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!shape) return;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    btVector3 inertia(0, 0, 0);
    collision_shape->calculateLocalInertia(mass, inertia);
    *out_x = inertia.x();
    *out_y = inertia.y();
    *out_z = inertia.z();
}

void nk_shape_set_margin(nkShapeHandle shape, nkReal margin)
{
    if (!shape) return;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    collision_shape->setMargin(margin);
}

nkReal nk_shape_get_margin(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    return collision_shape->getMargin();
}

nkReal nk_shape_get_angular_motion_disc(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    return collision_shape->getAngularMotionDisc();
}

nkReal nk_shape_get_contact_breaking_threshold(nkShapeHandle shape, nkReal default_contact_threshold)
{
    if (!shape) return default_contact_threshold;
    btCollisionShape* collision_shape = static_cast<btCollisionShape*>(shape);
    return collision_shape->getContactBreakingThreshold(default_contact_threshold);
}

void nk_box_get_half_extents(nkShapeHandle shape, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!shape) return;
    btBoxShape* box_shape = static_cast<btBoxShape*>(shape);
    btVector3 extents = box_shape->getHalfExtentsWithMargin();
    *out_x = extents.x();
    *out_y = extents.y();
    *out_z = extents.z();
}

nkReal nk_sphere_get_radius(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btSphereShape* sphere_shape = static_cast<btSphereShape*>(shape);
    return sphere_shape->getRadius();
}

nkReal nk_capsule_get_radius(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btCapsuleShape* capsule_shape = static_cast<btCapsuleShape*>(shape);
    return capsule_shape->getRadius();
}

nkReal nk_capsule_get_half_height(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btCapsuleShape* capsule_shape = static_cast<btCapsuleShape*>(shape);
    return capsule_shape->getHalfHeight();
}

nkReal nk_cylinder_get_radius(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btCylinderShape* cylinder_shape = static_cast<btCylinderShape*>(shape);
    return cylinder_shape->getRadius();
}

nkReal nk_cylinder_get_half_height(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btCylinderShape* cylinder_shape = static_cast<btCylinderShape*>(shape);
    btVector3 half_extents = cylinder_shape->getHalfExtentsWithMargin();
    return half_extents.y();
}

nkReal nk_cone_get_radius(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btConeShape* cone_shape = static_cast<btConeShape*>(shape);
    return cone_shape->getRadius();
}

nkReal nk_cone_get_height(nkShapeHandle shape)
{
    if (!shape) return 0.0f;
    btConeShape* cone_shape = static_cast<btConeShape*>(shape);
    return cone_shape->getHeight();
}
