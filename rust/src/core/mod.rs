pub mod collision;
pub mod constraint;
pub mod ghost;
pub mod rigidbody;
pub mod transform;
pub mod types;
pub mod world;

#[cfg(feature = "character")]
pub mod character;
#[cfg(feature = "gimpact")]
pub mod gimpact;
#[cfg(feature = "gimpact")]
pub mod gimpact_decomp;
#[cfg(feature = "hacd")]
pub mod hacd;
#[cfg(feature = "inverse-dynamics")]
pub mod id_utils;
#[cfg(feature = "inverse-dynamics")]
pub mod inverse_dynamics;
#[cfg(feature = "inverse-dynamics")]
pub mod mlcp_solver;
#[cfg(feature = "multibody")]
pub mod multibody_constraint;
#[cfg(feature = "softbody")]
pub mod reduced_softbody;
#[cfg(feature = "softbody")]
pub mod softbody;
#[cfg(feature = "vehicle")]
pub mod vehicle;
#[cfg(feature = "vhacd")]
pub mod vhacd;

pub use collision::{CollisionShape, CollisionShapeBuilder, CollisionShapeType, ShapeHandle};
pub use constraint::{
    ConstraintBuilder, ConstraintType, 
    Generic6DofConstraint, Generic6DofSpringConstraint, ConeTwistConstraint,
    UniversalConstraint, Hinge2Constraint, GearConstraint, Generic6DofSpring2Constraint,
    HingeConstraint, SliderConstraint,
    RotateOrder,
};
pub use ghost::GhostObject;
pub use rigidbody::{RigidBody, RigidBodyBuilder};
pub use transform::*;
pub use types::*;
pub use world::{BroadphaseType, PhysicsWorld, PhysicsWorldBuilder, RayTestResult, RigidBodyHandle, ConstraintHandle};

#[cfg(feature = "character")]
pub use character::CharacterController;
#[cfg(feature = "gimpact")]
pub use gimpact::GImpactShape;
#[cfg(feature = "gimpact")]
pub use gimpact_decomp::GImpactDecompShape;
#[cfg(feature = "hacd")]
pub use hacd::{HACD, HACDParams};
#[cfg(feature = "hacd")]
pub use hacd::ConvexHull as HACDConvexHull;
#[cfg(feature = "inverse-dynamics")]
pub use id_utils::{MultiBodyNameMap, MultiBodyTreeCreator, CloneTreeCreator, SimpleTreeCreator, User2InternalIndex, clone_multibody};
#[cfg(feature = "inverse-dynamics")]
pub use id_utils::random;
#[cfg(feature = "inverse-dynamics")]
pub use inverse_dynamics::{JointType, MultiBody};
#[cfg(feature = "inverse-dynamics")]
pub use mlcp_solver::{DantzigSolver, MLCPSolver};
#[cfg(feature = "multibody")]
pub use multibody_constraint::MultiBodyConstraint;
#[cfg(feature = "softbody")]
pub use reduced_softbody::ReducedDeformableBody;
#[cfg(feature = "softbody")]
pub use softbody::{SoftBody, SoftBodyWorldInfo};
#[cfg(feature = "vehicle")]
pub use vehicle::{Vehicle, VehicleRaycaster, VehicleTuning};
#[cfg(feature = "vhacd")]
pub use vhacd::{VHACD, VHACDParams};
#[cfg(feature = "vhacd")]
pub use vhacd::ConvexHull as VHACDConvexHull;
