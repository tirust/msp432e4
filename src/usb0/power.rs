#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::POWER {
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
pub struct USB_POWER_PWRDNPHYR {
    bits: bool,
}
impl USB_POWER_PWRDNPHYR {
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
pub struct _USB_POWER_PWRDNPHYW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_PWRDNPHYW<'a> {
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
pub struct USB_POWER_SUSPENDR {
    bits: bool,
}
impl USB_POWER_SUSPENDR {
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
pub struct _USB_POWER_SUSPENDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_SUSPENDW<'a> {
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
pub struct USB_POWER_RESUMER {
    bits: bool,
}
impl USB_POWER_RESUMER {
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
pub struct _USB_POWER_RESUMEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_RESUMEW<'a> {
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
pub struct USB_POWER_RESETR {
    bits: bool,
}
impl USB_POWER_RESETR {
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
pub struct _USB_POWER_RESETW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_RESETW<'a> {
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
pub struct USB_POWER_HSMODER {
    bits: bool,
}
impl USB_POWER_HSMODER {
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
pub struct _USB_POWER_HSMODEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_HSMODEW<'a> {
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
pub struct USB_POWER_HSENABR {
    bits: bool,
}
impl USB_POWER_HSENABR {
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
pub struct _USB_POWER_HSENABW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_HSENABW<'a> {
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
#[doc = r"Value of the field"]
pub struct USB_POWER_SOFTCONNR {
    bits: bool,
}
impl USB_POWER_SOFTCONNR {
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
pub struct _USB_POWER_SOFTCONNW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_SOFTCONNW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u8) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_POWER_ISOUPR {
    bits: bool,
}
impl USB_POWER_ISOUPR {
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
pub struct _USB_POWER_ISOUPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_POWER_ISOUPW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u8) & 1) << 7;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Power Down PHY"]
    #[inline(always)]
    pub fn usb_power_pwrdnphy(&self) -> USB_POWER_PWRDNPHYR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_POWER_PWRDNPHYR { bits }
    }
    #[doc = "Bit 1 - SUSPEND Mode"]
    #[inline(always)]
    pub fn usb_power_suspend(&self) -> USB_POWER_SUSPENDR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_POWER_SUSPENDR { bits }
    }
    #[doc = "Bit 2 - RESUME Signaling"]
    #[inline(always)]
    pub fn usb_power_resume(&self) -> USB_POWER_RESUMER {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_POWER_RESUMER { bits }
    }
    #[doc = "Bit 3 - RESET Signaling"]
    #[inline(always)]
    pub fn usb_power_reset(&self) -> USB_POWER_RESETR {
        let bits = ((self.bits >> 3) & 1) != 0;
        USB_POWER_RESETR { bits }
    }
    #[doc = "Bit 4 - High Speed Enable"]
    #[inline(always)]
    pub fn usb_power_hsmode(&self) -> USB_POWER_HSMODER {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_POWER_HSMODER { bits }
    }
    #[doc = "Bit 5 - High Speed Enable"]
    #[inline(always)]
    pub fn usb_power_hsenab(&self) -> USB_POWER_HSENABR {
        let bits = ((self.bits >> 5) & 1) != 0;
        USB_POWER_HSENABR { bits }
    }
    #[doc = "Bit 6 - Soft Connect/Disconnect"]
    #[inline(always)]
    pub fn usb_power_softconn(&self) -> USB_POWER_SOFTCONNR {
        let bits = ((self.bits >> 6) & 1) != 0;
        USB_POWER_SOFTCONNR { bits }
    }
    #[doc = "Bit 7 - Isochronous Update"]
    #[inline(always)]
    pub fn usb_power_isoup(&self) -> USB_POWER_ISOUPR {
        let bits = ((self.bits >> 7) & 1) != 0;
        USB_POWER_ISOUPR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Power Down PHY"]
    #[inline(always)]
    pub fn usb_power_pwrdnphy(&mut self) -> _USB_POWER_PWRDNPHYW {
        _USB_POWER_PWRDNPHYW { w: self }
    }
    #[doc = "Bit 1 - SUSPEND Mode"]
    #[inline(always)]
    pub fn usb_power_suspend(&mut self) -> _USB_POWER_SUSPENDW {
        _USB_POWER_SUSPENDW { w: self }
    }
    #[doc = "Bit 2 - RESUME Signaling"]
    #[inline(always)]
    pub fn usb_power_resume(&mut self) -> _USB_POWER_RESUMEW {
        _USB_POWER_RESUMEW { w: self }
    }
    #[doc = "Bit 3 - RESET Signaling"]
    #[inline(always)]
    pub fn usb_power_reset(&mut self) -> _USB_POWER_RESETW {
        _USB_POWER_RESETW { w: self }
    }
    #[doc = "Bit 4 - High Speed Enable"]
    #[inline(always)]
    pub fn usb_power_hsmode(&mut self) -> _USB_POWER_HSMODEW {
        _USB_POWER_HSMODEW { w: self }
    }
    #[doc = "Bit 5 - High Speed Enable"]
    #[inline(always)]
    pub fn usb_power_hsenab(&mut self) -> _USB_POWER_HSENABW {
        _USB_POWER_HSENABW { w: self }
    }
    #[doc = "Bit 6 - Soft Connect/Disconnect"]
    #[inline(always)]
    pub fn usb_power_softconn(&mut self) -> _USB_POWER_SOFTCONNW {
        _USB_POWER_SOFTCONNW { w: self }
    }
    #[doc = "Bit 7 - Isochronous Update"]
    #[inline(always)]
    pub fn usb_power_isoup(&mut self) -> _USB_POWER_ISOUPW {
        _USB_POWER_ISOUPW { w: self }
    }
}
