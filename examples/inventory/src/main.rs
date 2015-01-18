#![feature(plugin)]

#[plugin] #[no_link] #[macro_use]
extern crate sparkle_macros;
extern crate sparkle;

use std::collections::{HashMap, HashSet};

use sparkle::prelude::*;
use sparkle::component::ComponentIndex;

#[sparkle_component]
struct InventoryItem {
    name: String,
    owner: Entity
}

#[sparkle_component]
struct InventoryOwner {
    name: String
}

struct InventoryDisplayer {
    inventory_view: HashMap<Entity, HashSet<Entity>>
}

impl InventoryDisplayer {
    fn new() -> InventoryDisplayer {
        InventoryDisplayer {
            inventory_view: HashMap::new()
        }
    }
}

impl System for InventoryDisplayer {
    fn update(&mut self, _em: &mut EntityMapper, cm: &mut ComponentMapper, _dt: f32) {
        println!("===== Inventories: =====");
        for (&owner, items) in self.inventory_view.iter() {
            let owner = cm.get::<InventoryOwner>(owner);
            println!("{} owns:", owner.name);
            for &item in items.iter() {
                let item = cm.get::<InventoryItem>(item);
                println!("  - {}", item.name);
            }
        }
        println!("========================");
    }

    fn on_entity_changed(&mut self, cm: &ComponentMapper, mentity: &MetaEntity) {
        if mentity.components.contains(&ComponentIndex::of(None::<InventoryOwner>)) {
            self.inventory_view.insert(mentity.entity, HashSet::new());
        } else if mentity.components.contains(&ComponentIndex::of(None::<InventoryItem>)) {
            for (_, items) in self.inventory_view.iter_mut() {
                items.remove(&mentity.entity);
            }
            let item = cm.get::<InventoryItem>(mentity.entity);
            self.inventory_view.get_mut(&item.owner).map(|set| set.insert(mentity.entity));
        }
    }

    fn on_entity_removed(&mut self, _cm: &ComponentMapper, mentity: &MetaEntity) {
        self.inventory_view.remove(&mentity.entity);
        for (_, items) in self.inventory_view.iter_mut() {
            items.remove(&mentity.entity);
        }
    }
}

macro_rules! expand_inventory {
    (of $owner:ident with $($item:ident),* within $space:ident) => ({
        let owner = $owner;
        $(
            let $item = $space.em.create_entity();    
        )*
        $(
            {                
                let mitem = $space.em.get_mentity_mut($item);
                let name = stringify!($item).to_string();
                $space.cm.insert(mitem, InventoryItem { name: name, owner: owner });
            }
        )*
        ($($item),*)
    })
}

fn main() {
    let mut space = Space::new();
    space.sm.insert(|_| InventoryDisplayer::new());
    
    let bob = space.em.create_entity();
    {
        let mbob = space.em.get_mentity_mut(bob);
        space.cm.insert(mbob, InventoryOwner { name: "bob".to_string() } );
    }
    let (_hat, _boots) = expand_inventory!(of bob with hat, boots within space);
    
    let joe = space.em.create_entity();
    {
        let mjoe = space.em.get_mentity_mut(joe);
        space.cm.insert(mjoe, InventoryOwner { name: "joe".to_string() } );
    }
    let (_food, crowbar) = expand_inventory!(of joe with food, crowbar within space);
    
    space.update(0.);
    
    // bob steals joe's crowbar
    {
        let mut crowbar_item = space.cm.get_mut::<InventoryItem>(crowbar);
        // FIXME: necessary to pop an entity changed event for now
        let _ = space.em.get_mentity_mut(crowbar);
        crowbar_item.owner = bob;
    }
    
    space.update(0.);
}
