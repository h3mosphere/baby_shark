use nalgebra::Point3;

use crate::geometry::traits::{RealNumber, HasBBox3, HasScalarType};

use super::box3::Box3;

/// 3D sphere
pub struct Sphere3<TScalar: RealNumber> {
    center: Point3<TScalar>,
    radius: TScalar
}

impl<TScalar: RealNumber> Sphere3<TScalar> {
    pub fn new(center: Point3<TScalar>, radius: TScalar) -> Self { 
        return Self { center, radius };
    }

    #[inline]
    pub fn intersects_box3(&self, bbox: &Box3<TScalar>) -> bool {
        return bbox.squared_distance(&self.center) <= self.radius * self.radius;
    }
}

impl<TScalar: RealNumber> HasScalarType for Sphere3<TScalar> {
    type ScalarType = TScalar;
}

impl<TScalar: RealNumber> HasBBox3 for Sphere3<TScalar> {
    #[inline]
    fn bbox(&self) -> Box3<Self::ScalarType> {
        return Box3::new(
            self.center.coords.add_scalar(-self.radius).into(), 
            self.center.coords.add_scalar(self.radius).into()
        );
    }
}
