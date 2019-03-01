#![deny(missing_docs)]
#![warn(clippy::all, clippy::pedantic)]

//! A library for Rotations and Orientations.

use std::ops::{Add, Sub};
use std::fmt;
const DBL_EPSILON: f64 = 2.220_446_049_250_313e-16;

/// A 3-d vector
#[derive(Copy, Clone, PartialEq)]
pub struct Vector3d {
    data: [f64; 3]
}

impl Vector3d {
    /// Create a new Vector3d.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([1.0, 2.0, 3.0]);
    /// ```
    pub fn new(data: [f64; 3]) -> Self {
        Self{ data }
    }

    /// Computes the dot product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([1.0, 2.0, 3.0]);
    /// let y = Vector3d::new([4.0, 5.0, 6.0]);
    /// assert_eq!(32.0, x.dot(&y));
    /// ```
    pub fn dot(&self, other: &Self) -> f64 {
        let mut dot_product: f64 = 0.0;
        for i in 0..3 {
            dot_product += self.data[i] * other.data[i];
        }
        dot_product
    }

    /// Computes the cross product of two vectors.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([1.0, 2.0, 3.0]);
    /// let y = Vector3d::new([4.0, 5.0, 6.0]);
    /// let expected = Vector3d::new([-3.0, 6.0, -3.0]);
    /// assert_eq!(expected, x.cross(&y));
    /// ```
    pub fn cross(&self, other: &Self) -> Self {
        let x1 = self.data[1] * other.data[2] - self.data[2] * other.data[1];
        let x2 = self.data[2] * other.data[0] - self.data[0] * other.data[2];
        let x3 = self.data[0] * other.data[1] - self.data[1] * other.data[0];
        Self::new([x1, x2, x3])
    }

    /// Computes the square of the (l2) norm of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([1.0, 2.0, 3.0]);
    /// assert_eq!(14.0, x.norm_squared());
    /// ```
    pub fn norm_squared(&self) -> f64 {
        self.dot(&self)
    }

    /// Computes the (l2) norm of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([1.0, 2.0, 2.0]);
    /// assert_eq!(3.0, x.norm());
    /// ```
    pub fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

    /// Computes the scalar multiple of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([1.0, 2.0, 3.0]);
    /// let alpha = 2.0;
    /// let expected = Vector3d::new([2.0, 4.0, 6.0]);
    /// assert_eq!(expected, x.scalar_multiple(alpha));
    /// ```
    pub fn scalar_multiple(&self, alpha: f64) -> Self {
        Self::new(
            [
                alpha * self.data[0],
                alpha * self.data[1],
                alpha * self.data[2]
            ]
        )
    }

    /// Computes the negative of a vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([1.0, 2.0, 3.0]);
    /// let expected = Vector3d::new([-1.0, -2.0, -3.0]);
    /// assert_eq!(expected, x.negate());
    /// ```
    pub fn negate(&self) -> Self {
        Self::new(
            [
                -self.data[0],
                -self.data[1],
                -self.data[2]
            ]
        )
    }

    /// Return a vector with the same direction as self but unit
    /// magnitude. The return value is wrapped in a Result in case
    /// the vector has zero mangitude, in which case the result will
    /// be an error that the caller must handle.
    ///
    /// # Errors
    /// If vector has norm close to zero, the result will be an Error.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([2.0, 0.0, 0.0]);
    /// assert_eq!(Vector3d::x(), x.normalized().unwrap());
    /// ```
    pub fn normalized(&self) -> Result<Self, String> {
        let n = self.norm();
        if n < DBL_EPSILON {
            Err(String::from("Cannot normalize vector with zero magnitude"))
        } else {
            Ok(self.scalar_multiple(1.0 / n))
        }
    }

    /// Returns the zero vector.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let expected = Vector3d::new([0.0, 0.0, 0.0]);
    /// assert_eq!(expected, Vector3d::zero());
    /// ```
    pub fn zero() -> Self {
        Self::new( [0.0, 0.0, 0.0] )
    }

    /// Create a new unit Vector3d aligned with the x-axis.
    pub fn x() -> Self {
        Self::new( [1.0, 0.0, 0.0] )
    }

    /// Create a new unit Vector3d aligned with the x-axis.
    pub fn y() -> Self {
        Self::new( [0.0, 1.0, 0.0] )
    }

    /// Create a new unit Vector3d aligned with the x-axis.
    pub fn z() -> Self {
        Self::new( [0.0, 0.0, 1.0] )
    }
}

impl Add for Vector3d {
    type Output = Self;

    /// Add two vectors.
    fn add(self, other: Self) -> Self {
        Self::new([
            self.data[0] + other.data[0],
            self.data[1] + other.data[1],
            self.data[2] + other.data[2]
        ])
    }

}

impl Sub for Vector3d {
    type Output = Self;

    /// Subtract a vector from another.
    fn sub(self, other: Self) -> Self {
        Self::new([
            self.data[0] - other.data[0],
            self.data[1] - other.data[1],
            self.data[2] - other.data[2]
        ])
    }

}

impl fmt::Debug for Vector3d {
    /// Pretty-print a vector.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}, {}, {}]",
               self.data[0], self.data[1], self.data[2])
    }
}


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

/// Orientation trait
pub trait Orientation {
}

/// A quaternion
#[derive(Clone, PartialEq)]
pub struct Quaternion {
    real_part: f64,
    imaginary_part: Vector3d
}

impl Quaternion {
    /// Create a new Quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// let real_part = 1.0;
    /// let imaginary_part = orientations::Vector3d::zero();
    /// let q = orientations::Quaternion::new(real_part, imaginary_part);
    /// ```
    pub fn new(real_part: f64, imaginary_part: Vector3d) -> Self {
        Self {
            real_part,
            imaginary_part
        }
    }

    /// Create a quaternion from the corresponding angle and axis of rotation.
    ///
    /// # Panics
    /// Panics if axis has norm close to zero.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let angle = std::f64::consts::PI / 2.0;
    /// let q = Quaternion::from_angle_axis(angle, &Vector3d::x());
    /// ```
    pub fn from_angle_axis(angle: f64, axis: &Vector3d) -> Self {
        let axis_norm = axis.norm();
        if axis_norm < DBL_EPSILON {
            panic!("Axis has zero norm")
        }

        let half_angle = angle / 2.0;
        let real_part = half_angle.cos();
        let imaginary_part = axis.scalar_multiple(half_angle.sin() / axis_norm);
        Self::new(real_part, imaginary_part)
    }

    /// Compute the conjugate of a quaternion.
    fn conjugate(&self) -> Self {
        Self::new(self.real_part, self.imaginary_part.negate())
    }

    /// Compute the square of the (l2) norm of the quaternion.
    fn norm_squared(&self) -> f64 {
        self.real_part * self.real_part + self.imaginary_part.norm_squared()
    }

    /// Compute the (l2) norm of the quaternion.
    fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

}

impl fmt::Debug for Quaternion {
    /// Pretty-print a quaternion.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let signs: Vec<char> = self.imaginary_part.data.iter()
            .map(|x| if x >= &0.0 {'+'} else {'-'})
            .collect();

        write!(f, "Quaternion {} {} {}i {} {}j {} {}k",
               self.real_part,
               signs[0], self.imaginary_part.data[0].abs(),
               signs[1], self.imaginary_part.data[1].abs(),
               signs[2], self.imaginary_part.data[2].abs())
    }
}

impl Rotation for Quaternion {
    type R = Self;

    /// Return the identity Quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let expected = Quaternion::new(1.0, Vector3d::zero());
    /// assert_eq!(expected, Quaternion::identity());
    /// ```
    fn identity() -> Self {
        Self::new(1.0, Vector3d::zero())
    }

    /// Calculate the inverse of a quaternion.
    ///
    /// # Errors
    ///
    /// Returns an error if the quaternion is close to zero. This can
    /// happen if Quaternion::new() is misused (e.g. by instantiating
    /// an all-zero quaternion), or through the accumulation of
    /// floating point errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let angle = std::f64::consts::PI / 2.0;
    /// let q = Quaternion::from_angle_axis(angle, &Vector3d::x());
    /// let expected = Quaternion::from_angle_axis(angle, &Vector3d::x().negate());
    /// assert_eq!(expected, q.inverse().unwrap());
    /// ```
    fn inverse(&self) -> Result<Self, String> {
        // Check that norm is > 0
        let norm_squared = self.norm_squared();
        if norm_squared < DBL_EPSILON {
            return Err(String::from("Quaternion close to zero; cannot invert."))
        }

        let inv_norm_squared = 1.0 / norm_squared;
        let c = self.conjugate();
        let real_part = c.real_part * inv_norm_squared;
        let imaginary_part = c.imaginary_part.scalar_multiple(inv_norm_squared);
        Ok(Self::new(real_part, imaginary_part))
    }

    /// Get the quaternion representation of a rotation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let r = Quaternion::from_angle_axis(0.03, &Vector3d::x());
    /// assert_eq!(r, r.as_quaternion());
    /// ```
    fn as_quaternion(&self) -> Self {
        self.clone()
    }

    /// Get the angle and axis associated with a rotation. If the
    /// rotation is the identity (and therefore there is no axis of
    /// rotation), the z-axis will be returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let q = Quaternion::identity();
    /// let (angle, axis) = q.angle_axis();
    /// assert_eq!(angle, 0.0);
    /// assert_eq!(axis, Vector3d::z());
    /// ```
    fn angle_axis(&self) -> (f64, Vector3d) {
        let n = self.norm();
        if n <= DBL_EPSILON {
            // If the quaternion is too close to zero, just return the
            // identity.
            return Self::identity().angle_axis()
        }

        let angle = (self.real_part / n).acos() * 2.0;
        let axis = match self.imaginary_part.normalized() {
            Ok(axis) => axis,
            Err(_error) => Vector3d::z()
        };

        (angle, axis)
    }

    /// Compose two rotations.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let angle = std::f64::consts::PI / 2.0;
    /// let r = Quaternion::from_angle_axis(angle, &Vector3d::x());
    /// let q = Quaternion::from_angle_axis(angle, &Vector3d::x().negate());
    /// assert_eq!(Quaternion::identity(), r.multiply(&q));
    /// assert_eq!(Quaternion::identity(), q.multiply(&r));
    /// ```
    fn multiply<T: Rotation>(&self, r: &T) -> Self {
        let rr = r.as_quaternion();
        let real_part = self.real_part * rr.real_part - self.imaginary_part.dot(&rr.imaginary_part);
        let imaginary_part = rr.imaginary_part.scalar_multiple(self.real_part)
            + self.imaginary_part.scalar_multiple(rr.real_part)
            + self.imaginary_part.cross(&rr.imaginary_part);

        Self::new(real_part, imaginary_part)
    }

    /// Compose two rotations.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let q = Quaternion::identity();
    /// let r = Quaternion::identity();
    ///
    /// // q.before(&r) is the rotation equivalent to rotating first
    /// // by q then by r.
    /// assert_eq!(Quaternion::identity(), q.before(&r));
    /// ```
    fn before<T: Rotation<R = T>>(&self, r: &T) -> T {
        r.multiply(self)
    }

    /// Compose two rotations.
    ///
    /// # Panics
    ///
    /// This function *can* panic if the inverse of the operand is
    /// poorly defined. See the comment in the example below for more
    /// specific context. This doesn't happen in the `before` function
    /// due to implementation details.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let q = Quaternion::identity();
    /// let r = Quaternion::identity();
    ///
    /// // q.after(&r) is the rotation equivalent to rotating first
    /// // by r then by q. This will panic if r is close to zero,
    /// // in which case it is not a valid rotation!
    /// assert_eq!(Quaternion::identity(), q.after(&r));
    /// ```
    fn after<T: Rotation<R = T>>(&self, r: &T) -> T {
        r.inverse().unwrap().multiply(&self.inverse().unwrap()).inverse().unwrap()
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn vector3d_dot() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let y = Vector3d::new([4.0, 5.0, 6.0]);
        let expected = 32.0;
        let actual = x.dot(&y);
        assert_eq!(actual, expected);
    }

    #[test]
    fn vector3d_cross() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let y = Vector3d::new([4.0, 5.0, 6.0]);
        let expected = Vector3d::new([-3.0, 6.0, -3.0]);
        assert_eq!(expected, x.cross(&y));
    }

    #[test]
    fn vector3d_x_cross_y_equals_z() {
        let x = Vector3d::x();
        let y = Vector3d::y();
        assert_eq!(Vector3d::z(), x.cross(&y));
    }

    macro_rules! norm_squared_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                assert_eq!(expected, input.norm_squared());
            }
        )*
        }
    }

    norm_squared_tests! {
        norm_squared_0: (Vector3d::new([1.0, 2.0, 3.0]), 14.0),
        norm_squared_1: (Vector3d::new([-1.0, -2.0, -3.0]), 14.0),
        norm_squared_2: (Vector3d::new([-1.0, 2.0, 3.0]), 14.0),
        norm_squared_3: (Vector3d::new([1.0, 0.0, 3.0]), 10.0),
    }

    macro_rules! zero_dot_x_is_zero_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let zero = Vector3d::zero();
                assert_eq!(0.0, zero.dot(&$value));
            }
        )*
        }
    }

    zero_dot_x_is_zero_tests! {
        zero_dot_x_is_zero_0: Vector3d::new([1.0, 2.0, 3.0]),
        zero_dot_x_is_zero_1: Vector3d::new([-1.0, 2.0, 3.0]),
        zero_dot_x_is_zero_2: Vector3d::new([1.0, -2.0, 3.0]),
        zero_dot_x_is_zero_3: Vector3d::new([1.0, 2.0, -3.0]),
    }

    #[test]
    fn vector3d_zero_normalized() {
        let zero = Vector3d::zero();
        match zero.normalized() {
            Ok(_) => assert!(false, "Should not be able to normalize zero vector"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn vector3d_norm() {
        let x = Vector3d::new([1.0, 2.0, 2.0]);
        let expected = 3.0;
        assert_eq!(expected, x.norm());
    }

    #[test]
    fn vector3d_scalar_multiple() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let alpha = 2.0;
        let expected = Vector3d::new([2.0, 4.0, 6.0]);
        assert_eq!(expected, x.scalar_multiple(alpha));
    }

    #[test]
    fn vector3d_negate() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let expected = Vector3d::new([-1.0, -2.0, -3.0]);
        assert_eq!(expected, x.negate());
    }

    /// Asserts that two vectors are approximately (~1.0e-6) equal to each other.
    ///
    /// On panic, this macro will print the values of the expressions with their
    /// debug representations. You can optionally add an optional diff value. If you
    /// don't supply a diff value as an argument, `1.0e-6` is the default used.
    ///
    /// Source: https://github.com/ashleygwilliams/assert_approx_eq
    macro_rules! assert_vector_approx_eq {
        ($a:expr, $b:expr) => {{
            let eps = 1.0e-6;
            let err = $a - $b;
            assert!(
                err.norm() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                &$a,
                &$b,
                eps,
                err.norm()
            );
        }};
        ($a:expr, $b:expr, $eps:expr) => {{
            let eps = $eps;
            let err = $a - $b;
            assert!(
                err.norm() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                &$a,
                &$b,
                eps,
                err.norm()
            );
        }};
    }

    macro_rules! assert_float_approx_eq {
        ($a:expr, $b:expr) => {{
            let eps = 1.0e-6;
            let err = $a - $b;
            assert!(
                err.abs() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                &$a,
                &$b,
                eps,
                err.abs()
            );
        }};
        ($a:expr, $b:expr, $eps:expr) => {{
            let eps = $eps;
            let err = $a - $b;
            assert!(
                err.abs() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect diff: `{:?}`, real diff: `{:?}`)",
                &$a,
                &$b,
                eps,
                err.abs()
            );
        }};
    }

    macro_rules! assert_quat_approx_eq {
        ($a:expr, $b:expr) => {{
            let eps = 1.0e-6;

            let (angle_a, axis_a) = $a.angle_axis();
            let (angle_b, axis_b) = $b.angle_axis();

            assert!(
                (angle_a - angle_b).abs() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, \
                 expect angle: `{:?}`, real angle: `{:?}`) \
                 expect axis: `{:?}`, real axis: `{:?}`",
                &$a,
                &$b,
                angle_a,
                angle_b,
                axis_a,
                axis_b
            );

            assert!(
                (axis_a - axis_b).norm() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, \
                 expect angle: `{:?}`, real angle: `{:?}`) \
                 expect axis: `{:?}`, real axis: `{:?}`",
                &$a,
                &$b,
                angle_a,
                angle_b,
                axis_a,
                axis_b
            );
        }};
        ($a:expr, $b:expr, $eps:expr) => {{
            let eps = $eps;

            let (angle_a, axis_a) = $a.angle_axis();
            let (angle_b, axis_b) = $b.angle_axis();

            assert!(
                (angle_a - angle_b).abs() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, \
                 expect angle: `{:?}`, real angle: `{:?}`) \
                 expect axis: `{:?}`, real axis: `{:?}`",
                &$a,
                &$b,
                angle_a,
                angle_b,
                axis_a,
                axis_b
            );

            assert!(
                (axis_a - axis_b).norm() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, \
                 expect angle: `{:?}`, real angle: `{:?}`) \
                 expect axis: `{:?}`, real axis: `{:?}`",
                &$a,
                &$b,
                angle_a,
                angle_b,
                axis_a,
                axis_b
            );
        }};
    }

    #[test]
    fn quaternion_from_angle_axis() {
        let angle = PI / 2.0;
        let q = Quaternion::from_angle_axis(angle, &Vector3d::x());
        let sqrt2_over_2 = (2.0_f64).sqrt() / 2.0;
        let expected = Quaternion::new(sqrt2_over_2, Vector3d::x().scalar_multiple(sqrt2_over_2));
        assert_quat_approx_eq!(expected, q);
    }

    #[test]
    #[should_panic]
    fn quaternion_from_angle_zero_axis() {
        Quaternion::from_angle_axis(0.0, &Vector3d::zero());
    }

    #[test]
    fn quaternion_conjugate() {
        let q = Quaternion::new(0.2, Vector3d::new([0.3, 0.4, 0.5]));
        let expected = Quaternion::new(0.2, Vector3d::new([-0.3, -0.4, -0.5]));
        assert_eq!(expected, q.conjugate());
    }

    #[test]
    fn quaternion_norm_squared() {
        let q = Quaternion::new(0.2, Vector3d::new([0.3, 0.4, 0.5]));
        assert_eq!(0.54, q.norm_squared());
    }

    #[test]
    fn quaternion_identity() {
        let expected = Quaternion::new(1.0, Vector3d::zero());
        assert_eq!(expected, Quaternion::identity());
    }

    #[test]
    fn quaternion_angle_axis() {
        let theta = 0.03;
        let xyz = Vector3d::new([1.0, 2.0, 3.0]).normalized().unwrap();
        let q = Quaternion::from_angle_axis(theta, &xyz);
        let (angle, axis) = q.angle_axis();
        assert_float_approx_eq!(theta, angle);
        assert_vector_approx_eq!(xyz, axis);
    }

    #[test]
    fn quaternion_zero_angle_axis() {
        let q = Quaternion::new(0.0, Vector3d::zero());
        let (angle, axis) = q.angle_axis();
        assert_eq!(0.0, angle);
        assert_eq!(Vector3d::z(), axis);
    }

    #[test]
    fn quaternion_inverse() {
        let sqrt2 = (2 as f64).sqrt() / 2.0;
        let q = Quaternion::new(sqrt2, Vector3d::new([sqrt2, 0.0, 0.0]));
        let expected = Quaternion::new(sqrt2, Vector3d::new([-sqrt2, 0.0, 0.0]));
        assert_quat_approx_eq!(expected, q.inverse().unwrap());
    }

    #[test]
    fn quaternion_zero_inverse() {
        let zero = Quaternion::new(0.0, Vector3d::new([0.0, 0.0, 0.0]));
        match zero.inverse() {
            Ok(_) => assert!(false, "Should not be able to invert zero"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn quaternion_as_quaternion() {
        let r = Quaternion::identity();
        assert_eq!(Quaternion::identity(), r.as_quaternion());
    }

    #[test]
    fn quaternion_multiply() {
        let q = Quaternion::identity();
        let r = Quaternion::identity();
        assert_eq!(Quaternion::identity(), q.multiply(&r));
    }

    #[test]
    fn quaternion_before() {
        let angle = PI / 2.0;
        let q = Quaternion::from_angle_axis(angle, &Vector3d::x());
        let r = Quaternion::from_angle_axis(angle, &Vector3d::y());
        let expected = Quaternion::new(0.5, Vector3d::new([0.5, 0.5, -0.5]));
        assert_quat_approx_eq!(expected, q.before(&r));
        assert_quat_approx_eq!(expected, r.after(&q));
    }

    #[test]
    fn quaternion_after() {
        let angle = PI / 2.0;
        let q = Quaternion::from_angle_axis(angle, &Vector3d::x());
        let r = Quaternion::from_angle_axis(angle, &Vector3d::y());
        let expected = Quaternion::new(0.5, Vector3d::new([0.5, 0.5, 0.5]));
        assert_quat_approx_eq!(expected, q.after(&r));
        assert_quat_approx_eq!(expected, r.before(&q));
    }


}
