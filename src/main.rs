use hound::WavReader;
use lame::Config;
use std::fs::File;
use std::io::{BufReader, BufWriter, Write};

fn main() {
    // Open the input WAV file
    let input_filename = "input.wav";
    let reader = WavReader::open(input_filename).unwrap();
    let spec = reader.spec();
    let samples: Vec<i16> = reader.into_samples::<i16>().map(|x| x.unwrap()).collect();

    // Open the output MP3 file
    let output_filename = "output.mp3";
    let file = File::create(output_filename).unwrap();
    let mut writer = BufWriter::new(file);

    // Configure the MP3 encoder
    let config = Config::new()
        .sample_rate(spec.sample_rate)
        .channels(spec.channels)
        .bitrate(192);

    // Encode the audio samples to MP3 and write them to the output file
    let mut encoder = config.build().unwrap();
    let mut mp3_samples = Vec::new();
    let mut remaining_samples = &samples[..];
    loop {
        match encoder.encode(remaining_samples, &mut mp3_samples) {
            Ok(_) => {}
            Err(_) => break,
        };
        if remaining_samples.len() <= encoder.samples_to_encode() {
            break;
        }
        remaining_samples = &remaining_samples[encoder.samples_to_encode()..];
    }
    match encoder.flush(&mut mp3_samples) {
        Ok(_) => {}
        Err(_) => {}
    }
    writer.write_all(&mp3_samples).unwrap();

    println!("Finished converting {} to {}", input_filename, output_filename);
}
