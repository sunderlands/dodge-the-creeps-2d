use godot::{
    classes::{AnimatedSprite2D, IRigidBody2D, RigidBody2D, VisibleOnScreenNotifier2D},
    prelude::*,
};

use crate::ext::PickRandom;

#[derive(GodotClass)]
#[class(base=RigidBody2D)]
pub struct Mob {
    base: Base<RigidBody2D>,
}
#[godot_api]
impl IRigidBody2D for Mob {
    fn init(base: Base<RigidBody2D>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        let mut sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        let random_name = sprite
            .get_sprite_frames()
            .unwrap()
            .get_animation_names()
            .pick_random();

        sprite.set_animation(random_name.arg());
        sprite.play();

        self.base()
            .get_node_as::<VisibleOnScreenNotifier2D>("VisibleOnScreenNotifier2D")
            .signals()
            .screen_exited()
            .connect_other(&*self, Self::on_screen_exited);
    }
}
#[godot_api]
impl Mob {
    fn on_screen_exited(&mut self) {
        self.base_mut().queue_free();
    }
}
