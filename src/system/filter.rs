use std::collections::BitvSet;
use component::{Component, ComponentIndex};
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

    pub fn insert_mandatory<T>(&mut self) 
        where T: Component + ComponentIndex 
    {
        let index = ComponentIndex::of(None::<T>);
        self.mandatory_components.insert(index);
    }

    pub fn check(&self, mentity: &MetaEntity) -> bool {
        self.mandatory_components.is_subset(&mentity.component_bits)
    }
}

#[macro_export]
macro_rules! filter(
    ($($component_type:ident),*) => ({
        let mut filter = sparkle::system::Filter::new();
        $(
            filter.insert_mandatory::<$component_type>();
        )*

        filter
    })
);