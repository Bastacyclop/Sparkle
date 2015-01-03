use entity::Manager as EntityManager;

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