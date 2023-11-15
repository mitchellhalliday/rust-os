use core::arch::asm;

pub struct SerialPort {
    port: u16,
}

impl SerialPort {
    pub const unsafe fn new(port: u16) -> Self {
        Self { port }
    }

    pub fn init(&mut self) {
        unsafe {
            // Enable DLAB (set baud rate divisor)
            asm!("out dx, al", in("dx") self.port + 3, in("al") 0x80 as u8);

            // Set divisor to 3 (lo byte) 38400 baud
            asm!("out dx, al", in("dx") self.port + 0, in("al") 0x03 as u8);

            //                  (hi byte)
            asm!("out dx, al", in("dx") self.port + 1, in("al") 0x00 as u8);

            // 8 bits, no parity, one stop bit
            asm!("out dx, al", in("dx") self.port + 3, in("al") 0x03 as u8);

            // Enable FIFO, clear them, with 14-byte threshold
            asm!("out dx, al", in("dx") self.port + 2, in("al") 0xC7 as u8);

            // IRQs enabled, RTS/DSR set
            asm!("out dx, al", in("dx") self.port + 4, in("al") 0x0B as u8);
        }
    }

    pub fn write(&mut self, data: u8) {
        while !self.is_transmit_empty() {}

        unsafe {
            asm!("out dx, al", in("dx") self.port, in("al") data);
        }
    }

    pub fn read(&mut self) -> u8 {
        while !self.is_data_available() {}

        let mut data: u8;

        unsafe {
            asm!("in al, dx", out("al") data, in("dx") self.port);
        }

        data
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            self.write(byte);
        }
    }

    fn is_transmit_empty(&self) -> bool {
        let mut status: u8;

        unsafe {
            asm!("in al, dx", out("al") status, in("dx") self.port + 5);
        }

        status & 0x20 != 0
    }

    fn is_data_available(&self) -> bool {
        let mut status: u8;

        unsafe {
            asm!("in al, dx", out("al") status, in("dx") self.port + 5);
        }

        status & 0x01 != 0
    }
}
