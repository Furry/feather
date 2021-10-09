use std::collections::HashMap;

use base::BlockKind;

use crate::TickLoop;

pub struct TileEntityRegistry {
    registry: HashMap<i32, TileEntity>
}

pub struct TileEntity {
    id: i32,
    block: BlockKind,
    on_tick: TickLoop
}

impl TileEntityRegistry {
    pub fn new() -> Self {
        TileEntityRegistry {
            registry: HashMap::new()
        }
    }

    pub fn register(&mut self, id: i32, tile_entity: TileEntity) {
        self.registry.insert(id, tile_entity);
    }

    pub fn remove(&mut self, id: i32) {
        self.registry.remove_entry(&id);
    }

    pub fn get(&mut self, id: i32) -> Option<&TileEntity> {
        self.registry.get(&id)
    }
}

pub mod furnace;