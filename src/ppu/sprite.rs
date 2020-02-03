pub struct Sprite
{
    pub index    : u32,      // Index of sprite, 0 - 127
    pub xcoord   : u32,      // X coordinate, top left for text sprites
    pub ycoord   : u32,      // Y coordinate, center for affine sprites
    pub width    : u32,      // Width in pixels
    pub height   : u32,      // Height in pixels 
    pub mode     : u32,      // 0 - normal, 1 - semi-transparent, 2 - window
    pub double_f : bool,     // Double size flag
    pub affine_f : bool,     // Rotational / scaling flag
    pub mosaic_f : bool,     // Mosaic flag
    pub palette_f: bool,     // Palette type, 1 - 256, 2 - 16
    pub hflip    : bool,     // Horizontal flip bit
    pub vflip    : bool,     // Vertical flip bit
    pub affine_i : u32,      // Rotation / scaling data index
    pub tile_n   : u32,      // Tile number
    pub priority : u32,
    pub palette_n: u32,      // Palette number (for 16 color sprites)

    // Transform matrix used for Rotation and scaling
    pub matrix   : (u16, u16, u16, u16) 
}