use claxon::FlacReader;
use druid::Data;
use hound::{SampleFormat, WavReader};
use rodio::{source::Source, Decoder, OutputStreamHandle};
use std::fs::File;
use std::io::BufReader;
use std::path::Path;

// Process
//
// 1. Check what if playing
// 2. Get audio file info (e. g.: bits, frequencies, channels...)
// 3. Play the file

#[derive(Debug, Clone, Data)]
pub struct AudioFile {
    pub file_path: String,
    pub bits: Option<Bits>,
    pub sample_rate: u32, // e. g. 44_100, 48_000...
    pub channels: u8,
}

#[derive(Debug, Clone, Data, PartialEq)]
pub enum Bits {
    Eight,
    Sixteen,
    TwentyFour,
    ThirtyTwoInt,
    ThirtyTwoFloat,
    SixtyFourInt,
    SixtyFourFloat,
    Int(u8),
    Float(u8),
}

impl AudioFile {
    pub fn new(file: &Path) -> Option<AudioFile> {
        // Initialize
        let mut bits: Option<Bits> = None;
        let mut sample_rate: u32 = 0;
        let mut channels: u8 = 1;
        let file_string = file.to_str().unwrap();
        let file_extention = file.extension().unwrap().to_str().unwrap();
        match file_extention {
            "wav" => {
                let wav_file = WavReader::open(file_string).unwrap();
                let file_spec = wav_file.spec();
                let bit_and_format: (u16, SampleFormat) =
                    (file_spec.bits_per_sample, file_spec.sample_format);
                sample_rate = file_spec.sample_rate;
                channels = file_spec.channels as u8;
                bits = match bit_and_format {
                    (16, SampleFormat::Int) => Some(Bits::Sixteen),
                    (24, SampleFormat::Int) => Some(Bits::TwentyFour),
                    (32, SampleFormat::Int) => Some(Bits::ThirtyTwoInt),
                    (32, SampleFormat::Float) => Some(Bits::ThirtyTwoFloat),
                    (64, SampleFormat::Int) => Some(Bits::SixtyFourInt),
                    (64, SampleFormat::Float) => Some(Bits::SixtyFourFloat),
                    (bit_num, SampleFormat::Int) => Some(Bits::Int(bit_num as u8)),
                    (bit_num, SampleFormat::Float) => Some(Bits::Float(bit_num as u8)),
                    _ => None,
                };
            }
            "flac" => {
                let flac_file = FlacReader::open(file_string).expect("FLAC Open Fail");
                let file_spec = flac_file.streaminfo();
                let bit_num = file_spec.bits_per_sample;
                sample_rate = file_spec.sample_rate;
                channels = file_spec.channels as u8;
                bits = match bit_num {
                    16 => Some(Bits::Sixteen),
                    24 => Some(Bits::TwentyFour),
                    bit_num => Some(Bits::Int(bit_num as u8)),
                    _ => None,
                };
            }
            "ogg" | "mp3" => {
                let buf = BufReader::new(File::open(file_string).unwrap());
                let non_bit_specific = Decoder::new(buf).unwrap();
                bits = None;
                sample_rate = non_bit_specific.sample_rate();
                channels = non_bit_specific.channels() as u8;
            }
            _ => {
                return None;
            }
        }
        let file_path = String::from(file_string);
        let audio_file = AudioFile {
            file_path,
            bits,
            sample_rate,
            channels,
        };
        Some(audio_file)
    }
    pub fn play(audio_file: &AudioFile, stream_handle: &OutputStreamHandle) {
        let buf = BufReader::new(File::open(&audio_file.file_path).unwrap());
        let audio_src = Decoder::new(buf).unwrap();

        stream_handle.play_raw(audio_src.convert_samples());
    }
}
