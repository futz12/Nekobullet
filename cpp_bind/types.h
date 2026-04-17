#ifndef NEKOBULLET_TYPES_HPP
#define NEKOBULLET_TYPES_HPP

#include <stdint.h>

#ifdef BT_USE_DOUBLE_PRECISION
typedef double nkReal;
#else
typedef float nkReal;
#endif

typedef nkReal nkVector3[3];
typedef nkReal nkQuaternion[4];

typedef struct nkTransform
{
    nkVector3 origin;
    nkQuaternion rotation;
} nkTransform;

typedef void* nkWorldHandle;
typedef void* nkRigidBodyHandle;
typedef void* nkShapeHandle;
typedef void* nkConstraintHandle;
typedef void* nkGhostObjectHandle;
typedef void* nkSoftBodyHandle;
typedef void* nkSoftBodyWorldInfoHandle;
typedef void* nkVehicleHandle;
typedef void* nkVehicleRaycasterHandle;
typedef void* nkCharacterHandle;
typedef void* nkMultiBodyHandle;
typedef void* nkHACDHandle;
typedef void* nkVHACDHandle;
typedef void* nkWorldImporterHandle;
typedef void* nkGImpactShapeHandle;

#define NK_SHAPE_TYPE_BOX 0
#define NK_SHAPE_TYPE_SPHERE 1
#define NK_SHAPE_TYPE_CAPSULE 2
#define NK_SHAPE_TYPE_CYLINDER 3
#define NK_SHAPE_TYPE_CONE 4
#define NK_SHAPE_TYPE_COMPOUND 5
#define NK_SHAPE_TYPE_CONVEX_HULL 6
#define NK_SHAPE_TYPE_TRIANGLE_MESH 7
#define NK_SHAPE_TYPE_STATIC_PLANE 9

#define NK_ACTIVE_TAG 1
#define NK_ISLAND_SLEEPING 2
#define NK_WANTS_DEACTIVATION 3
#define NK_DISABLE_DEACTIVATION 4
#define NK_DISABLE_SIMULATION 5

#define NK_CONSTRAINT_TYPE_POINT2POINT 0
#define NK_CONSTRAINT_TYPE_HINGE 1
#define NK_CONSTRAINT_TYPE_SLIDER 2
#define NK_CONSTRAINT_TYPE_FIXED 3
#define NK_CONSTRAINT_TYPE_GENERIC_6DOF 4
#define NK_CONSTRAINT_TYPE_CONE_TWIST 5
#define NK_CONSTRAINT_TYPE_GENERIC_6DOF_SPRING 6
#define NK_CONSTRAINT_TYPE_UNIVERSAL 7
#define NK_CONSTRAINT_TYPE_HINGE2 8
#define NK_CONSTRAINT_TYPE_GEAR 9
#define NK_CONSTRAINT_TYPE_GENERIC_6DOF_SPRING2 10

#define NK_JOINT_TYPE_FIXED 0
#define NK_JOINT_TYPE_REVOLUTE 1
#define NK_JOINT_TYPE_PRISMATIC 2
#define NK_JOINT_TYPE_FLOATING 3
#define NK_JOINT_TYPE_SPHERICAL 4

#endif
