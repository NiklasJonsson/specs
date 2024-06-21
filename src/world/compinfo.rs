use ahash::AHashMap as HashMap;

/// A description of a component
pub struct ComponentInfo {
    /// The name of the component, as per `std::any::type_name`.
    pub type_name: &'static str,
    /// The type id of the component
    pub type_id: std::any::TypeId,
}

impl ComponentInfo {
    fn new<T: 'static>() -> Self {
        Self {
            type_name: std::any::type_name::<T>(),
            type_id: std::any::TypeId::of::<T>(),
        }
    }
}

#[derive(Default)]
pub struct ComponentInfoTable {
    map: HashMap<std::any::TypeId, ComponentInfo>,
}

impl ComponentInfoTable {
    pub fn id<T>(&self) -> std::any::TypeId
    where
        T: 'static,
    {
        std::any::TypeId::of::<T>()
    }

    pub fn get<T>(&self) -> Option<&ComponentInfo>
    where
        T: 'static,
    {
        self.get_by_id(self.id::<T>())
    }

    pub fn get_by_id(&self, id: std::any::TypeId) -> Option<&ComponentInfo> {
        self.map.get(&id)
    }

    pub fn register<T>(&mut self) -> Option<ComponentInfo>
    where
        T: 'static,
    {
        let id = self.id::<T>();
        self.map.insert(id, ComponentInfo::new::<T>())
    }
}
