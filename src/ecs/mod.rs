use std::any::Any;
use std::collections::HashMap;

type GID = usize;

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum EntityKind {
    Player,
}

#[derive(Debug, Hash, Eq, PartialEq, Clone, Copy)]
enum ComponentKind {
    Location,
}

struct EntityTemplate {
    kind: EntityKind,
    components: Vec<ComponentKind>,
}

struct ComponentTemplate<Data> {
    kind: ComponentKind,
    initial_data: Data,
}

trait System {}

struct Manager {
    id: usize,
    entity_templates: HashMap<EntityKind, EntityTemplate>,
    component_templates: HashMap<ComponentKind, ComponentTemplate<Box<dyn Any>>>,
}

impl Manager {
    pub fn new() -> Manager {
        Manager {
            id: 0 as usize,
            entity_templates: HashMap::new(),
            component_templates: HashMap::new(),
        }
    }

    fn define_component(&mut self, kind: ComponentKind, data: Box<dyn Any>) {
        self.component_templates.insert(
            kind,
            ComponentTemplate {
                kind,
                initial_data: data,
            },
        );
    }
}

fn main() {
    let mut manager = Manager::new();
    manager.define_component(ComponentKind::Location, Box::new((0, 0, 0)));
}
