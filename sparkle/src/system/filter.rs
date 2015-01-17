//! Filtering entities by groups and component types.
//!
//! ## Entity filters
//!
//! An `EntityFilter` permits you to designate certain entities.  
//! Usually, you won't create filters by yourself, using the `sparkle_filter!` macro instead:
//!
//! ```ignore
//! let filter = sparkle_filter!(
//!     require components: MandatoryComponentType
//!     forbid groups: "forbidden_group", "another_forbidden_group"
//! );
//! ```
//!
//! ## Entity views
//!
//! An `EntityView` is useful to keep an eye on particular entities.  
//! You'll need to specify an `EntityFilter` to be used to identify those entities.  
//! Typically, you can use it in a `System` like this:
//!
//! ````ignore
//! struct RenderSystem {
//!     view: StandardEntityView
//! }
//!
//! impl RenderSystem {
//!     fn new() -> RenderSystem {
//!         let filter = sparkle_filter!(require components: Drawable);
//!         RenderSystem {
//!             view: EntityView::new(filter)
//!         }
//!     }
//! }
//!
//! impl System for RenderSystem {
//!     fn update(...) {
//!         // ...
//!         for entity in self.view.iter() {
//!             let drawable = drawable_store.get(entity).unwrap();
//!             self.draw(drawable);
//!         }
//!     }
//!
//!     fn on_entity_changed(&mut self, mentity: &MetaEntity) {
//!         self.view.update(mentity);
//!     }
//!
//!     fn on_entity_removed(&mut self, mentity: &MetaEntity) {
//!         self.view.remove(mentity);
//!     }
//! }
//! ````

use std::ops::{Deref};
use std::collections::{HashSet, BitvSet};

use entity::{Entity, MetaEntity};
use component::{Component, ComponentIndex};

/// The standard `EntityView`, just an alias.
pub type StandardEntityView = EntityView<StandardEntityFilter>;

/// A specific view over entities.
pub struct EntityView<Filter>
    where Filter: EntityFilter
{
    entities: HashSet<Entity>,
    filter: Filter
}

impl<Filter> EntityView<Filter>
    where Filter: EntityFilter
{
    /// Creates a new `EntityView` with the given `EntityFilter`.
    pub fn new(filter: Filter) -> EntityView<Filter> {
        EntityView {
            entities: HashSet::new(),
            filter: filter
        }
    }
    
    /// Updates the view with the given entity.
    pub fn update(&mut self, mentity: &MetaEntity) {
        let contains = self.entities.contains(&mentity.entity);
        if contains && !mentity.is_awake {
            self.entities.remove(&mentity.entity);
            return;
        }
        
        let pass_filter = self.filter.pass(mentity);

        match (contains, pass_filter) {
            (true, false) => { self.entities.remove(&mentity.entity); },
            (false, true) => { self.entities.insert(mentity.entity); },
            _ => {}
        }
    }

    /// Removes the given entity from the view.
    pub fn remove(&mut self, mentity: &MetaEntity) {
        self.entities.remove(&mentity.entity);
    }
}

impl<Filter> Deref for EntityView<Filter>
    where Filter: EntityFilter
{
    type Target = HashSet<Entity>;
    fn deref(&self) -> &HashSet<Entity> {
        &self.entities
    }
}

/// Filters entities to keep the ones you are interested in.
pub trait EntityFilter {
    /// Determines if an entity passes the filter.
    fn pass(&self, mentity: &MetaEntity) -> bool;
}

/// The provided implementation of an `EntityFilter`.
pub struct StandardEntityFilter {
    mandatory_components: BitvSet,
    forbidden_components: BitvSet,
    mandatory_groups: HashSet<String>,
    forbidden_groups: HashSet<String>
}

impl StandardEntityFilter {
    /// Creates a new `StandardEntityFilter`, letting pass every entity.
    pub fn new() -> StandardEntityFilter {
        StandardEntityFilter {
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
}

impl EntityFilter for StandardEntityFilter {
    fn pass(&self, mentity: &MetaEntity) -> bool {
        self.mandatory_components.is_subset(&mentity.components) &&
        self.forbidden_components.is_disjoint(&mentity.components) &&
        self.mandatory_groups.is_subset(&mentity.groups) &&
        self.forbidden_groups.is_disjoint(&mentity.groups)
    }
}
