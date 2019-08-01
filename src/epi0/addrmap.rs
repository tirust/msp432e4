#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADDRMAP {
    #[doc = r"Modifies the contents of the register"]
    #[inline(always)]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        self.register.set(f(&R { bits }, &mut W { bits }).bits);
    }
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r"Writes to the register"]
    #[inline(always)]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        self.register.set(
            f(&mut W {
                bits: Self::reset_value(),
            })
            .bits,
        );
    }
    #[doc = r"Reset value of the register"]
    #[inline(always)]
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `EPI_ADDRMAP_ERADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ERADRR {
    #[doc = "Not mapped"]
    EPI_ADDRMAP_ERADR_NONE,
    #[doc = "At 0x6000.0000"]
    EPI_ADDRMAP_ERADR_6000,
    #[doc = "At 0x8000.0000"]
    EPI_ADDRMAP_ERADR_8000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    EPI_ADDRMAP_ERADR_HBQS,
}
impl EPI_ADDRMAP_ERADRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_NONE => 0,
            EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_6000 => 1,
            EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_8000 => 2,
            EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_HBQS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_ADDRMAP_ERADRR {
        match value {
            0 => EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_NONE,
            1 => EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_6000,
            2 => EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_8000,
            3 => EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_NONE`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_none(&self) -> bool {
        *self == EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_6000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_6000(&self) -> bool {
        *self == EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_6000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_8000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_8000(&self) -> bool {
        *self == EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_8000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERADR_HBQS`"]
    #[inline(always)]
    pub fn is_epi_addrmap_eradr_hbqs(&self) -> bool {
        *self == EPI_ADDRMAP_ERADRR::EPI_ADDRMAP_ERADR_HBQS
    }
}
#[doc = "Values that can be written to the field `EPI_ADDRMAP_ERADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ERADRW {
    #[doc = "Not mapped"]
    EPI_ADDRMAP_ERADR_NONE,
    #[doc = "At 0x6000.0000"]
    EPI_ADDRMAP_ERADR_6000,
    #[doc = "At 0x8000.0000"]
    EPI_ADDRMAP_ERADR_8000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    EPI_ADDRMAP_ERADR_HBQS,
}
impl EPI_ADDRMAP_ERADRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_NONE => 0,
            EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_6000 => 1,
            EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_8000 => 2,
            EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_HBQS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_ADDRMAP_ERADRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_ADDRMAP_ERADRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_ADDRMAP_ERADRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_none(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_NONE)
    }
    #[doc = "At 0x6000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_6000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_6000)
    }
    #[doc = "At 0x8000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_8000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_8000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS0n maps to 0x6000.0000 and CS1n maps to 0x8000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_eradr_hbqs(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERADRW::EPI_ADDRMAP_ERADR_HBQS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_ADDRMAP_ERSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ERSZR {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_ERSZ_256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_ERSZ_64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_ERSZ_16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    EPI_ADDRMAP_ERSZ_256MB,
}
impl EPI_ADDRMAP_ERSZR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_256B => 0,
            EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_64KB => 1,
            EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_16MB => 2,
            EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_256MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_ADDRMAP_ERSZR {
        match value {
            0 => EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_256B,
            1 => EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_64KB,
            2 => EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_16MB,
            3 => EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_256b(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_64KB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_64kb(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_64KB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_16MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_16mb(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ERSZ_256MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ersz_256mb(&self) -> bool {
        *self == EPI_ADDRMAP_ERSZR::EPI_ADDRMAP_ERSZ_256MB
    }
}
#[doc = "Values that can be written to the field `EPI_ADDRMAP_ERSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ERSZW {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_ERSZ_256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_ERSZ_64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_ERSZ_16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    EPI_ADDRMAP_ERSZ_256MB,
}
impl EPI_ADDRMAP_ERSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_256B => 0,
            EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_64KB => 1,
            EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_16MB => 2,
            EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_256MB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_ADDRMAP_ERSZW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_ADDRMAP_ERSZW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_ADDRMAP_ERSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_256b(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_64kb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_16mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ersz_256mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ERSZW::EPI_ADDRMAP_ERSZ_256MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_ADDRMAP_EPADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_EPADRR {
    #[doc = "Not mapped"]
    EPI_ADDRMAP_EPADR_NONE,
    #[doc = "At 0xA000.0000"]
    EPI_ADDRMAP_EPADR_A000,
    #[doc = "At 0xC000.0000"]
    EPI_ADDRMAP_EPADR_C000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    EPI_ADDRMAP_EPADR_HBQS,
}
impl EPI_ADDRMAP_EPADRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_NONE => 0,
            EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_A000 => 1,
            EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_C000 => 2,
            EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_HBQS => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_ADDRMAP_EPADRR {
        match value {
            0 => EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_NONE,
            1 => EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_A000,
            2 => EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_C000,
            3 => EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_HBQS,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_NONE`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_none(&self) -> bool {
        *self == EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_A000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_a000(&self) -> bool {
        *self == EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_A000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_C000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_c000(&self) -> bool {
        *self == EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_C000
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPADR_HBQS`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epadr_hbqs(&self) -> bool {
        *self == EPI_ADDRMAP_EPADRR::EPI_ADDRMAP_EPADR_HBQS
    }
}
#[doc = "Values that can be written to the field `EPI_ADDRMAP_EPADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_EPADRW {
    #[doc = "Not mapped"]
    EPI_ADDRMAP_EPADR_NONE,
    #[doc = "At 0xA000.0000"]
    EPI_ADDRMAP_EPADR_A000,
    #[doc = "At 0xC000.0000"]
    EPI_ADDRMAP_EPADR_C000,
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    EPI_ADDRMAP_EPADR_HBQS,
}
impl EPI_ADDRMAP_EPADRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_NONE => 0,
            EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_A000 => 1,
            EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_C000 => 2,
            EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_HBQS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_ADDRMAP_EPADRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_ADDRMAP_EPADRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_ADDRMAP_EPADRW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_none(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_NONE)
    }
    #[doc = "At 0xA000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_a000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_A000)
    }
    #[doc = "At 0xC000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_c000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_C000)
    }
    #[doc = "Only to be used with Host Bus quad chip select. In quad chip select mode, CS2n maps to 0xA000.0000 and CS3n maps to 0xC000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_epadr_hbqs(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPADRW::EPI_ADDRMAP_EPADR_HBQS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_ADDRMAP_EPSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_EPSZR {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_EPSZ_256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_EPSZ_64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_EPSZ_16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    EPI_ADDRMAP_EPSZ_256MB,
}
impl EPI_ADDRMAP_EPSZR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_256B => 0,
            EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_64KB => 1,
            EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_16MB => 2,
            EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_256MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_ADDRMAP_EPSZR {
        match value {
            0 => EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_256B,
            1 => EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_64KB,
            2 => EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_16MB,
            3 => EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_256b(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_64KB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_64kb(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_64KB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_16MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_16mb(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_EPSZ_256MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_epsz_256mb(&self) -> bool {
        *self == EPI_ADDRMAP_EPSZR::EPI_ADDRMAP_EPSZ_256MB
    }
}
#[doc = "Values that can be written to the field `EPI_ADDRMAP_EPSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_EPSZW {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_EPSZ_256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_EPSZ_64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_EPSZ_16MB,
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    EPI_ADDRMAP_EPSZ_256MB,
}
impl EPI_ADDRMAP_EPSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_256B => 0,
            EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_64KB => 1,
            EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_16MB => 2,
            EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_256MB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_ADDRMAP_EPSZW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_ADDRMAP_EPSZW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_ADDRMAP_EPSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_256b(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_64kb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_16mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_16MB)
    }
    #[doc = "256 MB; lower address range: 0x000.0000 to 0xFFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_epsz_256mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_EPSZW::EPI_ADDRMAP_EPSZ_256MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_ADDRMAP_ECADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ECADRR {
    #[doc = "Not mapped"]
    EPI_ADDRMAP_ECADR_NONE,
    #[doc = "At 0x1000.0000"]
    EPI_ADDRMAP_ECADR_1000,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_ADDRMAP_ECADRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ECADRR::EPI_ADDRMAP_ECADR_NONE => 0,
            EPI_ADDRMAP_ECADRR::EPI_ADDRMAP_ECADR_1000 => 1,
            EPI_ADDRMAP_ECADRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_ADDRMAP_ECADRR {
        match value {
            0 => EPI_ADDRMAP_ECADRR::EPI_ADDRMAP_ECADR_NONE,
            1 => EPI_ADDRMAP_ECADRR::EPI_ADDRMAP_ECADR_1000,
            i => EPI_ADDRMAP_ECADRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECADR_NONE`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecadr_none(&self) -> bool {
        *self == EPI_ADDRMAP_ECADRR::EPI_ADDRMAP_ECADR_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECADR_1000`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecadr_1000(&self) -> bool {
        *self == EPI_ADDRMAP_ECADRR::EPI_ADDRMAP_ECADR_1000
    }
}
#[doc = "Values that can be written to the field `EPI_ADDRMAP_ECADR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ECADRW {
    #[doc = "Not mapped"]
    EPI_ADDRMAP_ECADR_NONE,
    #[doc = "At 0x1000.0000"]
    EPI_ADDRMAP_ECADR_1000,
}
impl EPI_ADDRMAP_ECADRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ECADRW::EPI_ADDRMAP_ECADR_NONE => 0,
            EPI_ADDRMAP_ECADRW::EPI_ADDRMAP_ECADR_1000 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_ADDRMAP_ECADRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_ADDRMAP_ECADRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_ADDRMAP_ECADRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Not mapped"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr_none(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECADRW::EPI_ADDRMAP_ECADR_NONE)
    }
    #[doc = "At 0x1000.0000"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr_1000(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECADRW::EPI_ADDRMAP_ECADR_1000)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_ADDRMAP_ECSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ECSZR {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_ECSZ_256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_ECSZ_64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_ECSZ_16MB,
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    EPI_ADDRMAP_ECSZ_256MB,
}
impl EPI_ADDRMAP_ECSZR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_256B => 0,
            EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_64KB => 1,
            EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_16MB => 2,
            EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_256MB => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_ADDRMAP_ECSZR {
        match value {
            0 => EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_256B,
            1 => EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_64KB,
            2 => EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_16MB,
            3 => EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_256MB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_256b(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_64KB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_64kb(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_64KB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_16MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_16mb(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_16MB
    }
    #[doc = "Checks if the value of the field is `EPI_ADDRMAP_ECSZ_256MB`"]
    #[inline(always)]
    pub fn is_epi_addrmap_ecsz_256mb(&self) -> bool {
        *self == EPI_ADDRMAP_ECSZR::EPI_ADDRMAP_ECSZ_256MB
    }
}
#[doc = "Values that can be written to the field `EPI_ADDRMAP_ECSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_ADDRMAP_ECSZW {
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    EPI_ADDRMAP_ECSZ_256B,
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    EPI_ADDRMAP_ECSZ_64KB,
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    EPI_ADDRMAP_ECSZ_16MB,
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    EPI_ADDRMAP_ECSZ_256MB,
}
impl EPI_ADDRMAP_ECSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_256B => 0,
            EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_64KB => 1,
            EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_16MB => 2,
            EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_256MB => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_ADDRMAP_ECSZW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_ADDRMAP_ECSZW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_ADDRMAP_ECSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "256 bytes; lower address range: 0x00 to 0xFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_256b(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_256B)
    }
    #[doc = "64 KB; lower address range: 0x0000 to 0xFFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_64kb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_64KB)
    }
    #[doc = "16 MB; lower address range: 0x00.0000 to 0xFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_16mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_16MB)
    }
    #[doc = "256MB; lower address range: 0x000.0000 to 0x0FFF.FFFF"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz_256mb(self) -> &'a mut W {
        self.variant(EPI_ADDRMAP_ECSZW::EPI_ADDRMAP_ECSZ_256MB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn epi_addrmap_eradr(&self) -> EPI_ADDRMAP_ERADRR {
        EPI_ADDRMAP_ERADRR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn epi_addrmap_ersz(&self) -> EPI_ADDRMAP_ERSZR {
        EPI_ADDRMAP_ERSZR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epi_addrmap_epadr(&self) -> EPI_ADDRMAP_EPADRR {
        EPI_ADDRMAP_EPADRR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epi_addrmap_epsz(&self) -> EPI_ADDRMAP_EPSZR {
        EPI_ADDRMAP_EPSZR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr(&self) -> EPI_ADDRMAP_ECADRR {
        EPI_ADDRMAP_ECADRR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz(&self) -> EPI_ADDRMAP_ECSZR {
        EPI_ADDRMAP_ECSZR::_from(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - External RAM Address"]
    #[inline(always)]
    pub fn epi_addrmap_eradr(&mut self) -> _EPI_ADDRMAP_ERADRW {
        _EPI_ADDRMAP_ERADRW { w: self }
    }
    #[doc = "Bits 2:3 - External RAM Size"]
    #[inline(always)]
    pub fn epi_addrmap_ersz(&mut self) -> _EPI_ADDRMAP_ERSZW {
        _EPI_ADDRMAP_ERSZW { w: self }
    }
    #[doc = "Bits 4:5 - External Peripheral Address"]
    #[inline(always)]
    pub fn epi_addrmap_epadr(&mut self) -> _EPI_ADDRMAP_EPADRW {
        _EPI_ADDRMAP_EPADRW { w: self }
    }
    #[doc = "Bits 6:7 - External Peripheral Size"]
    #[inline(always)]
    pub fn epi_addrmap_epsz(&mut self) -> _EPI_ADDRMAP_EPSZW {
        _EPI_ADDRMAP_EPSZW { w: self }
    }
    #[doc = "Bits 8:9 - External Code Address"]
    #[inline(always)]
    pub fn epi_addrmap_ecadr(&mut self) -> _EPI_ADDRMAP_ECADRW {
        _EPI_ADDRMAP_ECADRW { w: self }
    }
    #[doc = "Bits 10:11 - External Code Size"]
    #[inline(always)]
    pub fn epi_addrmap_ecsz(&mut self) -> _EPI_ADDRMAP_ECSZW {
        _EPI_ADDRMAP_ECSZW { w: self }
    }
}
