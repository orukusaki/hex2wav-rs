# hex2wav-rs
[![Build Status](https://travis-ci.com/orukusaki/hex2wav-rs.svg?branch=main)](https://travis-ci.com/orukusaki/hex2wav-rs)

Converts an intel .hex firmware file into a .wav using [Differential Manchester encoding](https://en.wikipedia.org/wiki/Differential_Manchester_encoding).

Bytes are split up into frames (128 bytes by default). Each frame starts with a synchronisation sequence and page number, followed by the frame bytes, and finally a 16 bit CRC.  A special stop frame marks the end of transmission.

## USAGE:
    hex2wav [OPTIONS] <INPUT_FILE> [OUTPUT_FILE]

    FLAGS:
        -h, --help       Prints help information
        -V, --version    Prints version information

    OPTIONS:
        -c, --cuttoff <FREQ>              low pass filter cuttoff Hz [default: 10000]
        -f, --framesize <SIZE>            Frame size in bytes [default: 128]
        -s, --samplerate <SAMPLE_RATE>    Output file sample rate [default: 48000]

    ARGS:
        <INPUT_FILE>     Input file, should be in intel .hex format
        <OUTPUT_FILE>    Output .wav file
