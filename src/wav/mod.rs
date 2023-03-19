use log::debug;
use std::fs::{File, Metadata};
use std::io::Read;
use std::ops::Range;

#[derive(Debug)]
pub(crate) struct Wav {
    buffer: Vec<u8>,
}

#[derive(Debug)]
pub(crate) struct Header {
    pub(crate) riff: String,
    pub(crate) overall_file_size_bytes: u32,
    pub(crate) wave_chunk: String,
    pub(crate) fmt_chunk: String,
    pub(crate) fmt_chunk_len: u32,
    pub(crate) fmt_type: u32,
    pub(crate) num_of_channels: u32,
    pub(crate) sample_rate: u32,
    pub(crate) byte_rate: u32,
    pub(crate) block_alignment: u32,
    pub(crate) bits_per_sample: u32,
    pub(crate) data_chunk: String,
    pub(crate) data_chunk_size_bytes: u32,
}

impl Wav {
    pub(crate) fn new(mut file: File, metadata: Metadata) -> Wav {
        let mut buffer = vec![0; metadata.len() as usize];
        let _ = file.read(&mut buffer).unwrap();
        Wav { buffer }
    }

    pub fn header(&mut self) -> Header {
        Header {
            riff: Wav::create_utf8_string(self.buf_slice(0..4)),
            overall_file_size_bytes: Wav::wav_u32_le(self.buf_slice(4..8)),
            wave_chunk: Wav::create_utf8_string(self.buf_slice(8..12)),
            fmt_chunk: Wav::create_utf8_string(self.buf_slice(12..16)),
            fmt_chunk_len: Wav::wav_u32_le(self.buf_slice(16..20)),
            fmt_type: Wav::wav_u32_le(self.buf_slice(20..22)),
            num_of_channels: Wav::wav_u32_le(self.buf_slice(22..24)),
            sample_rate: Wav::wav_u32_le(self.buf_slice(24..28)),
            byte_rate: Wav::wav_u32_le(self.buf_slice(28..32)),
            block_alignment: Wav::wav_u32_le(self.buf_slice(32..34)),
            bits_per_sample: Wav::wav_u32_le(self.buf_slice(34..36)),
            data_chunk: Wav::create_utf8_string(self.buf_slice(36..40)),
            data_chunk_size_bytes: Wav::wav_u32_le(self.buf_slice(40..44)),
        }
    }

    pub(crate) fn body(&mut self) -> &[u8] {
        &self.buffer[44..]
    }

    fn create_utf8_string(data: &[u8]) -> String {
        String::from_utf8(data.to_vec()).unwrap()
    }

    fn buf_slice(&mut self, range: Range<usize>) -> &[u8] {
        let data: &[u8] = &self.buffer[range];
        data
    }

    pub(crate) fn wav_u32_le(data: &[u8]) -> u32 {
        let mut last_bit = 0;
        let conversion = [0, 8, 16, 24];
        for (i, bit) in data.iter().enumerate() {
            last_bit += (*bit as u32) << conversion[i];
        }
        last_bit
    }

    pub(crate) fn wav_i16_le(data: &[u8]) -> i16 {
        let mut last_bit = 0;
        let conversion = [0, 8, 16, 24];
        for (i, bit) in data.iter().enumerate() {
            last_bit += (*bit as i16) << conversion[i];
        }
        last_bit
    }

    // fn wav_u32_be(data: &[u8]) -> u32 {
    //     let mut last_bit = 0;
    //     let conversion = [24, 16, 8, 0];
    //     for (i, bit) in data.iter().enumerate() {
    //         last_bit += (*bit as u32) << conversion[i];
    //     }
    //     last_bit
    // }
}

impl Header {
    pub(crate) fn number_of_samples(&self) -> i32 {
        let numerator = 8 * (self.data_chunk_size_bytes as u64);
        let denominator = (self.num_of_channels * self.bits_per_sample) as u64;
        (numerator / denominator) as i32
    }

    pub(crate) fn size_of_each_sample(&self) -> i32 {
        ((self.num_of_channels * self.bits_per_sample) / 8) as i32
    }

    pub(crate) fn approx_wav_duration(&self) -> f32 {
        self.data_chunk_size_bytes as f32 / self.byte_rate as f32
    }
}

pub(crate) struct WriteWav {
    header: Header,
    body: Vec<u8>,
}

impl WriteWav {
    pub(crate) fn new(header: Header, body: Vec<u8>) -> WriteWav {
        WriteWav { header, body }
    }

    pub(crate) fn construct_wav(self) -> Vec<u8> {
        let header_pattern = [
            self.header.riff.into_bytes(),
            self.header.overall_file_size_bytes.to_le_bytes().to_vec(),
            self.header.wave_chunk.into_bytes(),
            self.header.fmt_chunk.into_bytes(),
            self.header.fmt_chunk_len.to_le_bytes().to_vec(),
            self.header.fmt_type.to_le_bytes().to_vec()[0..2].to_vec(),
            self.header.num_of_channels.to_le_bytes().to_vec()[0..2].to_vec(),
            self.header.sample_rate.to_le_bytes().to_vec(),
            self.header.byte_rate.to_le_bytes().to_vec(),
            self.header.block_alignment.to_le_bytes().to_vec()[0..2].to_vec(),
            self.header.bits_per_sample.to_le_bytes().to_vec()[0..2].to_vec(),
            self.header.data_chunk.into_bytes(),
            self.header.data_chunk_size_bytes.to_le_bytes().to_vec(),
        ];
        let capacity = self.body.len();
        let mut header_bits = Vec::<u8>::with_capacity(capacity + 44);
        for section in header_pattern {
            header_bits.extend(section);
        }
        header_bits.extend(self.body);
        debug!("Total Byte Length: {}", header_bits.len());
        header_bits
    }
}
