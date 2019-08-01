#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HB16TIME4 {
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
#[doc = r"Value of the field"]
pub struct EPI_HB16TIME4_RDWSMR {
    bits: bool,
}
impl EPI_HB16TIME4_RDWSMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16TIME4_RDWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16TIME4_RDWSMW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16TIME4_WRWSMR {
    bits: bool,
}
impl EPI_HB16TIME4_WRWSMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16TIME4_WRWSMW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16TIME4_WRWSMW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16TIME4_CAPWIDTHR {
    bits: u8,
}
impl EPI_HB16TIME4_CAPWIDTHR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16TIME4_CAPWIDTHW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16TIME4_CAPWIDTHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_HB16TIME4_PSRAMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16TIME4_PSRAMSZR {
    #[doc = "No row size limitation"]
    EPI_HB16TIME4_PSRAMSZ_0,
    #[doc = "128 B"]
    EPI_HB16TIME4_PSRAMSZ_128B,
    #[doc = "256 B"]
    EPI_HB16TIME4_PSRAMSZ_256B,
    #[doc = "512 B"]
    EPI_HB16TIME4_PSRAMSZ_512B,
    #[doc = "1024 B"]
    EPI_HB16TIME4_PSRAMSZ_1KB,
    #[doc = "2048 B"]
    EPI_HB16TIME4_PSRAMSZ_2KB,
    #[doc = "4096 B"]
    EPI_HB16TIME4_PSRAMSZ_4KB,
    #[doc = "8192 B"]
    EPI_HB16TIME4_PSRAMSZ_8KB,
}
impl EPI_HB16TIME4_PSRAMSZR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_0 => 0,
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_128B => 1,
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_256B => 2,
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_512B => 3,
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_1KB => 4,
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_2KB => 5,
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_4KB => 6,
            EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_8KB => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_HB16TIME4_PSRAMSZR {
        match value {
            0 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_0,
            1 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_128B,
            2 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_256B,
            3 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_512B,
            4 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_1KB,
            5 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_2KB,
            6 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_4KB,
            7 => EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_8KB,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_0`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_0(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_0
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_128B`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_128b(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_128B
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_256B`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_256b(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_256B
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_512B`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_512b(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_512B
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_1KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_1kb(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_1KB
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_2KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_2kb(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_2KB
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_4KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_4kb(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_4KB
    }
    #[doc = "Checks if the value of the field is `EPI_HB16TIME4_PSRAMSZ_8KB`"]
    #[inline(always)]
    pub fn is_epi_hb16time4_psramsz_8kb(&self) -> bool {
        *self == EPI_HB16TIME4_PSRAMSZR::EPI_HB16TIME4_PSRAMSZ_8KB
    }
}
#[doc = "Values that can be written to the field `EPI_HB16TIME4_PSRAMSZ`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_HB16TIME4_PSRAMSZW {
    #[doc = "No row size limitation"]
    EPI_HB16TIME4_PSRAMSZ_0,
    #[doc = "128 B"]
    EPI_HB16TIME4_PSRAMSZ_128B,
    #[doc = "256 B"]
    EPI_HB16TIME4_PSRAMSZ_256B,
    #[doc = "512 B"]
    EPI_HB16TIME4_PSRAMSZ_512B,
    #[doc = "1024 B"]
    EPI_HB16TIME4_PSRAMSZ_1KB,
    #[doc = "2048 B"]
    EPI_HB16TIME4_PSRAMSZ_2KB,
    #[doc = "4096 B"]
    EPI_HB16TIME4_PSRAMSZ_4KB,
    #[doc = "8192 B"]
    EPI_HB16TIME4_PSRAMSZ_8KB,
}
impl EPI_HB16TIME4_PSRAMSZW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_0 => 0,
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_128B => 1,
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_256B => 2,
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_512B => 3,
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_1KB => 4,
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_2KB => 5,
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_4KB => 6,
            EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_8KB => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16TIME4_PSRAMSZW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16TIME4_PSRAMSZW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_HB16TIME4_PSRAMSZW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No row size limitation"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_0(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_0)
    }
    #[doc = "128 B"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_128b(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_128B)
    }
    #[doc = "256 B"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_256b(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_256B)
    }
    #[doc = "512 B"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_512b(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_512B)
    }
    #[doc = "1024 B"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_1kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_1KB)
    }
    #[doc = "2048 B"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_2kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_2KB)
    }
    #[doc = "4096 B"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_4kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_4KB)
    }
    #[doc = "8192 B"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz_8kb(self) -> &'a mut W {
        self.variant(EPI_HB16TIME4_PSRAMSZW::EPI_HB16TIME4_PSRAMSZ_8KB)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 16);
        self.w.bits |= ((value as u32) & 7) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_HB16TIME4_IRDYDLYR {
    bits: u8,
}
impl EPI_HB16TIME4_IRDYDLYR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EPI_HB16TIME4_IRDYDLYW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_HB16TIME4_IRDYDLYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 24);
        self.w.bits |= ((value as u32) & 3) << 24;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - CS3n Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time4_rdwsm(&self) -> EPI_HB16TIME4_RDWSMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EPI_HB16TIME4_RDWSMR { bits }
    }
    #[doc = "Bit 4 - CS3n Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time4_wrwsm(&self) -> EPI_HB16TIME4_WRWSMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_HB16TIME4_WRWSMR { bits }
    }
    #[doc = "Bits 12:13 - CS3n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb16time4_capwidth(&self) -> EPI_HB16TIME4_CAPWIDTHR {
        let bits = ((self.bits >> 12) & 3) as u8;
        EPI_HB16TIME4_CAPWIDTHR { bits }
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz(&self) -> EPI_HB16TIME4_PSRAMSZR {
        EPI_HB16TIME4_PSRAMSZR::_from(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bits 24:25 - CS3n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb16time4_irdydly(&self) -> EPI_HB16TIME4_IRDYDLYR {
        let bits = ((self.bits >> 24) & 3) as u8;
        EPI_HB16TIME4_IRDYDLYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - CS3n Read Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time4_rdwsm(&mut self) -> _EPI_HB16TIME4_RDWSMW {
        _EPI_HB16TIME4_RDWSMW { w: self }
    }
    #[doc = "Bit 4 - CS3n Write Wait State Minus One"]
    #[inline(always)]
    pub fn epi_hb16time4_wrwsm(&mut self) -> _EPI_HB16TIME4_WRWSMW {
        _EPI_HB16TIME4_WRWSMW { w: self }
    }
    #[doc = "Bits 12:13 - CS3n Inter-transfer Capture Width"]
    #[inline(always)]
    pub fn epi_hb16time4_capwidth(&mut self) -> _EPI_HB16TIME4_CAPWIDTHW {
        _EPI_HB16TIME4_CAPWIDTHW { w: self }
    }
    #[doc = "Bits 16:18 - PSRAM Row Size"]
    #[inline(always)]
    pub fn epi_hb16time4_psramsz(&mut self) -> _EPI_HB16TIME4_PSRAMSZW {
        _EPI_HB16TIME4_PSRAMSZW { w: self }
    }
    #[doc = "Bits 24:25 - CS3n Input Ready Delay"]
    #[inline(always)]
    pub fn epi_hb16time4_irdydly(&mut self) -> _EPI_HB16TIME4_IRDYDLYW {
        _EPI_HB16TIME4_IRDYDLYW { w: self }
    }
}
