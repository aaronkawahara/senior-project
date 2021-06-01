use crate::stm32::{dma2d, DMA2D, RCC};

pub trait Dma2dExt {
    fn constrain(self) -> Dma2d;
}

impl Dma2dExt for DMA2D {
    fn constrain(self) -> Dma2d {
        Dma2d {
            registers: unsafe { &*DMA2D::ptr() },
            buffer_address: None,
            display_width: None,
        }
    }
}

pub struct Dma2d {
    registers: &'static dma2d::RegisterBlock,
    buffer_address: Option<u32>,
    display_width: Option<u16>,
}

impl Dma2d {
    pub fn init(&mut self, mode: dma2d::cr::MODE_A, display_width: u16, buffer_start_address: u32) {
        self.display_width = Some(display_width);
        self.buffer_address = Some(buffer_start_address);

        let rcc = unsafe { &*RCC::ptr() };
        rcc.ahb1enr.modify(|_, w| w.dma2den().set_bit());
        while rcc.ahb1enr.read().dma2den().bit_is_clear() {}
    }

    pub fn fill_background(&self, argb: u32, width: u16, height: u16) {
        while self.registers.cr.read().start().bit_is_set() {}

        self.registers
            .cr
            .modify(|_, w| w.mode().variant(dma2d::cr::MODE_A::REGISTERTOMEMORY));
        self.registers
            .opfccr
            .modify(|_, w| w.cm().variant(dma2d::opfccr::CM_A::ARGB8888));
        self.registers.oor.modify(|_, w| unsafe { w.lo().bits(0) });
        self.registers
            .nlr
            .modify(|_, w| w.pl().bits(width).nl().bits(height));
        self.registers
            .omar
            .modify(|_, w| unsafe { w.bits(self.buffer_address.unwrap()) });
        self.registers.ocolr.modify(|_, w| unsafe { w.bits(argb) });
        self.registers.cr.modify(|_, w| w.start().set_bit());
    }

    pub fn fill_rgb8_rect(&self, rgb: u8, x: u16, y: u16, width: u16, height: u16) {
        let repeat_rgb: u32 = u32::from(rgb) << 8 | u32::from(rgb);
        let width = width >> 1;

        while self.registers.cr.read().start().bit_is_set() {}

        self.registers
            .cr
            .modify(|_, w| w.mode().variant(dma2d::cr::MODE_A::REGISTERTOMEMORY));
        self.registers
            .opfccr
            .modify(|_, w| w.cm().variant(dma2d::opfccr::CM_A::ARGB4444));
        self.registers.omar.modify(|_, w| unsafe {
            w.bits(self.buffer_address.unwrap() + u32::from(x + y * self.display_width.unwrap()))
        });
        self.registers
            .oor
            .modify(|_, w| unsafe { w.lo().bits((self.display_width.unwrap() >> 1) - width) });
        self.registers
            .nlr
            .modify(|_, w| w.pl().bits(width).nl().bits(height));
        self.registers
            .ocolr
            .modify(|_, w| unsafe { w.bits(repeat_rgb) });
        self.registers.cr.modify(|_, w| w.start().set_bit());
    }

    pub fn draw_rgb8_image(&self, image_address: u32, x: u32, y: u32, width: u16, height: u16) {
        while self.registers.cr.read().start().bit_is_set() {}

        self.registers
            .cr
            .modify(|_, w| w.mode().variant(dma2d::cr::MODE_A::MEMORYTOMEMORY));
        self.registers
            .fgmar
            .modify(|_, w| unsafe { w.bits(image_address) });
        self.registers.omar.modify(|_, w| unsafe {
            w.bits(self.buffer_address.unwrap() + (x + y * (self.display_width.unwrap() as u32)))
        });
        self.registers.fgpfccr.modify(|_, w| w.cm().l8());
        self.registers
            .nlr
            .modify(|_, w| w.pl().bits(width).nl().bits(height));
        self.registers.fgor.modify(|_, w| unsafe { w.lo().bits(0) });
        self.registers
            .oor
            .modify(|_, w| unsafe { w.lo().bits(self.display_width.unwrap() - width) });
        self.registers.cr.modify(|_, w| w.start().set_bit());
    }
}
