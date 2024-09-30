use bevy::prelude::*;

#[derive(Component, Default, Clone, Copy, Debug)]
pub struct GridCell {
    pub x: i8,
    pub y: i8
}

#[allow(dead_code)]
#[derive(Component, Default, Clone, Copy, Debug)]
pub enum Direction {
    Up,
    #[default]
    Down,
    Left,
    Right
}

impl GridCell {
    pub fn in_front(&self, direction: &Direction) -> GridCell {
        match direction {
            Direction::Up => GridCell { x: self.x, y: self.y + 1 },
            Direction::Down => GridCell { x: self.x, y: self.y - 1 },
            Direction::Right => GridCell { x: self.x + 1, y: self.y },
            Direction::Left => GridCell { x: self.x - 1, y: self.y },
        }
    }
    pub fn behind(&self, direction: &Direction) -> GridCell {
        match direction {
            Direction::Up => GridCell { x: self.x, y: self.y - 1 },
            Direction::Down => GridCell { x: self.x, y: self.y + 1 },
            Direction::Right => GridCell { x: self.x - 1, y: self.y },
            Direction::Left => GridCell { x: self.x + 1, y: self.y },
        }
    }
}

pub fn grid_system(
    mut q_grid_items: Query<(&GridCell, &Direction, &mut Transform)>
) {
    for (cell, dir, mut transform) in q_grid_items.iter_mut() {
        transform.translation = Vec3::new(cell.x as f32 * 15.0, cell.y as f32 * 15.0, 0.0);
        match dir {
            Direction::Down => transform.rotation = Quat::from_rotation_z(0.0),
            Direction::Up => transform.rotation = Quat::from_rotation_z(std::f32::consts::PI),
            Direction::Left => transform.rotation = Quat::from_rotation_z(std::f32::consts::PI * 1.5),
            Direction::Right => transform.rotation = Quat::from_rotation_z(std::f32::consts::PI * 0.5),
        }
    }
}