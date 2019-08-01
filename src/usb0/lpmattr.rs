#[doc = r"Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::LPMATTR {
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
    pub const fn reset_value() -> u16 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = "Possible values of the field `USB_LPMATTR_LS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_LPMATTR_LSR {
    #[doc = "Sleep State (L1)"]
    USB_LPMATTR_LS_L1,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl USB_LPMATTR_LSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_LPMATTR_LSR::USB_LPMATTR_LS_L1 => 1,
            USB_LPMATTR_LSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_LPMATTR_LSR {
        match value {
            1 => USB_LPMATTR_LSR::USB_LPMATTR_LS_L1,
            i => USB_LPMATTR_LSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_LPMATTR_LS_L1`"]
    #[inline(always)]
    pub fn is_usb_lpmattr_ls_l1(&self) -> bool {
        *self == USB_LPMATTR_LSR::USB_LPMATTR_LS_L1
    }
}
#[doc = "Values that can be written to the field `USB_LPMATTR_LS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_LPMATTR_LSW {
    #[doc = "Sleep State (L1)"]
    USB_LPMATTR_LS_L1,
}
impl USB_LPMATTR_LSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_LPMATTR_LSW::USB_LPMATTR_LS_L1 => 1,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_LPMATTR_LSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMATTR_LSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_LPMATTR_LSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Sleep State (L1)"]
    #[inline(always)]
    pub fn usb_lpmattr_ls_l1(self) -> &'a mut W {
        self.variant(USB_LPMATTR_LSW::USB_LPMATTR_LS_L1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u16) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMATTR_HIRDR {
    bits: u8,
}
impl USB_LPMATTR_HIRDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_LPMATTR_HIRDW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMATTR_HIRDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u16) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMATTR_RMTWAKR {
    bits: bool,
}
impl USB_LPMATTR_RMTWAKR {
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
pub struct _USB_LPMATTR_RMTWAKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMATTR_RMTWAKW<'a> {
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
        self.w.bits |= ((value as u16) & 1) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMATTR_ENDPTR {
    bits: u8,
}
impl USB_LPMATTR_ENDPTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_LPMATTR_ENDPTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMATTR_ENDPTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u16) & 15) << 12;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:3 - Link State"]
    #[inline(always)]
    pub fn usb_lpmattr_ls(&self) -> USB_LPMATTR_LSR {
        USB_LPMATTR_LSR::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration"]
    #[inline(always)]
    pub fn usb_lpmattr_hird(&self) -> USB_LPMATTR_HIRDR {
        let bits = ((self.bits >> 4) & 15) as u8;
        USB_LPMATTR_HIRDR { bits }
    }
    #[doc = "Bit 8 - Remote Wake"]
    #[inline(always)]
    pub fn usb_lpmattr_rmtwak(&self) -> USB_LPMATTR_RMTWAKR {
        let bits = ((self.bits >> 8) & 1) != 0;
        USB_LPMATTR_RMTWAKR { bits }
    }
    #[doc = "Bits 12:15 - Endpoint"]
    #[inline(always)]
    pub fn usb_lpmattr_endpt(&self) -> USB_LPMATTR_ENDPTR {
        let bits = ((self.bits >> 12) & 15) as u8;
        USB_LPMATTR_ENDPTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Link State"]
    #[inline(always)]
    pub fn usb_lpmattr_ls(&mut self) -> _USB_LPMATTR_LSW {
        _USB_LPMATTR_LSW { w: self }
    }
    #[doc = "Bits 4:7 - Host Initiated Resume Duration"]
    #[inline(always)]
    pub fn usb_lpmattr_hird(&mut self) -> _USB_LPMATTR_HIRDW {
        _USB_LPMATTR_HIRDW { w: self }
    }
    #[doc = "Bit 8 - Remote Wake"]
    #[inline(always)]
    pub fn usb_lpmattr_rmtwak(&mut self) -> _USB_LPMATTR_RMTWAKW {
        _USB_LPMATTR_RMTWAKW { w: self }
    }
    #[doc = "Bits 12:15 - Endpoint"]
    #[inline(always)]
    pub fn usb_lpmattr_endpt(&mut self) -> _USB_LPMATTR_ENDPTW {
        _USB_LPMATTR_ENDPTW { w: self }
    }
}
