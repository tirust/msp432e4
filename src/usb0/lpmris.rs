#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::LPMRIS {
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
    pub const fn reset_value() -> u8 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMRIS_LPMSTR {
    bits: bool,
}
impl USB_LPMRIS_LPMSTR {
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
pub struct _USB_LPMRIS_LPMSTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMRIS_LPMSTW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMRIS_NYR {
    bits: bool,
}
impl USB_LPMRIS_NYR {
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
pub struct _USB_LPMRIS_NYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMRIS_NYW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMRIS_ACKR {
    bits: bool,
}
impl USB_LPMRIS_ACKR {
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
pub struct _USB_LPMRIS_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMRIS_ACKW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMRIS_NCR {
    bits: bool,
}
impl USB_LPMRIS_NCR {
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
pub struct _USB_LPMRIS_NCW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMRIS_NCW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMRIS_RESR {
    bits: bool,
}
impl USB_LPMRIS_RESR {
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
pub struct _USB_LPMRIS_RESW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMRIS_RESW<'a> {
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
        self.w.bits |= ((value as u8) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMRIS_ERRR {
    bits: bool,
}
impl USB_LPMRIS_ERRR {
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
pub struct _USB_LPMRIS_ERRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMRIS_ERRW<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u8) & 1) << 5;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - LPM STALL Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_lpmst(&self) -> USB_LPMRIS_LPMSTR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_LPMRIS_LPMSTR { bits }
    }
    #[doc = "Bit 1 - LPM NY Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ny(&self) -> USB_LPMRIS_NYR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_LPMRIS_NYR { bits }
    }
    #[doc = "Bit 2 - LPM ACK Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ack(&self) -> USB_LPMRIS_ACKR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_LPMRIS_ACKR { bits }
    }
    #[doc = "Bit 3 - LPM NC Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_nc(&self) -> USB_LPMRIS_NCR {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_LPMRIS_NCR { bits }
    }
    #[doc = "Bit 4 - LPM Resume Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_res(&self) -> USB_LPMRIS_RESR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_LPMRIS_RESR { bits }
    }
    #[doc = "Bit 5 - LPM Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_err(&self) -> USB_LPMRIS_ERRR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_LPMRIS_ERRR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - LPM STALL Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_lpmst(&mut self) -> _USB_LPMRIS_LPMSTW {
        _USB_LPMRIS_LPMSTW { w: self }
    }
    #[doc = "Bit 1 - LPM NY Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ny(&mut self) -> _USB_LPMRIS_NYW {
        _USB_LPMRIS_NYW { w: self }
    }
    #[doc = "Bit 2 - LPM ACK Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_ack(&mut self) -> _USB_LPMRIS_ACKW {
        _USB_LPMRIS_ACKW { w: self }
    }
    #[doc = "Bit 3 - LPM NC Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_nc(&mut self) -> _USB_LPMRIS_NCW {
        _USB_LPMRIS_NCW { w: self }
    }
    #[doc = "Bit 4 - LPM Resume Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_res(&mut self) -> _USB_LPMRIS_RESW {
        _USB_LPMRIS_RESW { w: self }
    }
    #[doc = "Bit 5 - LPM Interrupt Status"]
    #[inline(always)]
    pub fn usb_lpmris_err(&mut self) -> _USB_LPMRIS_ERRW {
        _USB_LPMRIS_ERRW { w: self }
    }
}
