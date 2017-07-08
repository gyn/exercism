/// Convert a list of numbers to a stream of bytes encoded with variable length encoding.
pub fn to_bytes(values: &[u32]) -> Vec<u8> {
    let mut result:Vec<u8> = Vec::with_capacity(values.len() * 5);

    for w in values {
        let mut zeroable = false;

        let mut b = *w >> 28 & 0x0000_007F;
        if b != 0 {
            zeroable = true;
            result.push(0x80u8 + b as u8);
        }

        b = *w >> 21 & 0x0000_007F;
        if b != 0 || zeroable {
            zeroable = true;
            result.push(0x80u8 + b as u8);
        }

        b = *w >> 14 & 0x0000_007F;
        if b != 0 || zeroable {
            zeroable = true;
            result.push(0x80u8 + b as u8);
        }

        b = *w >> 7 & 0x0000_007F;
        if b != 0 || zeroable {
            result.push(0x80u8 + b as u8);
        }

        b = *w & 0x0000_007F;
        result.push(b as u8);
    }

    result
}

/// Given a stream of bytes, extract all numbers which are encoded in there.
pub fn from_bytes(bytes: &[u8]) -> Result<Vec<u32>, &'static str> {
    let mut length = 0;
    let mut value: u32 = 0;
    let mut result:Vec<u32> = Vec::new();

    for b in bytes {
        length += 1;

        if length > 5 {
            return Err("Invalid VLQ code");
        }

        if let Some(v) = value.checked_mul(128) {
            value = v;
        } else {
            return Err("Overflowed VLQ code");
        }

        if *b & 0x80 != 0 {
            value += (*b - 0x80u8) as u32;
        } else {
            value += *b as u32;

            result.push(value);

            value = 0;
            length = 0;
        }
    }

    if length != 0 {
        return Err("Invalid VLQ code");
    }

    Ok(result)
}
