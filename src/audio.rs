use bevy::prelude::*;
use minimp3::{Decoder, Error, Frame};
use std::fs::File;

#[derive(Resource)]
pub struct Music {
    /// loudness for every samples of the music, with 0 being aboslutely quiet
    // and 1 being absolutely loud.
    max_sample: f32,
    samples: Vec<i16>,
    samples_rate: u32,
}

impl Music {
    // Return the current music loudness between 0 and 1, with 0 being the
    // the quietest, and 1 the loudness.
    pub fn current_loudness(&self, timer: Res<Time>) -> f32 {
        let current_sample = (((timer.elapsed().as_millis() as f64 / 1000.0)
            * self.samples_rate as f64)
            + 0.5) as usize;
        return self.samples[current_sample].abs() as f32 / self.max_sample;
    }
}

impl Default for Music {
    fn default() -> Self {
        let (samples, samples_rate) =
            read_mp3_to_mono("assets/music/MountainKing.mp3");
        let max_sample =
            samples.iter().map(|&x| x.abs() as f32).fold(0.0, f32::max);

        Music {
            max_sample,
            samples,
            samples_rate,
        }
    }
}

// credit to phip1611
pub fn read_mp3_to_mono(file: &str) -> (Vec<i16>, u32) {
    let mut decoder = Decoder::new(File::open(file).unwrap());

    let mut sampling_rate = 0;
    let mut mono_samples = vec![];
    loop {
        match decoder.next_frame() {
            Ok(Frame {
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
            Err(Error::Eof) => break,
            Err(e) => panic!("{:?}", e),
        }
    }

    (mono_samples, sampling_rate as u32)
}
