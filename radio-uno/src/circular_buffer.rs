#[derive(Debug)]
pub struct BufferNotReadyError;

pub struct CircularBitBuffer {
    /// Contains 128 bits
    buffer: [u8; Self::BUFFER_SIZE],

    /// The next bit to be written, from 0 (MSB of buffer[0]) to 127 (LSB of buffer[15])
    head: usize,

    /// The next bit to be read, from 0 (MSB of buffer[0]) to 127 (LSB of buffer[15])
    tail: usize,

    /// The number of bits in the buffer, from 0 to 128
    size: usize,
}

impl CircularBitBuffer {
    const BUFFER_SIZE: usize = 16;
    const BIT_BUFFER_SIZE: usize = Self::BUFFER_SIZE * 8;

    pub fn new() -> Self {
        CircularBitBuffer {
            buffer: [0; Self::BUFFER_SIZE],
            head: 0,
            tail: 0,
            size: 0,
        }
    }

    pub fn write_ready(&self) -> bool {
        self.size < Self::BIT_BUFFER_SIZE
    }

    pub fn write_bit(&mut self, bit: bool) -> Result<(), BufferNotReadyError> {
        if !self.write_ready() {
            return Err(BufferNotReadyError);
        }

        let buf_index: usize = (self.head / 8).into();
        let bit_index = 7 - (self.head % 8);
        let old_byte = self.buffer[buf_index];
        let set_byte = if bit { 1u8 << bit_index } else { 0u8 };
        let new_byte = (old_byte & !(1u8 << bit_index)) | set_byte;
        self.buffer[buf_index] = new_byte;
        self.head = (self.head + 1) % Self::BIT_BUFFER_SIZE;
        self.size += 1;
        Ok(())
    }

    pub fn read_ready(&self) -> bool {
        self.size > 0
    }

    pub fn read_bit(&mut self) -> Result<bool, BufferNotReadyError> {
        if !self.read_ready() {
            return Err(BufferNotReadyError)
        }

        let buf_index: usize = (self.tail / 8).into();
        let bit_index = 7 - (self.tail % 8);
        let bit = (self.buffer[buf_index] & (1u8 << bit_index)) >> bit_index;
        self.tail = (self.tail + 1) % Self::BIT_BUFFER_SIZE;
        self.size -= 1;
        Ok(bit == 1)
    }
}