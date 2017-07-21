extern crate portaudio;
use portaudio as pa;
use std::f64::consts::PI;

mod sound_source;
mod source_formats;
mod vector;

const TABLE_SIZE: usize = 200;

fn main() {
    println!("Hello, portaudio-using world!");
    run().unwrap();
}

fn run() -> Result<(), pa::Error> {
    let pa = pa::PortAudio::new()?;
    let num_devices = pa.device_count()?;
    println!("Number of devices: {}", num_devices);

    println!("Default input device: {:?}", pa.default_input_device());
    println!("Default output device: {:?}", pa.default_output_device());

    for device in pa.devices()? {
        let (_, info) = device?;
        println!("-----------------------------");
        println!("{:#?}", &info);
    }

    // Sine playback
    let sine_args = source_formats::SineInitArgs {frequency:1.0};
    let sine_source = sound_source::SoundSource::new(sine_args);

    const CHANNELS: i32 = 2;
    const FREQ: f64 = 44_100.0;
    const BUFFER_SIZE: u32 = 64;
    let mut settings = try!(pa.default_output_stream_settings(CHANNELS, FREQ, BUFFER_SIZE));
    settings.flags = pa::stream_flags::CLIP_OFF;

    let callback = move |pa::OutputStreamCallbackArgs {buffer, frames, ..}| {
        sine_source.get_bytes(buffer, frames);
        pa::Continue
    };

    let mut stream = pa.open_non_blocking_stream(settings, callback)?;
    stream.start()?;

    let seconds = 1;
    println!("Playing for {} second(s)", seconds);
    pa.sleep(seconds * 1_000); // time to sleep in ms

    stream.stop()?;
    stream.close()?;

    println!("Done! (^_^)v");

    Ok(())
}
