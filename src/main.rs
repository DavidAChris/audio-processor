mod app_args;
mod logger;
mod wav;

use app_args::Cli;
use clap::Parser;
use log::info;
use logger::LoggerConfig;
use std::fs::{self, File, OpenOptions};
use std::io::Write;
use wav::{Header, Wav, WriteWav};

// const FILE_PATH: &str = "data/wav/623740-1-aud-20221022-030644.wav";
// const FILE_PATH: &str = "data/wav/623824-4-aud-20221025-153812.wav";
// 1.3 Gb file
// const FILE_PATH: &str = "data/wav/623851-2-aud-20221024-154008.wav";

fn main() {
    let args = Cli::parse();
    let log_level = match args.debug {
        0 => log::LevelFilter::Info,
        _ => log::LevelFilter::Debug,
    };
    let log_file = match args.no_log_file {
        0 => String::from(&args.log_file),
        _ => String::new(),
    };
    let log = LoggerConfig::new("[%Y-%m-%d %H:%M:%S]", log_level, log_file);
    log.set_logger();

    let file = File::open(&args.file).unwrap();
    let meta = fs::metadata(&args.file).unwrap();
    let mut wav = Wav::new(file, meta);
    let mut headers = wav.header();
    let body = wav.body();
    info!("{:?}", headers);
    info!("Size of each sample: {:?}", headers.size_of_each_sample());
    info!(
        "Current Signature Duration: {}",
        headers.approx_wav_duration()
    );
    info!("Number Of Samples: {:?}", headers.number_of_samples());
    let is_correct = headers.size_of_each_sample() / headers.num_of_channels as i32;
    let cross = is_correct * headers.num_of_channels as i32;
    info!(
        "Valid Wav File: {}",
        (cross == headers.size_of_each_sample())
    );
    let decode_body: Vec<u8> = remove_silence(&headers, body, args.accuracy);
    info!(
        "New Sig Length: {}, Previous Sig Length: {}",
        decode_body.len(),
        body.len()
    );
    headers.data_chunk_size_bytes = decode_body.len() as u32;
    headers.overall_file_size_bytes = (decode_body.len() + 36) as u32;
    info!("New Signature Duration: {}", headers.approx_wav_duration());
    info!("Overall File Size: {}", headers.overall_file_size_bytes);
    let new_wav = WriteWav::new(headers, decode_body);
    let out_bits = new_wav.construct_wav();
    let mut out_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open("new_wav.wav")
        .unwrap();
    info!("Writing New Wav File");
    out_file.write_all(out_bits.as_slice()).unwrap();
    info!("Completed");
}

fn remove_silence(headers: &Header, body: &[u8], accuracy: i32) -> Vec<u8> {
    let duration = headers.approx_wav_duration() as i32 * accuracy;
    let mut signature: Vec<u8> = Vec::<u8>::with_capacity(body.len());
    for second in 0..=duration {
        let section: (usize, usize, bool) = if second != duration {
            validate_signature(body, second, false, accuracy)
        } else {
            validate_signature(body, second, true, accuracy)
        };
        if section.2 && second != duration {
            signature.extend_from_slice(&body[section.0..section.1]);
        } else if section.2 && second == duration {
            signature.extend_from_slice(&body[section.0..]);
        }
    }
    signature
}

fn validate_signature(body: &[u8], second: i32, end: bool, accuracy: i32) -> (usize, usize, bool) {
    let seek_offset = 192000 / accuracy;
    let sig_capacity = (seek_offset / 4) as usize;
    let mut decode_body: Vec<i16> = Vec::with_capacity(sig_capacity);
    let mut seek = (second * seek_offset) as usize;
    let stop = if end {
        body.len()
    } else {
        seek + seek_offset as usize
    };
    loop {
        if seek == stop {
            break;
        }
        let section = &body[seek..seek + 2];
        let decoded = Wav::wav_i16_le(section);
        decode_body.push(decoded);
        seek += 4;
    }
    let start = (second * seek_offset) as usize;
    if !check_min_max(decode_body) {
        (start, seek, false)
    } else {
        (start, seek, true)
    }
}

fn check_min_max(wav_bytes: Vec<i16>) -> bool {
    let maxi = wav_bytes.iter().max().unwrap();
    let mini = wav_bytes.iter().min().unwrap();
    !(mini > &-5 && maxi < &5)
}
