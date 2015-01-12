use space::Space;
use entity::Entity;
use command::{Command};

pub struct CreateEntity<F>(pub F)
    where F: for<'a> FnMut<(&'a mut Space, Entity), ()> + 'static;

impl<F> Command<Space> for CreateEntity<F>
    where F: for<'a> FnMut<(&'a mut Space, Entity), ()> + 'static
{
    fn run(&mut self, space: &mut Space) {
        let entity = space.em.view_mut().create_entity();
        self.0.call_mut((space, entity));
    }
}

#[derive(Copy)]
pub struct RemoveEntity(pub Entity);
impl Command<Space> for RemoveEntity {
    fn run(&mut self, space: &mut Space) {
        space.em.view_mut().remove_entity(self.0);
    }
}
