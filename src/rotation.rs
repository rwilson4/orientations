use crate::vector3d::Vector3d;
use crate::quaternion::Quaternion;

/// Rotation trait
pub trait Rotation {
    /// The representation of the Rotation.
    /// Every implementor must specify this type. Generally, it will
    /// be the same type as the implementor. For example, a Quaternion
    /// will specify type R = Quaternion.
    type R: Rotation;

    /// The identity rotation equivalent to no rotation at all.
    fn identity() -> Self::R;

    /// The inverse of a rotation.
    fn inverse(&self) -> Result<Self::R, String>;

    /// Get the quaternion representation of a rotation.
    fn as_quaternion(&self) -> Quaternion;

    /// Get the angle and axis associated with a rotation.
    fn angle_axis(&self) -> (f64, Vector3d);

    /// Compose two rotations.
    fn before<T: Rotation<R = T>>(&self, r: &T) -> T;

    /// Compose two rotations.
    fn after<T: Rotation<R = T>>(&self, r: &T) -> T;

    /// Convenience function; should not be used.
    fn multiply<T: Rotation>(&self, r: &T) -> Self::R;
}

