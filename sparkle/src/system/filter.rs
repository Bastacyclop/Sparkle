use std::collections::{HashSet, BitvSet};
use component::{Component, ComponentIndex};
use entity::MetaEntity;

pub struct Filter {
    mandatory_components: BitvSet,
    forbidden_components: BitvSet,
    mandatory_groups: HashSet<String>,
    forbidden_groups: HashSet<String>
}

impl Filter {
    pub fn new() -> Filter {
        Filter {
            mandatory_components: BitvSet::new(),
            forbidden_components: BitvSet::new(),
            mandatory_groups: HashSet::new(),
            forbidden_groups: HashSet::new(),
        }
    }

    pub fn require_component<T>(&mut self)
        where T: Component + ComponentIndex
    {
        let index = ComponentIndex::of(None::<T>);
        self.mandatory_components.insert(index);
    }

    pub fn forbid_component<T>(&mut self)
        where T: Component + ComponentIndex
    {
        let index = ComponentIndex::of(None::<T>);
        self.forbidden_components.insert(index);
    }

    pub fn require_group(&mut self, group: &str) {
        self.mandatory_groups.insert(group.to_string());
    }

    pub fn forbid_group(&mut self, group: &str) {
        self.forbidden_groups.insert(group.to_string());
    }

    pub fn check(&self, mentity: &MetaEntity) -> bool {
        self.mandatory_components.is_subset(&mentity.component_bits) &&
        self.forbidden_components.is_disjoint(&mentity.component_bits) &&
        self.mandatory_groups.is_subset(&mentity.groups) &&
        self.forbidden_groups.is_disjoint(&mentity.groups)
    }
}
