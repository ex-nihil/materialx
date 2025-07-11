use bevy::{
    core_pipeline::{fxaa::Fxaa, Skybox},
    prelude::*,
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, (camera_setup,))
            .add_systems(Update, (move_camera,));
    }
}

pub const CAMERA_START: Transform = Transform::from_translation(Vec3::new(-1.25, 2.25, 20.5));

fn camera_setup(mut commands: Commands, assets: Res<AssetServer>) {
    commands.spawn((
        PointLight {
            shadows_enabled: true,
            ..default()
        },
        Transform::from_xyz(4.0, 8.0, 4.0),
    ));

    commands.spawn((
        Name::from("Main Camera"),
        CAMERA_START.looking_at(Vec3::ZERO, Vec3::Y),
        Camera3d::default(),
        EnvironmentMapLight {
            diffuse_map: assets.load("examples/pisa_diffuse_rgb9e5_zstd.ktx2"),
            specular_map: assets.load("examples/pisa_specular_rgb9e5_zstd.ktx2"),
            intensity: 5000.0,
            ..default()
        },
        Skybox {
            image: assets.load("examples/pisa_specular_rgb9e5_zstd.ktx2"),
            brightness: 5000.0,
            ..default()
        },
        Fxaa::default(),
    ));
    // .insert(Skybox {
    //     image: assets.load("examples/DaySkyHDRI029A_2K-HDR.exr"),
    //     brightness: 5000.0,
    // })
    // .insert(ScreenSpaceReflectionsBundle::default())
}

fn move_camera(
    mut camera: Query<(Entity, &mut Transform), With<Camera3d>>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok((_camera, mut transform)) = camera.single_mut() else {
        return;
    };
    if input.pressed(KeyCode::KeyW) {
        transform.translation = transform.translation + transform.local_y() * 0.1;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation = transform.translation - transform.local_y() * 0.1;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation = transform.translation - transform.local_x() * 0.1;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation = transform.translation + transform.local_x() * 0.1;
    }
    if input.pressed(KeyCode::KeyQ) {
        transform.rotation *= Quat::from_rotation_y(0.05);
    }
    if input.pressed(KeyCode::KeyE) {
        transform.rotation *= Quat::from_rotation_y(-0.05);
    }
}
