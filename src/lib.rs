mod decode;
pub use decode::{decode, decode_to_slice, FromHexError};

mod encode;
pub use encode::{encode, encode_to_slice};

#[cfg(test)]
mod tests {
    use crate::decode as Decode;
    use Decode::FromHexError;
    use crate::encode;

    #[test]
    fn decode() {
        let v = "f9B4Ca".as_bytes();
        let result = Decode::decode(v).unwrap();
        assert_eq!(result, vec![0xf9, 0xb4, 0xca]);

        let v = "f9b".as_bytes();
        let result: Result<Vec<u8>, _> = Decode::decode(v);
        assert_eq!(result, Err(Decode::FromHexError::OddLength));
    }
    #[test]
    fn decode_to_slice() {
        let v = "abcdef".as_bytes();
        let mut out = [0u8; 3];
        Decode::decode_to_slice(v, &mut out);
        assert_eq!(out, [0xab, 0xcd, 0xef]);

        let mut out = [0u8; 4];
        assert_eq!(Decode::decode_to_slice(v, &mut out), Err(Decode::FromHexError::InvalidStringLength));
    }

    #[test]
    fn check_encode() {
        let input = [0x12, 0x34, 0x56, 0x78, 0x90, 0xab, 0xcd, 0xef];

        assert_eq!(String::from("1234567890abcdef"), encode::encode(&input));
    }
}
