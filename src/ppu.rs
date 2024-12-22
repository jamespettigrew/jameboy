use crate::util::{bit, set_bits};
use crate::{Address, Memory};
use image::{GrayImage, Luma};
use std::collections::VecDeque;

const DOTS_PER_OAM_SCAN: usize = 80;
const DOTS_PER_SCANLINE: usize = 456;
const SCANLINES_PER_FRAME: usize = 153;
const SCANLINES_PER_VERTICAL_BLANK: usize = 10;
const PIXELS_PER_SCANLINE: u8 = 160;
const TILE_DIMENSION: usize = 8;
const TILE_BYTES: u16 = 16;

const ADDRESS_INTERRUPT_FLAG_REGISTER: u16 = 0xFF0F;
const ADDRESS_LCDC_REGISTER: u16 = 0xFF40;
const ADDRESS_LCD_STATUS_REGISTER: u16 = 0xFF41;
const ADDRESS_SCY: u16 = 0xFF42;
const ADDRESS_SCX: u16 = 0xFF43;
const ADDRESS_LY: u16 = 0xFF44;
const ADDRESS_LYC: u16 = 0xFF45;
const ADDRESS_BGP: u16 = 0xFF47;
const ADDRESS_WY: u16 = 0xFF4A;
const ADDRESS_WX: u16 = 0xFF4B;

#[repr(u16)]
enum BgWindowTileArea {
    Area8000 = 0x8000,
    Area8800 = 0x8800,
}

#[repr(u16)]
enum TileMapArea {
    Area9800 = 0x9800,
    Area9C00 = 0x9C00,
}

#[repr(u8)]
enum PpuMode {
    HorizontalBlank = 0,
    VerticalBlank = 1,
    OamScan = 2,
    Drawing = 3,
}

enum Palette {
    Bgp,
    Obp0,
    Obp1,
}

fn read_window_tile_map_area(memory: &Memory) -> TileMapArea {
    match bit(memory.read(Address(ADDRESS_LCDC_REGISTER)), 6) == 0 {
        true => TileMapArea::Area9800,
        false => TileMapArea::Area9C00,
    }
}

fn read_window_enabled(memory: &Memory) -> bool {
    bit(memory.read(Address(ADDRESS_LCDC_REGISTER)), 5) == 1
}

fn read_bg_window_tile_area(memory: &Memory) -> BgWindowTileArea {
    match bit(memory.read(Address(ADDRESS_LCDC_REGISTER)), 4) == 0 {
        false => BgWindowTileArea::Area8000,
        true => BgWindowTileArea::Area8800,
    }
}

fn read_bg_tile_map_area(memory: &Memory) -> TileMapArea {
    match bit(memory.read(Address(ADDRESS_LCDC_REGISTER)), 3) == 0 {
        true => TileMapArea::Area9800,
        false => TileMapArea::Area9C00,
    }
}

fn read_bg_window_enable(memory: &Memory) -> bool {
    bit(memory.read(Address(ADDRESS_LCDC_REGISTER)), 0) != 0
}

fn read_ppu_mode(memory: &Memory) -> PpuMode {
    let lcd_status_register = memory.read(Address(ADDRESS_LCD_STATUS_REGISTER));
    let (msb, lsb) = (bit(lcd_status_register, 1), bit(lcd_status_register, 0));

    match (msb == 0, lsb == 0) {
        (true, true) => PpuMode::HorizontalBlank,
        (true, false) => PpuMode::VerticalBlank,
        (false, true) => PpuMode::OamScan,
        (false, false) => PpuMode::Drawing,
    }
}

fn request_vblank_interrupt(memory: &mut Memory) {
    let status_register = memory.read(Address(ADDRESS_INTERRUPT_FLAG_REGISTER));
    memory.write(
        Address(ADDRESS_INTERRUPT_FLAG_REGISTER),
        set_bits(status_register, 1, 0b0000_0001),
    );
}

fn write_coincidence_flag(memory: &mut Memory, enabled: bool) {
    let status_register = memory.read(Address(ADDRESS_LCD_STATUS_REGISTER));
    memory.write(
        Address(ADDRESS_LCD_STATUS_REGISTER),
        set_bits(status_register, enabled as u8, 0b0000_0100),
    );
}

fn write_ppu_mode(memory: &mut Memory, ppu_mode: PpuMode) {
    let status_register = memory.read(Address(ADDRESS_LCD_STATUS_REGISTER));
    memory.write(
        Address(ADDRESS_LCD_STATUS_REGISTER),
        set_bits(status_register, ppu_mode as u8, 0b0000_0011),
    );
}

fn fetch_non_window_tile_data_address(tile_data_area: BgWindowTileArea, tile_number: u8, ly: u16, scy: u16) -> u16 {
    let tile_offset = tile_number as u16 * TILE_BYTES;
    let tile_byte_offset = 2 * ((ly as u16 + scy) % 8) as u16;
    
    (tile_data_area as u16) + tile_offset + tile_byte_offset
}

fn fetch_window_tile_data_address(tile_data_area: BgWindowTileArea, tile_number: u8, window_line: u8) -> u16 {
    let tile_offset = tile_number as u16 * TILE_BYTES;
    let tile_byte_offset = ((2 * window_line) % 8) as u16;
    
    (tile_data_area as u16) + tile_offset + tile_byte_offset
}

#[derive(Copy, Clone, Debug)]
enum ObjectBackgroundPriority {
    Object,     // Sprite is always rendered above background
    Background, // Background colors 1-3 overlay sprite, sprite is still rendered above color 0
}

enum SpriteHeight {
    Normal = 8,
    Tall = 16,
}

#[derive(Copy, Clone, Debug)]
struct Sprite {
    y_position: u8,
    x_position: u8,
    tile_number: u8,
    flags: SpriteFlags,
}

impl Sprite {
    fn visible(&self, ly: u8, height: SpriteHeight) -> bool {
        self.x_position > 0
            && self.y_position <= ly + 16
            && self.y_position + height as u8 > ly + 16
    }
}

#[derive(Copy, Clone, Debug)]
struct SpriteFlags {
    priority: ObjectBackgroundPriority,
    y_flip: bool,
    x_flip: bool,
    palette: bool,
}

impl From<u8> for SpriteFlags {
    fn from(item: u8) -> Self {
        SpriteFlags {
            priority: match bit(item, 7) == 0 {
                true => ObjectBackgroundPriority::Object,
                false => ObjectBackgroundPriority::Background,
            },
            y_flip: bit(item, 6) == 1,
            x_flip: bit(item, 5) == 1,
            palette: bit(item, 4) == 1,
        }
    }
}

impl From<&[u8]> for Sprite {
    fn from(item: &[u8]) -> Self {
        Sprite {
            y_position: item[0],
            x_position: item[1],
            tile_number: item[2],
            flags: SpriteFlags::from(item[3]),
        }
    }
}

struct Pixel {
    colour: PixelColour,
    palette: Palette,
    priority: ObjectBackgroundPriority,
}

impl Pixel {
    fn mix(background_pixel: Pixel, sprite_pixel: Pixel) -> Pixel {
        if let PixelColour::White = sprite_pixel.colour {
            return background_pixel;
        }

        if let ObjectBackgroundPriority::Background = sprite_pixel.priority {
            return match background_pixel.colour {
                PixelColour::White => sprite_pixel,
                _ => background_pixel,
            };
        }

        sprite_pixel
    }
}

enum FetchStep {
    Paused,
    FetchTileNumber,
    FetchTileLow(u8),
    FetchTileHigh(u16, u8),
    Push([PixelColour; TILE_DIMENSION]),
}

struct BackgroundFetcher {
    x_position: u8,
    fetch_step: FetchStep,
    fifo: VecDeque<Pixel>,
}

impl BackgroundFetcher {
    fn paused(&self) -> bool {
        match self.fetch_step {
            FetchStep::Paused => true,
            _ => false,
        }
    }

    fn reset(&mut self) {
        self.x_position = 0;
        self.fetch_step = FetchStep::FetchTileNumber;
    }

    fn step(&mut self, memory: &Memory, scanline_x_position: u8) {
        let ly = memory.read(Address(ADDRESS_LY)) as u16;
        let scy = memory.read(Address(ADDRESS_SCY)) as u16;
        let scx = memory.read(Address(ADDRESS_SCX)) as u16;
        let wy = memory.read(Address(ADDRESS_WY)) as u16;
        let wx = memory.read(Address(ADDRESS_WX)) as u16;
        let tile_data_area = read_bg_window_tile_area(memory);

        match &self.fetch_step {
            FetchStep::Paused => {}
            FetchStep::FetchTileNumber => {
                let is_window_tile = read_window_enabled(memory) && (scanline_x_position as u16) >= wx - 7 && ly >= wy;
                let tile_map_area = if is_window_tile { read_window_tile_map_area(memory) } else { read_bg_tile_map_area(memory) };

                let y_offset = (32 * (((ly as u16 + scy) & 0xFF) / 8)) & 0x3FF;
                let scx_offset = (scx & 0x1F);
                let x_offset = (self.x_position as u16 + scx_offset) & 0x3FF;

                let tile_number_address = tile_map_area as u16 + x_offset + y_offset;
                let tile_number = memory.read(Address(tile_number_address));
                self.fetch_step = FetchStep::FetchTileLow(tile_number);
            }
            FetchStep::FetchTileLow(tile_number) => {
                let address = fetch_non_window_tile_data_address(tile_data_area, *tile_number, ly, scy);
                let tile_data_low = memory.read(Address(address));
                self.fetch_step = FetchStep::FetchTileHigh(address, tile_data_low);
            }
            FetchStep::FetchTileHigh(tile_data_low_address, tile_data_low) => {
                let tile_data_high = memory.read(Address(tile_data_low_address + 1));
                let pixel_colours = line_bytes_to_pixel_colours(*tile_data_low, tile_data_high);
                self.fetch_step = FetchStep::Push(pixel_colours);
            }
            FetchStep::Push(pixel_colours) => {
                if self.fifo.is_empty() {
                    let pixels = pixel_colours.into_iter().map(|colour| Pixel {
                        colour: *colour,
                        palette: Palette::Bgp,
                        priority: ObjectBackgroundPriority::Background, // Irrelevant for background pixels
                    });
                    self.x_position = (self.x_position + 1) & 0x1F;
                    self.fifo.extend(pixels);
                    self.fetch_step = FetchStep::FetchTileNumber;
                }
            }
        };
    }
}

struct SpriteFetcher {
    fetch_step: FetchStep,
    fifo: VecDeque<Pixel>,
    sprite: Option<Sprite>,
}

impl SpriteFetcher {
    fn paused(&self) -> bool {
        match self.fetch_step {
            FetchStep::Paused => true,
            _ => false,
        }
    }

    fn reset(&mut self) {
        self.fifo.clear();
    }

    fn step(&mut self, memory: &Memory, ppu_x_position: u8) {
        let ly = memory.read(Address(ADDRESS_LY)) as u16;
        let scy = memory.read(Address(ADDRESS_SCY)) as u16;
        let tile_data_area = BgWindowTileArea::Area8000;

        match &self.fetch_step {
            FetchStep::Paused => {}
            FetchStep::FetchTileNumber => {
                let sprite = self.sprite.expect("SpriteFetcher sprite is not None");
                self.fetch_step = FetchStep::FetchTileLow(sprite.tile_number);
            }
            FetchStep::FetchTileLow(tile_number) => {
                let address = fetch_non_window_tile_data_address(tile_data_area, *tile_number, ly, scy);
                let tile_data_low = memory.read(Address(address));
                self.fetch_step = FetchStep::FetchTileHigh(address, tile_data_low);
            }
            FetchStep::FetchTileHigh(tile_data_low_address, tile_data_low) => {
                let tile_data_high = memory.read(Address(tile_data_low_address + 1));
                let pixel_colours = line_bytes_to_pixel_colours(*tile_data_low, tile_data_high);
                self.fetch_step = FetchStep::Push(pixel_colours);
            }
            FetchStep::Push(pixel_colours) => {
                let sprite = self.sprite.expect("SpriteFetcher sprite is not None");
                let visible_pixel_count = sprite.x_position - ppu_x_position;
                let pixels = pixel_colours
                    .into_iter()
                    .map(|colour| Pixel {
                        colour: *colour,
                        palette: Palette::Bgp,
                        priority: sprite.flags.priority,
                    })
                    .take(visible_pixel_count.into())
                    .take(self.fifo.capacity() - self.fifo.len());
                self.fifo.extend(pixels.into_iter());
                self.fetch_step = FetchStep::Paused;
            }
        }
    }
}

pub struct Ppu {
    // Dot count in the current scanline
    dot: usize,
    // Sprite buffer for current scanline
    sprite_buffer: Vec<Sprite>,
    // Pixels pushed in the current scanline
    x_position: u8,
    background_fetcher: BackgroundFetcher,
    sprite_fetcher: SpriteFetcher,
    // Number of pixels to discard from the background FIFO at the start of mode 3 (PpuMode::Drawing)
    discard_count: usize,
    pub image_buffer: image::GrayImage,
}

impl Ppu {
    pub fn init() -> Ppu {
        Ppu {
            dot: 0,
            sprite_buffer: Vec::<Sprite>::with_capacity(10),
            x_position: 0,
            background_fetcher: BackgroundFetcher {
                x_position: 0,
                fetch_step: FetchStep::FetchTileNumber,
                fifo: VecDeque::<Pixel>::with_capacity(8),
            },
            sprite_fetcher: SpriteFetcher {
                fetch_step: FetchStep::Paused,
                fifo: VecDeque::<Pixel>::with_capacity(8),
                sprite: None,
            },
            discard_count: 0,
            image_buffer: GrayImage::new(160, 144),
        }
    }

    pub fn get_tile_buffer(&self, memory: &Memory) -> image::GrayImage {
        let tile_count = 384usize;
        let width_in_tiles = 12usize;
        let bytes_per_tile = 16usize;

        let image_width = width_in_tiles * TILE_DIMENSION;
        let image_height = (tile_count / width_in_tiles) * TILE_DIMENSION;
        let mut tile_buffer = GrayImage::new(image_width as u32, image_height as u32);

        let tile_data = memory.read_range(Address(0x8000), (tile_count * bytes_per_tile) as u16);
        for (tile_idx, tile_chunk) in tile_data.chunks(bytes_per_tile).enumerate() {
            let offset_x = (tile_idx % width_in_tiles) * TILE_DIMENSION;
            let offset_y = (tile_idx / width_in_tiles) * TILE_DIMENSION;

            for (row_idx, line_bytes) in tile_chunk.chunks(2).enumerate() {
                let pixel_colours = line_bytes_to_pixel_colours(line_bytes[0], line_bytes[1]);
                for (column_idx, pc) in pixel_colours.iter().enumerate() {
                    tile_buffer.put_pixel(
                        (offset_x + column_idx) as u32,
                        (offset_y + row_idx) as u32,
                        pc.to_grayscale(),
                    );
                }
            }
        }

        tile_buffer
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let ppu_mode = read_ppu_mode(memory);
        let ly = memory.read(Address(ADDRESS_LY));
        let lyc = memory.read(Address(ADDRESS_LYC));
        write_coincidence_flag(memory, ly == lyc);

        match ppu_mode {
            PpuMode::OamScan => {
                // Each sprite takes 2 dots to fetch, skip odd dots.
                if self.dot % 2 == 0 {
                    let byte_offset = (self.dot / 2) * 4;
                    let sprite_address = Address(0xFE00 + byte_offset as u16);
                    let sprite_memory = memory.read_range(sprite_address, 4);
                    let sprite = Sprite::from(sprite_memory);
                    let sprite_height = SpriteHeight::Normal; // TODO: fetch from register
                    
                    // Render conditions for sprite
                    if self.sprite_buffer.len() < 10 && sprite.visible(ly, sprite_height) {
                        self.sprite_buffer.push(sprite);
                    }
                }

                self.dot += 1;
                if self.dot == DOTS_PER_OAM_SCAN {
                    write_ppu_mode(memory, PpuMode::Drawing);
                    self.sprite_buffer
                        .sort_by(|s1, s2| (*s2).x_position.cmp(&s1.x_position));
                    // SCX mod 8 pixels should be discarded at the start of each scanline
                    let scx = memory.read(Address(ADDRESS_SCX)) as u16;
                    self.discard_count = (scx % 8) as usize;
                }
            }
            PpuMode::Drawing => {
                // Initiate sprite fetch if the X-Position of any sprite in the sprite buffer
                // is less than or equal to the current Pixel-X-Position + 8
                if self.sprite_fetcher.paused()
                    && self
                        .sprite_buffer
                        .iter()
                        .any(|s| s.x_position <= self.x_position + 8)
                {
                    self.background_fetcher.fetch_step = FetchStep::Paused;
                    self.sprite_fetcher.sprite = self.sprite_buffer.pop();
                    self.sprite_fetcher.fetch_step = FetchStep::FetchTileNumber;
                };

                self.background_fetcher.step(memory);
                self.sprite_fetcher.step(memory, self.x_position);

                if self.sprite_fetcher.paused() {
                    if self.background_fetcher.paused() {
                        self.background_fetcher.fetch_step = FetchStep::FetchTileNumber;
                    }

                    if let Some(background_pixel) = self.background_fetcher.fifo.pop_front() {
                        // Pause rendering while we discard SCX mod 8 pixels from leftmost tile
                        if self.discard_count > 0 {
                            self.discard_count -= 1;
                            self.dot += 1;

                            return;
                        }

                        let mixed_pixel = match self.sprite_fetcher.fifo.pop_front() {
                            Some(sprite_pixel) => Pixel::mix(background_pixel, sprite_pixel),
                            None => background_pixel,
                        };
                        self.image_buffer.put_pixel(
                            self.x_position as u32,
                            ly as u32,
                            mixed_pixel.colour.to_grayscale(),
                        );

                        self.x_position += 1;
                        if self.x_position == PIXELS_PER_SCANLINE {
                            write_ppu_mode(memory, PpuMode::HorizontalBlank);
                            self.x_position = 0;
                            self.sprite_buffer.clear();
                            self.background_fetcher.reset();
                            self.background_fetcher.fifo.clear();
                            self.sprite_fetcher.reset();
                        }
                    }
                }
                self.dot += 1;
            }
            PpuMode::HorizontalBlank => {
                self.dot += 1;
                if self.dot >= DOTS_PER_SCANLINE {
                    self.dot = 0;
                    memory.write(Address(ADDRESS_LY), ly + 1);
                    let ppu_mode =
                        if ly as usize >= SCANLINES_PER_FRAME - SCANLINES_PER_VERTICAL_BLANK {
                            PpuMode::VerticalBlank
                        } else {
                            PpuMode::OamScan
                        };
                    write_ppu_mode(memory, ppu_mode);
                }
            }
            PpuMode::VerticalBlank => {
                // VBlank interrupt should be requested at the beginning of each VBlank period.
                if self.dot == 0 {
                    request_vblank_interrupt(memory);
                }

                self.dot += 1;
                if self.dot >= DOTS_PER_SCANLINE {
                    self.dot = 0;
                    let mut new_ly = ly + 1;

                    if new_ly as usize > SCANLINES_PER_FRAME {
                        new_ly = 0;
                        write_ppu_mode(memory, PpuMode::OamScan);
                    }
                    memory.write(Address(ADDRESS_LY), new_ly);
                }
            }
        };
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
enum PixelColour {
    White,
    LightGray,
    DarkGray,
    Black,
}

impl TryFrom<u8> for PixelColour {
    type Error = ();

    fn try_from(item: u8) -> Result<Self, Self::Error> {
        match item {
            0 => Ok(PixelColour::White),
            1 => Ok(PixelColour::LightGray),
            2 => Ok(PixelColour::DarkGray),
            3 => Ok(PixelColour::Black),
            _ => Err(()),
        }
    }
}

impl PixelColour {
    fn to_grayscale(&self) -> Luma<u8> {
        match self {
            Self::White => Luma([255]),
            Self::LightGray => Luma([211]),
            Self::DarkGray => Luma([169]),
            Self::Black => Luma([0]),
        }
    }
}

fn line_bytes_to_pixel_colours(first_byte: u8, second_byte: u8) -> [PixelColour; TILE_DIMENSION] {
    let mut pixels = [PixelColour::White; TILE_DIMENSION];
    for i in 0..TILE_DIMENSION {
        let bit = 7 - i;
        let lsb = first_byte >> bit & 1;
        let msb = second_byte >> bit & 1;

        pixels[i] = PixelColour::try_from((msb << 1) | lsb)
            .expect("Only 2 bits should be passed to PixelColour::try_from");
    }

    pixels
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_bytes_to_pixels() {
        assert_eq!(
            [PixelColour::White, PixelColour::DarkGray, PixelColour::Black, PixelColour::Black, PixelColour::Black, PixelColour::Black, PixelColour::DarkGray, PixelColour::White],
            line_bytes_to_pixel_colours(0b00111100, 0b01111110)
        );
        assert_eq!(
            [PixelColour::White, PixelColour::Black, PixelColour::White, PixelColour::White, PixelColour::White, PixelColour::White, PixelColour::Black, PixelColour::White],
            line_bytes_to_pixel_colours(0b01000010, 0b01000010)
        );

        assert_eq!(
            [PixelColour::White, PixelColour::LightGray, PixelColour::LightGray, PixelColour::LightGray, PixelColour::Black, PixelColour::LightGray, PixelColour::Black, PixelColour::White],
            line_bytes_to_pixel_colours(0b01111110, 0b00001010)
        );
    }
}
