#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `EPI_CFG_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_CFG_MODER {
    #[doc = "General Purpose"]
    EPI_CFG_MODE_NONE,
    #[doc = "SDRAM"]
    EPI_CFG_MODE_SDRAM,
    #[doc = "8-Bit Host-Bus (HB8)"]
    EPI_CFG_MODE_HB8,
    #[doc = "16-Bit Host-Bus (HB16)"]
    EPI_CFG_MODE_HB16,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_CFG_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_CFG_MODER::EPI_CFG_MODE_NONE => 0,
            EPI_CFG_MODER::EPI_CFG_MODE_SDRAM => 1,
            EPI_CFG_MODER::EPI_CFG_MODE_HB8 => 2,
            EPI_CFG_MODER::EPI_CFG_MODE_HB16 => 3,
            EPI_CFG_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_CFG_MODER {
        match value {
            0 => EPI_CFG_MODER::EPI_CFG_MODE_NONE,
            1 => EPI_CFG_MODER::EPI_CFG_MODE_SDRAM,
            2 => EPI_CFG_MODER::EPI_CFG_MODE_HB8,
            3 => EPI_CFG_MODER::EPI_CFG_MODE_HB16,
            i => EPI_CFG_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_NONE`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_none(&self) -> bool {
        *self == EPI_CFG_MODER::EPI_CFG_MODE_NONE
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_SDRAM`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_sdram(&self) -> bool {
        *self == EPI_CFG_MODER::EPI_CFG_MODE_SDRAM
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_HB8`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_hb8(&self) -> bool {
        *self == EPI_CFG_MODER::EPI_CFG_MODE_HB8
    }
    #[doc = "Checks if the value of the field is `EPI_CFG_MODE_HB16`"]
    #[inline(always)]
    pub fn is_epi_cfg_mode_hb16(&self) -> bool {
        *self == EPI_CFG_MODER::EPI_CFG_MODE_HB16
    }
}
#[doc = "Values that can be written to the field `EPI_CFG_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_CFG_MODEW {
    #[doc = "General Purpose"]
    EPI_CFG_MODE_NONE,
    #[doc = "SDRAM"]
    EPI_CFG_MODE_SDRAM,
    #[doc = "8-Bit Host-Bus (HB8)"]
    EPI_CFG_MODE_HB8,
    #[doc = "16-Bit Host-Bus (HB16)"]
    EPI_CFG_MODE_HB16,
}
impl EPI_CFG_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_CFG_MODEW::EPI_CFG_MODE_NONE => 0,
            EPI_CFG_MODEW::EPI_CFG_MODE_SDRAM => 1,
            EPI_CFG_MODEW::EPI_CFG_MODE_HB8 => 2,
            EPI_CFG_MODEW::EPI_CFG_MODE_HB16 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_CFG_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_CFG_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_CFG_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "General Purpose"]
    #[inline(always)]
    pub fn epi_cfg_mode_none(self) -> &'a mut W {
        self.variant(EPI_CFG_MODEW::EPI_CFG_MODE_NONE)
    }
    #[doc = "SDRAM"]
    #[inline(always)]
    pub fn epi_cfg_mode_sdram(self) -> &'a mut W {
        self.variant(EPI_CFG_MODEW::EPI_CFG_MODE_SDRAM)
    }
    #[doc = "8-Bit Host-Bus (HB8)"]
    #[inline(always)]
    pub fn epi_cfg_mode_hb8(self) -> &'a mut W {
        self.variant(EPI_CFG_MODEW::EPI_CFG_MODE_HB8)
    }
    #[doc = "16-Bit Host-Bus (HB16)"]
    #[inline(always)]
    pub fn epi_cfg_mode_hb16(self) -> &'a mut W {
        self.variant(EPI_CFG_MODEW::EPI_CFG_MODE_HB16)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_CFG_BLKENR {
    bits: bool,
}
impl EPI_CFG_BLKENR {
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
pub struct _EPI_CFG_BLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_CFG_BLKENW<'a> {
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
pub struct EPI_CFG_INTDIVR {
    bits: bool,
}
impl EPI_CFG_INTDIVR {
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
pub struct _EPI_CFG_INTDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_CFG_INTDIVW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn epi_cfg_mode(&self) -> EPI_CFG_MODER {
        EPI_CFG_MODER::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn epi_cfg_blken(&self) -> EPI_CFG_BLKENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EPI_CFG_BLKENR { bits }
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn epi_cfg_intdiv(&self) -> EPI_CFG_INTDIVR {
        let bits = ((self.bits >> 8) & 1) != 0;
        EPI_CFG_INTDIVR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Mode Select"]
    #[inline(always)]
    pub fn epi_cfg_mode(&mut self) -> _EPI_CFG_MODEW {
        _EPI_CFG_MODEW { w: self }
    }
    #[doc = "Bit 4 - Block Enable"]
    #[inline(always)]
    pub fn epi_cfg_blken(&mut self) -> _EPI_CFG_BLKENW {
        _EPI_CFG_BLKENW { w: self }
    }
    #[doc = "Bit 8 - Integer Clock Divider Enable"]
    #[inline(always)]
    pub fn epi_cfg_intdiv(&mut self) -> _EPI_CFG_INTDIVW {
        _EPI_CFG_INTDIVW { w: self }
    }
}
