#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum VgaColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct VgaColorCode(u8);

impl VgaColorCode {
    fn new(foreground: VgaColor, background: VgaColor) -> VgaColorCode {
        VgaColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VgaChar {
    ascii_character: u8,
    color_code: VgaColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct VgaTextBuffer {
    chars: [[VgaChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct VgaWriter {
    column_position: usize,
    color_code: VgaColorCode,
    buffer: &'static mut VgaTextBuffer,
}

impl VgaWriter {
    fn new(color_code: VgaColorCode) -> VgaWriter {
        return VgaWriter {
            column_position: 0,
            color_code,
            buffer: unsafe { &mut *(0xb8000 as *mut VgaTextBuffer) },
        };
    }

    pub fn write_byte(&mut self, byte: u8) {
        if byte == b'\n' || self.column_position >= BUFFER_WIDTH {
            self.new_line();
        } else {
            if self.column_position >= BUFFER_WIDTH {
                self.new_line();
            }

            let row: usize = BUFFER_HEIGHT - 1;
            let col: usize = self.column_position;

            let color_code: VgaColorCode = self.color_code;
            self.buffer.chars[row][col] = VgaChar {
                ascii_character: byte,
                color_code,
            };
            self.column_position += 1;
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range, so we print ■
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) { /* TODO */
    }
}

pub fn print_something() {
    let mut writer: VgaWriter =
        VgaWriter::new(VgaColorCode::new(VgaColor::Yellow, VgaColor::Black));

    writer.write_byte(b'H');
    writer.write_string("ello ");
    writer.write_string("Wörld!");
}
