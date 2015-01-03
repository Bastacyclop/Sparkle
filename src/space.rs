use entity::Manager as EntityManager;
use group::Manager as GroupManager;
use tag::Manager as TagManager;

pub struct Space {
    entities: EntityManager
}

impl Space {
    pub fn new() -> Space {
        Space {
            entities: EntityManager::new()
        }
    }
}