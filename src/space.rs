use entity::Manager as EntityManager;
use system::Manager as SystemManager;

pub struct Space {
    pub entities: EntityManager,
    pub systems: SystemManager
}

impl Space {
    pub fn new() -> Space {
        Space {
            entities: EntityManager::new(),
            systems: SystemManager::new()
        }
    }

    pub fn update(&mut self) {
        self.entities.notify_observer_and_flush(&mut self.systems);
        self.systems.process_systems(&mut self.entities);
    }
}