use bevy::{
    prelude::*, sprite::Mesh2dHandle
};
mod assets;
use assets::{SnakeAssets, register_assets};
mod grid;
use grid::{GridCell, Direction, grid_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<SnakeAssets>()
        .insert_resource(Time::<Fixed>::from_seconds(0.5))
        .add_event::<GrowEvent>()
        .add_systems(Startup, 
            (
                register_assets,
                spawn_snake_head,
                setup_camera
            ).chain()
        )
        .add_systems(Update, 
            (
                grid_system,
                keyboard_system,
                snake_growth_system,
                debug_system,
            ).chain()
        )
        .add_systems(FixedUpdate,
            (
                head_movement_system,
                segment_movement_system
            ).chain()
        )
        .run();
}

#[derive(Component)]
struct SnakeHeadMarker;

#[derive(Component, Default, Clone, Copy, Debug)]
struct SegmentRelationships {
    parent: Option<Entity>,
    child: Option<Entity>
}

#[derive(Bundle, Default, Debug)]
struct SnakeSegment {
    relationships: SegmentRelationships,
    mesh: Mesh2dHandle,
    grid_cell: GridCell,
    direction: Direction,
    //The below are needed so the bevy render systems can do their thing
    material: Handle<ColorMaterial>,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
}

impl SnakeSegment {
    fn new(cell: &GridCell) -> Self {
        SnakeSegment {
            grid_cell: *cell,
            ..Default::default()
        }
    }
    fn with_parent(mut self, parent: Entity) -> Self {
        self.relationships.parent = Some(parent);
        self
    }
    fn with_direction(mut self, direction: &Direction) -> Self {
        self.direction = direction.clone();
        self
    }
    fn with_mesh(mut self, mesh_handle: &Mesh2dHandle) -> Self {
        self.mesh = mesh_handle.clone();
        self
    }
    fn with_material(mut self, material_handle: &Handle<ColorMaterial>) -> Self {
        self.material = material_handle.clone();
        self
    }
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_snake_head(
    mut cmds: Commands,
    assets: Res<SnakeAssets>
){
    cmds.spawn(
        (
            SnakeSegment::new(&GridCell { x: 0, y: 0 })
                .with_material(assets.green_handle.as_ref().unwrap())
                .with_mesh(assets.bigger_triangle_handle.as_ref().unwrap()),
            SnakeHeadMarker
        )
    );
}

///Make the segment without a child yellow, and the others green
fn debug_system(
    mut q_segments: Query<(&mut Handle<ColorMaterial>, &SegmentRelationships)>,
    assets: Res<SnakeAssets>
) {
    //find the segment without a child (there will only be one, right at the end), and make it yellow.
    for (mut color_handle, relationship) in q_segments.iter_mut() {
        if relationship.child.is_none() {
            *color_handle = assets.yellow_handle.as_ref().unwrap().clone();
        }
        else {
            *color_handle = assets.green_handle.as_ref().unwrap().clone();
        }
    }
}


#[derive(Event)]
struct GrowEvent;
fn snake_growth_system(
    mut cmds: Commands,
    mut grow_event_reader: EventReader<GrowEvent>,
    mut q_segments: Query<(Entity, &GridCell, &Direction, &mut SegmentRelationships)>,
    assets: Res<SnakeAssets>,
){
    //every time there is a grow event...
    for _ in grow_event_reader.read() {
        //find the segment without a child (there will only be one, right at the end), and fill it with a segment with a parent and no child.
        for (segment_id, cell, direction, mut relationship) in q_segments.iter_mut() {
            if relationship.child.is_none() {
                relationship.child = Some(
                    cmds.spawn(
                        SnakeSegment::new(&cell.behind(direction))
                            .with_parent(segment_id)
                            .with_direction(direction)
                            .with_material(assets.white_handle.as_ref().unwrap())
                            .with_mesh(assets.triangle_handle.as_ref().unwrap())
                    ).id()
                );
            }
        }
    }
}

fn head_movement_system(
    mut q_head: Query<(&mut GridCell, &Direction), With<SnakeHeadMarker>>){
    for (mut cell, direction) in q_head.iter_mut() {
        *cell = cell.in_front(direction);
    }
}

///Move each child segment to the position of its parent.
fn segment_movement_system(mut q_segments: Query<(&SegmentRelationships, &mut Direction, &mut GridCell)>){

    //collect into a vec
    let vec = q_segments.iter().map(|(relationships, direction, cell)| {
        (*relationships, *direction, *cell)
    }).collect::<Vec<_>>();

    //for each segment with a child, use the query to get the child cell and set it to the parent cell.
    for (this_relationships, this_direction, this_cell) in vec.iter() {
        if let Some(this_child) = this_relationships.child {
            let mut child_cell = q_segments.get_mut(this_child).unwrap().2;
            *child_cell.as_mut() = this_cell.behind(&this_direction);
            let mut child_direction = q_segments.get_mut(this_child).unwrap().1;
            *child_direction.as_mut() = *this_direction;
        }
    }
}

fn keyboard_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut grow_events: EventWriter<GrowEvent>,
    mut q_head: Query<&mut Direction, With<SnakeHeadMarker>>
){
    if keyboard_input.just_pressed(KeyCode::Space) {
        grow_events.send(GrowEvent);
    }
    if keyboard_input.just_pressed(KeyCode::KeyW) {
        for mut direction in q_head.iter_mut() {
            *direction = Direction::Up;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyS) {
        for mut direction in q_head.iter_mut() {
            *direction = Direction::Down;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        for mut direction in q_head.iter_mut() {
            *direction = Direction::Left;
        }
    }
    if keyboard_input.just_pressed(KeyCode::KeyD) {
        for mut direction in q_head.iter_mut() {
            *direction = Direction::Right;
        }
    }
}