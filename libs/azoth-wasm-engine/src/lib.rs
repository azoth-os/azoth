use crate::decode::Decoder;

pub mod decode;

pub enum SectionId {
    Custom = 0,
    Type = 1,
    Import = 2,
    Function = 3,
    Table = 4,
    Memory = 5,
    Global = 6,
    Export = 7,
    Start = 8,
    Element = 9,
    Code = 10,
    Data = 11,
    Unknown,
}

impl From<u8> for SectionId {
    fn from(id: u8) -> Self {
        match id {
            0 => SectionId::Custom,
            1 => SectionId::Type,
            2 => SectionId::Import,
            3 => SectionId::Function,
            7 => SectionId::Export,
            10 => SectionId::Code,
            _ => SectionId::Unknown,
        }
    }
}

pub fn parse_module(data: &[u8]) {
    let mut decoder = Decoder::new(data);

    // 1. Valider le Header
    let magic = [
        decoder.read_byte(),
        decoder.read_byte(),
        decoder.read_byte(),
        decoder.read_byte(),
    ];
    let version = decoder.read_u32();

    if &magic != b"\0asm" || version != 1 {
        panic!("Format Wasm invalide ou version non supportée");
    }

    // 2. Boucle des Sections
    while decoder.ptr < data.len() {
        let id = SectionId::from(decoder.read_byte());
        let section_size = decoder.read_u32();
        let section_end = decoder.ptr + section_size as usize;

        match id {
            SectionId::Type => {
                // TODO: Parser les signatures de fonctions
                println!("Parsing Type Section...");
            }
            SectionId::Export => {
                // TODO: Identifier les points d'entrée (ex: "main")
                println!("Parsing Export Section...");
            }
            SectionId::Code => {
                // C'EST ICI QUE LA TRADUCTION WASM3 COMMENCE
                println!("Parsing Code Section...");
            }
            _ => {
                // On saute les sections non gérées pour l'instant
                decoder.ptr = section_end;
            }
        }

        // Sécurité : on s'assure de ne pas dépasser la section
        decoder.ptr = section_end;
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

    #[test]
    fn test_minimal_module_parsing() {
        // Header + Section Type (ID 1) de taille 0
        let data = [0x00, 0x61, 0x73, 0x6D, 0x01, 0x00, 0x00, 0x00, 0x01, 0x00];
        parse_module(&data); // Ne doit pas paniquer
    }
}
