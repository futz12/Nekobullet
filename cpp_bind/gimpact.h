#ifndef NEKOBULLET_GIMPACT_HPP
#define NEKOBULLET_GIMPACT_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkGImpactShapeHandle;

nkGImpactShapeHandle nk_gimpact_shape_create(nkShapeHandle trimesh, float scaleX, float scaleY, float scaleZ);
void nk_gimpact_shape_destroy(nkGImpactShapeHandle shape);
void nk_gimpact_shape_update_bound(nkGImpactShapeHandle shape);
int nk_gimpact_shape_get_num_child_shapes(nkGImpactShapeHandle shape);
void nk_gimpact_shape_set_margin(nkGImpactShapeHandle shape, float margin);
float nk_gimpact_shape_get_margin(nkGImpactShapeHandle shape);

#ifdef __cplusplus
}
#endif

#endif
