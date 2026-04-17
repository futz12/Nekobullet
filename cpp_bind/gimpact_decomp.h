#ifndef NEKOBULLET_GIMPACT_DECOMP_HPP
#define NEKOBULLET_GIMPACT_DECOMP_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkGImpactDecompShapeHandle;

nkGImpactDecompShapeHandle nk_gimpact_decomp_shape_create(
    nkShapeHandle trimesh,
    nkReal scale_x, nkReal scale_y, nkReal scale_z,
    nkReal margin,
    int transform_subshapes);

void nk_gimpact_decomp_shape_destroy(nkGImpactDecompShapeHandle shape);

int nk_gimpact_decomp_shape_get_num_child_shapes(nkGImpactDecompShapeHandle shape);

void nk_gimpact_decomp_shape_update_bound(nkGImpactDecompShapeHandle shape);

void nk_gimpact_decomp_shape_set_margin(nkGImpactDecompShapeHandle shape, nkReal margin);

nkReal nk_gimpact_decomp_shape_get_margin(nkGImpactDecompShapeHandle shape);

nkShapeHandle nk_gimpact_decomp_shape_get_child_shape(nkGImpactDecompShapeHandle shape, int index);

void nk_gimpact_decomp_shape_get_child_transform(nkGImpactDecompShapeHandle shape, int index,
    nkReal* out_pos_x, nkReal* out_pos_y, nkReal* out_pos_z,
    nkReal* out_rot_x, nkReal* out_rot_y, nkReal* out_rot_z, nkReal* out_rot_w);

#ifdef __cplusplus
}
#endif

#endif
