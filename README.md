# Rust Raycasting Game

## 🎮 About

The readme is made by gemini, it is trash... but my game is based on cursed nintendo ricet and reggie kills you if yopu dont steal the wiis at time :)

here's the gameplay: https://youtu.be/Fa36qZ34N9o
## ✨ Features

*   **Raycasting Engine:** Pseudo-3D rendering of a maze environment.
*   **Maze Generation:** Dynamic maze structures (e.g., `maze.txt`, `maze_hard.txt`).
*   **Player Mechanics:** Movement, rotation, and interaction within the maze.
*   **Enemy System:** Basic enemy AI and interaction.
*   **Audio Integration:** Sound effects and background music for an enhanced experience.
*   **Texture Mapping:** Walls and sprites with various textures.

## 🚀 Installation & Usage

To build and run this project, you need to have Rust and Cargo installed.

1.  **Clone the repository:**
    ```bash
    git clone https://github.com/your-username/raycasting_graficas.git
    cd raycasting_graficas/project
    ```
2.  **Run the game:**
    ```bash
    cargo run
    ```

## 🕹️ Controls

*(Add specific game controls here, e.g., W, A, S, D for movement, mouse for looking, etc.)*

## 📁 Project Structure

*   `src/main.rs`: Main entry point of the application.
*   `src/audio.rs`: Handles sound effects and background music.
*   `src/caster.rs`: Implements the raycasting logic for rendering the 3D view.
*   `src/collectable.rs`: Defines collectable items within the game.
*   `src/enemy.rs`: Manages enemy behavior and rendering.
*   `src/framebuffer.rs`: Handles pixel manipulation and rendering to the screen.
*   `src/maze.rs`: Logic for loading and managing the maze structure.
*   `src/player.rs`: Manages player state, movement, and interactions.
*   `src/textures.rs`: Handles loading and applying textures to game elements.

## 📜 License

This project is licensed under the MIT License. See the `LICENSE` file for details.
