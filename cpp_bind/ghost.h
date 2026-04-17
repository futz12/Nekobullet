#ifndef NEKOBULLET_GHOST_HPP
#define NEKOBULLET_GHOST_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkGhostObjectHandle nk_ghost_create();
void nk_ghost_destroy(nkGhostObjectHandle ghost);
void nk_ghost_set_shape(nkGhostObjectHandle ghost, nkShapeHandle shape);
void nk_ghost_set_transform(nkGhostObjectHandle ghost, nkTransform* transform);
void nk_ghost_get_transform(nkGhostObjectHandle ghost, nkTransform* out_transform);
void nk_world_add_ghost(nkWorldHandle world, nkGhostObjectHandle ghost);
void nk_world_remove_ghost(nkWorldHandle world, nkGhostObjectHandle ghost);

int nk_ghost_get_num_overlapping_objects(nkGhostObjectHandle ghost);
nkRigidBodyHandle nk_ghost_get_overlapping_object(nkGhostObjectHandle ghost, int index);

#ifdef __cplusplus
}
#endif

#endif
