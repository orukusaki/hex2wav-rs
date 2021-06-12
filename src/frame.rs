use std::iter;
use crc::{Crc, CRC_16_XMODEM};

const PROGCOMMAND :u8 = 2;
const RUNCOMMAND  :u8 = 3;

pub struct Frame {
    command: u8,
    page_index: u16,
    page: Vec<u8>,
}

impl From<Frame> for Vec<u8> {
    fn from(f: Frame) -> Vec<u8> {
        let mut v = vec![0,0,0,1];
        v.push(f.command);
        v.push((f.page_index & 0xff) as u8);
        v.push((f.page_index >> 8) as u8);
        v.append(&mut f.page.to_vec());

        let crc = Crc::<u16>::new(&CRC_16_XMODEM);
        let checksum = crc.checksum(&v[4..]);

        v.push((checksum & 0xff) as u8);
        v.push((checksum >> 8) as u8);
        v
    }
}

impl Frame {
    pub fn bytes_to_frames(bytes: &[u8], frame_size: usize) -> Vec<Frame> {
        bytes
            .chunks(frame_size)
            .enumerate()
            .map(|(frame_number, chunk)| Frame::data(frame_number as u16, chunk, frame_size))
            .chain(iter::once(Frame::run(frame_size)))
            .collect()
    }

    fn data(page_index: u16, bytes: &[u8], frame_size: usize) -> Frame {

        let mut page = bytes.to_vec();
        while page.len() < frame_size {
            page.push(0);
        }

        Frame {
            command: PROGCOMMAND,
            page_index: page_index,
            page: page,
        }
    }

    fn run(frame_size: usize) -> Frame {
        Frame {
            command: RUNCOMMAND,
            page_index: 0,
            page: vec![0; frame_size],
        }
    }
}
