#include "vehicle.h"
#include "BulletDynamics/Vehicle/btRaycastVehicle.h"
#include "BulletDynamics/Dynamics/btRigidBody.h"
#include "btBulletDynamicsCommon.h"

nkVehicleRaycasterHandle nk_vehicle_raycaster_create(nkWorldHandle world)
{
    if (!world) return nullptr;
    btDiscreteDynamicsWorld* dynamics_world = static_cast<btDiscreteDynamicsWorld*>(world);
    btVehicleRaycaster* raycaster = new btDefaultVehicleRaycaster(dynamics_world);
    return static_cast<nkVehicleRaycasterHandle>(raycaster);
}

void nk_vehicle_raycaster_destroy(nkVehicleRaycasterHandle raycaster)
{
    if (!raycaster) return;
    btVehicleRaycaster* vehicle_raycaster = static_cast<btVehicleRaycaster*>(raycaster);
    delete vehicle_raycaster;
}

void nk_vehicle_tuning_init_default(nkVehicleTuning* tuning)
{
    if (!tuning) return;
    tuning->suspension_stiffness = 5.88f;
    tuning->suspension_compression = 0.83f;
    tuning->suspension_damping = 0.88f;
    tuning->max_suspension_travel_cm = 500.0f;
    tuning->friction_slip = 10.5f;
    tuning->max_suspension_force = 6000.0f;
}

nkVehicleHandle nk_vehicle_create(nkRigidBodyHandle chassis, nkVehicleRaycasterHandle raycaster, nkVehicleTuning* tuning)
{
    if (!chassis || !raycaster) return nullptr;
    
    btRigidBody* body = static_cast<btRigidBody*>(chassis);
    btVehicleRaycaster* vehicle_raycaster = static_cast<btVehicleRaycaster*>(raycaster);
    
    btRaycastVehicle::btVehicleTuning bt_tuning;
    if (tuning)
    {
        bt_tuning.m_suspensionStiffness = tuning->suspension_stiffness;
        bt_tuning.m_suspensionCompression = tuning->suspension_compression;
        bt_tuning.m_suspensionDamping = tuning->suspension_damping;
        bt_tuning.m_maxSuspensionTravelCm = tuning->max_suspension_travel_cm;
        bt_tuning.m_frictionSlip = tuning->friction_slip;
        bt_tuning.m_maxSuspensionForce = tuning->max_suspension_force;
    }
    
    btRaycastVehicle* vehicle = new btRaycastVehicle(bt_tuning, body, vehicle_raycaster);
    body->setActivationState(DISABLE_DEACTIVATION);
    
    return static_cast<nkVehicleHandle>(vehicle);
}

void nk_vehicle_destroy(nkVehicleHandle vehicle)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    delete raycast_vehicle;
}

int nk_vehicle_add_wheel(nkVehicleHandle vehicle,
    nkReal connection_x, nkReal connection_y, nkReal connection_z,
    nkReal direction_x, nkReal direction_y, nkReal direction_z,
    nkReal axle_x, nkReal axle_y, nkReal axle_z,
    nkReal suspension_rest_length, nkReal wheel_radius,
    int is_front_wheel, nkVehicleTuning* tuning)
{
    if (!vehicle) return -1;
    
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    
    btVector3 connection_point(connection_x, connection_y, connection_z);
    btVector3 wheel_direction(direction_x, direction_y, direction_z);
    btVector3 wheel_axle(axle_x, axle_y, axle_z);
    
    btRaycastVehicle::btVehicleTuning bt_tuning;
    if (tuning)
    {
        bt_tuning.m_suspensionStiffness = tuning->suspension_stiffness;
        bt_tuning.m_suspensionCompression = tuning->suspension_compression;
        bt_tuning.m_suspensionDamping = tuning->suspension_damping;
        bt_tuning.m_maxSuspensionTravelCm = tuning->max_suspension_travel_cm;
        bt_tuning.m_frictionSlip = tuning->friction_slip;
        bt_tuning.m_maxSuspensionForce = tuning->max_suspension_force;
    }
    
    int wheel_index = raycast_vehicle->getNumWheels();
    raycast_vehicle->addWheel(
        connection_point,
        wheel_direction,
        wheel_axle,
        suspension_rest_length,
        wheel_radius,
        bt_tuning,
        is_front_wheel != 0
    );
    
    return wheel_index;
}

int nk_vehicle_get_num_wheels(nkVehicleHandle vehicle)
{
    if (!vehicle) return 0;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    return raycast_vehicle->getNumWheels();
}

void nk_vehicle_set_steering_value(nkVehicleHandle vehicle, nkReal steering, int wheel)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    raycast_vehicle->setSteeringValue(steering, wheel);
}

nkReal nk_vehicle_get_steering_value(nkVehicleHandle vehicle, int wheel)
{
    if (!vehicle) return 0.0f;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    return raycast_vehicle->getSteeringValue(wheel);
}

void nk_vehicle_apply_engine_force(nkVehicleHandle vehicle, nkReal force, int wheel)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    raycast_vehicle->applyEngineForce(force, wheel);
}

void nk_vehicle_set_brake(nkVehicleHandle vehicle, nkReal brake, int wheel)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    raycast_vehicle->setBrake(brake, wheel);
}

nkReal nk_vehicle_get_current_speed_km_hour(nkVehicleHandle vehicle)
{
    if (!vehicle) return 0.0f;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    return raycast_vehicle->getCurrentSpeedKmHour();
}

void nk_vehicle_get_forward_vector(nkVehicleHandle vehicle, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    btVector3 forward = raycast_vehicle->getForwardVector();
    *out_x = forward.x();
    *out_y = forward.y();
    *out_z = forward.z();
}

void nk_vehicle_set_coordinate_system(nkVehicleHandle vehicle, int right_index, int up_index, int forward_index)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    raycast_vehicle->setCoordinateSystem(right_index, up_index, forward_index);
}

void nk_vehicle_reset_suspension(nkVehicleHandle vehicle)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    raycast_vehicle->resetSuspension();
}

void nk_vehicle_get_wheel_transform(nkVehicleHandle vehicle, int wheel_index, nkTransform* out_transform)
{
    if (!vehicle || !out_transform) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    
    if (wheel_index < 0 || wheel_index >= raycast_vehicle->getNumWheels()) return;
    
    btTransform transform = raycast_vehicle->getWheelTransformWS(wheel_index);
    
    out_transform->origin[0] = transform.getOrigin().x();
    out_transform->origin[1] = transform.getOrigin().y();
    out_transform->origin[2] = transform.getOrigin().z();
    
    out_transform->rotation[0] = transform.getRotation().x();
    out_transform->rotation[1] = transform.getRotation().y();
    out_transform->rotation[2] = transform.getRotation().z();
    out_transform->rotation[3] = transform.getRotation().w();
}

void nk_vehicle_update_wheel_transform(nkVehicleHandle vehicle, int wheel_index, int interpolated)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    raycast_vehicle->updateWheelTransform(wheel_index, interpolated != 0);
}

int nk_vehicle_is_wheel_in_contact(nkVehicleHandle vehicle, int wheel_index)
{
    if (!vehicle) return 0;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    
    if (wheel_index < 0 || wheel_index >= raycast_vehicle->getNumWheels()) return 0;
    
    btWheelInfo& wheel = raycast_vehicle->getWheelInfo(wheel_index);
    return wheel.m_raycastInfo.m_isInContact ? 1 : 0;
}

void nk_vehicle_get_wheel_contact_normal(nkVehicleHandle vehicle, int wheel_index, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    
    if (wheel_index < 0 || wheel_index >= raycast_vehicle->getNumWheels()) return;
    
    btWheelInfo& wheel = raycast_vehicle->getWheelInfo(wheel_index);
    btVector3 normal = wheel.m_raycastInfo.m_contactNormalWS;
    
    *out_x = normal.x();
    *out_y = normal.y();
    *out_z = normal.z();
}

void nk_vehicle_get_wheel_contact_point(nkVehicleHandle vehicle, int wheel_index, nkReal* out_x, nkReal* out_y, nkReal* out_z)
{
    if (!vehicle) return;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    
    if (wheel_index < 0 || wheel_index >= raycast_vehicle->getNumWheels()) return;
    
    btWheelInfo& wheel = raycast_vehicle->getWheelInfo(wheel_index);
    btVector3 point = wheel.m_raycastInfo.m_contactPointWS;
    
    *out_x = point.x();
    *out_y = point.y();
    *out_z = point.z();
}

nkReal nk_vehicle_get_wheel_suspension_length(nkVehicleHandle vehicle, int wheel_index)
{
    if (!vehicle) return 0.0f;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    
    if (wheel_index < 0 || wheel_index >= raycast_vehicle->getNumWheels()) return 0.0f;
    
    btWheelInfo& wheel = raycast_vehicle->getWheelInfo(wheel_index);
    return wheel.m_raycastInfo.m_suspensionLength;
}

nkReal nk_vehicle_get_wheel_radius(nkVehicleHandle vehicle, int wheel_index)
{
    if (!vehicle) return 0.0f;
    btRaycastVehicle* raycast_vehicle = static_cast<btRaycastVehicle*>(vehicle);
    
    if (wheel_index < 0 || wheel_index >= raycast_vehicle->getNumWheels()) return 0.0f;
    
    btWheelInfo& wheel = raycast_vehicle->getWheelInfo(wheel_index);
    return wheel.m_wheelsRadius;
}
