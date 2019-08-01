#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PP {
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
pub struct SSI_PP_HSCLKR {
    bits: bool,
}
impl SSI_PP_HSCLKR {
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
pub struct _SSI_PP_HSCLKW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_PP_HSCLKW<'a> {
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
#[doc = "Possible values of the field `SSI_PP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_PP_MODER {
    #[doc = "Legacy SSI mode"]
    SSI_PP_MODE_LEGACY,
    #[doc = "Legacy mode, Advanced SSI mode and Bi-SSI mode enabled"]
    SSI_PP_MODE_ADVBI,
    #[doc = "Legacy mode, Advanced mode, Bi-SSI and Quad-SSI mode enabled"]
    SSI_PP_MODE_ADVBIQUAD,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SSI_PP_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SSI_PP_MODER::SSI_PP_MODE_LEGACY => 0,
            SSI_PP_MODER::SSI_PP_MODE_ADVBI => 1,
            SSI_PP_MODER::SSI_PP_MODE_ADVBIQUAD => 2,
            SSI_PP_MODER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SSI_PP_MODER {
        match value {
            0 => SSI_PP_MODER::SSI_PP_MODE_LEGACY,
            1 => SSI_PP_MODER::SSI_PP_MODE_ADVBI,
            2 => SSI_PP_MODER::SSI_PP_MODE_ADVBIQUAD,
            i => SSI_PP_MODER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SSI_PP_MODE_LEGACY`"]
    #[inline(always)]
    pub fn is_ssi_pp_mode_legacy(&self) -> bool {
        *self == SSI_PP_MODER::SSI_PP_MODE_LEGACY
    }
    #[doc = "Checks if the value of the field is `SSI_PP_MODE_ADVBI`"]
    #[inline(always)]
    pub fn is_ssi_pp_mode_advbi(&self) -> bool {
        *self == SSI_PP_MODER::SSI_PP_MODE_ADVBI
    }
    #[doc = "Checks if the value of the field is `SSI_PP_MODE_ADVBIQUAD`"]
    #[inline(always)]
    pub fn is_ssi_pp_mode_advbiquad(&self) -> bool {
        *self == SSI_PP_MODER::SSI_PP_MODE_ADVBIQUAD
    }
}
#[doc = "Values that can be written to the field `SSI_PP_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_PP_MODEW {
    #[doc = "Legacy SSI mode"]
    SSI_PP_MODE_LEGACY,
    #[doc = "Legacy mode, Advanced SSI mode and Bi-SSI mode enabled"]
    SSI_PP_MODE_ADVBI,
    #[doc = "Legacy mode, Advanced mode, Bi-SSI and Quad-SSI mode enabled"]
    SSI_PP_MODE_ADVBIQUAD,
}
impl SSI_PP_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSI_PP_MODEW::SSI_PP_MODE_LEGACY => 0,
            SSI_PP_MODEW::SSI_PP_MODE_ADVBI => 1,
            SSI_PP_MODEW::SSI_PP_MODE_ADVBIQUAD => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSI_PP_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_PP_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSI_PP_MODEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Legacy SSI mode"]
    #[inline(always)]
    pub fn ssi_pp_mode_legacy(self) -> &'a mut W {
        self.variant(SSI_PP_MODEW::SSI_PP_MODE_LEGACY)
    }
    #[doc = "Legacy mode, Advanced SSI mode and Bi-SSI mode enabled"]
    #[inline(always)]
    pub fn ssi_pp_mode_advbi(self) -> &'a mut W {
        self.variant(SSI_PP_MODEW::SSI_PP_MODE_ADVBI)
    }
    #[doc = "Legacy mode, Advanced mode, Bi-SSI and Quad-SSI mode enabled"]
    #[inline(always)]
    pub fn ssi_pp_mode_advbiquad(self) -> &'a mut W {
        self.variant(SSI_PP_MODEW::SSI_PP_MODE_ADVBIQUAD)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 1);
        self.w.bits |= ((value as u32) & 3) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_PP_FSSHLDFRMR {
    bits: bool,
}
impl SSI_PP_FSSHLDFRMR {
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
pub struct _SSI_PP_FSSHLDFRMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_PP_FSSHLDFRMW<'a> {
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
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - High Speed Capability"]
    #[inline(always)]
    pub fn ssi_pp_hsclk(&self) -> SSI_PP_HSCLKR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SSI_PP_HSCLKR { bits }
    }
    #[doc = "Bits 1:2 - Mode of Operation"]
    #[inline(always)]
    pub fn ssi_pp_mode(&self) -> SSI_PP_MODER {
        SSI_PP_MODER::_from(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bit 3 - FSS Hold Frame Capability"]
    #[inline(always)]
    pub fn ssi_pp_fsshldfrm(&self) -> SSI_PP_FSSHLDFRMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SSI_PP_FSSHLDFRMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - High Speed Capability"]
    #[inline(always)]
    pub fn ssi_pp_hsclk(&mut self) -> _SSI_PP_HSCLKW {
        _SSI_PP_HSCLKW { w: self }
    }
    #[doc = "Bits 1:2 - Mode of Operation"]
    #[inline(always)]
    pub fn ssi_pp_mode(&mut self) -> _SSI_PP_MODEW {
        _SSI_PP_MODEW { w: self }
    }
    #[doc = "Bit 3 - FSS Hold Frame Capability"]
    #[inline(always)]
    pub fn ssi_pp_fsshldfrm(&mut self) -> _SSI_PP_FSSHLDFRMW {
        _SSI_PP_FSSHLDFRMW { w: self }
    }
}
