mod model;

use model::{World, Room};
use std::fs;
use std::io::{self, Write};
use std::process::Command;

struct GameState {
    current_room_id: String,
    inventory: Vec<String>,
    world: World,
}

impl GameState {
    fn new(world: World) -> Self {
        Self {
            current_room_id: world.starting_room.clone(),
            inventory: Vec::new(),
            world,
        }
    }

    fn get_current_room(&self) -> &Room {
        self.world.get_room(&self.current_room_id).expect("Current room not found!")
    }

    fn get_current_room_mut(&mut self) -> &mut Room {
        self.world.rooms.iter_mut().find(|r| r.id == self.current_room_id).expect("Current room not found!")
    }

    fn print_status(&self) {
        let room = self.get_current_room();
        println!("\n=== {} ===", room.name);
        println!("{}", room.description);

        if !room.items.is_empty() {
            println!("\nYou see:");
            for item_id in &room.items {
                if let Some(item) = self.world.get_item(item_id) {
                    println!(" - {}", item.name);
                }
            }
        }
        
        println!("\nExits: {}", room.exits.keys().cloned().collect::<Vec<String>>().join(", "));
    }
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd").args(&["/C", "cls"]).status().unwrap();
    } else {
        Command::new("clear").status().unwrap();
    }
}

fn main() {
    clear_screen();
    println!("Welcome to Text Adventure!");
    println!("Loading world...");

    let data = fs::read_to_string("data/world.json").expect("Unable to read data/world.json");
    let world: World = serde_json::from_str(&data).expect("Unable to parse JSON");

    let mut state = GameState::new(world);

    println!("World loaded. Type 'help' for commands.");

    loop {
        state.print_status();
        print!("\n> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        let parts: Vec<&str> = input.split_whitespace().collect();

        if parts.is_empty() {
            continue;
        }

        let command = parts[0].to_lowercase();
        let argument = if parts.len() > 1 { parts[1..].join(" ") } else { String::new() };

        match command.as_str() {
            "n" | "north" => move_player(&mut state, "n"),
            "s" | "south" => move_player(&mut state, "s"),
            "e" | "east" => move_player(&mut state, "e"),
            "w" | "west" => move_player(&mut state, "w"),
            "i" | "inventory" => show_inventory(&state),
            "l" | "look" => clear_screen(), // Just loops back to print_status
            "take" | "grab" => take_item(&mut state, &argument),
            "use" => use_item(&mut state, &argument),
            "q" | "quit" => {
                println!("Goodbye!");
                break;
            }
            "help" => print_help(),
            _ => println!("I don't understand that command."),
        }
    }
}

fn move_player(state: &mut GameState, dir: &str) {
    let next_room_id_opt = state.get_current_room().exits.get(dir).cloned();

    if let Some(next_room_id) = next_room_id_opt {
        // Check if locked
        let next_room_locked = {
            let next_room = state.world.get_room(&next_room_id).expect("Invalid room link");
            next_room.locked
        };

        if next_room_locked {
            println!("The door is locked.");
        } else {
            state.current_room_id = next_room_id;
            clear_screen();
        }
    } else {
        println!("You can't go that way.");
    }
}

fn show_inventory(state: &GameState) {
    if state.inventory.is_empty() {
        println!("You are not carrying anything.");
    } else {
        println!("You are carrying:");
        for item_id in &state.inventory {
            if let Some(item) = state.world.get_item(item_id) {
                println!(" - {}", item.name);
            }
        }
    }
}

fn take_item(state: &mut GameState, item_name_query: &str) {
    if item_name_query.is_empty() {
        println!("Take what?");
        return;
    }

    let mut found_index = None;
    let mut found_item_name = String::new();

    // 1. Find the item immutably
    {
        let room = state.get_current_room();
        for (idx, item_id) in room.items.iter().enumerate() {
            if let Some(item_def) = state.world.items.iter().find(|i| i.id == *item_id) {
                if item_def.name.to_lowercase().contains(&item_name_query.to_lowercase()) {
                    found_index = Some(idx);
                    found_item_name = item_def.name.clone();
                    break;
                }
            }
        }
    }

    // 2. Mutate if found
    if let Some(idx) = found_index {
        let room = state.get_current_room_mut();
        let item_id = room.items.remove(idx);
        println!("You picked up the {}.", found_item_name);
        state.inventory.push(item_id);
    } else {
        println!("I don't see that here.");
    }
}

fn use_item(state: &mut GameState, item_name_query: &str) {
     if item_name_query.is_empty() {
        println!("Use what?");
        return;
    }

    // Check if player has the item
    let item_id_opt = state.inventory.iter().find(|id| {
         if let Some(item_def) = state.world.get_item(id) {
            item_def.name.to_lowercase().contains(&item_name_query.to_lowercase())
        } else {
            false
        }
    }).cloned();

    if let Some(item_id) = item_id_opt {
        // Check if this item unlocks any adjacent room
        let current_exits = state.get_current_room().exits.clone();
        let mut unlocked = false;

        for (_, target_room_id) in current_exits {
             // We need to find the room in the mutable world list
             if let Some(target_room) = state.world.rooms.iter_mut().find(|r| r.id == target_room_id) {
                 if target_room.locked && target_room.key_id.as_deref() == Some(&item_id) {
                     target_room.locked = false;
                     println!("You unlocked the door to the {}!", target_room.name);
                     unlocked = true;
                 }
             }
        }
        
        if !unlocked {
             println!("You can't use that here.");
        }

    } else {
        println!("You don't have that.");
    }
}

fn print_help() {
    println!("Commands:");
    println!("  n, s, e, w - Move");
    println!("  take <item> - Pick up an item");
    println!("  use <item> - Use an item (e.g., to unlock doors)");
    println!("  i / inventory - Show carried items");
    println!("  l / look - Refresh screen");
    println!("  q / quit - Quit game");
}