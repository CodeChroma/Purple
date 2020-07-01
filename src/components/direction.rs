use amethyst::ecs::{Component, DenseVecStorage};

#[derive(PartialEq, Clone, Copy)]
pub enum Directions {
    Up,
    Down,
    Left,
    Right,
    Directionless,
}

#[derive(Component)]
#[storage(DenseVecStorage)]
pub struct Direction {
    pub default_x: Directions,
    pub default_y: Directions,
    pub x: Directions,
    pub y: Directions,
}

impl Default for Direction {
    fn default() -> Self {
        Self {
            default_x: Directions::Directionless,
            default_y: Directions::Directionless,
            x: Directions::Directionless,
            y: Directions::Directionless,
        }
    }
}

impl Direction {
    pub fn new(default_x: Directions, default_y: Directions, x: Directions, y: Directions) -> Self {
        Direction {
            default_x,
            default_y,
            x,
            y,
        }
    }

    pub fn update_velocity_x(&mut self, velocity_x: f32) {
        self.x = if velocity_x.abs() < f32::EPSILON {
            Directions::Directionless
        } else if velocity_x > 0. {
            Directions::Right
        } else {
            Directions::Left
        }
    }
}
