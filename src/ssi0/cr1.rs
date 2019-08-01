#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CR1 {
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
pub struct SSI_CR1_LBMR {
    bits: bool,
}
impl SSI_CR1_LBMR {
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
pub struct _SSI_CR1_LBMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_LBMW<'a> {
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
pub struct SSI_CR1_SSER {
    bits: bool,
}
impl SSI_CR1_SSER {
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
pub struct _SSI_CR1_SSEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_SSEW<'a> {
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
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR1_MSR {
    bits: bool,
}
impl SSI_CR1_MSR {
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
pub struct _SSI_CR1_MSW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_MSW<'a> {
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
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR1_EOTR {
    bits: bool,
}
impl SSI_CR1_EOTR {
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
pub struct _SSI_CR1_EOTW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_EOTW<'a> {
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
#[doc = "Possible values of the field `SSI_CR1_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_CR1_MODER {
    #[doc = "Legacy SSI mode"]
    SSI_CR1_MODE_LEGACY,
    #[doc = "Bi-SSI mode"]
    SSI_CR1_MODE_BI,
    #[doc = "Quad-SSI Mode"]
    SSI_CR1_MODE_QUAD,
    #[doc = "Advanced SSI Mode with 8-bit packet size"]
    SSI_CR1_MODE_ADVANCED,
}
impl SSI_CR1_MODER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SSI_CR1_MODER::SSI_CR1_MODE_LEGACY => 0,
            SSI_CR1_MODER::SSI_CR1_MODE_BI => 1,
            SSI_CR1_MODER::SSI_CR1_MODE_QUAD => 2,
            SSI_CR1_MODER::SSI_CR1_MODE_ADVANCED => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SSI_CR1_MODER {
        match value {
            0 => SSI_CR1_MODER::SSI_CR1_MODE_LEGACY,
            1 => SSI_CR1_MODER::SSI_CR1_MODE_BI,
            2 => SSI_CR1_MODER::SSI_CR1_MODE_QUAD,
            3 => SSI_CR1_MODER::SSI_CR1_MODE_ADVANCED,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_LEGACY`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_legacy(&self) -> bool {
        *self == SSI_CR1_MODER::SSI_CR1_MODE_LEGACY
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_BI`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_bi(&self) -> bool {
        *self == SSI_CR1_MODER::SSI_CR1_MODE_BI
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_QUAD`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_quad(&self) -> bool {
        *self == SSI_CR1_MODER::SSI_CR1_MODE_QUAD
    }
    #[doc = "Checks if the value of the field is `SSI_CR1_MODE_ADVANCED`"]
    #[inline(always)]
    pub fn is_ssi_cr1_mode_advanced(&self) -> bool {
        *self == SSI_CR1_MODER::SSI_CR1_MODE_ADVANCED
    }
}
#[doc = "Values that can be written to the field `SSI_CR1_MODE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SSI_CR1_MODEW {
    #[doc = "Legacy SSI mode"]
    SSI_CR1_MODE_LEGACY,
    #[doc = "Bi-SSI mode"]
    SSI_CR1_MODE_BI,
    #[doc = "Quad-SSI Mode"]
    SSI_CR1_MODE_QUAD,
    #[doc = "Advanced SSI Mode with 8-bit packet size"]
    SSI_CR1_MODE_ADVANCED,
}
impl SSI_CR1_MODEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SSI_CR1_MODEW::SSI_CR1_MODE_LEGACY => 0,
            SSI_CR1_MODEW::SSI_CR1_MODE_BI => 1,
            SSI_CR1_MODEW::SSI_CR1_MODE_QUAD => 2,
            SSI_CR1_MODEW::SSI_CR1_MODE_ADVANCED => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SSI_CR1_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_MODEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SSI_CR1_MODEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Legacy SSI mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode_legacy(self) -> &'a mut W {
        self.variant(SSI_CR1_MODEW::SSI_CR1_MODE_LEGACY)
    }
    #[doc = "Bi-SSI mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode_bi(self) -> &'a mut W {
        self.variant(SSI_CR1_MODEW::SSI_CR1_MODE_BI)
    }
    #[doc = "Quad-SSI Mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode_quad(self) -> &'a mut W {
        self.variant(SSI_CR1_MODEW::SSI_CR1_MODE_QUAD)
    }
    #[doc = "Advanced SSI Mode with 8-bit packet size"]
    #[inline(always)]
    pub fn ssi_cr1_mode_advanced(self) -> &'a mut W {
        self.variant(SSI_CR1_MODEW::SSI_CR1_MODE_ADVANCED)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR1_DIRR {
    bits: bool,
}
impl SSI_CR1_DIRR {
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
pub struct _SSI_CR1_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_DIRW<'a> {
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
#[doc = r"Value of the field"]
pub struct SSI_CR1_HSCLKENR {
    bits: bool,
}
impl SSI_CR1_HSCLKENR {
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
pub struct _SSI_CR1_HSCLKENW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_HSCLKENW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR1_FSSHLDFRMR {
    bits: bool,
}
impl SSI_CR1_FSSHLDFRMR {
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
pub struct _SSI_CR1_FSSHLDFRMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_FSSHLDFRMW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SSI_CR1_EOMR {
    bits: bool,
}
impl SSI_CR1_EOMR {
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
pub struct _SSI_CR1_EOMW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_CR1_EOMW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn ssi_cr1_lbm(&self) -> SSI_CR1_LBMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        SSI_CR1_LBMR { bits }
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn ssi_cr1_sse(&self) -> SSI_CR1_SSER {
        let bits = ((self.bits >> 1) & 1) != 0;
        SSI_CR1_SSER { bits }
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ssi_cr1_ms(&self) -> SSI_CR1_MSR {
        let bits = ((self.bits >> 2) & 1) != 0;
        SSI_CR1_MSR { bits }
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn ssi_cr1_eot(&self) -> SSI_CR1_EOTR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SSI_CR1_EOTR { bits }
    }
    #[doc = "Bits 6:7 - SSI Mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode(&self) -> SSI_CR1_MODER {
        SSI_CR1_MODER::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - SSI Direction of Operation"]
    #[inline(always)]
    pub fn ssi_cr1_dir(&self) -> SSI_CR1_DIRR {
        let bits = ((self.bits >> 8) & 1) != 0;
        SSI_CR1_DIRR { bits }
    }
    #[doc = "Bit 9 - High Speed Clock Enable"]
    #[inline(always)]
    pub fn ssi_cr1_hsclken(&self) -> SSI_CR1_HSCLKENR {
        let bits = ((self.bits >> 9) & 1) != 0;
        SSI_CR1_HSCLKENR { bits }
    }
    #[doc = "Bit 10 - FSS Hold Frame"]
    #[inline(always)]
    pub fn ssi_cr1_fsshldfrm(&self) -> SSI_CR1_FSSHLDFRMR {
        let bits = ((self.bits >> 10) & 1) != 0;
        SSI_CR1_FSSHLDFRMR { bits }
    }
    #[doc = "Bit 11 - Stop Frame (End of Message)"]
    #[inline(always)]
    pub fn ssi_cr1_eom(&self) -> SSI_CR1_EOMR {
        let bits = ((self.bits >> 11) & 1) != 0;
        SSI_CR1_EOMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SSI Loopback Mode"]
    #[inline(always)]
    pub fn ssi_cr1_lbm(&mut self) -> _SSI_CR1_LBMW {
        _SSI_CR1_LBMW { w: self }
    }
    #[doc = "Bit 1 - SSI Synchronous Serial Port Enable"]
    #[inline(always)]
    pub fn ssi_cr1_sse(&mut self) -> _SSI_CR1_SSEW {
        _SSI_CR1_SSEW { w: self }
    }
    #[doc = "Bit 2 - SSI Master/Slave Select"]
    #[inline(always)]
    pub fn ssi_cr1_ms(&mut self) -> _SSI_CR1_MSW {
        _SSI_CR1_MSW { w: self }
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn ssi_cr1_eot(&mut self) -> _SSI_CR1_EOTW {
        _SSI_CR1_EOTW { w: self }
    }
    #[doc = "Bits 6:7 - SSI Mode"]
    #[inline(always)]
    pub fn ssi_cr1_mode(&mut self) -> _SSI_CR1_MODEW {
        _SSI_CR1_MODEW { w: self }
    }
    #[doc = "Bit 8 - SSI Direction of Operation"]
    #[inline(always)]
    pub fn ssi_cr1_dir(&mut self) -> _SSI_CR1_DIRW {
        _SSI_CR1_DIRW { w: self }
    }
    #[doc = "Bit 9 - High Speed Clock Enable"]
    #[inline(always)]
    pub fn ssi_cr1_hsclken(&mut self) -> _SSI_CR1_HSCLKENW {
        _SSI_CR1_HSCLKENW { w: self }
    }
    #[doc = "Bit 10 - FSS Hold Frame"]
    #[inline(always)]
    pub fn ssi_cr1_fsshldfrm(&mut self) -> _SSI_CR1_FSSHLDFRMW {
        _SSI_CR1_FSSHLDFRMW { w: self }
    }
    #[doc = "Bit 11 - Stop Frame (End of Message)"]
    #[inline(always)]
    pub fn ssi_cr1_eom(&mut self) -> _SSI_CR1_EOMW {
        _SSI_CR1_EOMW { w: self }
    }
}
