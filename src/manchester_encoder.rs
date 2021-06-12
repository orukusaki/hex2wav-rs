pub struct ManchesterEncoder {
    state: u8,
}

impl ManchesterEncoder {

    pub fn new() -> ManchesterEncoder {
        ManchesterEncoder {state: 0}
    }

    pub fn encode(&mut self, byte: u8) -> Vec<u8> {
        let mut v = Vec::new();
        let mut b:u8 = byte;

        for _ in 0..8 {
            self.state = !self.state;
            v.push(self.state);
            v.push(self.state);

            if b & 0x80 != 0 {
                self.state = !self.state;
            }

            v.push(self.state);
            v.push(self.state);

            b = b << 1;
        }

        v
    }

    pub fn stop(&mut self) -> Vec<u8> {

        self.state = !self.state;

        let mut v = Vec::new();
        v.push(self.state);
        v.push(self.state);
        v
    }
}
