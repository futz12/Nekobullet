#![allow(clippy::missing_safety_doc)]
#![allow(clippy::too_many_arguments)]
#![allow(dead_code)]
#![allow(non_camel_case_types)]
#![allow(ambiguous_glob_reexports)]

pub mod error;
pub mod ffi;
pub mod core;

pub use error::*;
pub use core::*;

#[cfg(feature = "collision")]
pub mod collision;

#[cfg(feature = "dynamics")]
pub mod dynamics;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_creation() {
        let world = PhysicsWorld::new();
        assert_eq!(world.body_count(), 0);
    }

    #[test]
    fn test_world_gravity() {
        let world = PhysicsWorld::new();
        let gravity = Vec3::new(0.0, -10.0, 0.0);
        world.set_gravity(gravity);
        let retrieved = world.get_gravity();
        assert!((retrieved.y - gravity.y).abs() < 0.001);
    }

    #[test]
    fn test_world_builder() {
        let world = PhysicsWorldBuilder::new()
            .gravity(Vec3::new(0.0, -20.0, 0.0))
            .build();
        let gravity = world.get_gravity();
        assert!((gravity.y - (-20.0)).abs() < 0.001);
    }

    #[test]
    fn test_box_shape_creation() {
        let shape = CollisionShapeBuilder::new()
            .box_shape(Vec3::new(1.0, 1.0, 1.0))
            .build();
        assert!(shape.is_some());
    }

    #[test]
    fn test_sphere_shape_creation() {
        let shape = CollisionShapeBuilder::new()
            .sphere(1.0)
            .build();
        assert!(shape.is_some());
    }

    #[test]
    fn test_capsule_shape_creation() {
        let shape = CollisionShapeBuilder::new()
            .capsule(0.5, 1.0)
            .build();
        assert!(shape.is_some());
    }

    #[test]
    fn test_cylinder_shape_creation() {
        let shape = CollisionShapeBuilder::new()
            .cylinder(Vec3::new(0.5, 0.5, 0.5))
            .build();
        assert!(shape.is_some());
    }

    #[test]
    fn test_cone_shape_creation() {
        let shape = CollisionShapeBuilder::new()
            .cone(0.5, 1.0)
            .build();
        assert!(shape.is_some());
    }

    #[test]
    fn test_rigid_body_creation() {
        let mut world = PhysicsWorld::new();
        
        let shape = CollisionShapeBuilder::new()
            .box_shape(Vec3::new(1.0, 1.0, 1.0))
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(1.0)
            .position(Vec3::new(0.0, 10.0, 0.0))
            .build()
            .unwrap();
        
        let handle = world.add_rigid_body(body);
        assert_eq!(world.body_count(), 1);
        
        let retrieved = world.get_rigid_body(handle);
        assert!(retrieved.is_some());
    }

    #[test]
    fn test_rigid_body_transform() {
        let mut world = PhysicsWorld::new();
        
        let shape = CollisionShapeBuilder::new()
            .sphere(1.0)
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(1.0)
            .position(Vec3::new(5.0, 10.0, 15.0))
            .build()
            .unwrap();
        
        let handle = world.add_rigid_body(body);
        
        let retrieved = world.get_rigid_body(handle).unwrap();
        let pos = retrieved.get_position();
        
        assert!((pos.x - 5.0).abs() < 0.001);
        assert!((pos.y - 10.0).abs() < 0.001);
        assert!((pos.z - 15.0).abs() < 0.001);
    }

    #[test]
    fn test_physics_simulation() {
        let mut world = PhysicsWorld::new();
        world.set_gravity(Vec3::new(0.0, -10.0, 0.0));
        
        let shape = CollisionShapeBuilder::new()
            .sphere(0.5)
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(1.0)
            .position(Vec3::new(0.0, 10.0, 0.0))
            .build()
            .unwrap();
        
        let handle = world.add_rigid_body(body);
        
        let initial_pos = world.get_rigid_body(handle).unwrap().get_position();
        
        for _ in 0..60 {
            world.step(1.0 / 60.0);
        }
        
        let final_pos = world.get_rigid_body(handle).unwrap().get_position();
        
        assert!(final_pos.y < initial_pos.y);
    }

    #[test]
    fn test_rigid_body_removal() {
        let mut world = PhysicsWorld::new();
        
        let shape = CollisionShapeBuilder::new()
            .box_shape(Vec3::new(1.0, 1.0, 1.0))
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(1.0)
            .position(Vec3::new(0.0, 0.0, 0.0))
            .build()
            .unwrap();
        
        let handle = world.add_rigid_body(body);
        assert_eq!(world.body_count(), 1);
        
        let removed = world.remove_rigid_body(handle);
        assert!(removed.is_some());
        assert_eq!(world.body_count(), 0);
    }

    #[test]
    fn test_rigid_body_properties() {
        let shape = CollisionShapeBuilder::new()
            .box_shape(Vec3::new(1.0, 1.0, 1.0))
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(2.0)
            .position(Vec3::new(0.0, 0.0, 0.0))
            .friction(0.5)
            .restitution(0.8)
            .build()
            .unwrap();
        
        assert!((body.get_friction() - 0.5).abs() < 0.001);
        assert!((body.get_restitution() - 0.8).abs() < 0.001);
        
        body.set_linear_velocity(Vec3::new(1.0, 2.0, 3.0));
        let vel = body.get_linear_velocity();
        assert!((vel.x - 1.0).abs() < 0.001);
        assert!((vel.y - 2.0).abs() < 0.001);
        assert!((vel.z - 3.0).abs() < 0.001);
    }

    #[test]
    fn test_static_body() {
        let shape = CollisionShapeBuilder::new()
            .box_shape(Vec3::new(10.0, 0.1, 10.0))
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(0.0)
            .position(Vec3::new(0.0, 0.0, 0.0))
            .build()
            .unwrap();
        
        assert!(body.is_static());
    }

    #[test]
    fn test_kinematic_body() {
        let shape = CollisionShapeBuilder::new()
            .box_shape(Vec3::new(1.0, 2.0, 1.0))
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(1.0)
            .position(Vec3::new(0.0, 0.0, 0.0))
            .kinematic()
            .build()
            .unwrap();
        
        assert!(body.is_kinematic());
    }

    #[test]
    fn test_ray_cast() {
        let mut world = PhysicsWorld::new();
        world.set_gravity(Vec3::new(0.0, -10.0, 0.0));
        
        let shape = CollisionShapeBuilder::new()
            .sphere(1.0)
            .build()
            .unwrap();
        
        let body = RigidBodyBuilder::new()
            .shape(shape)
            .mass(1.0)
            .position(Vec3::new(0.0, 0.0, 0.0))
            .build()
            .unwrap();
        
        world.add_rigid_body(body);
        
        let result = world.ray_test_closest(
            Vec3::new(0.0, 10.0, 0.0),
            Vec3::new(0.0, -10.0, 0.0),
        );
        
        assert!(result.is_some());
        let hit = result.unwrap();
        assert!(hit.hit_fraction > 0.0 && hit.hit_fraction < 1.0);
    }

    #[test]
    fn test_constraint_point2point() {
        let mut world = PhysicsWorld::new();
        
        let shape_a = CollisionShapeBuilder::new()
            .sphere(0.5)
            .build()
            .unwrap();
        
        let shape_b = CollisionShapeBuilder::new()
            .sphere(0.5)
            .build()
            .unwrap();
        
        let body_a = RigidBodyBuilder::new()
            .shape(shape_a)
            .mass(1.0)
            .position(Vec3::new(-1.0, 0.0, 0.0))
            .build()
            .unwrap();
        
        let body_b = RigidBodyBuilder::new()
            .shape(shape_b)
            .mass(1.0)
            .position(Vec3::new(1.0, 0.0, 0.0))
            .build()
            .unwrap();
        
        let handle_a = world.add_rigid_body(body_a);
        let handle_b = world.add_rigid_body(body_b);
        
        let constraint = world.create_point2point_constraint(
            handle_a,
            handle_b,
            Vec3::new(0.5, 0.0, 0.0),
            Vec3::new(-0.5, 0.0, 0.0),
        );
        
        assert!(constraint.is_some());
    }
}
