use bevy::ecs::event::{Events, ManualEventReader};
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier3d::geometry::Collider;
use bevy::ui::*;
use bevy_rapier3d::prelude::RigidBody;

const PLAYER_SPAWN_COORD: [f32; 3] = [0.0, 2.0, 0.0];
const PLAYER_HEIGHT_BODY: f32 = 1.5;
const PLAYER_HEIGHT_VIEW: f32 = 1.8;
const PLAYER_COLOR: StandardMaterial = StandardMaterial {
    base_color: Color::BLACK,
    base_color_texture: None,
    emissive: Color::BLACK,
    emissive_texture: None,
    perceptual_roughness: 0.0,
    metallic: 0.0,
    metallic_roughness_texture: None,
    reflectance: 0.0,
    normal_map_texture: None,
    flip_normal_map_y: false,
    occlusion_texture: None,
    double_sided: false,
    cull_mode: None,
    unlit: false,
    alpha_mode: AlphaMode::Opaque,
    depth_bias: 0.0,
};

#[derive(Default)]
struct InputState {
    reader_motion: ManualEventReader<MouseMotion>,
    pitch: f32,
    yaw: f32,
}

pub struct MovementSettings {
    pub sensitivity: f32,
    pub speed: f32,
}

impl Default for MovementSettings {
    fn default() -> Self {
        Self {
            sensitivity: 0.00012,
            speed: 12.,
        }
    }
}

#[derive(Component)]
pub struct PlayerCamera;

#[derive(Component)]
pub struct PlayerBody;

#[derive(Component)]
pub struct PlayerHead;

#[derive(Component)]
pub struct Player;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<InputState>()
            .init_resource::<MovementSettings>()
            .add_startup_system(setup_player)
            .add_startup_system(initial_grab_cursor)
            .add_system(player_move)
            .add_system(cursor_grab)
            .add_system(player_look);
    }
}

fn setup_player(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    let head = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Circle {
            radius: 1.0,
            vertices: 10,
        })),
        material: materials.add(PLAYER_COLOR),
        transform: Transform::from_xyz(0.0, PLAYER_HEIGHT_VIEW, 0.0),
        ..default()
    };
    let camera = Camera3dBundle {
        transform: Transform::from_xyz(1.1, PLAYER_HEIGHT_VIEW, 0.0),
        ..default()
    };
    let body = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Capsule {
            radius: 1.0,
            rings: 1,
            depth: PLAYER_HEIGHT_BODY,
            latitudes: 10,
            longitudes: 10,
            uv_profile: Default::default(),
        })),
        material: materials.add(PLAYER_COLOR),
        transform: Transform::from_xyz(PLAYER_SPAWN_COORD[0], PLAYER_SPAWN_COORD[1], PLAYER_SPAWN_COORD[2]),
        ..default()
    };

    //PLAYER
    commands.spawn_bundle(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Cube {
            size: 1.0
        })),
        transform: Transform::from_xyz(0.0, PLAYER_HEIGHT_VIEW, 0.0),
        ..default()
    }).with_children(|commands| {
        //BODY
        commands.spawn_bundle(body)
            .insert(PlayerBody)
            .insert(Name::new("PlayerBody"));
    })
        .with_children(|commands| {
            //HEAD
            commands.spawn_bundle(head)
                .with_children(|commands| {
                    //CAMERA
                    commands.spawn_bundle(camera)
                        .insert(PlayerCamera)
                        .insert(Name::new("PlayerCamera"));
                }).insert(PlayerHead)
                .insert(Name::new("PlayerHead"));
        }).insert(Player)
        .insert(Name::new("Player"));
}

fn initial_grab_cursor(mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        toggle_grab_cursor(window);
    }
}

fn cursor_grab(keys: Res<Input<KeyCode>>, mut windows: ResMut<Windows>) {
    if let Some(window) = windows.get_primary_mut() {
        if keys.just_pressed(KeyCode::Escape) {
            toggle_grab_cursor(window);
        }
    }
}

fn toggle_grab_cursor(window: &mut Window) {
    window.set_cursor_lock_mode(!window.cursor_locked());
    window.set_cursor_visibility(!window.cursor_visible());
}

fn player_move(
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    windows: Res<Windows>,
    settings: Res<MovementSettings>,
    mut query: Query<&mut Transform, With<Player>>,
) {
    if let Some(_window) = windows.get_primary() {
        for mut transform in query.iter_mut() {
            let mut velocity = Vec3::ZERO;
            let local_z = transform.local_z();
            let forward = -Vec3::new(local_z.x, 0., local_z.z);
            let right = Vec3::new(local_z.z, 0., -local_z.x);

            for key in keys.get_pressed() {
                match key {
                    KeyCode::Z => velocity += forward,
                    KeyCode::S => velocity -= forward,
                    KeyCode::Q => velocity -= right,
                    KeyCode::D => velocity += right,
                    KeyCode::Space => velocity += Vec3::Y,
                    KeyCode::LShift => velocity -= Vec3::Y,
                    _ => (),
                }
            }

            velocity = velocity.normalize_or_zero();

            transform.translation += velocity * time.delta_seconds() * settings.speed
        }
    }
}

fn player_look(
    settings: Res<MovementSettings>,
    windows: Res<Windows>,
    mut state: ResMut<InputState>,
    motion: Res<Events<MouseMotion>>,
    mut query_player: Query<&mut Transform, With<Player>>,
) {
    if let Some(window) = windows.get_primary() {
        let mut delta_state = state.as_mut();


        for mut transform in query_player.iter_mut() {
            for ev in delta_state.reader_motion.iter(&motion) {
                if window.cursor_locked() {
                    let window_scale = window.height().min(window.width());
                    delta_state.pitch -=
                        (settings.sensitivity * ev.delta.y * window_scale).to_radians();
                    delta_state.yaw -=
                        (settings.sensitivity * ev.delta.x * window_scale).to_radians();
                }

                delta_state.pitch = delta_state.pitch.clamp(-1.54, 1.54);

                transform.rotation = Quat::from_axis_angle(Vec3::Y, delta_state.yaw)
                    * Quat::from_axis_angle(Vec3::X, delta_state.pitch);
            }
        }
    }
}
