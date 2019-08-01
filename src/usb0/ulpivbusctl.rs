#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::ULPIVBUSCTL {
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
pub struct USB_ULPIVBUSCTL_USEEXTVBUSR {
    bits: bool,
}
impl USB_ULPIVBUSCTL_USEEXTVBUSR {
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
pub struct _USB_ULPIVBUSCTL_USEEXTVBUSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_ULPIVBUSCTL_USEEXTVBUSW<'a> {
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
pub struct USB_ULPIVBUSCTL_USEEXTVBUSINDR {
    bits: bool,
}
impl USB_ULPIVBUSCTL_USEEXTVBUSINDR {
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
pub struct _USB_ULPIVBUSCTL_USEEXTVBUSINDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_ULPIVBUSCTL_USEEXTVBUSINDW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Use External VBUS"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbus(&self) -> USB_ULPIVBUSCTL_USEEXTVBUSR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_ULPIVBUSCTL_USEEXTVBUSR { bits }
    }
    #[doc = "Bit 1 - Use External VBUS Indicator"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbusind(&self) -> USB_ULPIVBUSCTL_USEEXTVBUSINDR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_ULPIVBUSCTL_USEEXTVBUSINDR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Use External VBUS"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbus(&mut self) -> _USB_ULPIVBUSCTL_USEEXTVBUSW {
        _USB_ULPIVBUSCTL_USEEXTVBUSW { w: self }
    }
    #[doc = "Bit 1 - Use External VBUS Indicator"]
    #[inline(always)]
    pub fn usb_ulpivbusctl_useextvbusind(&mut self) -> _USB_ULPIVBUSCTL_USEEXTVBUSINDW {
        _USB_ULPIVBUSCTL_USEEXTVBUSINDW { w: self }
    }
}
