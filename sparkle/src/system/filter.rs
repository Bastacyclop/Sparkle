//! Filtering entities by groups and component types.
//!
//! A `Filter` is a great tool to select entities for systems.
//! Generally, you won't create this structure by yourself and
//! use the `sparkle_filter!`` macro instead:
//!
//! ```ignore
//! // This will create a filter that lets pass entities with a RequiredComponentType
//! // attached to them and that are not in the "Forbidden" group.
//! 
//! let filter = sparkle_filter!(
//!     require components: RequiredComponentType,
//!     forbid groups: "Forbidden"
//! );
//! ```

use std::collections::{HashSet, BitvSet};
use component::{Component, ComponentIndex};
use entity::MetaEntity;

/// Filters entities to keep the ones you are interested in.
pub struct Filter {
    mandatory_components: BitvSet,
    forbidden_components: BitvSet,
    mandatory_groups: HashSet<String>,
    forbidden_groups: HashSet<String>
}

impl Filter {
    /// Creates a new `Filter`, letting pass every entity.
    pub fn new() -> Filter {
        Filter {
            mandatory_components: BitvSet::new(),
            forbidden_components: BitvSet::new(),
            mandatory_groups: HashSet::new(),
            forbidden_groups: HashSet::new(),
        }
    }

    /// Adds a mandatory component type.
    pub fn require_component<T>(&mut self)
        where T: Component + ComponentIndex
    {
        let index = ComponentIndex::of(None::<T>);
        self.mandatory_components.insert(index);
    }

    /// Adds a forbidden component type.
    pub fn forbid_component<T>(&mut self)
        where T: Component + ComponentIndex
    {
        let index = ComponentIndex::of(None::<T>);
        self.forbidden_components.insert(index);
    }

    /// Adds a mandatory group.
    pub fn require_group(&mut self, group: &str) {
        self.mandatory_groups.insert(group.to_string());
    }

    /// Adds a forbidden group.
    pub fn forbid_group(&mut self, group: &str) {
        self.forbidden_groups.insert(group.to_string());
    }

    /// Determines if an entity passes the filter.
    pub fn pass(&self, mentity: &MetaEntity) -> bool {
        self.mandatory_components.is_subset(&mentity.components) &&
        self.forbidden_components.is_disjoint(&mentity.components) &&
        self.mandatory_groups.is_subset(&mentity.groups) &&
        self.forbidden_groups.is_disjoint(&mentity.groups)
    }
}
