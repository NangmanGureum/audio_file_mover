use rodio::OutputStream;
use std::path::Path;
mod audio;
mod file_maneger;

use druid::widget::Label;
use druid::{AppLauncher, PlatformError, Widget, WindowDesc};

fn main() -> Result<(), PlatformError> {
    // Audio Reading Test
    // let sample_file = file_maneger::File::new("./example_audio.mp3");
    // let sample_audio = audio::AudioFile::new(Path::new(&sample_file.full_name)).unwrap();

    // File Play Test
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // audio::AudioFile::play(&sample_audio, &stream_handle);
    AppLauncher::with_window(WindowDesc::new(main_window())).launch(())?;
    Ok(())
}

fn main_window() -> impl Widget<()> {
    Label::new("Hello world")
}
