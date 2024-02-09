use crate::{
    objects::{Region, Sketch},
    operations::insert::Insert,
    storage::Handle,
    Instance,
};

/// Update a [`Sketch`]
pub trait UpdateSketch {
    /// Add a region to the sketch
    #[must_use]
    fn add_regions(
        &self,
        regions: impl IntoIterator<Item = Handle<Region>>,
    ) -> Self;

    /// Update a region of the sketch
    ///
    /// # Panics
    ///
    /// Panics, if the object can't be found.
    ///
    /// Panics, if the update results in a duplicate object.
    #[must_use]
    fn update_region<T, const N: usize>(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>, &mut Instance) -> [T; N],
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>;
}

impl UpdateSketch for Sketch {
    fn add_regions(
        &self,
        regions: impl IntoIterator<Item = Handle<Region>>,
    ) -> Self {
        Sketch::new(self.regions().iter().cloned().chain(regions))
    }

    fn update_region<T, const N: usize>(
        &self,
        handle: &Handle<Region>,
        update: impl FnOnce(&Handle<Region>, &mut Instance) -> [T; N],
        core: &mut Instance,
    ) -> Self
    where
        T: Insert<Inserted = Handle<Region>>,
    {
        let regions = self
            .regions()
            .replace(
                handle,
                update(handle, core)
                    .map(|object| object.insert(&mut core.services)),
            )
            .expect("Region not found");
        Sketch::new(regions)
    }
}
