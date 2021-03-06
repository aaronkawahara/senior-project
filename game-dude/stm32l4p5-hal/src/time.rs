/// Bits per second
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Bps(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct Hertz(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct KiloHertz(pub u32);

#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct MegaHertz(pub u32);

/// Extension trait that adds convenience methods to the `u32` type
pub trait U32Ext {
    /// Wrap in `Bps`
    fn bps(self) -> Bps;

    /// Wrap in `Hertz`
    fn hz(self) -> Hertz;

    /// Wrap in `KiloHertz`
    fn khz(self) -> KiloHertz;

    /// Wrap in `MegaHertz`
    fn mhz(self) -> MegaHertz;

    /// Wrap in `MilliSeconds`
    fn ms(self) -> MilliSeconds;
}

impl U32Ext for u32 {
    fn bps(self) -> Bps {
        Bps(self)
    }

    fn hz(self) -> Hertz {
        Hertz(self)
    }

    fn khz(self) -> KiloHertz {
        KiloHertz(self)
    }

    fn mhz(self) -> MegaHertz {
        MegaHertz(self)
    }

    fn ms(self) -> MilliSeconds {
        MilliSeconds(self)
    }
}

impl From<u32> for Hertz {
    fn from(t: u32) -> Self {
        t.hz()
    }
}

impl Into<Hertz> for KiloHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000)
    }
}

impl Into<Hertz> for MegaHertz {
    fn into(self) -> Hertz {
        Hertz(self.0 * 1_000_000)
    }
}

impl Into<KiloHertz> for MegaHertz {
    fn into(self) -> KiloHertz {
        KiloHertz(self.0 * 1_000)
    }
}

impl From<Hertz> for u32 {
    fn from(t: Hertz) -> Self {
        t.0
    }
}

/// Time unit
#[derive(PartialEq, PartialOrd, Clone, Copy, Debug)]
pub struct MilliSeconds(pub u32);