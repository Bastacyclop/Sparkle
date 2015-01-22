#![feature(plugin)]

#[plugin] #[no_link] #[macro_use]
extern crate sparkle_macros;
extern crate sparkle;

use std::ops::Deref;
use std::collections::{HashMap, HashSet};

use sparkle::prelude::*;
use sparkle::command::CommandReceiver;
use sparkle::component;

#[sparkle_component]
struct InventoryItem {
    name: String,
    owner: Entity
}

#[sparkle_component]
struct InventoryOwner {
    name: String
}

struct InventoryView {
    map: HashMap<Entity, HashSet<Entity>>
}

impl InventoryView {
    fn new() -> InventoryView {
        InventoryView {
            map: HashMap::new()
        }
    }
    
    fn update(&mut self, mentity: &MetaEntity) {
        if mentity.components.contains(&component::index_of::<InventoryOwner>()) {
            self.map.insert(mentity.entity, HashSet::new());
        }
    }
    
    fn update_item(&mut self, cm: &ComponentMapper, mentity: &MetaEntity) {
        if mentity.components.contains(&component::index_of::<InventoryItem>()) {
            for (_, items) in self.map.iter_mut() {
                items.remove(&mentity.entity);
            }
            let item = cm.get::<InventoryItem>(mentity.entity);
            let inventory = self.map.get_mut(&item.owner).expect("Unvalid item owner");
            inventory.insert(mentity.entity);
        }
    }
    
    fn remove(&mut self, entity: Entity) {
        self.map.remove(&entity);
        for (_, items) in self.map.iter_mut() {
            items.remove(&entity);
        }
    }
}

impl Deref for InventoryView {
    type Target = HashMap<Entity, HashSet<Entity>>;
    fn deref(&self) -> &HashMap<Entity, HashSet<Entity>> {
        &self.map
    }
}

pub type InventoryCommand = Box<for<'a> Command<Args = (&'a mut InventoryView,
                                                        &'a mut EntityMapper,
                                                        &'a mut ComponentMapper)>>;

struct InventoryMaintainer {
    inventory_view: BlackboardEntry<InventoryView>,
    query_recvr: CommandReceiver<InventoryCommand>
}

impl InventoryMaintainer {
    fn new(blackboard: &Blackboard, query_recvr: CommandReceiver<InventoryCommand>) -> InventoryMaintainer {
        InventoryMaintainer {
            inventory_view: blackboard.get("inventory_view"),
            query_recvr: query_recvr
        }
    }
}

impl System for InventoryMaintainer {
    fn update(&mut self, em: &mut EntityMapper, cm: &mut ComponentMapper, _dt: f32) {
        let mut inventory_view = self.inventory_view.borrow_mut();
        while let Some(mut cmd) = self.query_recvr.recv() {
            cmd.run((&mut *inventory_view, em, cm));
        }
    }

    fn on_entity_changed(&mut self, _cm: &ComponentMapper, mentity: &MetaEntity) {
        self.inventory_view.borrow_mut().update(mentity);
    }

    fn on_entity_removed(&mut self, _cm: &ComponentMapper, mentity: &MetaEntity) {
        self.inventory_view.borrow_mut().remove(mentity.entity);
    }
}

struct CreateItem {
    item_e: Entity,
    item: Option<InventoryItem>
}

impl<'a> Command for CreateItem {
    type Args = (&'a mut InventoryView,
                 &'a mut EntityMapper,
                 &'a mut ComponentMapper);
    fn run(&mut self, args: (&'a mut InventoryView,
                             &'a mut EntityMapper,
                             &'a mut ComponentMapper)) {
        let (view, em, cm) = args;
        
        let mitem = em.get_mentity_mut(self.item_e);
        cm.insert(mitem, self.item.take().unwrap());
        view.update_item(&*cm, &*mitem);
    }
}

struct SetItemOwner {
    item_e: Entity,
    owner_e: Entity
}

impl<'a> Command for SetItemOwner {
    type Args = (&'a mut InventoryView,
                 &'a mut EntityMapper,
                 &'a mut ComponentMapper);
    fn run(&mut self, args: (&'a mut InventoryView,
                             &'a mut EntityMapper,
                             &'a mut ComponentMapper)) {
        let (view, em, cm) = args;
        
        {
            let mut item = cm.get_mut::<InventoryItem>(self.item_e);
            item.owner = self.owner_e;
        }
        let mitem = em.get_mentity(self.item_e);
        view.update_item(&*cm, mitem);
    }
}

struct InventoryDisplayer {
    inventory_view: BlackboardEntry<InventoryView>
}

impl InventoryDisplayer {
    fn new(blackboard: &Blackboard) -> InventoryDisplayer {
        InventoryDisplayer {
            inventory_view: blackboard.get("inventory_view")
        }
    }
}

impl System for InventoryDisplayer {
    fn update(&mut self, _em: &mut EntityMapper, cm: &mut ComponentMapper, _dt: f32) {
        println!("===== Inventories: =====");
        for (&owner, items) in self.inventory_view.borrow().iter() {
            let owner = cm.get::<InventoryOwner>(owner);
            println!("{} owns:", owner.name);
            for &item in items.iter() {
                let item = cm.get::<InventoryItem>(item);
                println!("  - {}", item.name);
            }
        }
        println!("========================");
    }
}

macro_rules! expand_inventory {
    (of $owner:ident with $($item:ident),* using $space:ident and $sender:ident) => ({
        let owner = $owner;
        $(
            let $item = $space.em.create_entity();
        )*
        $(
            {
                let name = stringify!($item).to_string();
                $sender.send(Box::new(CreateItem {
                    item_e: $item,
                    item: Some(InventoryItem { name: name, owner: owner })
                }));
            }
        )*
        ($($item),*)
    })
}

fn main() {
    let mut blackboard = Blackboard::new();
    blackboard.insert("inventory_view", InventoryView::new());
    
    let (mut space, _) = Space::new();
    let (mut inventory_cmd_sender, inventory_cmd_rcvr) = sparkle::command::stream();
    space.sm.insert(InventoryMaintainer::new(&blackboard, inventory_cmd_rcvr));
    space.sm.insert(InventoryDisplayer::new(&blackboard));
    
    let bob = space.em.create_entity();
    {
        let mbob = space.em.get_mentity_mut(bob);
        space.cm.insert(mbob, InventoryOwner { name: "bob".to_string() } );
    }
    expand_inventory!(of bob with hat, boots using space and inventory_cmd_sender);
    
    let joe = space.em.create_entity();
    {
        let mjoe = space.em.get_mentity_mut(joe);
        space.cm.insert(mjoe, InventoryOwner { name: "joe".to_string() } );
    }
    let (_, crowbar) = expand_inventory!(of joe with food, crowbar using space and inventory_cmd_sender);
    
    space.update(0.);
    
    // bob steals joe's crowbar
    inventory_cmd_sender.send(Box::new(SetItemOwner { item_e: crowbar, owner_e: bob }));
    
    space.update(0.);
}
