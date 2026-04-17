#ifndef NEKOBULLET_SHAPE_HPP
#define NEKOBULLET_SHAPE_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkShapeHandle nk_shape_create_box(nkReal half_extent_x, nkReal half_extent_y, nkReal half_extent_z);
nkShapeHandle nk_shape_create_sphere(nkReal radius);
nkShapeHandle nk_shape_create_capsule(nkReal radius, nkReal height);
nkShapeHandle nk_shape_create_cylinder(nkReal radius, nkReal height);
nkShapeHandle nk_shape_create_cone(nkReal radius, nkReal height);
nkShapeHandle nk_shape_create_plane(nkReal normal_x, nkReal normal_y, nkReal normal_z, nkReal constant);
nkShapeHandle nk_shape_create_convex_hull(const nkReal* points, int num_points);
nkShapeHandle nk_shape_create_compound();
nkShapeHandle nk_shape_create_triangle_mesh(
    const nkReal* vertices, int num_vertices,
    const int* indices, int num_indices);
nkShapeHandle nk_shape_create_heightfield(
    int width, int length,
    const nkReal* height_data,
    nkReal min_height, nkReal max_height,
    int up_axis);

void nk_compound_add_child(nkShapeHandle compound, nkShapeHandle child_shape, nkTransform* local_transform);
void nk_shape_destroy(nkShapeHandle shape);
int nk_shape_get_type(nkShapeHandle shape);
void nk_shape_set_local_scaling(nkShapeHandle shape, nkReal x, nkReal y, nkReal z);
void nk_shape_get_local_scaling(nkShapeHandle shape, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_shape_calculate_local_inertia(nkShapeHandle shape, nkReal mass, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_shape_set_margin(nkShapeHandle shape, nkReal margin);
nkReal nk_shape_get_margin(nkShapeHandle shape);
nkReal nk_shape_get_angular_motion_disc(nkShapeHandle shape);
nkReal nk_shape_get_contact_breaking_threshold(nkShapeHandle shape, nkReal default_contact_threshold);

void nk_box_get_half_extents(nkShapeHandle shape, nkReal* out_x, nkReal* out_y, nkReal* out_z);
nkReal nk_sphere_get_radius(nkShapeHandle shape);
nkReal nk_capsule_get_radius(nkShapeHandle shape);
nkReal nk_capsule_get_half_height(nkShapeHandle shape);
nkReal nk_cylinder_get_radius(nkShapeHandle shape);
nkReal nk_cylinder_get_half_height(nkShapeHandle shape);
nkReal nk_cone_get_radius(nkShapeHandle shape);
nkReal nk_cone_get_height(nkShapeHandle shape);

#ifdef __cplusplus
}
#endif

#endif
