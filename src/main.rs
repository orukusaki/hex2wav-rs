use std::fs;
use std::fs::File;
use std::path::Path;

use wav::Header;
use wav::header::WAV_FORMAT_PCM;
use wav::bit_depth::BitDepth;

use mencoder::create_audio_data;
use mencoder::Options;

#[macro_use]
extern crate clap;
use clap::App;

fn main() {

    let options = get_options()
        .unwrap_or_else(|e| {
            println!("{}", e);
            std::process::exit(1);
        });

    let content = fs::read_to_string(&options.in_filename)
        .expect("Could not load file");
    let mut out_file = File::create(Path::new(&options.out_filename))
        .expect("could not open output file");

    let header = Header::new(WAV_FORMAT_PCM, 1, options.sample_rate, 8);
    let data = BitDepth::from(create_audio_data(content, &options));

    wav::write(header, &data, &mut out_file)
        .expect("could not write wav");
}

fn get_options() -> Result<Options, String>
{
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let in_filename = matches.value_of("INPUT_FILE").unwrap().to_string();
    let out_filename = matches.value_of("OUTPUT_FILE").unwrap_or("output.wav").to_string();

    let frame_size = matches
        .value_of("frame_size")
        .unwrap()
        .parse::<u16>()
        .map_err(|_|  "Frame size should be a number")?;

    let cuttoff = matches
        .value_of("cuttoff")
        .unwrap()
        .parse::<u32>()
        .map_err(|_|  "Cuttoff frequency should be a number")?;

    let sample_rate = matches.value_of("sample_rate")
        .unwrap()
        .parse::<u32>()
        .map_err(|_|  "Sample rate should be a number")?;

    Ok(
        Options {
            in_filename: in_filename,
            out_filename: out_filename,
            frame_size: frame_size,
            cuttoff: cuttoff,
            sample_rate: sample_rate,
        }
    )
}