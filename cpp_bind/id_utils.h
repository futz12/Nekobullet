#ifndef NEKOBULLET_ID_UTILS_HPP
#define NEKOBULLET_ID_UTILS_HPP

#include "types.h"

#ifdef __cplusplus
extern "C" {
#endif

typedef void* nkMultiBodyNameMapHandle;
typedef void* nkMultiBodyTreeCreatorHandle;
typedef void* nkUser2InternalIndexHandle;

nkMultiBodyNameMapHandle nk_multibody_name_map_create();
void nk_multibody_name_map_destroy(nkMultiBodyNameMapHandle map);

int nk_multibody_name_map_add_body(nkMultiBodyNameMapHandle map, int index, const char* name);
int nk_multibody_name_map_add_joint(nkMultiBodyNameMapHandle map, int index, const char* name);

int nk_multibody_name_map_get_body_name(nkMultiBodyNameMapHandle map, int index, char* out_name, int max_len);
int nk_multibody_name_map_get_joint_name(nkMultiBodyNameMapHandle map, int index, char* out_name, int max_len);

int nk_multibody_name_map_get_body_index(nkMultiBodyNameMapHandle map, const char* name, int* out_index);
int nk_multibody_name_map_get_joint_index(nkMultiBodyNameMapHandle map, const char* name, int* out_index);

nkMultiBodyTreeCreatorHandle nk_multibody_tree_creator_from_bt_multibody(nkMultiBodyHandle btmb, int verbose);
void nk_multibody_tree_creator_destroy(nkMultiBodyTreeCreatorHandle creator);

nkMultiBodyHandle nk_multibody_create_from_creator(nkMultiBodyTreeCreatorHandle creator);

nkMultiBodyTreeCreatorHandle nk_clone_tree_creator_create(nkMultiBodyHandle reference);
void nk_clone_tree_creator_destroy(nkMultiBodyTreeCreatorHandle creator);

nkMultiBodyHandle nk_multibody_clone(nkMultiBodyHandle reference);

nkMultiBodyTreeCreatorHandle nk_simple_tree_creator_create(int num_bodies);
void nk_simple_tree_creator_destroy(nkMultiBodyTreeCreatorHandle creator);

nkUser2InternalIndexHandle nk_user2internal_index_create();
void nk_user2internal_index_destroy(nkUser2InternalIndexHandle handle);
void nk_user2internal_index_add_body(nkUser2InternalIndexHandle handle, int body, int parent);
int nk_user2internal_index_build_mapping(nkUser2InternalIndexHandle handle);
int nk_user2internal_index_user2internal(nkUser2InternalIndexHandle handle, int user, int* out_internal);
int nk_user2internal_index_internal2user(nkUser2InternalIndexHandle handle, int internal, int* out_user);

void nk_id_random_init();
void nk_id_random_init_with_seed(unsigned int seed);
int nk_id_random_int(int low, int high);
nkReal nk_id_random_float(nkReal low, nkReal high);
nkReal nk_id_random_mass();
void nk_id_random_inertia_principal(nkReal* out_x, nkReal* out_y, nkReal* out_z);
void nk_id_random_axis(nkReal* out_x, nkReal* out_y, nkReal* out_z);

#ifdef __cplusplus
}
#endif

#endif
