use std::ops;
use super::Vector3;
use super::Matrix4;

pub struct Quaternion {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Quaternion {

    pub fn with_angle_axis(angle: f32, axis: &Vector3) -> Quaternion {
        let half_angle = angle / 2.;
        let s = half_angle.sin();

        Quaternion {
            x: axis.x * s,
            y: axis.y * s,
            z: axis.z * s,
            w: half_angle.cos()
        }
    }

    pub fn to_matrix(&self) -> Matrix4 {
        let (x, y, z, w) = (self.x, self.y, self.z, self.w);
        let (x2, y2, z2) = (x * 2., y * 2., z * 2.);
        let (xx, xy, xz) = (x * x2, x * y2, x * z2);
        let (yy, yz, zz) = (y * y2, y * z2, z * z2);
        let (wx, wy, wz) = (w * x2, w * y2, w * z2);
        let mut m: [f32; 16] = [0.; 16];

        m[ 0 ] = 1. - ( yy + zz );
        m[ 4 ] = xy - wz;
        m[ 8 ] = xz + wy;

        m[ 1 ] = xy + wz;
        m[ 5 ] = 1. - ( xx + zz );
        m[ 9 ] = yz - wx;

        m[ 2 ] = xz - wy;
        m[ 6 ] = yz + wx;
        m[ 10 ] = 1. - ( xx + yy );

        m[ 3 ] = 0.;
        m[ 7 ] = 0.;
        m[ 11 ] = 0.;

        m[ 12 ] = 0.;
        m[ 13 ] = 0.;
        m[ 14 ] = 0.;
        m[ 15 ] = 1.;

        Matrix4 { m }
    }

    pub fn transform(&self, v: &Vector3) -> Vector3 {
        let (x, y, z) = (v.x, v.y, v.z);
        let (qx, qy, qz, qw) = (self.x, self.y, self.z, self.w);

        // Calculate quaternion * vector.
        let ix = qw * x + qy * z - qz * y;
        let iy = qw * y + qz * x - qx * z;
        let iz = qw * z + qx * y - qy * x;
        let iw = -qx * x - qy * y - qz * z;

        // Calculate result * inverse quaternion.
        let rx = ix * qw + iw * -qx + iy * -qz - iz * -qy;
        let ry = iy * qw + iw * -qy + iz * -qx - ix * -qz;
        let rz = iz * qw + iw * -qz + ix * -qy - iy * -qx;

        Vector3::new(rx, ry, rz)
    }
}

impl ops::Mul<Self> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Quaternion) -> Quaternion {
        let (qax, qay, qaz, qaw) = (self.x, self.y, self.z, self.w);
        let (qbx, qby, qbz, qbw) = (rhs.x, rhs.y, rhs.z, rhs.w);
        let x = qax * qbw + qaw * qbx + qay * qbz - qaz * qby;
        let y = qay * qbw + qaw * qby + qaz * qbx - qax * qbz;
        let z = qaz * qbw + qaw * qbz + qax * qby - qay * qbx;
        let w = qaw * qbw - qax * qbx - qay * qby - qaz * qbz;
        Quaternion { x, y, z, w }
    }
}
