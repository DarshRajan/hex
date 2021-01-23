use crate::decode::FromHexError;

const HEX_TABLE: [u8; 16] = [ b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7',
                              b'8', b'9', b'a', b'b', b'c', b'd', b'e', b'f'];

fn byte_to_hex(byte: u8) -> (u8, u8) {
    let n1 = HEX_TABLE[ ((byte >> 4) & 0x0f) as usize];
    let n2 = HEX_TABLE[ (byte & 0x0f) as usize];

    (n1, n2)
}

//writes hex output to given buffer.
//length of output slice must be exactly twice of input slice
pub fn encode_to_slice<T: AsRef<[u8]>>(input: T, output: &mut [u8]) -> Result<(), FromHexError> {
    let input = input.as_ref();
    if output.len() != 2 * input.len() {
        return Err(FromHexError::InvalidStringLength);
    }
    
    for (out, byte) in output.chunks_exact_mut(2).zip(input.iter()) {
        let nibbles = byte_to_hex(*byte);
        out[0] = nibbles.0;
        out[1] = nibbles.1;
    }

    Ok(())
}

pub fn encode<T: AsRef<[u8]>>(input: T) -> String {
    let input = input.as_ref();

    let mut output = vec![0u8; input.len() * 2];
    
    encode_to_slice(&input, &mut output).unwrap();
    String::from_utf8(output).expect("Invalid UTF-8")
}

#[cfg(test)]
mod tests {
    use super::encode_to_slice;
    use crate::decode::FromHexError;
    
    #[test]
    fn check_encode_to_slice() {
        let input = [0x12, 0xdf];
        let mut output = vec![0u8; 4];

        encode_to_slice(&input, &mut output);
        assert_eq!(vec![b'1', b'2', b'd', b'f'], output);
        
        let mut output = vec![0u8, 3];
        let result = encode_to_slice(&input, &mut output);
        assert_eq!(result, Err(FromHexError::InvalidStringLength));
    }
}
