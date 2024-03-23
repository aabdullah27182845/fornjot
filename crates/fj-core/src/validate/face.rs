use crate::{
    geometry::Geometry,
    topology::Face,
    validation::{
        checks::{FaceHasNoBoundary, InteriorCycleHasInvalidWinding},
        ValidationCheck, ValidationConfig, ValidationError,
    },
};

use super::Validate;

impl Validate for Face {
    fn validate(
        &self,
        config: &ValidationConfig,
        errors: &mut Vec<ValidationError>,
        geometry: &Geometry,
    ) {
        errors.extend(
            FaceHasNoBoundary::check(self, geometry, config).map(Into::into),
        );
        errors.extend(
            InteriorCycleHasInvalidWinding::check(self, geometry, config)
                .map(Into::into),
        );
    }
}
