use entity::MetaEntity;

pub struct Pool {
    available: Vec<MetaEntity>,
    next_id: usize
}

impl Pool {
    pub fn new() -> Pool {
        Pool {
            available: Vec::new(),
            next_id: 0
        }
    }

    pub fn get(&mut self) -> MetaEntity {
        self.available.pop().unwrap_or_else(|| {
            let id = self.next_id;
            self.next_id += 1;

            MetaEntity::new(id)
        }).reset()
    }

    pub fn put(&mut self, entity: MetaEntity) {
        self.available.push(entity);
    }
}