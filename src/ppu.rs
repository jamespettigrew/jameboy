use crate::{Address, Memory};
use crate::util::{ bit, set_bits};
use std::collections::VecDeque;
use image::{GrayImage, Luma};

const DOTS_PER_OAM_SCAN: usize = 80;
const DOTS_PER_SCANLINE: usize = 456;
const SCANLINES_PER_FRAME: usize = 153;
const SCANLINES_PER_VERTICAL_BLANK: usize = 10;
const PIXELS_PER_SCANLINE: u8 = 160;

const ADDRESS_INTERRUPT_FLAG_REGISTER: u16 = 0xFF0F;
const ADDRESS_LCDC_REGISTER: u16 = 0xFF40;
const ADDRESS_LCD_STATUS_REGISTER: u16 = 0xFF41;
const ADDRESS_SCY: u16 = 0xFF42;
const ADDRESS_LY: u16 = 0xFF44;
const ADDRESS_LYC: u16 = 0xFF45;
const ADDRESS_BGP: u16 = 0xFF47;

#[repr(u16)]
enum BgWindowTileArea {
    Area8000 = 0x8000,
    Area8800 = 0x8800,
}

#[repr(u16)]
enum BgTileMapArea {
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

fn read_bg_window_tile_area(memory: &Memory) -> BgWindowTileArea {
    match bit(memory.read(Address(ADDRESS_LCDC_REGISTER)), 4) == 0 {
        true => BgWindowTileArea::Area8000,
        false => BgWindowTileArea::Area8800,
    }
}

fn read_bg_tile_map_area(memory: &Memory) -> BgTileMapArea {
    match bit(memory.read(Address(ADDRESS_LCDC_REGISTER)), 3) == 0 {
        true => BgTileMapArea::Area9800,
        false => BgTileMapArea::Area9C00,
    }
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
    memory.write(Address(ADDRESS_INTERRUPT_FLAG_REGISTER), set_bits(status_register, 1, 0b0000_0001));
}

fn write_coincidence_flag(memory: &mut Memory, enabled: bool) {
    let status_register = memory.read(Address(ADDRESS_LCD_STATUS_REGISTER));
    memory.write(Address(ADDRESS_LCD_STATUS_REGISTER), set_bits(status_register, enabled as u8, 0b0000_0100));
}

fn write_ppu_mode(memory: &mut Memory, ppu_mode: PpuMode) {
    let status_register = memory.read(Address(ADDRESS_LCD_STATUS_REGISTER));
    memory.write(Address(ADDRESS_LCD_STATUS_REGISTER), set_bits(status_register, ppu_mode as u8, 0b0000_0011));
}

enum ObjectBackgroundPriority {
    Object, // Sprite is always rendered above background
    Background, // Background colors 1-3 overlay sprite, sprite is still rendered above color 0
}

struct Sprite {
    y_position: u8,
    x_position: u8,
    tile_number: u8,
    flags: SpriteFlags,
}

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

enum FetchStep {
    Paused,
    FetchTileNumber,
    FetchTileLow(u8),
    FetchTileHigh(u8, u8),
    Push([PixelColour; TILE_DIMENSION]),
}

pub struct Ppu {
    dot: usize, // Dot count in the current scanline
    sprite_buffer: Vec<Sprite>, // Sprite buffer for current scanline
    x_position: u8, // Pixels pushed in the current scanline
    fetcher_x_position: u8, // Temporary solution: fetchers need to keep track of their own internal x-position
    background_fifo: VecDeque<Pixel>,
    sprite_fifo: VecDeque<Pixel>,
    background_fetch_step: FetchStep,
    sprite_fetch_step: FetchStep,
    pub image_buffer: image::GrayImage,
    pixel_shifting_enabled: bool,
}

impl Ppu {
    pub fn init() -> Ppu {
        Ppu {
            dot: 0,
            sprite_buffer: Vec::<Sprite>::with_capacity(10),
            x_position: 0,
            fetcher_x_position: 0,
            background_fifo: VecDeque::<Pixel>::with_capacity(8),
            sprite_fifo: VecDeque::<Pixel>::with_capacity(8),
            background_fetch_step: FetchStep::FetchTileNumber,
            sprite_fetch_step: FetchStep::Paused,
            image_buffer: GrayImage::new(160, 144),
            pixel_shifting_enabled: true,
        }
    }

    pub fn step(&mut self, memory: &mut Memory) {
        let ppu_mode = read_ppu_mode(memory);
        let ly = memory.read(Address(ADDRESS_LY));
        let lyc = memory.read(Address(ADDRESS_LYC));
        write_coincidence_flag(memory, ly == lyc);

        // Initiate sprite fetch
        // TODO: Is there a cleaner way of doing this?
        match self.sprite_fetch_step {
            FetchStep::Paused => {
            },
            _ => {
                if self.sprite_buffer.iter().any(|s| s.x_position <= self.x_position + 8) {
                    self.background_fetch_step = FetchStep::Paused;
                    self.sprite_fetch_step = FetchStep::Paused;
                    self.pixel_shifting_enabled = false;
                }
            },
        };

        match ppu_mode {
            PpuMode::OamScan => {
                // Each sprite takes 2 dots to fetch, skip odd dots.
                if self.dot  % 2 == 0 {
                    let byte_offset = self.dot * 4;
                    let sprite_address = Address(0xFE00 + byte_offset as u16);
                    let sprite_memory = memory.read_range(sprite_address, 4);
                    let sprite = Sprite::from(sprite_memory);
                    let sprite_height = 8; // TODO: 8 in Normal Mode, 16 in Tall-Sprite-Mode

                    // Render conditions for sprite
                    if self.sprite_buffer.len() < 10 || sprite.x_position > 0 || ly + 16 >= sprite.y_position || ly + 16 <= sprite.y_position + sprite_height {
                        self.sprite_buffer.push(sprite);
                    }
                }

                self.dot += 1;
                if self.dot == DOTS_PER_OAM_SCAN {
                    self.x_position = 0;
                    self.fetcher_x_position = 0;
                    self.background_fifo.clear();
                    self.sprite_fifo.clear();
                    write_ppu_mode(memory, PpuMode::Drawing);
                }
            },
            PpuMode::Drawing => {
                let scy = memory.read(Address(ADDRESS_SCY)) as u16;

                match &self.background_fetch_step {
                    FetchStep::Paused => {
                        // No-op?
                    }
                    FetchStep::FetchTileNumber => {
                        let tile_map_area = read_bg_tile_map_area(memory);
                        let offset = 32 * (((ly as u16 + scy) & 0xFF) / 8);
                        let tile_number_address = tile_map_area as u16 + self.fetcher_x_position as u16 + offset;
                        // TODO: Account for scroll
                        // TODO: Are we fetching BG or window tile?
                        // let scx = 0;
                        
                        let tile_number = memory.read(Address(tile_number_address));
                        self.background_fetch_step = FetchStep::FetchTileLow(tile_number);
                    }
                    FetchStep::FetchTileLow(tile_number) => {
                        let tile_data_area = 0x8000 as u16; // Could also be 0x8800, depending upon LCDC bit 4;
                        let tile_offset = *tile_number as u16 * 16;
                        let tile_byte_offset =  2 * ((ly as u16 + scy) % 8) as u16;
                        let address = Address(tile_data_area + tile_offset + tile_byte_offset);
                        let tile_data_low = memory.read(address);
                        self.background_fetch_step = FetchStep::FetchTileHigh(*tile_number, tile_data_low);
                    },
                    FetchStep::FetchTileHigh(tile_number, tile_data_low) => {
                        let tile_data_area = 0x8000 as u16; // Could also be 0x8800, depending upon LCDC bit 4;
                        let tile_offset = *tile_number as u16 * 16;
                        let tile_byte_offset =  2 * ((ly as u16 + scy) % 8) as u16;
                        let address = Address(tile_data_area + tile_offset + tile_byte_offset + 1);
                        let tile_data_high = memory.read(address);
                        let pixel_colours = line_bytes_to_pixel_colours(*tile_data_low, tile_data_high);

                        self.background_fetch_step = FetchStep::Push(pixel_colours);
                    }
                    FetchStep::Push(pixel_colours) => {
                        if self.background_fifo.is_empty() {
                            let pixels = pixel_colours.into_iter().map(|colour| Pixel {
                                colour: *colour,
                                palette: Palette::Bgp,
                                priority: ObjectBackgroundPriority::Background // Irrelevant for background pixels
                            });
                            self.fetcher_x_position = (self.fetcher_x_position + 1) & 0x1F;
                            self.background_fifo.extend(pixels);
                            self.background_fetch_step = FetchStep::FetchTileNumber;
                        }
                    }
                };

                match &self.sprite_fetch_step {
                    FetchStep::Paused => {
                    },
                    FetchStep::FetchTileNumber => {
                        let tile_map_area = read_bg_tile_map_area(memory);
                        let offset = 32 * ((ly as u16 + scy) & 0xFF) / 8;
                        let tile_number_address = tile_map_area as u16 + self.x_position as u16 + offset as u16;
                        // TODO: Account for scroll
                        // TODO: Are we fetching BG or window tile?
                        // let scx = 0;
                        
                        let tile_number = memory.read(Address(tile_number_address));
                        self.sprite_fetch_step = FetchStep::FetchTileLow(tile_number);
                    }
                    FetchStep::FetchTileLow(tile_number) => {
                        let offset =  2 * ((ly as u16 + scy) % 8);
                        let address = Address(0x8000 + (*tile_number * 16) as u16 + offset as u16);
                        let tile_data_low = memory.read(address);
                        self.sprite_fetch_step = FetchStep::FetchTileHigh(*tile_number, tile_data_low);
                    },
                    FetchStep::FetchTileHigh(tile_number, tile_data_low) => {
                        let offset =  2 * ((ly as u16 + scy) % 8);
                        let address = Address(0x8000 + (*tile_number * 16) as u16 + offset as u16 + 1);
                        let tile_data_high = memory.read(address);
                        let pixel_colours = line_bytes_to_pixel_colours(tile_data_high, *tile_data_low);

                        self.sprite_fetch_step = FetchStep::Push(pixel_colours);
                    }
                    FetchStep::Push(pixel_colours) => {
                        let pixels = pixel_colours
                            .into_iter()
                            .map(|colour| Pixel {
                                colour: *colour,
                                palette: Palette::Bgp,
                                priority: ObjectBackgroundPriority::Background
                            })
                            .take(self.sprite_fifo.capacity() - self.sprite_fifo.len());
                        self.sprite_fifo.extend(pixels.into_iter());

                        self.sprite_fetch_step = FetchStep::Paused;
                        self.background_fetch_step = FetchStep::FetchTileNumber;
                        self.pixel_shifting_enabled = true;
                    }
                };

                if self.pixel_shifting_enabled && !self.background_fifo.is_empty() {
                    let background_pixel = self.background_fifo.pop_front().expect("Background FIFO should be non-empty");
                    let sprite_pixel = self.sprite_fifo.pop_front();
                    let mixed_pixel = background_pixel;

                    if let Some(_) = sprite_pixel {
                        self.image_buffer.put_pixel(self.x_position as u32, ly as u32, PixelColour::Black.to_grayscale());
                    } else {
                        self.image_buffer.put_pixel(self.x_position as u32, ly as u32, mixed_pixel.colour.to_grayscale());
                    }
                    self.x_position += 1;

                    if self.x_position == PIXELS_PER_SCANLINE {
                        write_ppu_mode(memory, PpuMode::HorizontalBlank);
                        self.sprite_buffer.clear();
                    }
                }

                self.dot += 1;
            },
            PpuMode::HorizontalBlank => {
                self.dot += 1;
                if self.dot >= DOTS_PER_SCANLINE {
                    self.dot = 0;
                    memory.write(Address(ADDRESS_LY), ly + 1);
                    let ppu_mode = if ly as usize >= SCANLINES_PER_FRAME - SCANLINES_PER_VERTICAL_BLANK {
                        PpuMode::VerticalBlank
                    } else {
                        PpuMode::OamScan
                    };
                    write_ppu_mode(memory, ppu_mode);
                }
            },
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

const LINE_BYTES: usize = 2;
const TILE_LINES: usize = 8;
const TILE_DIMENSION: usize = 8;
const TILE_COUNT: usize = 128;
const TILE_MAP_COUNT: usize = 1024;

#[derive(Copy, Clone)]
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

        pixels[i] = PixelColour::try_from((msb << 1) | lsb).expect("Only 2 bits should be passed to PixelColour::try_from");
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
