#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ULPIREGCTL {
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
pub struct USB_ULPIREGCTL_REGACCR {
    bits: bool,
}
impl USB_ULPIREGCTL_REGACCR {
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
pub struct _USB_ULPIREGCTL_REGACCW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_ULPIREGCTL_REGACCW<'a> {
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
pub struct USB_ULPIREGCTL_REGCMPLTR {
    bits: bool,
}
impl USB_ULPIREGCTL_REGCMPLTR {
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
pub struct _USB_ULPIREGCTL_REGCMPLTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_ULPIREGCTL_REGCMPLTW<'a> {
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
pub struct USB_ULPIREGCTL_RDWRR {
    bits: bool,
}
impl USB_ULPIREGCTL_RDWRR {
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
pub struct _USB_ULPIREGCTL_RDWRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_ULPIREGCTL_RDWRW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Initiate Register Access"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regacc(&self) -> USB_ULPIREGCTL_REGACCR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_ULPIREGCTL_REGACCR { bits }
    }
    #[doc = "Bit 1 - Register Access Complete"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regcmplt(&self) -> USB_ULPIREGCTL_REGCMPLTR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_ULPIREGCTL_REGCMPLTR { bits }
    }
    #[doc = "Bit 2 - Read/Write Control"]
    #[inline(always)]
    pub fn usb_ulpiregctl_rdwr(&self) -> USB_ULPIREGCTL_RDWRR {
        let bits = ((self.bits >> 2) & 1) != 0;
        USB_ULPIREGCTL_RDWRR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Initiate Register Access"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regacc(&mut self) -> _USB_ULPIREGCTL_REGACCW {
        _USB_ULPIREGCTL_REGACCW { w: self }
    }
    #[doc = "Bit 1 - Register Access Complete"]
    #[inline(always)]
    pub fn usb_ulpiregctl_regcmplt(&mut self) -> _USB_ULPIREGCTL_REGCMPLTW {
        _USB_ULPIREGCTL_REGCMPLTW { w: self }
    }
    #[doc = "Bit 2 - Read/Write Control"]
    #[inline(always)]
    pub fn usb_ulpiregctl_rdwr(&mut self) -> _USB_ULPIREGCTL_RDWRW {
        _USB_ULPIREGCTL_RDWRW { w: self }
    }
}
