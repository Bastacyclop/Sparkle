//! Filtering entity by groups and components types.
//!
//! A Filter is a great tool to select entities for systems.
//! Generally you wont create this structure by yourself and
//! use the sparkle_filter! macros
//!
//! ```ignore
//! // This will create a filter that let pass entities with 
//! // AComponentType attached to them and that are not in the "Forbiden" group.
//! 
//! let filter = sparkle_filter!(
//!     require components: AComponentType,
//!     forbid groups: "Forbiden"
//! );
//! ```
use std::collections::{HashSet, BitvSet};
use component::{Component, ComponentIndex};
use entity::MetaEntity;

//! A Filter is type that help systems to know in which
//! entity they might be interested.
pub struct Filter {
    mandatory_components: BitvSet,
    forbidden_components: BitvSet,
    mandatory_groups: HashSet<String>,
    forbidden_groups: HashSet<String>
}

impl Filter {
    /// Create a new empty `Filter`
    pub fn new() -> Filter {
        Filter {
            mandatory_components: BitvSet::new(),
            forbidden_components: BitvSet::new(),
            mandatory_groups: HashSet::new(),
            forbidden_groups: HashSet::new(),
        }
    }

    /// Add a component type requirement.
    pub fn require_component<T>(&mut self)
        where T: Component + ComponentIndex
    {
        let index = ComponentIndex::of(None::<T>);
        self.mandatory_components.insert(index);
    }

    /// Add a component type that will exlude any entity
    /// that does have it.
    pub fn forbid_component<T>(&mut self)
        where T: Component + ComponentIndex
    {
        let index = ComponentIndex::of(None::<T>);
        self.forbidden_components.insert(index);
    }

    /// Add a group requirement.
    pub fn require_group(&mut self, group: &str) {
        self.mandatory_groups.insert(group.to_string());
    }

    /// Add group that will exlude any entity
    /// that does have it.
    pub fn forbid_group(&mut self, group: &str) {
        self.forbidden_groups.insert(group.to_string());
    }

    /// Determine if an entity pass the filter or not.
    pub fn pass(&self, mentity: &MetaEntity) -> bool {
        self.mandatory_components.is_subset(&mentity.components) &&
        self.forbidden_components.is_disjoint(&mentity.components) &&
        self.mandatory_groups.is_subset(&mentity.groups) &&
        self.forbidden_groups.is_disjoint(&mentity.groups)
    }
}
