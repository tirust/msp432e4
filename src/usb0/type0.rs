#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TYPE0 {
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
#[doc = "Possible values of the field `USB_TYPE0_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_TYPE0_SPEEDR {
    #[doc = "High"]
    USB_TYPE0_SPEED_HIGH,
    #[doc = "Full"]
    USB_TYPE0_SPEED_FULL,
    #[doc = "Low"]
    USB_TYPE0_SPEED_LOW,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl USB_TYPE0_SPEEDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_HIGH => 1,
            USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_FULL => 2,
            USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_LOW => 3,
            USB_TYPE0_SPEEDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_TYPE0_SPEEDR {
        match value {
            1 => USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_HIGH,
            2 => USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_FULL,
            3 => USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_LOW,
            i => USB_TYPE0_SPEEDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_TYPE0_SPEED_HIGH`"]
    #[inline(always)]
    pub fn is_usb_type0_speed_high(&self) -> bool {
        *self == USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_HIGH
    }
    #[doc = "Checks if the value of the field is `USB_TYPE0_SPEED_FULL`"]
    #[inline(always)]
    pub fn is_usb_type0_speed_full(&self) -> bool {
        *self == USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_FULL
    }
    #[doc = "Checks if the value of the field is `USB_TYPE0_SPEED_LOW`"]
    #[inline(always)]
    pub fn is_usb_type0_speed_low(&self) -> bool {
        *self == USB_TYPE0_SPEEDR::USB_TYPE0_SPEED_LOW
    }
}
#[doc = "Values that can be written to the field `USB_TYPE0_SPEED`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_TYPE0_SPEEDW {
    #[doc = "High"]
    USB_TYPE0_SPEED_HIGH,
    #[doc = "Full"]
    USB_TYPE0_SPEED_FULL,
    #[doc = "Low"]
    USB_TYPE0_SPEED_LOW,
}
impl USB_TYPE0_SPEEDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_TYPE0_SPEEDW::USB_TYPE0_SPEED_HIGH => 1,
            USB_TYPE0_SPEEDW::USB_TYPE0_SPEED_FULL => 2,
            USB_TYPE0_SPEEDW::USB_TYPE0_SPEED_LOW => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_TYPE0_SPEEDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TYPE0_SPEEDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_TYPE0_SPEEDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "High"]
    #[inline(always)]
    pub fn usb_type0_speed_high(self) -> &'a mut W {
        self.variant(USB_TYPE0_SPEEDW::USB_TYPE0_SPEED_HIGH)
    }
    #[doc = "Full"]
    #[inline(always)]
    pub fn usb_type0_speed_full(self) -> &'a mut W {
        self.variant(USB_TYPE0_SPEEDW::USB_TYPE0_SPEED_FULL)
    }
    #[doc = "Low"]
    #[inline(always)]
    pub fn usb_type0_speed_low(self) -> &'a mut W {
        self.variant(USB_TYPE0_SPEEDW::USB_TYPE0_SPEED_LOW)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u8) & 3) << 6;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_type0_speed(&self) -> USB_TYPE0_SPEEDR {
        USB_TYPE0_SPEEDR::_from(((self.bits >> 6) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 6:7 - Operating Speed"]
    #[inline(always)]
    pub fn usb_type0_speed(&mut self) -> _USB_TYPE0_SPEEDW {
        _USB_TYPE0_SPEEDW { w: self }
    }
}
