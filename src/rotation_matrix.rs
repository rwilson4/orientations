use std::fmt;
use crate::vector3d::Vector3d;
use crate::rotation::Rotation;
use crate::quaternion::Quaternion;

/// A rotation matrix
#[derive(Copy, Clone, PartialEq)]
pub struct RotationMatrix {
    rows: [Vector3d; 3]
}

impl RotationMatrix {
    /// Create a new RotationMatrix from rows.
    pub fn from_rows(rows: [Vector3d; 3]) -> Self {
        Self {rows}
    }

    /// Create a new RotationMatrix from columns.
    pub fn from_columns(columns: [Vector3d; 3]) -> Self {
        let r11 = columns[0].data[0];
        let r21 = columns[0].data[1];
        let r31 = columns[0].data[2];
        let r12 = columns[1].data[0];
        let r22 = columns[1].data[1];
        let r32 = columns[1].data[2];
        let r13 = columns[2].data[0];
        let r23 = columns[2].data[1];
        let r33 = columns[2].data[2];

        let r1 = Vector3d::new([r11, r12, r13]);
        let r2 = Vector3d::new([r21, r22, r23]);
        let r3 = Vector3d::new([r31, r32, r33]);
        RotationMatrix::from_rows([r1, r2, r3])
    }

    /// Get the rows.
    fn rows(&self) -> [Vector3d; 3] {
        self.rows.clone()
    }

    /// Get the columns.
    fn columns(&self) -> [Vector3d; 3] {
        let r11 = self.rows[0].data[0];
        let r12 = self.rows[0].data[1];
        let r13 = self.rows[0].data[2];
        let r21 = self.rows[1].data[0];
        let r22 = self.rows[1].data[1];
        let r23 = self.rows[1].data[2];
        let r31 = self.rows[2].data[0];
        let r32 = self.rows[2].data[1];
        let r33 = self.rows[2].data[2];

        let c1 = Vector3d::new([r11, r21, r31]);
        let c2 = Vector3d::new([r12, r22, r32]);
        let c3 = Vector3d::new([r13, r23, r33]);
        [c1, c2, c3]
    }

    /// Transpose.
    fn transpose(&self) -> Self {
        RotationMatrix::from_rows(self.columns())
    }
}

impl fmt::Debug for RotationMatrix {
    /// Pretty-print a rotation matrix.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let r11 = self.rows[0].data[0];
        let r12 = self.rows[0].data[1];
        let r13 = self.rows[0].data[2];
        let r21 = self.rows[1].data[0];
        let r22 = self.rows[1].data[1];
        let r23 = self.rows[1].data[2];
        let r31 = self.rows[2].data[0];
        let r32 = self.rows[2].data[1];
        let r33 = self.rows[2].data[2];
        write!(f, "[[{} {} {}][{} {} {}][{} {} {}]",
               r11, r12, r13,
               r21, r22, r23,
               r31, r32, r33
               )
    }
}

impl Rotation for RotationMatrix {
    type R = Self;

    fn identity() -> Self {
        Self::from_rows([Vector3d::x(), Vector3d::y(), Vector3d::z()])
    }

    fn inverse(&self) -> Result<Self, String> {
        Ok(self.transpose())
    }

    fn inverse_unchecked(&self) -> Self {
        self.transpose()
    }

    fn as_quaternion(&self) -> Quaternion {
        Quaternion::identity()
    }

    fn as_rotation_matrix(&self) -> Self {
        self.clone()
    }

    fn angle_axis(&self) -> (f64, Vector3d) {
        let angle = 0.0;
        let axis = Vector3d::z();
        (angle, axis)
    }

    fn multiply<T: Rotation>(&self, r: &T) -> Self {
        let rr = r.as_rotation_matrix();
        let rows = self.rows();
        let cols = rr.columns();

        let r11 = rows[0].dot(&cols[0]);
        let r12 = rows[0].dot(&cols[1]);
        let r13 = rows[0].dot(&cols[2]);
        let r21 = rows[1].dot(&cols[0]);
        let r22 = rows[1].dot(&cols[1]);
        let r23 = rows[1].dot(&cols[2]);
        let r31 = rows[2].dot(&cols[0]);
        let r32 = rows[2].dot(&cols[1]);
        let r33 = rows[2].dot(&cols[2]);

        let r1 = Vector3d::new([r11, r12, r13]);
        let r2 = Vector3d::new([r21, r22, r23]);
        let r3 = Vector3d::new([r31, r32, r33]);

        RotationMatrix::from_rows([r1, r2, r3])
    }

    fn before<T: Rotation<R = T>>(&self, r: &T) -> T {
        r.multiply(self)
    }

    fn after<T: Rotation<R = T>>(&self, r: &T) -> T{
        r.inverse_unchecked().multiply(&self.inverse_unchecked()).inverse_unchecked()
    }

    fn rotate_vector(&self, v: &Vector3d) -> Vector3d {
        let rows = self.rows();
        let u1 = rows[0].dot(v);
        let u2 = rows[1].dot(v);
        let u3 = rows[2].dot(v);
        Vector3d::new([u1, u2, u3])
    }
}
