use crate::entity::Entity;

pub trait Object {
    fn apply(&mut self, entity: &mut impl Entity) {
        todo!()
    }

    fn name(&self) -> String;

    fn description(&self) -> String;
}
