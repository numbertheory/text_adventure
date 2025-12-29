# Text Adventure

A simple, extensible text adventure engine written in Rust.

## Description

This project is a command-line text adventure game where players explore a world defined in a JSON file. The engine handles room navigation, item interaction (taking and using), and locked door mechanics. The default scenario places you in a Dark Forest, searching for a way into a locked Treasure Room.

## Features

-   **Data-Driven World:** The entire game world (rooms, items, connections) is loaded from `data/world.json`, making it easy to create new adventures without recompiling code.
-   **Inventory System:** Pick up items and manage your inventory.
-   **Puzzle Mechanics:** Use items (like keys) to unlock paths to new areas.
-   **Cross-Platform:** Works on Windows and Unix-like systems (Linux, macOS).

## Getting Started

### Prerequisites

You need to have [Rust](https://www.rust-lang.org/tools/install) installed on your machine.

### Running the Game

1.  Clone this repository or download the source code.
2.  Navigate to the project directory in your terminal.
3.  Run the game using Cargo:

    ```bash
    cargo run
    ```

## How to Play

Once the game starts, you will see a description of your current location. Type commands to interact with the world.

### Commands

-   **Movement:**
    -   `n` or `north`: Move North
    -   `s` or `south`: Move South
    -   `e` or `east`: Move East
    -   `w` or `west`: Move West
-   **Interaction:**
    -   `take <item>` or `grab <item>`: Pick up an item from the current room.
    -   `use <item>`: Use an item from your inventory (e.g., to unlock a door).
-   **Game Info:**
    -   `i` or `inventory`: List items you are currently carrying.
    -   `l` or `look`: Re-print the current room description.
    -   `help`: Show the list of available commands.
    -   `q` or `quit`: Exit the game.

## Customizing the World

You can modify `data/world.json` to create your own adventures. The format requires:

-   `starting_room`: ID of the room where the player starts.
-   `rooms`: A list of room objects containing:
    -   `id`: Unique identifier.
    -   `name`: Display name.
    -   `description`: Description of the room.
    -   `items`: List of item IDs initially in the room.
    -   `exits`: Map of directions (`n`, `s`, `e`, `w`) to destination room IDs.
    -   `locked`: Boolean (optional, defaults to false).
    -   `key_id`: ID of the item required to unlock this room (optional).
-   `items`: A list of item objects containing `id`, `name`, and `description`.
