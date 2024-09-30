use bevy::{
    prelude::*, sprite::{MaterialMesh2dBundle, Mesh2dHandle}
};
use std::{collections::LinkedList, ops::DerefMut};

const GRID_CELL_SIZE: f32 = 10.0;
const SEGMENT_SIZE: f32 = 6.0;
const TIME_STEP: f32 = 0.5;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(SnakeState::default())
        .add_systems(Startup, setup)
        .add_systems(PreUpdate, direction_control_system)
        .add_systems(Update,
            (
                spawn_food_system,
            )
        )
        .add_systems(PostUpdate, 
            (
                movement_and_segment_propagation_system,
                render_system
            ).chain()
        )
        .run();
}

/// Stores the current direction, and grid coordinates of the snake's head and body segments
/// This is instead of maintaining parent child relationships between segment coordinates
#[derive(Resource, Default)]
struct SnakeState{
    head: Vec2,
    body: LinkedList<Vec2>,
    dir: Vec2,
}

/// Component to identify a snake segment by its index in the head/body list
#[derive(Component)]
struct SnakeSegment(usize);

/// Startup system. Set initial direction, and create snake segments and camera
fn setup(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut snake_state: ResMut<SnakeState>
)
{
    snake_state.dir = Vec2::new(GRID_CELL_SIZE, 0.0);

    for _ in 0..6 {
        spawn_segment(&mut cmds, snake_state.deref_mut(), &mut meshes, &mut materials);
    }

    cmds.spawn(Camera2dBundle::default());
}

/// Not a system, just a helper function to add a segment to the snake
fn spawn_segment(
    cmds: &mut Commands,
    snake_state: &mut SnakeState,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
){
    let head_clone = snake_state.head;
    snake_state.body.push_front(head_clone);
    let idx = snake_state.body.len();
    cmds.spawn(
        (
            SnakeSegment(idx),
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(SEGMENT_SIZE, SEGMENT_SIZE))),
                material: materials.add(Color::srgb(0.0, 1.0, 0.0)),
                // Hidden and with default transform.
                // The render system will use the SnakeSegment index and SnakeState to set the transform and visibility
                visibility: Visibility::Hidden,
                ..Default::default()
            }
        )
    );
}


/// System to handle keyboard input and change the direction of the snake
fn direction_control_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut snake_state: ResMut<SnakeState>,
) {
    let snake_state = snake_state.deref_mut();

    let dir = if keyboard_input.pressed(KeyCode::KeyW) {
        Vec2::new(0.0, GRID_CELL_SIZE)
    } else if keyboard_input.pressed(KeyCode::KeyS) {
        Vec2::new(0.0, -GRID_CELL_SIZE)
    } else if keyboard_input.pressed(KeyCode::KeyA) {
        Vec2::new(-GRID_CELL_SIZE, 0.0)
    } else if keyboard_input.pressed(KeyCode::KeyD) {
        Vec2::new(GRID_CELL_SIZE, 0.0)
    } else {
        snake_state.dir
    };
    snake_state.dir = dir;
}

/// System to move the snake (using SnakeState) and propagate the segments
fn movement_and_segment_propagation_system(mut snake_state: ResMut<SnakeState>, time: Res<Time>, mut timer: Local<Timer>) {
    timer.tick(time.delta());
    if timer.finished() {
    
        let snake = snake_state.deref_mut();
        let snake_dir = snake.dir;
        let old_head = snake.head;
        snake.head += snake_dir;
        snake.body.push_front(old_head);
        snake.body.pop_back();

        timer.set_mode(TimerMode::Repeating);
        timer.set_duration(std::time::Duration::from_secs_f32(TIME_STEP));
        timer.reset();
    }
}

/// System to render the snake using the snake state to set the position of the segments
fn render_system(snake_state: Res<SnakeState>, mut segment_q: Query<(&mut Transform, &mut Visibility, &SnakeSegment)>) {
    for (mut segment_transform, mut segment_vis, segment_index) in segment_q.iter_mut() {
        if segment_index.0 == 0 {
            segment_transform.translation = snake_state.head.extend(0.0);
            *segment_vis = Visibility::Visible;
        } else {
            let segment = snake_state.body.iter().nth(segment_index.0 -1).unwrap();
            segment_transform.translation = segment.extend(0.0);
            *segment_vis = Visibility::Visible;
        }
    }
}

#[derive(Component, Default)]
struct Food;

/// If there is no food, spawn one
fn spawn_food_system(
    mut cmds: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    food_query: Query<&Food>,
) {
    if food_query.iter().count() == 0 {
        let x = (rand::random::<i32>() % 20) * 10;
        let y = (rand::random::<i32>() % 20) * 10;
        cmds.spawn(
            (
                Food,
                MaterialMesh2dBundle {
                    mesh: Mesh2dHandle(meshes.add(Rectangle::new(SEGMENT_SIZE, SEGMENT_SIZE))),
                    material: materials.add(Color::srgb(1.0, 0.0, 0.0)),
                    transform: Transform::from_translation(Vec3::new(x as f32, y as f32, 0.0)),
                    ..Default::default()
                }
            )
        );
    }
}