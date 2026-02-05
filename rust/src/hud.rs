use godot::{
    classes::{Button, CanvasLayer, ICanvasLayer, Label, Timer},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=CanvasLayer)]
pub struct Hud {
    base: Base<CanvasLayer>,
}
#[godot_api]
impl ICanvasLayer for Hud {
    fn init(base: Base<CanvasLayer>) -> Self {
        Self { base }
    }

    fn ready(&mut self) {
        self.base()
            .get_node_as::<Button>("StartButton")
            .signals()
            .pressed()
            .connect_other(self, Self::on_start_button_pressed);

        self.base()
            .get_node_as::<Timer>("MessageTimer")
            .signals()
            .timeout()
            .connect_other(self, Self::on_message_timer_timeout);
    }
}
#[godot_api]
impl Hud {
    #[signal]
    pub fn start_game();

    pub fn show_message(&mut self, text: &str) {
        let mut message = self.base().get_node_as::<Label>("Message");

        message.set_text(text);
        message.show();

        self.base().get_node_as::<Timer>("MessageTimer").start();
    }

    pub async fn show_game_over(&mut self) {
        self.show_message("Game Over");
        self.base()
            .get_node_as::<Timer>("MessageTimer")
            .signals()
            .timeout()
            .to_future()
            .await;

        let mut message = self.base().get_node_as::<Label>("Message");

        message.set_text("Dodge the Creeps!");
        message.show();

        self.base()
            .get_tree()
            .unwrap()
            .create_timer(1.0)
            .unwrap()
            .signals()
            .timeout()
            .to_future()
            .await;

        self.base().get_node_as::<Button>("StartButton").show();
    }

    pub fn update_score(&self, score: i64) {
        self.base()
            .get_node_as::<Label>("ScoreLabel")
            .set_text(&score.to_string());
    }

    fn on_start_button_pressed(&mut self) {
        self.base().get_node_as::<Button>("StartButton").hide();
        self.signals().start_game().emit();
    }

    fn on_message_timer_timeout(&mut self) {
        self.base().get_node_as::<Label>("Message").hide();
    }
}
