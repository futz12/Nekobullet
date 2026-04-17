#include "softbody.h"
#include "BulletSoftBody/btSoftBody.h"
#include "BulletSoftBody/btSoftBodyHelpers.h"
#include "BulletDynamics/Dynamics/btRigidBody.h"

nkSoftBodyWorldInfoHandle nk_softbody_world_info_create()
{
    btSoftBodyWorldInfo* info = new btSoftBodyWorldInfo();
    return static_cast<nkSoftBodyWorldInfoHandle>(info);
}

void nk_softbody_world_info_destroy(nkSoftBodyWorldInfoHandle info)
{
    if (!info) return;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    delete world_info;
}

void nk_softbody_world_info_set_gravity(nkSoftBodyWorldInfoHandle info, nkReal x, nkReal y, nkReal z)
{
    if (!info) return;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    world_info->m_gravity.setValue(x, y, z);
}

void nk_softbody_world_info_set_air_density(nkSoftBodyWorldInfoHandle info, nkReal density)
{
    if (!info) return;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    world_info->air_density = density;
}

void nk_softbody_world_info_set_water_density(nkSoftBodyWorldInfoHandle info, nkReal density)
{
    if (!info) return;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    world_info->water_density = density;
}

void nk_softbody_world_info_set_water_offset(nkSoftBodyWorldInfoHandle info, nkReal offset)
{
    if (!info) return;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    world_info->water_offset = offset;
}

void nk_softbody_world_info_set_water_normal(nkSoftBodyWorldInfoHandle info, nkReal x, nkReal y, nkReal z)
{
    if (!info) return;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    world_info->water_normal.setValue(x, y, z);
}

nkSoftBodyHandle nk_softbody_create_rope(nkSoftBodyWorldInfoHandle info, 
    nkReal from_x, nkReal from_y, nkReal from_z,
    nkReal to_x, nkReal to_y, nkReal to_z,
    int res, int fixeds)
{
    if (!info) return nullptr;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    btSoftBody* softbody = btSoftBodyHelpers::CreateRope(
        *world_info,
        btVector3(from_x, from_y, from_z),
        btVector3(to_x, to_y, to_z),
        res, fixeds
    );
    return static_cast<nkSoftBodyHandle>(softbody);
}

nkSoftBodyHandle nk_softbody_create_patch(nkSoftBodyWorldInfoHandle info,
    nkReal corner00_x, nkReal corner00_y, nkReal corner00_z,
    nkReal corner10_x, nkReal corner10_y, nkReal corner10_z,
    nkReal corner01_x, nkReal corner01_y, nkReal corner01_z,
    nkReal corner11_x, nkReal corner11_y, nkReal corner11_z,
    int resx, int resy, int fixeds, int gendiags)
{
    if (!info) return nullptr;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    btSoftBody* softbody = btSoftBodyHelpers::CreatePatch(
        *world_info,
        btVector3(corner00_x, corner00_y, corner00_z),
        btVector3(corner10_x, corner10_y, corner10_z),
        btVector3(corner01_x, corner01_y, corner01_z),
        btVector3(corner11_x, corner11_y, corner11_z),
        resx, resy, fixeds, gendiags
    );
    return static_cast<nkSoftBodyHandle>(softbody);
}

nkSoftBodyHandle nk_softbody_create_ellipsoid(nkSoftBodyWorldInfoHandle info,
    nkReal center_x, nkReal center_y, nkReal center_z,
    nkReal radius_x, nkReal radius_y, nkReal radius_z,
    int res)
{
    if (!info) return nullptr;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    btSoftBody* softbody = btSoftBodyHelpers::CreateEllipsoid(
        *world_info,
        btVector3(center_x, center_y, center_z),
        btVector3(radius_x, radius_y, radius_z),
        res
    );
    return static_cast<nkSoftBodyHandle>(softbody);
}

nkSoftBodyHandle nk_softbody_create_from_trimesh(nkSoftBodyWorldInfoHandle info,
    const nkReal* vertices, const int* triangles, int ntriangles)
{
    if (!info || !vertices || !triangles) return nullptr;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    
    int nvertices = 0;
    for (int i = 0; i < ntriangles * 3; i++)
    {
        if (triangles[i] + 1 > nvertices)
            nvertices = triangles[i] + 1;
    }
    
    btSoftBody* softbody = btSoftBodyHelpers::CreateFromTriMesh(
        *world_info,
        vertices,
        triangles,
        ntriangles,
        false
    );
    return static_cast<nkSoftBodyHandle>(softbody);
}

nkSoftBodyHandle nk_softbody_create_from_convex_hull(nkSoftBodyWorldInfoHandle info,
    const nkReal* vertices, int nvertices)
{
    if (!info || !vertices) return nullptr;
    btSoftBodyWorldInfo* world_info = static_cast<btSoftBodyWorldInfo*>(info);
    
    btAlignedObjectArray<btVector3> points;
    for (int i = 0; i < nvertices; i++)
    {
        points.push_back(btVector3(vertices[i * 3], vertices[i * 3 + 1], vertices[i * 3 + 2]));
    }
    
    btSoftBody* softbody = btSoftBodyHelpers::CreateFromConvexHull(
        *world_info,
        &points[0],
        nvertices
    );
    return static_cast<nkSoftBodyHandle>(softbody);
}

void nk_softbody_destroy(nkSoftBodyHandle softbody)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    delete body;
}

int nk_softbody_get_num_nodes(nkSoftBodyHandle softbody)
{
    if (!softbody) return 0;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    return body->m_nodes.size();
}

void nk_softbody_get_node_position(nkSoftBodyHandle softbody, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!softbody || !out_x || !out_y || !out_z) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    if (index < 0 || index >= body->m_nodes.size()) return;
    
    const btSoftBody::Node& node = body->m_nodes[index];
    *out_x = node.m_x.x();
    *out_y = node.m_x.y();
    *out_z = node.m_x.z();
}

void nk_softbody_set_node_position(nkSoftBodyHandle softbody, int index, nkReal x, nkReal y, nkReal z)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    if (index < 0 || index >= body->m_nodes.size()) return;
    
    btSoftBody::Node& node = body->m_nodes[index];
    node.m_x.setValue(x, y, z);
}

void nk_softbody_get_node_velocity(nkSoftBodyHandle softbody, int index, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!softbody || !out_x || !out_y || !out_z) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    if (index < 0 || index >= body->m_nodes.size()) return;
    
    const btSoftBody::Node& node = body->m_nodes[index];
    *out_x = node.m_v.x();
    *out_y = node.m_v.y();
    *out_z = node.m_v.z();
}

void nk_softbody_set_node_velocity(nkSoftBodyHandle softbody, int index, nkReal x, nkReal y, nkReal z)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    if (index < 0 || index >= body->m_nodes.size()) return;
    
    btSoftBody::Node& node = body->m_nodes[index];
    node.m_v.setValue(x, y, z);
}

nkReal nk_softbody_get_node_mass(nkSoftBodyHandle softbody, int index)
{
    if (!softbody) return 0.0f;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    if (index < 0 || index >= body->m_nodes.size()) return 0.0f;
    
    const btSoftBody::Node& node = body->m_nodes[index];
    return node.m_im == 0 ? 0.0f : 1.0f / node.m_im;
}

void nk_softbody_set_node_mass(nkSoftBodyHandle softbody, int index, nkReal mass)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    if (index < 0 || index >= body->m_nodes.size()) return;
    
    btSoftBody::Node& node = body->m_nodes[index];
    node.m_im = mass == 0 ? 0 : 1.0f / mass;
}

nkReal nk_softbody_get_total_mass(nkSoftBodyHandle softbody)
{
    if (!softbody) return 0.0f;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    return body->getTotalMass();
}

void nk_softbody_set_total_mass(nkSoftBodyHandle softbody, nkReal mass)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->setTotalMass(mass);
}

void nk_softbody_set_velocity(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->setVelocity(btVector3(x, y, z));
}

void nk_softbody_add_velocity(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->addVelocity(btVector3(x, y, z));
}

void nk_softbody_apply_force(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->addForce(btVector3(x, y, z));
}

void nk_softbody_apply_impulse(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->addForce(btVector3(x, y, z));
}

void nk_softbody_clear_forces(nkSoftBodyHandle softbody)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    for (int i = 0; i < body->m_nodes.size(); i++)
    {
        body->m_nodes[i].m_f = btVector3(0, 0, 0);
    }
}

void nk_softbody_append_anchor(nkSoftBodyHandle softbody, int node_index, nkRigidBodyHandle body, 
    nkReal local_x, nkReal local_y, nkReal local_z, int disable_collision)
{
    if (!softbody || !body) return;
    btSoftBody* soft = static_cast<btSoftBody*>(softbody);
    btRigidBody* rigid = static_cast<btRigidBody*>(body);
    if (node_index < 0 || node_index >= soft->m_nodes.size()) return;
    
    soft->appendAnchor(node_index, rigid, btVector3(local_x, local_y, local_z), disable_collision != 0);
}

void nk_softbody_remove_anchor(nkSoftBodyHandle softbody, int node_index)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    if (node_index < 0 || node_index >= body->m_nodes.size()) return;
    
    for (int i = body->m_anchors.size() - 1; i >= 0; i--)
    {
        if (body->m_anchors[i].m_node == &body->m_nodes[node_index])
        {
            body->m_anchors.removeAtIndex(i);
            break;
        }
    }
}

void nk_softbody_set_material_stiffness(nkSoftBodyHandle softbody, nkReal kLST, nkReal kAST, nkReal kVST)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->m_materials[0]->m_kLST = kLST;
    body->m_materials[0]->m_kAST = kAST;
    body->m_materials[0]->m_kVST = kVST;
}

void nk_softbody_set_wind_velocity(nkSoftBodyHandle softbody, nkReal x, nkReal y, nkReal z)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->setWindVelocity(btVector3(x, y, z));
}

void nk_softbody_get_wind_velocity(nkSoftBodyHandle softbody, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!softbody || !out_x || !out_y || !out_z) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    btVector3 wind = body->getWindVelocity();
    *out_x = wind.x();
    *out_y = wind.y();
    *out_z = wind.z();
}

void nk_softbody_set_config_damping(nkSoftBodyHandle softbody, nkReal damping)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->m_cfg.kDP = damping;
}

void nk_softbody_set_config_drag(nkSoftBodyHandle softbody, nkReal drag)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->m_cfg.kDG = drag;
}

void nk_softbody_set_config_lift(nkSoftBodyHandle softbody, nkReal lift)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->m_cfg.kLF = lift;
}

void nk_softbody_set_config_pressure(nkSoftBodyHandle softbody, nkReal pressure)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->m_cfg.kPR = pressure;
}

void nk_softbody_set_config_volume_conversation(nkSoftBodyHandle softbody, nkReal volume)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->m_cfg.kVC = volume;
}

void nk_softbody_set_config_time_scale(nkSoftBodyHandle softbody, nkReal scale)
{
    if (!softbody) return;
    btSoftBody* body = static_cast<btSoftBody*>(softbody);
    body->m_cfg.timescale = scale;
}
