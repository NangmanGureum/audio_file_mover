use druid::widget::{Align, Flex, Label, Padding};
use druid::{AppLauncher, Data, LocalizedString, PlatformError, Widget, WindowDesc};
use rodio::OutputStream;
use std::env::consts::OS;
use std::path::Path;
mod audio;
mod file_maneger;

#[derive(Clone, Data)]
struct AppState {
    selected_file: Option<FileState>,
    curret_dir: CurrentDirState,
}

#[derive(Clone, Data)]
struct CurrentDirState {
    first_dir: String,
    second_dir: String,
}

#[derive(Clone, Data)]
struct FileState {
    file_name: String,
    path: String,
    bits: audio::Bits,
    sample_rate: u32,
    channels: u8,
}

fn main() -> Result<(), PlatformError> {
    // Audio Reading Test
    // let sample_file = file_maneger::File::new("./example_audio.mp3");
    // let sample_audio = audio::AudioFile::new(Path::new(&sample_file.full_name)).unwrap();

    // File Play Test
    // let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // audio::AudioFile::play(&sample_audio, &stream_handle);

    let window = WindowDesc::new(build_ui).title(LocalizedString::new("Audio File Mover"));

    let int_state = match OS {
        "windows" => AppState {
            selected_file: None,
            curret_dir: CurrentDirState {
                first_dir: "C:".to_string(),
                second_dir: "C:".to_string(),
            },
        },
        _ => AppState {
            selected_file: None,
            curret_dir: CurrentDirState {
                first_dir: "~".to_string(),
                second_dir: "~".to_string(),
            },
        },
    };
    AppLauncher::with_window(window).launch(())?;
    Ok(())
}

fn build_ui() -> impl Widget<()> {
    Label::new("Hello world")
}
