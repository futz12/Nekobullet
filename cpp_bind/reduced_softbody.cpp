#include "reduced_softbody.h"
#include "../src/BulletSoftBody/BulletReducedDeformableBody/btReducedDeformableBody.h"
#include "../src/BulletSoftBody/BulletReducedDeformableBody/btReducedDeformableBodyHelpers.h"
#include <vector>

nkReducedDeformableBodyHandle nk_reduced_softbody_create(
    nkSoftBodyWorldInfoHandle world_info,
    int node_count,
    const nkReal* positions,
    const nkReal* masses)
{
    if (!world_info || !positions || !masses || node_count <= 0) return nullptr;
    
    btSoftBodyWorldInfo* info = static_cast<btSoftBodyWorldInfo*>(world_info);
    
    std::vector<btVector3> x(node_count);
    std::vector<btScalar> m(node_count);
    
    for (int i = 0; i < node_count; i++) {
        x[i] = btVector3(positions[i * 3], positions[i * 3 + 1], positions[i * 3 + 2]);
        m[i] = masses[i];
    }
    
    btReducedDeformableBody* body = new btReducedDeformableBody(info, node_count, x.data(), m.data());
    return static_cast<nkReducedDeformableBodyHandle>(body);
}

void nk_reduced_softbody_destroy(nkReducedDeformableBodyHandle body)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    delete rsb;
}

void nk_reduced_softbody_set_reduced_modes(nkReducedDeformableBodyHandle body, int num_modes, int full_size)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setReducedModes(num_modes, full_size);
}

int nk_reduced_softbody_get_num_reduced_modes(nkReducedDeformableBodyHandle body)
{
    if (!body) return 0;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    return rsb->m_nReduced;
}

int nk_reduced_softbody_get_num_full_dofs(nkReducedDeformableBodyHandle body)
{
    if (!body) return 0;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    return rsb->m_nFull;
}

void nk_reduced_softbody_set_stiffness_scale(nkReducedDeformableBodyHandle body, nkReal ks)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setStiffnessScale(ks);
}

void nk_reduced_softbody_set_mass_scale(nkReducedDeformableBodyHandle body, nkReal rho)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setMassScale(rho);
}

void nk_reduced_softbody_set_damping(nkReducedDeformableBodyHandle body, nkReal alpha, nkReal beta)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setDamping(alpha, beta);
}

void nk_reduced_softbody_set_fixed_node(nkReducedDeformableBodyHandle body, int node_index)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setFixedNodes(node_index);
}

void nk_reduced_softbody_disable_reduced_modes(nkReducedDeformableBodyHandle body, int rigid_only)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->disableReducedModes(rigid_only != 0);
}

void nk_reduced_softbody_set_rigid_velocity(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setRigidVelocity(btVector3(x, y, z));
}

void nk_reduced_softbody_set_rigid_angular_velocity(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setRigidAngularVelocity(btVector3(x, y, z));
}

void nk_reduced_softbody_get_rigid_velocity(nkReducedDeformableBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    const btVector3& v = rsb->getLinearVelocity();
    if (out_x) *out_x = v.x();
    if (out_y) *out_y = v.y();
    if (out_z) *out_z = v.z();
}

void nk_reduced_softbody_get_rigid_angular_velocity(nkReducedDeformableBodyHandle body, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    const btVector3& w = rsb->getAngularVelocity();
    if (out_x) *out_x = w.x();
    if (out_y) *out_y = w.y();
    if (out_z) *out_z = w.z();
}

nkReal nk_reduced_softbody_get_total_mass(nkReducedDeformableBodyHandle body)
{
    if (!body) return 0.0f;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    return rsb->getTotalMass();
}

void nk_reduced_softbody_set_total_mass(nkReducedDeformableBodyHandle body, nkReal mass)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->setTotalMass(mass);
}

void nk_reduced_softbody_get_rigid_transform(nkReducedDeformableBodyHandle body, nkTransform* out_transform)
{
    if (!body || !out_transform) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    btTransform& t = rsb->getRigidTransform();
    btVector3 pos = t.getOrigin();
    btQuaternion rot = t.getRotation();
    out_transform->origin[0] = pos.x();
    out_transform->origin[1] = pos.y();
    out_transform->origin[2] = pos.z();
    out_transform->rotation[0] = rot.x();
    out_transform->rotation[1] = rot.y();
    out_transform->rotation[2] = rot.z();
    out_transform->rotation[3] = rot.w();
}

void nk_reduced_softbody_set_rigid_transform(nkReducedDeformableBodyHandle body, nkTransform* transform)
{
    if (!body || !transform) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    btTransform t;
    t.setOrigin(btVector3(transform->origin[0], transform->origin[1], transform->origin[2]));
    t.setRotation(btQuaternion(transform->rotation[0], transform->rotation[1], transform->rotation[2], transform->rotation[3]));
    rsb->transformTo(t);
}

void nk_reduced_softbody_apply_central_impulse(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->applyCentralImpulse(btVector3(x, y, z));
}

void nk_reduced_softbody_apply_torque_impulse(nkReducedDeformableBodyHandle body, nkReal x, nkReal y, nkReal z)
{
    if (!body) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    rsb->applyTorqueImpulse(btVector3(x, y, z));
}

int nk_reduced_softbody_get_num_nodes(nkReducedDeformableBodyHandle body)
{
    if (!body) return 0;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    return rsb->m_nodes.size();
}

void nk_reduced_softbody_get_node_position(nkReducedDeformableBodyHandle body, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!body || index < 0) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    if (index >= rsb->m_nodes.size()) return;
    
    const btVector3& pos = rsb->m_nodes[index].m_x;
    if (out_x) *out_x = pos.x();
    if (out_y) *out_y = pos.y();
    if (out_z) *out_z = pos.z();
}

void nk_reduced_softbody_get_node_rest_position(nkReducedDeformableBodyHandle body, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!body || index < 0) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    if (index >= (int)rsb->m_x0.size()) return;
    
    const btVector3& pos = rsb->m_x0[index];
    if (out_x) *out_x = pos.x();
    if (out_y) *out_y = pos.y();
    if (out_z) *out_z = pos.z();
}

nkReducedDeformableBodyHandle nk_reduced_softbody_create_from_vtk(
    nkSoftBodyWorldInfoHandle world_info,
    const char* vtk_file_path)
{
    if (!world_info || !vtk_file_path) return nullptr;
    btSoftBodyWorldInfo* info = static_cast<btSoftBodyWorldInfo*>(world_info);
    
    btReducedDeformableBody* body = btReducedDeformableBodyHelpers::createFromVtkFile(*info, vtk_file_path);
    return static_cast<nkReducedDeformableBodyHandle>(body);
}

void nk_reduced_softbody_read_reduced_info(nkReducedDeformableBodyHandle body, const char* file_path)
{
    if (!body || !file_path) return;
    btReducedDeformableBody* rsb = static_cast<btReducedDeformableBody*>(body);
    btReducedDeformableBodyHelpers::readReducedDeformableInfoFromFiles(rsb, file_path);
}
