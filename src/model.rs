use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Room {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub items: Vec<String>, // List of item IDs
    #[serde(default)]
    pub exits: HashMap<String, String>, // direction (n, s, e, w) -> room_id
    #[serde(default)]
    pub locked: bool,
    pub key_id: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct World {
    pub starting_room: String,
    pub rooms: Vec<Room>,
    pub items: Vec<Item>,
}

impl World {
    pub fn get_room(&self, id: &str) -> Option<&Room> {
        self.rooms.iter().find(|r| r.id == id)
    }

    pub fn get_item(&self, id: &str) -> Option<&Item> {
        self.items.iter().find(|i| i.id == id)
    }
}
