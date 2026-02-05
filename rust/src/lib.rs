mod ext;
mod hud;
mod mob;
mod player;
mod scene_main;

use godot::prelude::*;

struct DodgeTheCreeps2d;

#[gdextension]
unsafe impl ExtensionLibrary for DodgeTheCreeps2d {}
