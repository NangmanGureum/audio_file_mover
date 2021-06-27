use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;

struct Meta {
    source: Decoder<std::io::BufReader<std::fs::File>>,
    path: String,
    audio_format: String,
    bit_depth: usize,
    sample_rate: u32,
    channels: u16,
    length: f64,
}

// For debug, It will removed when it is done
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {
    // For debug, It will removed when it is done
    let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();

    // let file = File::open("./example_audio.mp3").unwrap();
    // let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
    let sameple_audio = load_audio(String::from("./example_audio.mp3"));

    // A divider
    println!("{}", "=".repeat(10));

    // Title
    println!("Audio Sample Play");

    // File Metadata
    println!("Path: {}", sameple_audio.path);
    println!("Format: {}", sameple_audio.audio_format);
    println!("Bit Depth: {}", sameple_audio.bit_depth);
    println!("Sample Rate: {}", sameple_audio.sample_rate);
    println!("Channels: {}", sameple_audio.channels);
    println!("Length: {}", sameple_audio.length);

    // A divider
    println!("{}", "=".repeat(10));

    // asdf
    println!("{:?}", sameple_audio.source.current_frame_len());

    // Play File
    stream_handle.play_raw(sameple_audio.source.convert_samples());

    loop {}
}

fn load_audio(path: String) -> Meta {
    let file = BufReader::new(File::open(path).unwrap());
    let source = Decoder::new(file).unwrap();

    //let audio_samples = source.convert_samples();

    let audio_sr = source.sample_rate();
    let audio_chan = source.channels();
    let duration = source.total_duration();
    let cfl = source.current_frame_len();

    println!("{:?}", duration);

    print_type_of(&source);

    Meta {
        source: source,
        path: String::from("IDK"),
        audio_format: String::from("IDK"),
        bit_depth: 16, // Temporary
        sample_rate: audio_sr,
        channels: audio_chan,
        length: 30.0, // Temporary
    }
}
