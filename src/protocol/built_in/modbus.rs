//! Modbus protocol implementation

use crate::error::{ProtocolError, Result, SerialError};
use crate::protocol::{Protocol, ProtocolStats};

/// Modbus protocol handler
#[derive(Clone)]
pub struct ModbusProtocol {
    mode: ModbusMode,
    stats: ProtocolStats,
}

/// Modbus mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ModbusMode {
    /// RTU mode (binary)
    Rtu,
    /// ASCII mode (text)
    Ascii,
}

impl ModbusProtocol {
    /// Create a new Modbus protocol
    pub fn new(mode: ModbusMode) -> Self {
        Self {
            mode,
            stats: ProtocolStats::default(),
        }
    }

    /// Calculate CRC16 for Modbus RTU
    pub fn calculate_crc(data: &[u8]) -> u16 {
        let mut crc: u16 = 0xFFFF;
        for &byte in data {
            crc ^= byte as u16;
            for _ in 0..8 {
                if crc & 0x0001 != 0 {
                    crc = (crc >> 1) ^ 0xA001;
                } else {
                    crc >>= 1;
                }
            }
        }
        crc
    }

    /// Calculate LRC for Modbus ASCII
    pub fn calculate_lrc(data: &[u8]) -> u8 {
        let mut lrc: u8 = 0;
        for &byte in data {
            lrc = lrc.wrapping_add(byte);
        }
        (!lrc).wrapping_add(1)
    }

    /// Encode Modbus request
    pub fn encode_request(&self, slave_id: u8, function_code: u8, data: &[u8]) -> Result<Vec<u8>> {
        let mut pdu = Vec::with_capacity(2 + data.len());
        pdu.push(slave_id);
        pdu.push(function_code);
        pdu.extend_from_slice(data);

        match self.mode {
            ModbusMode::Rtu => {
                let mut frame = Vec::with_capacity(pdu.len() + 2);
                frame.extend_from_slice(&pdu);
                let crc = Self::calculate_crc(&pdu);
                frame.extend_from_slice(&crc.to_le_bytes());
                Ok(frame)
            }
            ModbusMode::Ascii => {
                // Pre-allocate: 1 (:) + pdu.len()*2 (hex) + 2 (LRC) + 2 (\r\n)
                let mut ascii = Vec::with_capacity(1 + pdu.len() * 2 + 4);
                ascii.push(b':'); // Start delimiter

                for byte in &pdu {
                    ascii.extend_from_slice(&Self::byte_to_hex(*byte));
                }

                let lrc = Self::calculate_lrc(&pdu);
                ascii.extend_from_slice(&Self::byte_to_hex(lrc));

                ascii.extend_from_slice(b"\r\n"); // End delimiter
                Ok(ascii)
            }
        }
    }

    /// Parse Modbus response
    pub fn parse_response(&mut self, data: &[u8]) -> Result<(u8, u8, Vec<u8>)> {
        match self.mode {
            ModbusMode::Rtu => {
                if data.len() < 4 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "RTU frame too short".to_string(),
                    )));
                }

                let slave_id = data[0];
                let function_code = data[1];

                // Verify CRC
                let received_crc = u16::from_le_bytes([data[data.len() - 2], data[data.len() - 1]]);
                let calculated_crc = Self::calculate_crc(&data[..data.len() - 2]);

                if received_crc != calculated_crc {
                    return Err(SerialError::Protocol(ProtocolError::ChecksumFailed {
                        expected: format!("{:04X}", calculated_crc),
                        got: format!("{:04X}", received_crc),
                    }));
                }

                let pdu_data = data[2..data.len() - 2].to_vec();
                Ok((slave_id, function_code, pdu_data))
            }
            ModbusMode::Ascii => {
                // Parse ASCII mode frame
                // Format: :LLMMTT...CC\r\n
                // Where: L=Leader, MM=bytes, TT=data, CC=LRC

                // Check minimum frame size
                if data.len() < 3 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame too short".to_string(),
                    )));
                }

                // Check start delimiter
                if data[0] != b':' {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame must start with ':'".to_string(),
                    )));
                }

                // Find end delimiter
                let end_pos = data.iter().position(|&b| b == b'\r').ok_or_else(|| {
                    SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame missing CR delimiter".to_string(),
                    ))
                })?;

                if end_pos < 3 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame too short after start delimiter".to_string(),
                    )));
                }

                // Extract hex data (excluding : and \r\n)
                let hex_data = &data[1..end_pos];

                // Must have even number of hex digits
                if !hex_data.len().is_multiple_of(2) {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame must have even number of hex digits".to_string(),
                    )));
                }

                // Convert hex to bytes
                let byte_len = hex_data.len() / 2;
                let mut bytes = Vec::with_capacity(byte_len);
                for i in (0..hex_data.len()).step_by(2) {
                    let byte_str = &hex_data[i..i + 2];
                    let byte = Self::hex_to_byte(byte_str)?;
                    bytes.push(byte);
                }

                // Verify LRC (last byte)
                if bytes.len() < 2 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame missing LRC".to_string(),
                    )));
                }

                let lrc_index = bytes.len() - 1;
                let received_lrc = bytes[lrc_index];
                let data_for_lrc = &bytes[..lrc_index];
                let calculated_lrc = Self::calculate_lrc(data_for_lrc);

                if received_lrc != calculated_lrc {
                    self.stats.errors += 1;
                    return Err(SerialError::Protocol(ProtocolError::ChecksumFailed {
                        expected: format!("{:02X}", calculated_lrc),
                        got: format!("{:02X}", received_lrc),
                    }));
                }

                // Return slave_id, function_code, and pdu_data
                let slave_id = bytes[0];
                let function_code = bytes[1];
                let pdu_data = bytes[2..lrc_index].to_vec();

                Ok((slave_id, function_code, pdu_data))
            }
        }
    }

    fn byte_to_hex(byte: u8) -> [u8; 2] {
        let hex = b"0123456789ABCDEF";
        [hex[(byte >> 4) as usize], hex[(byte & 0x0F) as usize]]
    }

    /// Convert two hex characters to a byte
    fn hex_to_byte(hex: &[u8]) -> Result<u8> {
        if hex.len() != 2 {
            return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                "Hex must be exactly 2 characters".to_string(),
            )));
        }

        let high = Self::char_to_nibble(hex[0])?;
        let low = Self::char_to_nibble(hex[1])?;

        Ok((high << 4) | low)
    }

    /// Convert a hex character to its nibble value
    fn char_to_nibble(c: u8) -> Result<u8> {
        match c {
            b'0'..=b'9' => Ok(c - b'0'),
            b'A'..=b'F' => Ok(c - b'A' + 10),
            b'a'..=b'f' => Ok(c - b'a' + 10),
            _ => Err(SerialError::Protocol(ProtocolError::InvalidFrame(format!(
                "Invalid hex character: {}",
                c as char
            )))),
        }
    }
}

impl Protocol for ModbusProtocol {
    fn name(&self) -> &str {
        match self.mode {
            ModbusMode::Rtu => "modbus_rtu",
            ModbusMode::Ascii => "modbus_ascii",
        }
    }

    fn parse(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        self.stats.frames_parsed += 1;

        match self.mode {
            ModbusMode::Rtu => {
                if data.len() < 4 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "Frame too short".to_string(),
                    )));
                }

                // Verify CRC
                let received_crc = u16::from_le_bytes([data[data.len() - 2], data[data.len() - 1]]);
                let calculated_crc = Self::calculate_crc(&data[..data.len() - 2]);

                if received_crc != calculated_crc {
                    self.stats.errors += 1;
                    return Err(SerialError::Protocol(ProtocolError::ChecksumFailed {
                        expected: format!("{:04X}", calculated_crc),
                        got: format!("{:04X}", received_crc),
                    }));
                }

                Ok(data[..data.len() - 2].to_vec())
            }
            ModbusMode::Ascii => {
                // Parse ASCII mode frame and return decoded data
                // Format: :LLMMTT...CC\r\n

                // Check minimum frame size
                if data.len() < 3 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame too short".to_string(),
                    )));
                }

                // Check start delimiter
                if data[0] != b':' {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame must start with ':'".to_string(),
                    )));
                }

                // Find end delimiter
                let end_pos = data.iter().position(|&b| b == b'\r').ok_or_else(|| {
                    SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame missing CR delimiter".to_string(),
                    ))
                })?;

                if end_pos < 3 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame too short after start delimiter".to_string(),
                    )));
                }

                // Extract hex data (excluding : and \r\n)
                let hex_data = &data[1..end_pos];

                // Must have even number of hex digits
                if !hex_data.len().is_multiple_of(2) {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame must have even number of hex digits".to_string(),
                    )));
                }

                // Convert hex to bytes
                let byte_len = hex_data.len() / 2;
                let mut bytes = Vec::with_capacity(byte_len);
                for i in (0..hex_data.len()).step_by(2) {
                    let byte_str = &hex_data[i..i + 2];
                    let byte = Self::hex_to_byte(byte_str)?;
                    bytes.push(byte);
                }

                // Verify LRC (last byte)
                if bytes.len() < 2 {
                    return Err(SerialError::Protocol(ProtocolError::InvalidFrame(
                        "ASCII frame missing LRC".to_string(),
                    )));
                }

                let lrc_index = bytes.len() - 1;
                let received_lrc = bytes[lrc_index];
                let data_for_lrc = &bytes[..lrc_index];
                let calculated_lrc = Self::calculate_lrc(data_for_lrc);

                if received_lrc != calculated_lrc {
                    self.stats.errors += 1;
                    return Err(SerialError::Protocol(ProtocolError::ChecksumFailed {
                        expected: format!("{:02X}", calculated_lrc),
                        got: format!("{:02X}", received_lrc),
                    }));
                }

                // Return decoded data (excluding LRC)
                Ok(data_for_lrc.to_vec())
            }
        }
    }

    fn encode(&mut self, data: &[u8]) -> Result<Vec<u8>> {
        self.stats.frames_encoded += 1;

        match self.mode {
            ModbusMode::Rtu => {
                let mut frame = Vec::with_capacity(data.len() + 2);
                frame.extend_from_slice(data);
                let crc = Self::calculate_crc(data);
                frame.extend_from_slice(&crc.to_le_bytes());
                Ok(frame)
            }
            ModbusMode::Ascii => {
                // Pre-allocate: 1 (:) + data.len()*2 (hex) + 2 (LRC) + 2 (\r\n)
                let mut ascii = Vec::with_capacity(1 + data.len() * 2 + 4);
                ascii.push(b':'); // Start delimiter

                // Convert data to hex
                for byte in data {
                    ascii.extend_from_slice(&Self::byte_to_hex(*byte));
                }

                // Calculate and add LRC
                let lrc = Self::calculate_lrc(data);
                ascii.extend_from_slice(&Self::byte_to_hex(lrc));

                // Add end delimiter
                ascii.extend_from_slice(b"\r\n");

                Ok(ascii)
            }
        }
    }

    fn stats(&self) -> ProtocolStats {
        self.stats.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crc_calculation() {
        // Test consistency - encode and decode should work together
        let mut protocol = ModbusProtocol::new(ModbusMode::Rtu);
        let test_data = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x0A];

        // Encode adds CRC
        let encoded = protocol.encode(&test_data).unwrap();
        assert!(encoded.len() > test_data.len());

        // Parse removes and verifies CRC
        let decoded = protocol.parse(&encoded).unwrap();
        assert_eq!(decoded, test_data);

        // Verify that different data produces different CRCs
        let different_data = vec![0x01, 0x03, 0x00, 0x00, 0x00, 0x0B];
        let different_encoded = protocol.encode(&different_data).unwrap();
        assert_ne!(encoded, different_encoded);
    }

    #[test]
    fn test_modbus_rtu_parse() {
        let mut protocol = ModbusProtocol::new(ModbusMode::Rtu);

        // Test with data that's too short
        let short_data = [0x01, 0x03];
        assert!(protocol.parse(&short_data).is_err());

        // Test with valid frame - create frame and verify it can be parsed
        let original_data = vec![0x01, 0x03, 0x02, 0x00, 0x0A];
        let encoded = protocol.encode(&original_data).unwrap();
        let decoded = protocol.parse(&encoded).unwrap();

        // After parsing, CRC should be removed
        assert_eq!(decoded, original_data);
    }

    #[test]
    fn test_modbus_stats() {
        let mut protocol = ModbusProtocol::new(ModbusMode::Rtu);
        let data = vec![0x01, 0x03, 0x00, 0x00];

        // Encode first (adds CRC)
        let encoded = protocol.encode(&data).unwrap();
        assert_eq!(protocol.stats().frames_encoded, 1);

        // Parse the encoded data (validates CRC)
        let decoded = protocol.parse(&encoded).unwrap();
        assert_eq!(protocol.stats().frames_parsed, 1);
        assert_eq!(decoded, data);
    }

    #[test]
    fn test_modbus_ascii_encode() {
        let mut protocol = ModbusProtocol::new(ModbusMode::Ascii);
        let data = vec![0x01, 0x03, 0x00, 0x00];

        // Encode should produce ASCII frame with : prefix and \r\n suffix
        let encoded = protocol.encode(&data).unwrap();

        // Check start delimiter
        assert_eq!(encoded[0], b':');

        // Check end delimiter
        assert_eq!(encoded[encoded.len() - 2], b'\r');
        assert_eq!(encoded[encoded.len() - 1], b'\n');

        // Length should be: 1 (:) + data.len()*2 (hex) + 2 (LRC) + 2 (\r\n)
        let expected_len = 1 + data.len() * 2 + 2 + 2;
        assert_eq!(encoded.len(), expected_len);
    }

    #[test]
    fn test_modbus_ascii_parse() {
        let mut protocol = ModbusProtocol::new(ModbusMode::Ascii);
        let original_data = vec![0x01, 0x03, 0x02, 0x00, 0x0A];

        // Encode the data
        let encoded = protocol.encode(&original_data).unwrap();

        // Parse it back
        let decoded = protocol.parse(&encoded).unwrap();

        // Should match original
        assert_eq!(decoded, original_data);
    }

    #[test]
    fn test_modbus_ascii_lrc() {
        // Test LRC calculation
        let data = vec![0x01, 0x03, 0x00, 0x00];

        // Calculate LRC
        let lrc = ModbusProtocol::calculate_lrc(&data);

        // LRC should be non-zero for non-empty data
        assert_ne!(lrc, 0);

        // LRC of data with its LRC should be 0
        let mut data_with_lrc = data.clone();
        data_with_lrc.push(lrc);
        assert_eq!(ModbusProtocol::calculate_lrc(&data_with_lrc), 0);
    }

    #[test]
    fn test_hex_conversion() {
        // Test byte_to_hex
        assert_eq!(ModbusProtocol::byte_to_hex(0x00), [b'0', b'0']);
        assert_eq!(ModbusProtocol::byte_to_hex(0x0F), [b'0', b'F']);
        assert_eq!(ModbusProtocol::byte_to_hex(0xF0), [b'F', b'0']);
        assert_eq!(ModbusProtocol::byte_to_hex(0xFF), [b'F', b'F']);

        // Test hex_to_byte
        assert_eq!(ModbusProtocol::hex_to_byte(b"00").unwrap(), 0x00);
        assert_eq!(ModbusProtocol::hex_to_byte(b"0F").unwrap(), 0x0F);
        assert_eq!(ModbusProtocol::hex_to_byte(b"F0").unwrap(), 0xF0);
        assert_eq!(ModbusProtocol::hex_to_byte(b"FF").unwrap(), 0xFF);
        assert_eq!(ModbusProtocol::hex_to_byte(b"ff").unwrap(), 0xFF);
    }
}
