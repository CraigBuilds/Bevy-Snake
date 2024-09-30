use bevy::{
    prelude::*, sprite::Mesh2dHandle
};

#[derive(Resource, Default)]
pub struct SnakeAssets {
    pub square_handle: Option<Mesh2dHandle>,
    pub triangle_handle: Option<Mesh2dHandle>,
    pub bigger_triangle_handle: Option<Mesh2dHandle>,
    pub white_handle: Option<Handle<ColorMaterial>>,
    pub green_handle: Option<Handle<ColorMaterial>>,
    pub yellow_handle: Option<Handle<ColorMaterial>>
}

pub fn register_assets(
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut handles: ResMut<SnakeAssets>
) {
    handles.square_handle = Some(Mesh2dHandle(meshes.add(Rectangle::new(10.0, 10.0))));
    handles.triangle_handle = Some(Mesh2dHandle(meshes.add(Triangle2d::new(Vec2::new(5.0, 5.0), Vec2::new(-5.0, 5.0), Vec2::new(0.0, -5.0)))));
    handles.bigger_triangle_handle = Some(Mesh2dHandle(meshes.add(Triangle2d::new(Vec2::new(7.0, 7.0), Vec2::new(-7.0, 7.0), Vec2::new(0.0, -7.0)))));
    handles.white_handle = Some(materials.add(Color::srgb(1.0, 1.0, 1.0)));
    handles.green_handle = Some(materials.add(Color::srgb(0.0, 1.0, 0.0)));
    handles.yellow_handle = Some(materials.add(Color::srgb(1.0, 1.0, 0.0)));
}