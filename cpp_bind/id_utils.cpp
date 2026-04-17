#include "id_utils.h"
#include "BulletInverseDynamics/MultiBodyTree.hpp"
#include "../Extras/InverseDynamics/MultiBodyNameMap.hpp"
#include "../Extras/InverseDynamics/MultiBodyTreeCreator.hpp"
#include "../Extras/InverseDynamics/btMultiBodyTreeCreator.hpp"
#include "../Extras/InverseDynamics/CloneTreeCreator.hpp"
#include "../Extras/InverseDynamics/SimpleTreeCreator.hpp"
#include "../Extras/InverseDynamics/User2InternalIndex.hpp"
#include "../Extras/InverseDynamics/IDRandomUtil.hpp"
#include "BulletDynamics/Featherstone/btMultiBody.h"
#include <cstring>

nkMultiBodyNameMapHandle nk_multibody_name_map_create()
{
    btInverseDynamics::MultiBodyNameMap* map = new btInverseDynamics::MultiBodyNameMap();
    return static_cast<nkMultiBodyNameMapHandle>(map);
}

void nk_multibody_name_map_destroy(nkMultiBodyNameMapHandle map)
{
    if (!map) return;
    btInverseDynamics::MultiBodyNameMap* nameMap = 
        static_cast<btInverseDynamics::MultiBodyNameMap*>(map);
    delete nameMap;
}

int nk_multibody_name_map_add_body(nkMultiBodyNameMapHandle map, int index, const char* name)
{
    if (!map || !name) return -1;
    btInverseDynamics::MultiBodyNameMap* nameMap = 
        static_cast<btInverseDynamics::MultiBodyNameMap*>(map);
    return nameMap->addBody(index, std::string(name));
}

int nk_multibody_name_map_add_joint(nkMultiBodyNameMapHandle map, int index, const char* name)
{
    if (!map || !name) return -1;
    btInverseDynamics::MultiBodyNameMap* nameMap = 
        static_cast<btInverseDynamics::MultiBodyNameMap*>(map);
    return nameMap->addJoint(index, std::string(name));
}

int nk_multibody_name_map_get_body_name(nkMultiBodyNameMapHandle map, int index, char* out_name, int max_len)
{
    if (!map || !out_name || max_len <= 0) return -1;
    btInverseDynamics::MultiBodyNameMap* nameMap = 
        static_cast<btInverseDynamics::MultiBodyNameMap*>(map);
    
    std::string name;
    int result = nameMap->getBodyName(index, &name);
    if (result == 0) {
        strncpy(out_name, name.c_str(), max_len - 1);
        out_name[max_len - 1] = '\0';
    }
    return result;
}

int nk_multibody_name_map_get_joint_name(nkMultiBodyNameMapHandle map, int index, char* out_name, int max_len)
{
    if (!map || !out_name || max_len <= 0) return -1;
    btInverseDynamics::MultiBodyNameMap* nameMap = 
        static_cast<btInverseDynamics::MultiBodyNameMap*>(map);
    
    std::string name;
    int result = nameMap->getJointName(index, &name);
    if (result == 0) {
        strncpy(out_name, name.c_str(), max_len - 1);
        out_name[max_len - 1] = '\0';
    }
    return result;
}

int nk_multibody_name_map_get_body_index(nkMultiBodyNameMapHandle map, const char* name, int* out_index)
{
    if (!map || !name || !out_index) return -1;
    btInverseDynamics::MultiBodyNameMap* nameMap = 
        static_cast<btInverseDynamics::MultiBodyNameMap*>(map);
    return nameMap->getBodyIndex(std::string(name), out_index);
}

int nk_multibody_name_map_get_joint_index(nkMultiBodyNameMapHandle map, const char* name, int* out_index)
{
    if (!map || !name || !out_index) return -1;
    btInverseDynamics::MultiBodyNameMap* nameMap = 
        static_cast<btInverseDynamics::MultiBodyNameMap*>(map);
    return nameMap->getJointIndex(std::string(name), out_index);
}

nkMultiBodyTreeCreatorHandle nk_multibody_tree_creator_from_bt_multibody(nkMultiBodyHandle btmb, int verbose)
{
    if (!btmb) return nullptr;
    
    btMultiBody* bt_multibody = static_cast<btMultiBody*>(btmb);
    
    btInverseDynamics::btMultiBodyTreeCreator* creator = 
        new btInverseDynamics::btMultiBodyTreeCreator();
    
    if (creator->createFromBtMultiBody(bt_multibody, verbose != 0) != 0) {
        delete creator;
        return nullptr;
    }
    
    return static_cast<nkMultiBodyTreeCreatorHandle>(creator);
}

void nk_multibody_tree_creator_destroy(nkMultiBodyTreeCreatorHandle creator)
{
    if (!creator) return;
    btInverseDynamics::MultiBodyTreeCreator* treeCreator = 
        static_cast<btInverseDynamics::MultiBodyTreeCreator*>(creator);
    delete treeCreator;
}

nkMultiBodyHandle nk_multibody_create_from_creator(nkMultiBodyTreeCreatorHandle creator)
{
    if (!creator) return nullptr;
    
    btInverseDynamics::MultiBodyTreeCreator* treeCreator = 
        static_cast<btInverseDynamics::MultiBodyTreeCreator*>(creator);
    
    btInverseDynamics::MultiBodyTree* tree = 
        btInverseDynamics::CreateMultiBodyTree(*treeCreator);
    
    return static_cast<nkMultiBodyHandle>(tree);
}

nkMultiBodyTreeCreatorHandle nk_clone_tree_creator_create(nkMultiBodyHandle reference)
{
    if (!reference) return nullptr;
    
    btInverseDynamics::MultiBodyTree* refTree = 
        static_cast<btInverseDynamics::MultiBodyTree*>(reference);
    
    btInverseDynamics::CloneTreeCreator* creator = 
        new btInverseDynamics::CloneTreeCreator(refTree);
    
    return static_cast<nkMultiBodyTreeCreatorHandle>(creator);
}

void nk_clone_tree_creator_destroy(nkMultiBodyTreeCreatorHandle creator)
{
    if (!creator) return;
    btInverseDynamics::CloneTreeCreator* cloneCreator = 
        static_cast<btInverseDynamics::CloneTreeCreator*>(creator);
    delete cloneCreator;
}

nkMultiBodyHandle nk_multibody_clone(nkMultiBodyHandle reference)
{
    if (!reference) return nullptr;
    
    btInverseDynamics::MultiBodyTree* refTree = 
        static_cast<btInverseDynamics::MultiBodyTree*>(reference);
    
    btInverseDynamics::CloneTreeCreator creator(refTree);
    btInverseDynamics::MultiBodyTree* clonedTree = 
        btInverseDynamics::CreateMultiBodyTree(creator);
    
    return static_cast<nkMultiBodyHandle>(clonedTree);
}

nkMultiBodyTreeCreatorHandle nk_simple_tree_creator_create(int num_bodies)
{
    if (num_bodies <= 0) return nullptr;
    
    btInverseDynamics::SimpleTreeCreator* creator = 
        new btInverseDynamics::SimpleTreeCreator(num_bodies);
    
    return static_cast<nkMultiBodyTreeCreatorHandle>(creator);
}

void nk_simple_tree_creator_destroy(nkMultiBodyTreeCreatorHandle creator)
{
    if (!creator) return;
    btInverseDynamics::SimpleTreeCreator* simpleCreator = 
        static_cast<btInverseDynamics::SimpleTreeCreator*>(creator);
    delete simpleCreator;
}

nkUser2InternalIndexHandle nk_user2internal_index_create()
{
    btInverseDynamics::User2InternalIndex* index = new btInverseDynamics::User2InternalIndex();
    return static_cast<nkUser2InternalIndexHandle>(index);
}

void nk_user2internal_index_destroy(nkUser2InternalIndexHandle handle)
{
    if (!handle) return;
    btInverseDynamics::User2InternalIndex* index = 
        static_cast<btInverseDynamics::User2InternalIndex*>(handle);
    delete index;
}

void nk_user2internal_index_add_body(nkUser2InternalIndexHandle handle, int body, int parent)
{
    if (!handle) return;
    btInverseDynamics::User2InternalIndex* index = 
        static_cast<btInverseDynamics::User2InternalIndex*>(handle);
    index->addBody(body, parent);
}

int nk_user2internal_index_build_mapping(nkUser2InternalIndexHandle handle)
{
    if (!handle) return -1;
    btInverseDynamics::User2InternalIndex* index = 
        static_cast<btInverseDynamics::User2InternalIndex*>(handle);
    return index->buildMapping();
}

int nk_user2internal_index_user2internal(nkUser2InternalIndexHandle handle, int user, int* out_internal)
{
    if (!handle || !out_internal) return -1;
    btInverseDynamics::User2InternalIndex* index = 
        static_cast<btInverseDynamics::User2InternalIndex*>(handle);
    return index->user2internal(user, out_internal);
}

int nk_user2internal_index_internal2user(nkUser2InternalIndexHandle handle, int internal, int* out_user)
{
    if (!handle || !out_user) return -1;
    btInverseDynamics::User2InternalIndex* index = 
        static_cast<btInverseDynamics::User2InternalIndex*>(handle);
    return index->internal2user(internal, out_user);
}

void nk_id_random_init()
{
    btInverseDynamics::randomInit();
}

void nk_id_random_init_with_seed(unsigned int seed)
{
    btInverseDynamics::randomInit(seed);
}

int nk_id_random_int(int low, int high)
{
    return btInverseDynamics::randomInt(low, high);
}

nkReal nk_id_random_float(nkReal low, nkReal high)
{
    return btInverseDynamics::randomFloat(low, high);
}

nkReal nk_id_random_mass()
{
    return btInverseDynamics::randomMass();
}

void nk_id_random_inertia_principal(nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!out_x || !out_y || !out_z) return;
    btInverseDynamics::vec3 inertia = btInverseDynamics::randomInertiaPrincipal();
    *out_x = inertia(0);
    *out_y = inertia(1);
    *out_z = inertia(2);
}

void nk_id_random_axis(nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!out_x || !out_y || !out_z) return;
    btInverseDynamics::vec3 axis = btInverseDynamics::randomAxis();
    *out_x = axis(0);
    *out_y = axis(1);
    *out_z = axis(2);
}
