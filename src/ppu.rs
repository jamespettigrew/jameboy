const LINE_BYTES: usize = 2;
const TILE_LINES: usize = 8;
const TILE_DIMENSION: usize = 8;
const TILE_COUNT: usize = 128;
const TILE_MAP_COUNT: usize = 1024;

#[derive(Debug)]
struct Tile([u8; TILE_DIMENSION * TILE_DIMENSION]);

//fn read_tile_maps(vram: &[u8; 1024]) ->

fn read_tiles(vram: &[u8; 0x7FF]) -> [Tile; TILE_COUNT] {
    let mut tiles: Vec<Tile> = Vec::with_capacity(TILE_COUNT);

    let tile_chunks = vram.chunks(TILE_LINES * LINE_BYTES);
    for tile_chunk in tile_chunks {
        let mut tile_pixels = [0; TILE_DIMENSION * TILE_DIMENSION];
        for (line, line_bytes) in tile_chunk.chunks(LINE_BYTES).enumerate() {
            for (j, pixel) in line_bytes_to_pixels(line_bytes[0], line_bytes[1])
                .into_iter()
                .enumerate()
            {
                tile_pixels[line * TILE_DIMENSION + j] = pixel;
            }
        }

        tiles.push(Tile(tile_pixels));
    }

    tiles.try_into().expect("tiles Vec is converted into array")
}

fn line_bytes_to_pixels(first_byte: u8, second_byte: u8) -> [u8; TILE_DIMENSION] {
    let mut pixels = [0; TILE_DIMENSION];
    for i in 0..TILE_DIMENSION {
        let bit = 7 - i;
        let lsb = first_byte >> bit & 1;
        let msb = second_byte >> bit & 1;

        pixels[i] = (msb << 1) | lsb
    }

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_bytes_to_pixels() {
        assert_eq!(
            [0, 2, 3, 3, 3, 3, 2, 0],
            line_bytes_to_pixels(0b00111100, 0b01111110)
        );
        assert_eq!(
            [0, 3, 0, 0, 0, 0, 3, 0],
            line_bytes_to_pixels(0b01000010, 0b01000010)
        );

        assert_eq!(
            [0, 1, 1, 1, 3, 1, 3, 0],
            line_bytes_to_pixels(0b01111110, 0b00001010)
        );
    }
}
