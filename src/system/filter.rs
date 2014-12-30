use std::collections::BitvSet;
use component::{Component, ComponentType};
use entity::MetaEntity;

pub struct Filter {
    mandatory_components: BitvSet
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            mandatory_components: BitvSet::new()
        }
    }

    pub fn insert_mandatory<T>(&mut self) where T: Component {
        let index = ComponentType::get_index_of::<T>();
        self.mandatory_components.insert(index);
    }

    pub fn check(&self, mentity: &MetaEntity) -> bool {
        self.mandatory_components.is_subset(&mentity.component_bits)
    }
}

macro_rules! filter(
    ($($component_type:ident),*) => ({
        let mut filter = Filter::new();
        $(
            filter.insert_mandatory::<$component_type>();
        )*

        filter
    })
);