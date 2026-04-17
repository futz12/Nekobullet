#ifndef NEKOBULLET_SOFTBODY_HPP
#define NEKOBULLET_SOFTBODY_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

nkSoftBodyWorldInfoHandle nk_softbody_world_info_create();
void nk_softbody_world_info_destroy(nkSoftBodyWorldInfoHandle info);
void nk_softbody_world_info_set_gravity(nkSoftBodyWorldInfoHandle info, nkReal x, nkReal y, nkReal z);
void nk_softbody_world_info_set_air_density(nkSoftBodyWorldInfoHandle info, nkReal density);
void nk_softbody_world_info_set_water_density(nkSoftBodyWorldInfoHandle info, nkReal density);
void nk_softbody_world_info_set_water_offset(nkSoftBodyWorldInfoHandle info, nkReal offset);
void nk_softbody_world_info_set_water_normal(nkSoftBodyWorldInfoHandle info, nkReal x, nkReal y, nkReal z);

nkSoftBodyHandle nk_softbody_create_rope(nkSoftBodyWorldInfoHandle info, 
    nkReal from_x, nkReal from_y, nkReal from_z,
    nkReal to_x, nkReal to_y, nkReal to_z,
    int res, int fixeds);
nkSoftBodyHandle nk_softbody_create_patch(nkSoftBodyWorldInfoHandle info,
    nkReal corner00_x, nkReal corner00_y, nkReal corner00_z,
    nkReal corner10_x, nkReal corner10_y, nkReal corner10_z,
    nkReal corner01_x, nkReal corner01_y, nkReal corner01_z,
    nkReal corner11_x, nkReal corner11_y, nkReal corner11_z,
    int resx, int resy, int fixeds, int gendiags);
nkSoftBodyHandle nk_softbody_create_ellipsoid(nkSoftBodyWorldInfoHandle info,
    nkReal center_x, nkReal center_y, nkReal center_z,
    nkReal radius_x, nkReal radius_y, nkReal radius_z,
    int res);
nkSoftBodyHandle nk_softbody_create_from_trimesh(nkSoftBodyWorldInfoHandle info,
    const nkReal* vertices, const int* triangles, int ntriangles);
nkSoftBodyHandle nk_softbody_create_from_convex_hull(nkSoftBodyWorldInfoHandle info,
    const nkReal* vertices, int nvertices);
void nk_softbody_destroy(nkSoftBodyHandle softbody);

int nk_softbody_get_num_nodes(nkSoftBodyHandle softbody);
void nk_softbody_get_node_position(nkSoftBodyHandle softbody, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_softbody_set_node_position(nkSoftBodyHandle softbody, int index, nkReal x, nkReal y, nkReal z);
void nk_softbody_get_node_velocity(nkSoftBodyHandle softbody, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_softbody_set_node_velocity(nkSoftBodyHandle softbody, int index, nkReal x, nkReal y, nkReal z);
nkReal nk_softbody_get_node_mass(nkSoftBodyHandle softbody, int index);
void nk_softbody_set_node_mass(nkSoftBodyHandle softbody, int index, nkReal mass);

nkReal nk_softbody_get_total_mass(nkSoftBodyHandle softbody);
void nk_softbody_set_total_mass(nkSoftBodyHandle softbody, nkReal mass);
void nk_softbody_set_velocity(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z);
void nk_softbody_add_velocity(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z);
void nk_softbody_apply_force(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z);
void nk_softbody_apply_impulse(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z);
void nk_softbody_clear_forces(nkSoftBodyHandle softbody);

void nk_softbody_append_anchor(nkSoftBodyHandle softbody, int node_index, nkRigidBodyHandle body, 
    nkReal local_x, nkReal local_y, nkReal local_z, int disable_collision);
void nk_softbody_remove_anchor(nkSoftBodyHandle softbody, int node_index);

void nk_softbody_set_material_stiffness(nkSoftBodyHandle softbody, nkReal kLST, nkReal kAST, nkReal kVST);
void nk_softbody_set_wind_velocity(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z);
void nk_softbody_get_wind_velocity(nkSoftBodyHandle softbody, nkReal* out_x, nkReal* out_y, nkReal* out_z);

void nk_softbody_set_config_damping(nkSoftBodyHandle softbody, nkReal damping);
void nk_softbody_set_config_drag(nkSoftBodyHandle softbody, nkReal drag);
void nk_softbody_set_config_lift(nkSoftBodyHandle softbody, nkReal lift);
void nk_softbody_set_config_pressure(nkSoftBodyHandle softbody, nkReal pressure);
void nk_softbody_set_config_volume_conversation(nkSoftBodyHandle softbody, nkReal volume);
void nk_softbody_set_config_time_scale(nkSoftBodyHandle softbody, nkReal scale);

#ifdef __cplusplus
}
#endif

#endif
