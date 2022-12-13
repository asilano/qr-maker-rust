pub enum EncodingModes {
    Numeric,        // 0-9
    AlphaNumeric,   // 0-9, A-Z (ucase), sp, $%*+-./:
    Byte,           // 0x00-0xFF
    Kanji,          // Shift-JIS
    Dynamic
}
