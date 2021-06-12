mod manchester_encoder;
mod frame;
pub mod lpf;

pub use manchester_encoder::ManchesterEncoder;
pub use frame::Frame;

use ihex::{Reader, Record};

pub struct Options {
    pub in_filename: String,
    pub out_filename: String,
    pub frame_size: u16,
    pub cuttoff: u32,
    pub sample_rate: u32,
}

pub fn create_audio_data(content: String, options: &Options) -> Vec<u8> {
    let hex_reader = Reader::new(&content);

    let firmware_bytes = hex_reader
        .into_iter()
        .flat_map(|r| r.ok())
        .fold(Vec::new(), |mut v: Vec<u8>, r| {

            if let Record::Data{offset, mut value} = r {
                while (v.len() as u16) < offset {
                    v.push(0u8);
                }
                v.append(&mut value);
            }

            v
        });

    let frames = Frame::bytes_to_frames(&firmware_bytes, options.frame_size.into());

    let encoder = ManchesterEncoder::new();
    let audio_bytes = frames
        .into_iter()
        .scan(encoder, |encoder, frame|
            {
                let mut encoded = vec![127; 500]; // silence

                let frame_bytes: Vec<u8> = frame.into();
                for byte in frame_bytes.into_iter()
                {
                    encoded.append(&mut encoder.encode(byte));
                }
                encoded.append(&mut encoder.stop());
                Some(encoded)
            }
        )
        .flatten();

    let filtered_audio = lpf::filter(audio_bytes, options.cuttoff as f32, options.sample_rate as f32);
    filtered_audio.map(|s| s as u8).collect()
}
