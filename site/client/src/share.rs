//! The links that hold a program: `#code=1:…` is the program compressed (deflate, raw,
//! RFC 1951), `#code=0:…` as it is (UTF-8), both in URL-safe base64; Share takes the
//! shorter. The compression is the browser's (`CompressionStream`); this module has
//! the rest: base64, the limits, and an inflate for the browsers without
//! `DecompressionStream` (stored, fixed and dynamic blocks).

/// The most characters of a shared program, packed.
pub const LINK_MAX: usize = 8000;
/// The most bytes of a program opened from a link.
pub const CODE_MAX: usize = 200_000;

/// Why a link cannot be read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Broken {
    Damaged,
    TooLong,
}

/// `1:…` or `0:…`: whether it is compressed, and its base64.
pub fn packed(value: &str) -> Option<(bool, &str)> {
    let (kind, data) = value.split_once(':')?;
    let compressed = match kind {
        "1" => true,
        "0" => false,
        _ => return None,
    };
    data.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_').then_some((compressed, data))
}

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// URL-safe base64, without padding.
pub fn to_base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let n = (u32::from(chunk[0]) << 16) | (u32::from(*chunk.get(1).unwrap_or(&0)) << 8) | u32::from(*chunk.get(2).unwrap_or(&0));
        for k in 0..=chunk.len() {
            out.push(ALPHABET[(n >> (18 - 6 * k)) as usize & 63] as char);
        }
    }
    out
}

/// The bytes of URL-safe base64 (padding optional), as the browser's `atob` reads it.
pub fn from_base64(text: &str) -> Result<Vec<u8>, Broken> {
    let text = text.trim_end_matches('=');
    if text.len() % 4 == 1 {
        return Err(Broken::Damaged);
    }
    let mut out = Vec::with_capacity(text.len() * 3 / 4);
    let mut acc = 0u32;
    let mut bits = 0;
    for b in text.bytes() {
        let value = match b {
            b'A'..=b'Z' => b - b'A',
            b'a'..=b'z' => b - b'a' + 26,
            b'0'..=b'9' => b - b'0' + 52,
            b'-' | b'+' => 62,
            b'_' | b'/' => 63,
            _ => return Err(Broken::Damaged),
        };
        acc = (acc << 6) | u32::from(value);
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
            acc &= (1 << bits) - 1;
        }
    }
    Ok(out)
}

/// The text of the bytes of a link: UTF-8, at most [`CODE_MAX`] bytes.
pub fn program(bytes: Vec<u8>) -> Result<String, Broken> {
    if bytes.len() > CODE_MAX {
        return Err(Broken::TooLong);
    }
    String::from_utf8(bytes).map_err(|_| Broken::Damaged)
}

const LEN_BASE: [u16; 29] =
    [3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131, 163, 195, 227, 258];
const LEN_EXTRA: [u8; 29] = [0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0];
const DIST_BASE: [u16; 30] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537, 2049, 3073, 4097, 6145, 8193, 12289, 16385,
    24577,
];
const DIST_EXTRA: [u8; 30] = [0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13, 13];
const ORDER: [usize; 19] = [16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15];

/// A canonical Huffman code from its lengths: how many codes of each length, and the
/// symbols in the order of their codes.
struct Huffman {
    counts: [u16; 16],
    symbols: Vec<u16>,
}

impl Huffman {
    fn new(lengths: &[u8]) -> Huffman {
        let mut counts = [0u16; 16];
        for &l in lengths {
            counts[l as usize] += 1;
        }
        counts[0] = 0;
        let mut offsets = [0u16; 16];
        for l in 1..16 {
            offsets[l] = offsets[l - 1] + counts[l - 1];
        }
        let mut symbols = vec![0u16; lengths.len()];
        for (s, &l) in lengths.iter().enumerate() {
            if l > 0 {
                symbols[offsets[l as usize] as usize] = s as u16;
                offsets[l as usize] += 1;
            }
        }
        Huffman { counts, symbols }
    }
}

struct Bits<'a> {
    data: &'a [u8],
    pos: usize,
    bit: u32,
}

impl Bits<'_> {
    fn take(&mut self, n: u32) -> Result<u32, Broken> {
        let mut v = 0;
        for i in 0..n {
            let byte = *self.data.get(self.pos).ok_or(Broken::Damaged)?;
            v |= u32::from((byte >> self.bit) & 1) << i;
            self.bit += 1;
            if self.bit == 8 {
                self.bit = 0;
                self.pos += 1;
            }
        }
        Ok(v)
    }

    fn decode(&mut self, h: &Huffman) -> Result<u16, Broken> {
        let (mut code, mut first, mut index) = (0i32, 0i32, 0i32);
        for l in 1..16 {
            code |= self.take(1)? as i32;
            let count = i32::from(h.counts[l]);
            if code - count < first {
                return h.symbols.get((index + code - first) as usize).copied().ok_or(Broken::Damaged);
            }
            index += count;
            first = (first + count) << 1;
            code <<= 1;
        }
        Err(Broken::Damaged)
    }
}

/// Inflate (RFC 1951): stored, fixed and dynamic blocks, at most `max` bytes out.
pub fn inflate(data: &[u8], max: usize) -> Result<Vec<u8>, Broken> {
    let mut input = Bits { data, pos: 0, bit: 0 };
    let mut out: Vec<u8> = Vec::new();
    let put = |out: &mut Vec<u8>, b: u8| {
        if out.len() >= max {
            return Err(Broken::TooLong);
        }
        out.push(b);
        Ok(())
    };
    loop {
        let last = input.take(1)?;
        match input.take(2)? {
            0 => {
                if input.bit > 0 {
                    input.bit = 0;
                    input.pos += 1;
                }
                let at = input.pos;
                let header = data.get(at..at + 4).ok_or(Broken::Damaged)?;
                let len = usize::from(header[0]) | (usize::from(header[1]) << 8);
                let nlen = usize::from(header[2]) | (usize::from(header[3]) << 8);
                if len ^ 0xffff != nlen {
                    return Err(Broken::Damaged);
                }
                let bytes = data.get(at + 4..at + 4 + len).ok_or(Broken::Damaged)?;
                for &b in bytes {
                    put(&mut out, b)?;
                }
                input.pos = at + 4 + len;
            }
            kind @ (1 | 2) => {
                let (lit, dist) = if kind == 1 {
                    let lengths: Vec<u8> = (0..288)
                        .map(|i| {
                            if i < 144 {
                                8
                            } else if i < 256 {
                                9
                            } else if i < 280 {
                                7
                            } else {
                                8
                            }
                        })
                        .collect();
                    (Huffman::new(&lengths), Huffman::new(&[5; 30]))
                } else {
                    let nlit = input.take(5)? as usize + 257;
                    let ndist = input.take(5)? as usize + 1;
                    let ncode = input.take(4)? as usize + 4;
                    let mut code_lengths = [0u8; 19];
                    for &at in &ORDER[..ncode] {
                        code_lengths[at] = input.take(3)? as u8;
                    }
                    let length_code = Huffman::new(&code_lengths);
                    let mut lengths: Vec<u8> = Vec::with_capacity(nlit + ndist);
                    while lengths.len() < nlit + ndist {
                        let sym = input.decode(&length_code)?;
                        if sym < 16 {
                            lengths.push(sym as u8);
                            continue;
                        }
                        let (value, n) = match sym {
                            16 => (*lengths.last().ok_or(Broken::Damaged)?, 3 + input.take(2)?),
                            17 => (0, 3 + input.take(3)?),
                            _ => (0, 11 + input.take(7)?),
                        };
                        if lengths.len() + n as usize > nlit + ndist {
                            return Err(Broken::Damaged);
                        }
                        lengths.extend(std::iter::repeat_n(value, n as usize));
                    }
                    (Huffman::new(&lengths[..nlit]), Huffman::new(&lengths[nlit..]))
                };
                loop {
                    let sym = input.decode(&lit)?;
                    match sym {
                        0..=255 => put(&mut out, sym as u8)?,
                        256 => break,
                        _ => {
                            let sym = usize::from(sym - 257);
                            if sym >= 29 {
                                return Err(Broken::Damaged);
                            }
                            let len = usize::from(LEN_BASE[sym]) + input.take(u32::from(LEN_EXTRA[sym]))? as usize;
                            let d = usize::from(input.decode(&dist)?);
                            if d >= 30 {
                                return Err(Broken::Damaged);
                            }
                            let back = usize::from(DIST_BASE[d]) + input.take(u32::from(DIST_EXTRA[d]))? as usize;
                            if back > out.len() {
                                return Err(Broken::Damaged);
                            }
                            for _ in 0..len {
                                let b = out[out.len() - back];
                                put(&mut out, b)?;
                            }
                        }
                    }
                }
            }
            _ => return Err(Broken::Damaged),
        }
        if last == 1 {
            return Ok(out);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_round_trips() {
        for text in ["", "a", "ab", "abc", "été, 中文 |> ?&="] {
            let encoded = to_base64(text.as_bytes());
            assert!(!encoded.contains(['+', '/', '=']), "{encoded}");
            assert_eq!(from_base64(&encoded).unwrap(), text.as_bytes());
        }
        assert_eq!(to_base64(b"\xfb\xff"), "-_8");
        assert_eq!(from_base64("A"), Err(Broken::Damaged));
        assert_eq!(from_base64("a.b"), Err(Broken::Damaged));
        assert_eq!(packed("1:ab-_"), Some((true, "ab-_")));
        assert_eq!(packed("2:ab"), None);
        assert_eq!(packed("0:a b"), None);
    }

    #[test]
    fn inflates_stored_fixed_and_dynamic_blocks() {
        // Made by zlib (raw deflate): "hello hello hello" fixed, "hi" stored, and a text of
        // 823 bytes in a dynamic block.
        let fixed = [203, 72, 205, 201, 201, 87, 200, 64, 144, 0];
        assert_eq!(inflate(&fixed, 100).unwrap(), b"hello hello hello");
        let stored = [1, 2, 0, 253, 255, b'h', b'i'];
        assert_eq!(inflate(&stored, 100).unwrap(), b"hi");
        assert_eq!(inflate(&stored, 1), Err(Broken::TooLong));
        let dynamic = [
            69, 146, 73, 18, 130, 64, 16, 4, 191, 194, 19, 100, 24, 16, 158, 131, 160, 2, 23, 35, 244, 192, 247, 165, 59, 123, 233, 83,
            229, 133, 202, 168, 225, 252, 124, 215, 91, 179, 255, 154, 230, 188, 82, 43, 233, 161, 177, 72, 92, 22, 205, 157, 228, 117, 93,
            21, 170, 192, 243, 58, 165, 94, 232, 37, 167, 56, 8, 190, 245, 148, 239, 241, 233, 81, 210, 174, 113, 146, 120, 28, 52, 106,
            249, 60, 207, 16, 2, 215, 129, 38, 177, 152, 71, 235, 34, 238, 210, 214, 84, 199, 132, 108, 26, 128, 58, 108, 219, 6, 225, 113,
            29, 104, 46, 135, 233, 20, 215, 113, 163, 210, 70, 67, 193, 134, 108, 38, 128, 79, 2, 197, 36, 96, 78, 2, 187, 142, 27, 149,
            49, 27, 176, 97, 115, 51, 1, 124, 23, 40, 102, 1, 115, 21, 56, 94, 200, 140, 186, 62, 26, 58, 108, 200, 102, 2, 248, 46, 80,
            204, 194, 147, 231, 42, 112, 60, 147, 25, 213, 18, 13, 21, 27, 178, 153, 0, 190, 11, 20, 179, 128, 185, 10, 28, 207, 100, 70,
            117, 138, 134, 30, 27, 178, 153, 0, 190, 11, 20, 179, 128, 185, 10, 156, 127, 46, 70, 253, 144, 13, 216, 144, 205, 4, 240, 93,
            254,
        ];
        let text: Vec<String> = (0..60).map(|i| format!("word{i} is {}", "abcdefghij"[i % 10..i % 10 + 1].repeat(i % 7))).collect();
        assert_eq!(String::from_utf8(inflate(&dynamic, 1000).unwrap()).unwrap(), text.join(" "));
        assert_eq!(inflate(&[0, 0, 0], 100), Err(Broken::Damaged));
        assert_eq!(inflate(&[0x07], 100), Err(Broken::Damaged));
    }
}
