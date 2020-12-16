#[derive(Debug, Eq, PartialEq)]
pub enum FromHexError {
    OddLength,
    InvalidCharacter(u8),
    InvalidStringLength,
}

pub trait FromHex : Sized {
    type Error;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error>; //use generic type AsRef to accept any type that can be coerced to a slice of bytes like Vec<u8>
}

impl FromHex for Vec<u8> {
    type Error = FromHexError;

    fn from_hex<T: AsRef<[u8]>>(hex: T) -> Result<Self, Self::Error> {
        let hex = hex.as_ref();

        if (hex.len() % 2) != 0 {
            return Err(FromHexError::OddLength);
        }

        hex.chunks(2)
        .map(|pair| {
            let n1 = val(pair[0])? << 4;
            let n2 = val(pair[1])?;
            Ok( n1 | n2)
        })
        .collect()
    }
}

fn val(nibble: u8) -> Result<u8, FromHexError> {
    match nibble {
        b'a'..=b'f' => Ok(nibble - b'a' + 10),
        b'A'..=b'F' => Ok(nibble - b'A' + 10),
        b'0'..=b'9' => Ok(nibble - b'0'),
        _ => Err(FromHexError::InvalidCharacter(nibble)),
    }
}

pub fn decode<T: AsRef<[u8]>>(data: T) -> Result<Vec<u8>, FromHexError> {
    FromHex::from_hex(data)
}

pub fn decode_to_slice<T: AsRef<[u8]>>(data: T, out: &mut [u8]) -> Result<(), FromHexError> {
    let data = data.as_ref();

    if (data.len() % 2) != 0 {
        return Err(FromHexError::OddLength);
    }

    if out.len() != (data.len() / 2) {
        return Err(FromHexError::InvalidStringLength);
    }

    for (i, byte) in out.iter_mut().enumerate() {
        *byte = {
            let n1 = val( data[2*i])? << 4;
            let n2 = val( data[2*i + 1])?;

            n1 | n2
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::val;
    use super::FromHexError;
    use super::FromHex;

    #[test]
    fn check_nibble() {
        assert_eq!(val(b'a'), Ok(0xa));
        assert_eq!(val(b'D'), Ok(0xD));
        assert_eq!(val(b'5'), Ok(0x5));
        assert_eq!(val(b'Z'), Err(FromHexError::InvalidCharacter(b'Z')));
    }
    #[test]
    fn from_hex() {
        let v = vec![b'f', b'9', b'B', b'4', b'C', b'a'];
        assert_eq!(FromHex::from_hex(&v), Ok(vec![0xf9, 0xb4, 0xca]));

        let v = vec![b'f', b'9', b'B'];
        let result: Result<Vec<u8>, _> = FromHex::from_hex(&v);
        assert_eq!(result, Err(FromHexError::OddLength));
        println!("{:?}", v);
    }
}
