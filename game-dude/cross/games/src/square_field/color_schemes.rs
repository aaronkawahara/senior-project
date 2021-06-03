use stm32l4p5_hal::dma2d::Dma2d;

use crate::common::DiscreteSelection;

#[derive(Clone, Copy)]
pub enum ColorSchemes {
    Normal,
    Neon,
    Monochrome,
    Bright,
}

impl Default for ColorSchemes {
    fn default() -> Self {
        Self::Normal
    }
}

impl ColorSchemes {
    const NORMAL_BACKGROUND: u32 = 0xff_ff_ff_ff;
    const NEON_BACKGROUND: u32 = 0x00_00_00_00_00;
    const MONOCHROME_BACKGROUND: u32 = 0xff_ff_ff_ff;
    const BRIGHT_BACKGROUND: u32 = 0xff_ff_ff_ff;

    pub fn fill_background(&self, dma2d: &Dma2d) {
        dma2d.fill_background(
            match self {
                ColorSchemes::Normal => Self::NORMAL_BACKGROUND,
                ColorSchemes::Neon => Self::NEON_BACKGROUND,
                ColorSchemes::Monochrome => Self::MONOCHROME_BACKGROUND,
                ColorSchemes::Bright => Self::BRIGHT_BACKGROUND,
            }, 
            lcd::SCREEN_WIDTH_U16 / 4, 
            lcd::SCREEN_HEIGHT_U16
        );
    }
}

impl DiscreteSelection for ColorSchemes {
    fn next(self) -> Self {
        match self {
            ColorSchemes::Normal => ColorSchemes::Neon,
            ColorSchemes::Neon => ColorSchemes::Monochrome,
            ColorSchemes::Monochrome => ColorSchemes::Bright,
            ColorSchemes::Bright => ColorSchemes::Normal,
        }
    }

    fn previous(self) -> Self {
        match self {
            ColorSchemes::Normal => ColorSchemes::Bright,
            ColorSchemes::Neon => ColorSchemes::Normal,
            ColorSchemes::Monochrome => ColorSchemes::Neon,
            ColorSchemes::Bright => ColorSchemes::Monochrome,
        }
    }
}