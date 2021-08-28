use rodio::OutputStream;
use std::path::Path;
mod audio;
mod file_maneger;

fn main() {
    // Audio Reading Test
    // let sample_file = file_maneger::File::new("./example_audio.mp3");
    // let sample_audio = audio::AudioFile::new(Path::new(&sample_file.full_name)).unwrap();

    // File Play Test
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // audio::AudioFile::play(&sample_audio, &stream_handle);
}
