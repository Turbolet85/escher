//! An image's written form: its pixels as a PNG, and those bytes as base64 text.

/// `rgba` — `width` × `height` pixels of four bytes each, row by row — as a PNG of 8-bit RGBA.
/// It holds no time or text chunk, so equal pixels give equal bytes. `None` when the pixels
/// do not fill the size or the size is empty.
pub(crate) fn png(width: u32, height: u32, rgba: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::new();
    let mut encoder = png::Encoder::new(&mut out, width, height);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().ok()?;
    writer.write_image_data(rgba).ok()?;
    writer.finish().ok()?;
    Some(out)
}

const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// `bytes` in base64: RFC 4648 §4, the standard alphabet, padded with `=`.
pub(crate) fn base64(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b = [
            chunk.first().copied().unwrap_or(0),
            chunk.get(1).copied().unwrap_or(0),
            chunk.get(2).copied().unwrap_or(0),
        ];
        let n = u32::from(b[0]) << 16 | u32::from(b[1]) << 8 | u32::from(b[2]);
        let sextets =
            [n >> 18, n >> 12, n >> 6, n].map(|s| char::from(ALPHABET[(s & 63) as usize]));
        let kept = chunk.len() + 1;
        for (i, c) in sextets.into_iter().enumerate() {
            out.push(if i < kept { c } else { '=' });
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base64_reads_the_rfc_4648_test_vectors() {
        for (input, expected) in [
            ("", ""),
            ("f", "Zg=="),
            ("fo", "Zm8="),
            ("foo", "Zm9v"),
            ("foob", "Zm9vYg=="),
            ("fooba", "Zm9vYmE="),
            ("foobar", "Zm9vYmFy"),
        ] {
            assert_eq!(base64(input.as_bytes()), expected, "input {input:?}");
        }
    }

    #[test]
    fn base64_writes_every_sextet_of_the_alphabet() {
        let bytes: Vec<u8> = (0u8..=255).collect();
        let text = base64(&bytes);
        assert_eq!(text.len(), 344);
        assert!(text.ends_with("+/w=="), "{text}");
        assert!(text.bytes().all(|b| ALPHABET.contains(&b) || b == b'='));
    }

    fn decode(png_bytes: &[u8]) -> (u32, u32, Vec<u8>) {
        let decoder = png::Decoder::new(std::io::Cursor::new(png_bytes));
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size().unwrap()];
        let info = reader.next_frame(&mut buf).unwrap();
        assert_eq!(info.color_type, png::ColorType::Rgba);
        assert_eq!(info.bit_depth, png::BitDepth::Eight);
        buf.truncate(info.buffer_size());
        (info.width, info.height, buf)
    }

    #[test]
    fn a_png_decodes_to_the_pixels_it_was_given() {
        let (width, height) = (3, 2);
        let rgba: Vec<u8> = (0..width * height * 4)
            .map(|i| (i * 7 % 256) as u8)
            .collect();
        let bytes = png(width, height, &rgba).unwrap();
        assert!(bytes.starts_with(b"\x89PNG\r\n\x1a\n"));
        assert_eq!(decode(&bytes), (width, height, rgba.clone()));
        assert_eq!(
            png(width, height, &rgba),
            Some(bytes),
            "equal pixels, equal bytes"
        );
    }

    #[test]
    fn a_png_holds_no_time_or_text_chunk() {
        let bytes = png(1, 1, &[1, 2, 3, 255]).unwrap();
        for chunk in [b"tIME", b"tEXt", b"zTXt", b"iTXt"] {
            assert!(!bytes.windows(4).any(|w| w == chunk));
        }
    }

    #[test]
    fn pixels_that_do_not_fill_the_size_write_nothing() {
        assert_eq!(png(2, 2, &[0; 12]), None);
        assert_eq!(png(0, 0, &[]), None);
    }
}
