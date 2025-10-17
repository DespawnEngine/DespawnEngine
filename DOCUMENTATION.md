# Despawn Engine — Script Documentation

## Table of Contents
- [app.rs](#apprs)
    - [Purpose](#purpose)
    - [Key Responsibilities](#key-responsibilities)
- [display.rs](#displayrs)
    - [Purpose](#purpose-1)
    - [Key Responsibilities](#key-responsibilities-1)
- [scene_manager.rs](#scene_managerrs)
    - [Purpose](#purpose-2)
    - [Key Responsibilities](#key-responsibilities-2)

---

# app.rs

## Purpose:
The main application handler. Instantiated by main.rs.

## Key Responsibilities:
- Creates and manages the main window.
- Initializes Vulkan instance, device, queue, swapchain, and render pass.
- Builds and manages the graphics pipeline.
- Handles window and device events (mouse, keyboard, resize, close, etc.).
- Manages per-frame updates (delta time, scene updates, camera, UI, etc.).
- Executes draw commands for scene manager and egui UI.

---

# display.rs

## Purpose:
This is meant to handle the window specifically.

## Key Responsibilities:
- Creates the main application window with title, decorations, and icon.
- Defines the render pass used by Vulkan (color and depth attachments).
- Creates the simple test cube. **(FIRST BLOCK!)**
- Loads the application icon image and converts it.

---

# scene_manager.rs

## Purpose:
Manages all active scenes and their lifecycle states.  
Handles scene switching, updates, and rendering delegation.
Massively deboilerplates the program. `GameScene` tracks game logic, `MenuScene` tracks menu logic.

## Key Responsibilities:
- Maintains a list of all available scenes (`MenuScene`, `GameScene`).
- Tracks the **current** and **next** active scenes.
- Provides lifecycle management (`awake`, `start`, `update`, `fixed_update`, `late_update`, `draw`).
- Provides access to the current scene for camera and input updates.
- Implements a singleton-like pattern with `OnceLock` for global access.
