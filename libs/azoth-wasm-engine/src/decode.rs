pub struct Decoder<'a> {
    pub(crate) data: &'a [u8],
    pub(crate) ptr: usize,
}

impl<'a> Decoder<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Self { data, ptr: 0 }
    }
}

impl<'a> Decoder<'a> {
    pub fn read_byte(&mut self) -> u8 {
        let byte = self.data[self.ptr];
        self.ptr += 1;
        byte
    }

    /// Lit un entier non-signé (LEB128)
    pub fn read_u32(&mut self) -> u32 {
        let mut result = 0u32;
        let mut shift = 0;
        loop {
            let byte = self.read_byte();
            result |= ((byte & 0x7F) as u32) << shift;
            if (byte & 0x80) == 0 {
                break;
            }
            shift += 7;
        }
        result
    }

    pub fn read_i32(&mut self) -> i32 {
        let mut result = 0i32;
        let mut shift = 0;
        let mut byte;
        loop {
            byte = self.read_byte();
            result |= ((byte & 0x7F) as i32) << shift;
            shift += 7;
            if (byte & 0x80) == 0 {
                break;
            }
        }
        if shift < 32 && (byte & 0x40) != 0 {
            result |= !0 << shift;
        }
        result
    }

    pub fn read_string(&mut self) -> &'a str {
        let len = self.read_u32() as usize;
        let start = self.ptr;
        self.ptr += len;
        core::str::from_utf8(&self.data[start..self.ptr]).unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_u32_leb128() {
        // 624485 en LEB128 est [0xE5, 0x8E, 0x26]
        let data = [0xE5, 0x8E, 0x26];
        let mut decoder = Decoder::new(&data);
        assert_eq!(decoder.read_u32(), 624485);
    }

    #[test]
    fn test_read_i32_negative_leb128() {
        // -624485 en LEB128 signé est [0x9B, 0xF1, 0x59]
        let data = [0x9B, 0xF1, 0x59];
        let mut decoder = Decoder::new(&data);
        assert_eq!(decoder.read_i32(), -624485);
    }

    #[test]
    fn test_read_string() {
        // Chaîne "Azoth" : Taille 5 (0x05) + bytes
        let data = [0x05, b'A', b'z', b'o', b't', b'h'];
        let mut decoder = Decoder::new(&data);
        assert_eq!(decoder.read_string(), "Azoth");
    }

    #[test]
    fn test_magic_and_version() {
        // Header standard Wasm: \0asm + version 1
        let data = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00];
        let mut decoder = Decoder::new(&data);

        let magic = [
            decoder.read_byte(),
            decoder.read_byte(),
            decoder.read_byte(),
            decoder.read_byte(),
        ];
        assert_eq!(&magic, b"\0asm");
        assert_eq!(decoder.read_u32(), 1); // Version
    }
}
