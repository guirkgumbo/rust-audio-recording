use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use hound::{WavSpec, WavWriter};
use std::fs::File;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

fn main() {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .expect("Failed to get default input device");

    let config: cpal::SupportedStreamConfig = device
        .default_input_config()
        .expect("Failed to get default input config");
    println!("Using input config: {:?}", config);

    let spec = WavSpec {
        channels: config.channels(),
        sample_rate: config.sample_rate().0,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = WavWriter::create("output.wav", spec).unwrap();

    let (tx, rx) = std::sync::mpsc::channel();
    let stream = device.build_input_stream(
        &config.into(),
        move |data: &[i16], _: &_| {
            tx.send(data.to_vec()).unwrap();
        },
        err_fn,
    ).unwrap();

    stream.play().unwrap();

    // Capture audio for a fixed duration
    let capture_duration = std::time::Duration::from_secs(5);
    let mut samples = Vec::new();
    let mut total_samples = 0;

    while total_samples < (spec.sample_rate as usize * capture_duration.as_secs() as usize) {
        let audio_data = rx.recv().unwrap();
        samples.extend(audio_data.iter().cloned());
        total_samples += audio_data.len();
    }

    for sample in samples {
        writer.write_sample(sample).unwrap();
    }

    println!("Finished recording to output.wav");
}

fn err_fn(err: cpal::StreamError) {
    eprintln!("cpal stream error: {}", err);
}
