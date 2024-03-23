//! Solid approximation

use std::collections::BTreeSet;

use crate::{topology::Solid, Core};

use super::{edge::HalfEdgeApproxCache, face::FaceApprox, Approx, Tolerance};

impl Approx for &Solid {
    type Approximation = BTreeSet<FaceApprox>;
    type Cache = HalfEdgeApproxCache;

    fn approx_with_cache(
        self,
        tolerance: impl Into<Tolerance>,
        cache: &mut Self::Cache,
        core: &mut Core,
    ) -> Self::Approximation {
        let tolerance = tolerance.into();

        self.shells()
            .iter()
            .flat_map(|shell| shell.approx_with_cache(tolerance, cache, core))
            .collect()
    }
}
