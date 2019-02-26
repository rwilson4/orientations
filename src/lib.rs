use std::ops::Sub;
const DBL_EPSILON: f64 = 2.220_446_049_250_313e-16;

/// A 3-d vector
#[derive(Copy, Clone, PartialEq, Debug)]
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
    pub fn new(data: [f64; 3]) -> Vector3d {
        Vector3d{ data }
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
    pub fn dot(&self, other: &Vector3d) -> f64 {
        let mut dot_product: f64 = 0.0;
        for i in 0..3 {
            dot_product += self.data[i] * other.data[i];
        }
        dot_product
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

    /// Alias for norm
    pub fn abs(&self) -> f64 {
        self.norm()
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
    pub fn scalar_multiple(&self, alpha: f64) -> Vector3d {
        Vector3d::new(
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
    pub fn negate(&self) -> Vector3d {
        Vector3d::new(
            [
                -self.data[0],
                -self.data[1],
                -self.data[2]
            ]
        )
    }

    /// Return a vector with the same direction as self but unit
    /// magnitude. The return value is wrapped in an Option in case
    /// the vector has zero mangitude, in which case the result will
    /// be None.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::Vector3d;
    /// let x = Vector3d::new([2.0, 0.0, 0.0]);
    /// assert_eq!(Vector3d::x(), x.normalized().unwrap());
    /// ```
    pub fn normalized(&self) -> Result<Vector3d, String> {
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
    pub fn zero() -> Vector3d {
        Vector3d::new( [0.0, 0.0, 0.0] )
    }

    /// Create a new unit Vector3d aligned with the x-axis.
    pub fn x() -> Vector3d {
        Vector3d::new( [1.0, 0.0, 0.0] )
    }

    /// Create a new unit Vector3d aligned with the x-axis.
    pub fn y() -> Vector3d {
        Vector3d::new( [0.0, 1.0, 0.0] )
    }

    /// Create a new unit Vector3d aligned with the x-axis.
    pub fn z() -> Vector3d {
        Vector3d::new( [0.0, 0.0, 1.0] )
    }
}

impl Sub for Vector3d {
    type Output = Vector3d;

    fn sub(self, other: Vector3d) -> Vector3d {
        Vector3d::new([
            self.data[0] - other.data[0],
            self.data[1] - other.data[1],
            self.data[2] - other.data[2]
        ])
    }

}


/// A quaternion
#[derive(Clone, PartialEq, Debug)]
pub struct Quaternion {
    real_part: f64,
    imaginary_part: Vector3d
}

pub trait Rotation {
    type R: Rotation;
    fn identity() -> Self::R;
    fn inverse(&self) -> Self::R;
    fn as_quaternion(&self) -> Quaternion;
    fn angle_axis(&self) -> (f64, Vector3d);
    fn before<T: Rotation<R = T>>(&self, r: &T) -> T;
    fn after<T: Rotation<R = T>>(&self, r: &T) -> T;
    fn multiply<T: Rotation>(&self, r: &T) -> Self::R;
}

pub trait Orientation {
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
    pub fn new(real_part: f64, imaginary_part: Vector3d) -> Quaternion {
        Quaternion {
            real_part,
            imaginary_part
        }
    }

    /// Create a quaternion from the corresponding angle and axis of rotation.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let angle = std::f64::consts::PI / 2.0;
    /// let q = Quaternion::from_angle_axis(angle, &Vector3d::x());
    /// let sqrt2_over_2 = (2.0_f64).sqrt() / 2.0;
    /// let expected = Quaternion::new(sqrt2_over_2, Vector3d::x().scalar_multiple(sqrt2_over_2));
    /// // assert_eq!(expected, q);
    /// ```
    pub fn from_angle_axis(angle: f64, axis: &Vector3d) -> Quaternion {
        let real_part = (angle / 2.0).cos();
        let imaginary_part = axis.scalar_multiple((angle / 2.0).sin() / axis.norm());
        Quaternion::new(real_part, imaginary_part)
    }

    /// Compute the conjugate of a quaternion.
    fn conjugate(&self) -> Quaternion {
        Quaternion::new(self.real_part, self.imaginary_part.negate())
    }

    /// Compute the square of the (l2) norm of the quaternion.
    fn norm_squared(&self) -> f64 {
        self.real_part * self.real_part + self.imaginary_part.norm_squared()
    }

    fn norm(&self) -> f64 {
        self.norm_squared().sqrt()
    }

}

impl Rotation for Quaternion {
    type R = Quaternion;

    /// Return the identity Quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let expected = Quaternion::new(1.0, Vector3d::zero());
    /// assert_eq!(expected, Quaternion::identity());
    /// ```
    fn identity() -> Quaternion {
        Quaternion::new(1.0, Vector3d::zero())
    }

    /// Calculate the inverse of a quaternion.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let angle = std::f64::consts::PI / 2.0;
    /// let q = Quaternion::from_angle_axis(angle, &Vector3d::x());
    /// let expected = Quaternion::from_angle_axis(angle, &Vector3d::x().negate());
    /// assert_eq!(expected, q.inverse());
    /// ```
    fn inverse(&self) -> Quaternion {
        // Check that norm is > 0
        let norm_squared = self.norm_squared();
        if norm_squared < DBL_EPSILON {
            panic!("Quaternion close to zero; cannot invert.")
        }

        let inv_norm_squared = 1.0 / norm_squared;
        let c = self.conjugate();
        let real_part = c.real_part * inv_norm_squared;
        let imaginary_part = c.imaginary_part.scalar_multiple(inv_norm_squared);
        Quaternion::new(real_part, imaginary_part)
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
    fn as_quaternion(&self) -> Quaternion {
        self.clone()
    }

    /// Get the angle and axis associated with a rotation. If the
    /// rotation is the identity, the z-axis will be returned.
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
    /// ```
    fn multiply<T: Rotation>(&self, r: &T) -> Quaternion {
        let rr = r.as_quaternion();
        let real_part = self.real_part * rr.real_part - self.imaginary_part.dot(&rr.imaginary_part);
        let q_i = self.imaginary_part.data[1] * rr.imaginary_part.data[2] - self.imaginary_part.data[2] * rr.imaginary_part.data[1];
        let q_j = self.imaginary_part.data[2] * rr.imaginary_part.data[0] - self.imaginary_part.data[0] * rr.imaginary_part.data[2];
        let q_k = self.imaginary_part.data[0] * rr.imaginary_part.data[1] - self.imaginary_part.data[1] * rr.imaginary_part.data[0];
        let imaginary_part = Vector3d::new( [q_i, q_j, q_k] );

        Quaternion::new(real_part, imaginary_part)
    }

    /// Compose two rotations.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let angle = std::f64::consts::PI / 2.0;
    /// let r = Quaternion::from_angle_axis(angle, &Vector3d::x());
    /// let q = Quaternion::from_angle_axis(angle, &Vector3d::y());
    /// let expected = Quaternion::from_angle_axis(angle, &Vector3d::z().negate());
    /// // r.before(&q) is the rotation equivalent to rotating first
    /// // by r then by q.
    /// // assert_eq!(expected, r.before(&q));
    /// ```
    fn before<T: Rotation<R = T>>(&self, r: &T) -> T {
        r.multiply(self)
    }

    /// Compose two rotations.
    ///
    /// # Examples
    ///
    /// ```
    /// use orientations::*;
    /// let r = Quaternion::identity();
    /// let q = Quaternion::identity();
    ///
    /// // r.after(&q) is the rotation equivalent to rotating first
    /// // by q then by r.
    /// assert_eq!(Quaternion::identity(), r.after(&q));
    /// ```
    fn after<T: Rotation<R = T>>(&self, r: &T) -> T {
        r.inverse().multiply(&self.inverse()).inverse()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector3d_dot() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let y = Vector3d::new([4.0, 5.0, 6.0]);
        let expected = 32.0;
        let actual = x.dot(&y);
        assert_eq!(actual, expected);
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
                 (left: `{:?}`, right: `{:?}`, expect angle: `{:?}`, real angle: `{:?}`)",
                &$a,
                &$b,
                angle_a,
                angle_b
            );
            
            assert!(
                (axis_a - axis_b).norm() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect axis: `{:?}`, real axis: `{:?}`)",
                &$a,
                &$b,
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
                 (left: `{:?}`, right: `{:?}`, expect angle: `{:?}`, real angle: `{:?}`)",
                &$a,
                &$b,
                angle_a,
                angle_b
            );
            
            assert!(
                (axis_a - axis_b).norm() < eps,
                "assertion failed: `(left !== right)` \
                 (left: `{:?}`, right: `{:?}`, expect axis: `{:?}`, real axis: `{:?}`)",
                &$a,
                &$b,
                axis_a,
                axis_b
            );
        }};
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
    fn quaternion_inverse() {
        let sqrt2 = (2 as f64).sqrt() / 2.0;
        let q = Quaternion::new(sqrt2, Vector3d::new([sqrt2, 0.0, 0.0]));
        let expected = Quaternion::new(sqrt2, Vector3d::new([-sqrt2, 0.0, 0.0]));
        assert_quat_approx_eq!(expected, q.inverse());
    }

    #[test]
    fn quaternion_as_quaternion() {
        let r = Quaternion::identity();
        assert_eq!(Quaternion::identity(), r.as_quaternion());
    }

    #[test]
    fn quaternion_multiply() {
        let r = Quaternion::identity();
        let q = Quaternion::identity();
        assert_eq!(Quaternion::identity(), r.multiply(&q));
    }

    #[test]
    fn quaternion_before() {
        let r = Quaternion::identity();
        let q = Quaternion::identity();
        assert_eq!(Quaternion::identity(), r.before(&q));
    }

    #[test]
    fn quaternion_after() {
        let r = Quaternion::identity();
        let q = Quaternion::identity();
        assert_eq!(Quaternion::identity(), r.after(&q));
    }


}
