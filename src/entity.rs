use crate::permissions;

struct Entity {
    id: String,
    username: String,
    password: String,
    internal_permissions: Vec<permissions::Internal>,
    permissions: Vec<String>,
}

trait Persistence {
    fn get_entity_by_id(id: String) -> Option<Entity>;
    fn set_entity(&self) -> Option<String>;
    fn lookup_entity_by_id(id: String) -> Option<Entity>;
}

impl Persistence for Entity {
    fn get_entity_by_id(id: String) -> Option<Entity> {
        todo!()
    }

    fn set_entity(&self) -> Option<String> {
        todo!()
    }

    fn lookup_entity_by_id(id: String) -> Option<Entity> {
        todo!()
    }
}