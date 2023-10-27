use bevy::prelude::*;
use minimp3::{Decoder as Mp3Decoder, Error as Mp3Error, Frame as Mp3Frame};
use std::fs::File;
use std::thread;
use std::time::Duration;

const SCALE_FACTOR: f32 = 100.;
const SPRITE_PATH: &str = "img/jack.png";
const SPRITE_HEIGHT: f32 = 100.;
const SPRITE_WIDTH: f32 = 100.;

#[derive(Resource)]
struct Music {
    /// loudness for every samples of the music, with 0 being aboslutely quiet
    // and 1 being absolutely loud.
    samples_loudness: Vec<f32>,
    samples_rate: u32,
}

#[derive(Component)]
struct Jack;

impl Jack {
    fn new() -> Self {
        Jack {}
    }
}

fn calculate_loudness(samples: &Vec<i16>) -> Vec<f32> {
    // Find the maximum absolute value of the samples
    let max_sample =
        samples.iter().map(|&x| x.abs() as f32).fold(0.0, f32::max);

    // Normalize each sample to a float between 0 and 1
    let loudness: Vec<f32> = samples
        .iter()
        .map(|&x| (x.abs() as f32) / max_sample)
        .collect();

    loudness
}

impl Default for Music {
    fn default() -> Self {
        let (samples, samples_rate) =
            read_mp3_to_mono("assets/music/MountainKing.mp3");
        let samples_loudness = calculate_loudness(&samples);
        Music {
            samples_loudness,
            samples_rate,
        }
    }
}

fn read_mp3_to_mono(file: &str) -> (Vec<i16>, u32) {
    let mut decoder = Mp3Decoder::new(File::open(file).unwrap());

    let mut sampling_rate = 0;
    let mut mono_samples = vec![];
    loop {
        match decoder.next_frame() {
            Ok(Mp3Frame {
                data: samples_of_frame,
                sample_rate,
                channels,
                ..
            }) => {
                // that's a bird weird of the original API. Why should channels or sampling
                // rate change from frame to frame?

                // Should be constant throughout the MP3 file.
                sampling_rate = sample_rate;

                if channels == 2 {
                    for (i, sample) in
                        samples_of_frame.iter().enumerate().step_by(2)
                    {
                        let sample = *sample as i32;
                        let next_sample = samples_of_frame[i + 1] as i32;
                        mono_samples
                            .push(((sample + next_sample) as f32 / 2.0) as i16);
                    }
                } else if channels == 1 {
                    mono_samples.extend_from_slice(&samples_of_frame);
                } else {
                    panic!("Unsupported number of channels={}", channels);
                }
            }
            Err(Mp3Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    (mono_samples, sampling_rate as u32)
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
    let current_sample = (((timer.elapsed().as_millis() as f64 / 1000.0)
        * music.samples_rate as f64)
        + 0.5) as usize;
    let loudness = music.samples_loudness[current_sample] * SCALE_FACTOR;
    let scale = Vec3::new(loudness, loudness, loudness);
    println!(
        "loudness:{} sample: {} / {} ({}hz)",
        loudness,
        current_sample,
        music.samples_loudness.len(),
        music.samples_rate
    );
    for mut sprite in &mut sprites {
        sprite.scale = Vec3::ONE * scale;
    }
}
