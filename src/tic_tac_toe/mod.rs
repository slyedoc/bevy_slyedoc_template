use bevy::prelude::*;
use bevy_inspector_egui::Inspectable;
use bevy_inspector_egui::InspectorPlugin;
use bevy_mod_picking::PickableBundle;

use crate::{helpers::cleanup_system, GameState};

#[derive(Inspectable, Debug)]
pub struct TicTacToeData {
    clear_color: Color,

    #[inspectable(min = 0.0, max = 300.0, label = "Size")]
    size: f32,

    #[inspectable(min = 0.01, max = 20.0, label = "Line Thickness")]
    line_thickness: f32,

    board_material: Handle<ColorMaterial>,
    x_material: Handle<ColorMaterial>,
    o_material: Handle<ColorMaterial>,
    none_material: Handle<StandardMaterial>,
}

impl FromWorld for TicTacToeData {
    fn from_world(world: &mut World) -> Self {
        let world = world.cell();

        let mut materials = world
            .get_resource_mut::<Assets<ColorMaterial>>()
            .expect("ResMut<Assets<ColorMaterial>> not found.");

        let mut standard_materials = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("ResMut<Assets<StandardMaterial>> not found.");

        TicTacToeData {
            clear_color: Color::WHITE,
            board_material: materials.add(Color::BLACK.into()),
            o_material: materials.add(Color::BLUE.into()),
            x_material: materials.add(Color::RED.into()),
            none_material: standard_materials.add(StandardMaterial {
                base_color: Color::ANTIQUE_WHITE.into(),
                unlit: true,
                ..Default::default()
            }),
            size: 400.0,
            line_thickness: 10.0,
        }
    }
}

pub struct TicTacToePlugin;

impl Plugin for TicTacToePlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(InspectorPlugin::<TicTacToeData>::new().open(false))
            .add_system_set(SystemSet::on_enter(GameState::TicTacToe).with_system(setup.system()))
            .add_system_set(
                SystemSet::on_exit(GameState::TicTacToe)
                    .with_system(cleanup_system::<TicTacToe>.system()),
            )
            .add_system_set(
                SystemSet::on_update(GameState::TicTacToe).with_system(update.system()),
            );
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct TicTacToe;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Cell {
    None,
}

fn setup(
    mut commands: Commands,
    data: Res<TicTacToeData>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut clear_color: ResMut<ClearColor>,
) {
    clear_color.0 = data.clear_color;

    let board_center_offset = Vec2::new(0.0, 0.0);
    let board_left_edge: f32 = board_center_offset.x - 0.5 * data.size;
    let board_bot_edge: f32 = board_center_offset.y - 0.5 * data.size;

    let cell_size = data.size / 3.0;

    // Spawn Camera
    commands
        .spawn_bundle(UiCameraBundle::default())
        .insert(TicTacToe);

    commands
        .spawn_bundle(OrthographicCameraBundle::new_2d())
        .insert(TicTacToe);

    //Board Horizontal
    draw_line(
        &mut commands,
        Vec2::new(data.line_thickness, data.size),
        Vec2::new(board_left_edge + cell_size, 0.0),
        &data.board_material,
    );

    draw_line(
        &mut commands,
        Vec2::new(data.line_thickness, data.size),
        Vec2::new(board_left_edge + (cell_size * 2.0), 0.0),
        &data.board_material,
    );

    // Board Vertical
    draw_line(
        &mut commands,
        Vec2::new(data.size, data.line_thickness),
        Vec2::new(0.0, board_left_edge + cell_size),
        &data.board_material,
    );

    draw_line(
        &mut commands,
        Vec2::new(data.size, data.line_thickness),
        Vec2::new(0.0, board_left_edge + (cell_size * 2.0)),
        &data.board_material,
    );

    let mesh = meshes.add(Mesh::from(shape::Quad {
        size: Vec2::new(cell_size, cell_size),
        flip: false,
    }));

    for i in 0..=2 {
        for j in 0..=2 {
            commands
                .spawn_bundle(PbrBundle {
                    transform: Transform {
                        translation: Vec3::new(
                            board_left_edge + (cell_size * 0.5) + cell_size * i as f32,
                            board_bot_edge + (cell_size * 0.5) + cell_size * j as f32,
                            0.0,
                        ),
                        scale: Vec3::splat(0.8),
                        ..Default::default()
                    },
                    mesh: mesh.clone(),
                    material: data.none_material.clone(),
                    ..Default::default()
                })
                .insert(TicTacToe)
                .insert(Cell::None)
                .insert(Name::new(format!("Cell {}x{}", i, j)))
                .insert_bundle(PickableBundle::default());
        }
    }
}

fn draw_line<'a>(
    commands: &'a mut Commands,
    size: Vec2,
    pos: Vec2,
    material: &'a Handle<ColorMaterial>,
) {
    commands
        .spawn_bundle(SpriteBundle {
            sprite: Sprite {
                size: size,
                ..Default::default()
            },
            material: material.clone(),
            transform: Transform::from_xyz(pos.x, pos.y, 0.0),
            ..Default::default()
        })
        .insert(Name::new("Line"))
        .insert(TicTacToe);
}

fn update(
    mut _commands: Commands,
    mut clear_color: ResMut<ClearColor>,
    data: Res<TicTacToeData>,
    _query: Query<(Entity, &Sprite, &Transform, &Interaction), (With<Cell>, Changed<Interaction>)>,
) {
    // let (entity, _size, _pos) = query
    //     .iter()
    //     .filter(|(_, _, _, interaction)| matches!(interaction, Interaction::Clicked))
    //     .map(|(entity, s, t, _)| (Some(entity), s.size, t.translation))
    //     .next()
    //     .unwrap_or_else(|| (None, Vec2::new(10.0, 10.0), Vec3::ZERO));

    // if let Some(entity) = entity {
    //     commands.entity(entity).despawn_recursive();
    // }

    // TODO: Remove this hack, but it lets each state have its own background color
    clear_color.0 = data.clear_color;
}