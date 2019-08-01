#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SR {
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
pub struct SSI_SR_TFER {
    bits: bool,
}
impl SSI_SR_TFER {
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
pub struct _SSI_SR_TFEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_SR_TFEW<'a> {
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
pub struct SSI_SR_TNFR {
    bits: bool,
}
impl SSI_SR_TNFR {
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
pub struct _SSI_SR_TNFW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_SR_TNFW<'a> {
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
pub struct SSI_SR_RNER {
    bits: bool,
}
impl SSI_SR_RNER {
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
pub struct _SSI_SR_RNEW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_SR_RNEW<'a> {
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
pub struct SSI_SR_RFFR {
    bits: bool,
}
impl SSI_SR_RFFR {
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
pub struct _SSI_SR_RFFW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_SR_RFFW<'a> {
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
#[doc = r"Value of the field"]
pub struct SSI_SR_BSYR {
    bits: bool,
}
impl SSI_SR_BSYR {
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
pub struct _SSI_SR_BSYW<'a> {
    w: &'a mut W,
}
impl<'a> _SSI_SR_BSYW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    pub fn ssi_sr_tfe(&self) -> SSI_SR_TFER {
        let bits = ((self.bits >> 0) & 1) != 0;
        SSI_SR_TFER { bits }
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn ssi_sr_tnf(&self) -> SSI_SR_TNFR {
        let bits = ((self.bits >> 1) & 1) != 0;
        SSI_SR_TNFR { bits }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn ssi_sr_rne(&self) -> SSI_SR_RNER {
        let bits = ((self.bits >> 2) & 1) != 0;
        SSI_SR_RNER { bits }
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    pub fn ssi_sr_rff(&self) -> SSI_SR_RFFR {
        let bits = ((self.bits >> 3) & 1) != 0;
        SSI_SR_RFFR { bits }
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    pub fn ssi_sr_bsy(&self) -> SSI_SR_BSYR {
        let bits = ((self.bits >> 4) & 1) != 0;
        SSI_SR_BSYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SSI Transmit FIFO Empty"]
    #[inline(always)]
    pub fn ssi_sr_tfe(&mut self) -> _SSI_SR_TFEW {
        _SSI_SR_TFEW { w: self }
    }
    #[doc = "Bit 1 - SSI Transmit FIFO Not Full"]
    #[inline(always)]
    pub fn ssi_sr_tnf(&mut self) -> _SSI_SR_TNFW {
        _SSI_SR_TNFW { w: self }
    }
    #[doc = "Bit 2 - SSI Receive FIFO Not Empty"]
    #[inline(always)]
    pub fn ssi_sr_rne(&mut self) -> _SSI_SR_RNEW {
        _SSI_SR_RNEW { w: self }
    }
    #[doc = "Bit 3 - SSI Receive FIFO Full"]
    #[inline(always)]
    pub fn ssi_sr_rff(&mut self) -> _SSI_SR_RFFW {
        _SSI_SR_RFFW { w: self }
    }
    #[doc = "Bit 4 - SSI Busy Bit"]
    #[inline(always)]
    pub fn ssi_sr_bsy(&mut self) -> _SSI_SR_BSYW {
        _SSI_SR_BSYW { w: self }
    }
}
