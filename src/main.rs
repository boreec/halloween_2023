mod audio;

use audio::Music;
use bevy::prelude::*;

const SCALE_FACTOR: f32 = 100.;
const SPRITE_PATH: &str = "img/jack.png";
const SPRITE_HEIGHT: f32 = 100.;
const SPRITE_WIDTH: f32 = 100.;

#[derive(Component)]
struct Jack;

impl Jack {
    fn new() -> Self {
        Jack {}
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, scale_sprite)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.init_resource::<Music>();
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                custom_size: Some(Vec2::new(SPRITE_WIDTH, SPRITE_HEIGHT)),
                ..default()
            },
            transform: Transform::from_scale(Vec3::ONE),
            texture: asset_server.load(SPRITE_PATH),
            ..default()
        },
        Jack::new(),
    ));
    commands.spawn(AudioBundle {
        source: asset_server.load("music/MountainKing.mp3"),
        ..default()
    });
}

fn scale_sprite(
    mut sprites: Query<&mut Transform, &Jack>,
    timer: Res<Time>,
    music: Res<Music>,
) {
    let loudness = music.current_loudness(timer);
    let scale = Vec3::new(loudness, loudness, loudness) * SCALE_FACTOR;
    for mut sprite in &mut sprites {
        sprite.scale = Vec3::ONE * scale;
    }
}
