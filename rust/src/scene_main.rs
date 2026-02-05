use std::f64::consts::PI;

use godot::{
    classes::{AudioStreamPlayer, Marker2D, PathFollow2D, Timer},
    prelude::*,
    task::spawn,
};
use rand::Rng;

use crate::{hud::Hud, mob::Mob, player::Player};

#[derive(GodotClass)]
#[class(base=Node)]
struct Main {
    #[export]
    mob_scene: OnEditor<Gd<PackedScene>>,
    score: i64,
    base: Base<Node>,
}
#[godot_api]
impl INode for Main {
    fn init(base: Base<Node>) -> Self {
        Self {
            mob_scene: OnEditor::default(),
            score: 0,
            base,
        }
    }

    fn ready(&mut self) {
        self.base()
            .get_node_as::<Player>("Player")
            .signals()
            .hit()
            .connect_other(self, Self::game_over);

        self.base()
            .get_node_as::<Timer>("MobTimer")
            .signals()
            .timeout()
            .connect_other(self, Self::on_mob_timer_timeout);

        self.base()
            .get_node_as::<Timer>("ScoreTimer")
            .signals()
            .timeout()
            .connect_other(self, Self::on_score_timer_timeout);

        self.base()
            .get_node_as::<Timer>("StartTimer")
            .signals()
            .timeout()
            .connect_other(self, Self::on_start_timer_timeout);

        self.base()
            .get_node_as::<Hud>("Hud")
            .signals()
            .start_game()
            .connect_other(self, Self::new_game);
    }
}
#[godot_api]
impl Main {
    fn game_over(&mut self) {
        self.base().get_node_as::<Timer>("ScoreTimer").stop();
        self.base().get_node_as::<Timer>("MobTimer").stop();

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        spawn(async move {
            hud.bind_mut().show_game_over().await;
        });

        self.base().get_node_as::<AudioStreamPlayer>("Music").stop();
        self.base()
            .get_node_as::<AudioStreamPlayer>("DeathSound")
            .play();
    }

    fn new_game(&mut self) {
        self.score = 0;

        let position = self
            .base()
            .get_node_as::<Marker2D>("StartPosition")
            .get_position();

        let mut mob = self.base().get_node_as::<Player>("Player");
        {
            let mut player = mob.bind_mut();
            player.start(position);
        }

        self.base().get_node_as::<Timer>("StartTimer").start();

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        {
            let mut hud = hud.bind_mut();
            hud.update_score(self.score);
            hud.show_message("Get Ready");
        }

        self.base()
            .get_tree()
            .unwrap()
            .call_group("mobs", "queue_free", &[]);

        self.base().get_node_as::<AudioStreamPlayer>("Music").play();
    }

    fn on_score_timer_timeout(&mut self) {
        self.score += 1;

        let mut hud = self.base().get_node_as::<Hud>("Hud");
        {
            let hud = hud.bind_mut();
            hud.update_score(self.score);
        }
    }

    fn on_start_timer_timeout(&mut self) {
        self.base().get_node_as::<Timer>("MobTimer").start();
        self.base().get_node_as::<Timer>("ScoreTimer").start();
    }

    fn on_mob_timer_timeout(&mut self) {
        let mut rng = rand::rng();

        let mut mob = self.mob_scene.instantiate_as::<Mob>();

        let mut mob_spawn_location = self
            .base()
            .get_node_as::<PathFollow2D>("MobPath/MobSpawnLocation");
        mob_spawn_location.set_progress_ratio(rng.random_range(0.0..=1.0));

        mob.set_position(mob_spawn_location.get_position());

        let mut direction = mob_spawn_location.get_rotation() + PI as f32 / 2.0;
        direction += rng.random_range((-PI / 4.0)..=(PI / 4.0)) as f32;

        mob.set_rotation(direction);

        let velocity = Vector2::new(rng.random_range(150.0..=250.0), 0.0);
        mob.set_linear_velocity(velocity.rotated(direction));

        self.base_mut().add_child(&mob);
    }
}
