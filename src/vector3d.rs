use std::ops::{Add, Sub};
use std::fmt;
use crate::constants::DBL_EPSILON;

/// A 3-d vector
#[derive(Copy, Clone, PartialEq)]
pub struct Vector3d {
    /// The vector
    pub data: [f64; 3]
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dot() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let y = Vector3d::new([4.0, 5.0, 6.0]);
        let expected = 32.0;
        let actual = x.dot(&y);
        assert_eq!(actual, expected);
    }

    #[test]
    fn cross() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let y = Vector3d::new([4.0, 5.0, 6.0]);
        let expected = Vector3d::new([-3.0, 6.0, -3.0]);
        assert_eq!(expected, x.cross(&y));
    }

    #[test]
    fn x_cross_y_equals_z() {
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
    fn zero_normalized() {
        let zero = Vector3d::zero();
        match zero.normalized() {
            Ok(_) => assert!(false, "Should not be able to normalize zero vector"),
            Err(_) => assert!(true)
        }
    }

    #[test]
    fn norm() {
        let x = Vector3d::new([1.0, 2.0, 2.0]);
        let expected = 3.0;
        assert_eq!(expected, x.norm());
    }

    #[test]
    fn scalar_multiple() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let alpha = 2.0;
        let expected = Vector3d::new([2.0, 4.0, 6.0]);
        assert_eq!(expected, x.scalar_multiple(alpha));
    }

    #[test]
    fn negate() {
        let x = Vector3d::new([1.0, 2.0, 3.0]);
        let expected = Vector3d::new([-1.0, -2.0, -3.0]);
        assert_eq!(expected, x.negate());
    }
}
