use sled::Db;

use crate::model::RoomEntity;

pub struct PoolWrapper {
    pub pool: Db,
}

impl PoolWrapper {
    pub fn add_or_update_room(
        &self,
        derivation_path: &str,
        room: &RoomEntity,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let rooms_tree = self.pool.open_tree("rooms-".to_owned() + derivation_path)?;
        let room_key = room.id.as_bytes();
        let room_value = bincode::serialize(&room)?;
        rooms_tree.insert(room_key, room_value)?;
        Ok(())
    }

    pub fn get_all_rooms(
        &self,
        derivation_path: &str,
    ) -> Result<Vec<RoomEntity>, Box<dyn std::error::Error>> {
        let rooms_tree = self.pool.open_tree("rooms-".to_owned() + derivation_path)?;
        let mut rooms = Vec::new();

        for result in rooms_tree.iter() {
            let (_, value) = result?;
            let room: RoomEntity = bincode::deserialize(&value)?;
            rooms.push(room);
        }

        Ok(rooms)
    }
    pub fn get_room(
        &self,
        derivation_path: &str,
        room_id: &str,
    ) -> Result<RoomEntity, Box<dyn std::error::Error>> {
        let rooms_tree = self.pool.open_tree("rooms-".to_owned() + derivation_path)?;
        if let Ok(Some(room)) = rooms_tree.get(room_id) {
            let room: RoomEntity = bincode::deserialize(&room)?;
            Ok(room)
        } else {
            Err("erro".into())
        }
    }
}
