//! Length-prefixed framing on top of a byte stream.
//!
//! TCP provides a reliable byte stream, not a message boundary protocol.
//! `FrameCodec` turns that stream into discrete frames by prefixing each
//! payload with its length as a big-endian u32.
//!
//! Frame layout:
//!
//! ```text
//! [ 4 bytes: payload length N (big-endian) ][ N bytes: payload ]
//! ```

use crate::error::NetworkError;

/// Maximum allowed frame payload size (16 MiB).
///
/// This prevents a malicious or buggy peer from causing unbounded allocation.
pub const MAX_FRAME_SIZE: usize = 16 * 1024 * 1024;

/// Header length in bytes.
pub const HEADER_LEN: usize = 4;

/// A reusable length-prefixed frame encoder/decoder.
#[derive(Debug, Default, Clone)]
pub struct FrameCodec {
    max_frame_size: usize,
}

impl FrameCodec {
    /// Create a codec with the default maximum frame size.
    pub fn new() -> Self {
        Self {
            max_frame_size: MAX_FRAME_SIZE,
        }
    }

    /// Create a codec with a custom maximum frame size.
    pub fn with_max_frame_size(max_frame_size: usize) -> Self {
        Self { max_frame_size }
    }

    /// Encode `payload` into a length-prefixed frame.
    ///
    /// Returns an error if the payload exceeds the configured maximum.
    pub fn encode(&self, payload: &[u8]) -> Result<Vec<u8>, NetworkError> {
        let len = payload.len();
        if len > self.max_frame_size {
            return Err(NetworkError::FrameTooLarge {
                size: len,
                max: self.max_frame_size,
            });
        }

        let mut frame = Vec::with_capacity(HEADER_LEN + len);
        frame.extend_from_slice(&(len as u32).to_be_bytes());
        frame.extend_from_slice(payload);
        Ok(frame)
    }

    /// Attempt to decode one frame from the buffered bytes.
    ///
    /// Returns `Ok(Some(frame))` when a complete frame is available,
    /// `Ok(None)` when more bytes are needed, or an error for malformed data.
    ///
    /// Consumes the bytes belonging to any successfully decoded frame.
    pub fn decode(&self, buffer: &mut Vec<u8>) -> Result<Option<Vec<u8>>, NetworkError> {
        if buffer.len() < HEADER_LEN {
            return Ok(None);
        }

        let len_bytes = [buffer[0], buffer[1], buffer[2], buffer[3]];
        let payload_len = u32::from_be_bytes(len_bytes) as usize;

        if payload_len > self.max_frame_size {
            return Err(NetworkError::FrameTooLarge {
                size: payload_len,
                max: self.max_frame_size,
            });
        }

        let frame_len = HEADER_LEN + payload_len;
        if buffer.len() < frame_len {
            return Ok(None);
        }

        let payload = buffer[HEADER_LEN..frame_len].to_vec();
        buffer.drain(..frame_len);
        Ok(Some(payload))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode_roundtrip() {
        let codec = FrameCodec::new();
        let payload = b"hello, forge";
        let frame = codec.encode(payload).unwrap();
        assert_eq!(&frame[..4], &(payload.len() as u32).to_be_bytes());
        assert_eq!(&frame[4..], payload);

        let mut buffer = frame;
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();
        assert_eq!(decoded, payload);
        assert!(buffer.is_empty());
    }

    #[test]
    fn decode_partial_header() {
        let codec = FrameCodec::new();
        let mut buffer = vec![0u8, 0u8];
        assert_eq!(codec.decode(&mut buffer).unwrap(), None);
        assert_eq!(buffer.len(), 2);
    }

    #[test]
    fn decode_partial_payload() {
        let codec = FrameCodec::new();
        let mut buffer = codec.encode(b"hello").unwrap();
        buffer.pop(); // remove last byte
        assert_eq!(codec.decode(&mut buffer).unwrap(), None);
        assert_eq!(buffer.len(), 8); // 4 header + 4 payload bytes remain
    }

    #[test]
    fn decode_multiple_frames() {
        let codec = FrameCodec::new();
        let mut buffer = Vec::new();
        buffer.extend_from_slice(&codec.encode(b"one").unwrap());
        buffer.extend_from_slice(&codec.encode(b"two").unwrap());
        buffer.extend_from_slice(&codec.encode(b"three").unwrap());

        assert_eq!(codec.decode(&mut buffer).unwrap().unwrap(), b"one");
        assert_eq!(codec.decode(&mut buffer).unwrap().unwrap(), b"two");
        assert_eq!(codec.decode(&mut buffer).unwrap().unwrap(), b"three");
        assert!(buffer.is_empty());
    }

    #[test]
    fn decode_oversized_frame() {
        let codec = FrameCodec::with_max_frame_size(8);
        // Craft a frame header that claims 9 payload bytes, exceeding the max.
        let mut buffer = vec![0u8, 0u8, 0u8, 9u8];
        assert!(matches!(
            codec.decode(&mut buffer),
            Err(NetworkError::FrameTooLarge { size: 9, max: 8 })
        ));
    }

    #[test]
    fn encode_oversized_payload() {
        let codec = FrameCodec::with_max_frame_size(4);
        assert!(matches!(
            codec.encode(b"12345"),
            Err(NetworkError::FrameTooLarge { .. })
        ));
    }

    #[test]
    fn decode_malformed_zero_payload_is_valid() {
        // A zero-length payload is allowed by the format.
        let codec = FrameCodec::new();
        let mut buffer = vec![0u8, 0u8, 0u8, 0u8];
        let decoded = codec.decode(&mut buffer).unwrap().unwrap();
        assert!(decoded.is_empty());
        assert!(buffer.is_empty());
    }
}
