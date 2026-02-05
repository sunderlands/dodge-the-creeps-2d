use godot::{
    classes::{AnimatedSprite2D, Area2D, CollisionShape2D, IArea2D, Input},
    prelude::*,
};

#[derive(GodotClass)]
#[class(base=Area2D)]
pub struct Player {
    #[export]
    speed: real,
    screen_size: Vector2,

    base: Base<Area2D>,
}
#[godot_api]
impl IArea2D for Player {
    fn init(base: Base<Area2D>) -> Self {
        Self {
            speed: 400.0,
            screen_size: Vector2::ZERO,
            base,
        }
    }

    fn ready(&mut self) {
        self.screen_size = self.base().get_viewport_rect().size;

        self.base_mut().hide();

        self.signals()
            .body_entered()
            .connect_self(Self::on_body_entered);
    }

    fn process(&mut self, delta: f32) {
        let mut velocity = Vector2::ZERO;

        let input = Input::singleton();
        if input.is_action_pressed("move_right") {
            velocity += Vector2::RIGHT;
        }
        if input.is_action_pressed("move_left") {
            velocity += Vector2::LEFT;
        }
        if input.is_action_pressed("move_down") {
            velocity += Vector2::DOWN;
        }
        if input.is_action_pressed("move_up") {
            velocity += Vector2::UP;
        }

        let mut animated_sprite = self
            .base()
            .get_node_as::<AnimatedSprite2D>("AnimatedSprite2D");

        if velocity.length() > 0.0 {
            velocity = velocity.normalized() * self.speed;

            if velocity.x != 0.0 {
                animated_sprite.set_animation("walk");
                animated_sprite.set_flip_v(false);
                animated_sprite.set_flip_h(velocity.x < 0.0);
            } else {
                animated_sprite.set_animation("up");
                animated_sprite.set_flip_v(velocity.y > 0.0)
            }
            animated_sprite.play();
        } else {
            animated_sprite.stop();
        }

        let mut position = self.base().get_position() + velocity * delta;
        position = position.clamp(Vector2::ZERO, self.screen_size);
        self.base_mut().set_position(position);
    }
}
#[godot_api]
impl Player {
    #[signal]
    pub fn hit();

    fn on_body_entered(&mut self, _body: Gd<Node2D>) {
        self.base_mut().hide();
        self.signals().hit().emit();
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_deferred("disabled", &true.to_variant());
    }

    #[allow(dead_code)]
    pub fn start(&mut self, pos: Vector2) {
        self.base_mut().set_position(pos);
        self.base_mut().show();
        self.base()
            .get_node_as::<CollisionShape2D>("CollisionShape2D")
            .set_deferred("disabled", &false.to_variant());
    }
}
