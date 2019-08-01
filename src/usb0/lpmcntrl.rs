#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::LPMCNTRL {
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
pub struct USB_LPMCNTRL_TXLPMR {
    bits: bool,
}
impl USB_LPMCNTRL_TXLPMR {
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
pub struct _USB_LPMCNTRL_TXLPMW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMCNTRL_TXLPMW<'a> {
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
pub struct USB_LPMCNTRL_RESR {
    bits: bool,
}
impl USB_LPMCNTRL_RESR {
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
pub struct _USB_LPMCNTRL_RESW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMCNTRL_RESW<'a> {
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
#[doc = "Possible values of the field `USB_LPMCNTRL_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_LPMCNTRL_ENR {
    #[doc = "LPM and Extended transactions are not supported. In this case, the USB does not respond to LPM transactions and LPM transactions cause a timeout"]
    USB_LPMCNTRL_EN_NONE,
    #[doc = "LPM is not supported but extended transactions are supported. In this case, the USB does respond to an LPM transaction with a STALL"]
    USB_LPMCNTRL_EN_EXT,
    #[doc = "The USB supports LPM extended transactions. In this case, the USB responds with a NYET or an ACK as determined by the value of TXLPM and other conditions"]
    USB_LPMCNTRL_EN_LPMEXT,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl USB_LPMCNTRL_ENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_NONE => 0,
            USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_EXT => 1,
            USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_LPMEXT => 3,
            USB_LPMCNTRL_ENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_LPMCNTRL_ENR {
        match value {
            0 => USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_NONE,
            1 => USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_EXT,
            3 => USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_LPMEXT,
            i => USB_LPMCNTRL_ENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_LPMCNTRL_EN_NONE`"]
    #[inline(always)]
    pub fn is_usb_lpmcntrl_en_none(&self) -> bool {
        *self == USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_NONE
    }
    #[doc = "Checks if the value of the field is `USB_LPMCNTRL_EN_EXT`"]
    #[inline(always)]
    pub fn is_usb_lpmcntrl_en_ext(&self) -> bool {
        *self == USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_EXT
    }
    #[doc = "Checks if the value of the field is `USB_LPMCNTRL_EN_LPMEXT`"]
    #[inline(always)]
    pub fn is_usb_lpmcntrl_en_lpmext(&self) -> bool {
        *self == USB_LPMCNTRL_ENR::USB_LPMCNTRL_EN_LPMEXT
    }
}
#[doc = "Values that can be written to the field `USB_LPMCNTRL_EN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_LPMCNTRL_ENW {
    #[doc = "LPM and Extended transactions are not supported. In this case, the USB does not respond to LPM transactions and LPM transactions cause a timeout"]
    USB_LPMCNTRL_EN_NONE,
    #[doc = "LPM is not supported but extended transactions are supported. In this case, the USB does respond to an LPM transaction with a STALL"]
    USB_LPMCNTRL_EN_EXT,
    #[doc = "The USB supports LPM extended transactions. In this case, the USB responds with a NYET or an ACK as determined by the value of TXLPM and other conditions"]
    USB_LPMCNTRL_EN_LPMEXT,
}
impl USB_LPMCNTRL_ENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_LPMCNTRL_ENW::USB_LPMCNTRL_EN_NONE => 0,
            USB_LPMCNTRL_ENW::USB_LPMCNTRL_EN_EXT => 1,
            USB_LPMCNTRL_ENW::USB_LPMCNTRL_EN_LPMEXT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_LPMCNTRL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMCNTRL_ENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_LPMCNTRL_ENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "LPM and Extended transactions are not supported. In this case, the USB does not respond to LPM transactions and LPM transactions cause a timeout"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en_none(self) -> &'a mut W {
        self.variant(USB_LPMCNTRL_ENW::USB_LPMCNTRL_EN_NONE)
    }
    #[doc = "LPM is not supported but extended transactions are supported. In this case, the USB does respond to an LPM transaction with a STALL"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en_ext(self) -> &'a mut W {
        self.variant(USB_LPMCNTRL_ENW::USB_LPMCNTRL_EN_EXT)
    }
    #[doc = "The USB supports LPM extended transactions. In this case, the USB responds with a NYET or an ACK as determined by the value of TXLPM and other conditions"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en_lpmext(self) -> &'a mut W {
        self.variant(USB_LPMCNTRL_ENW::USB_LPMCNTRL_EN_LPMEXT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u8) & 3) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_LPMCNTRL_NAKR {
    bits: bool,
}
impl USB_LPMCNTRL_NAKR {
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
pub struct _USB_LPMCNTRL_NAKW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_LPMCNTRL_NAKW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bit 0 - Transmit LPM Transaction Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_txlpm(&self) -> USB_LPMCNTRL_TXLPMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        USB_LPMCNTRL_TXLPMR { bits }
    }
    #[doc = "Bit 1 - LPM Resume"]
    #[inline(always)]
    pub fn usb_lpmcntrl_res(&self) -> USB_LPMCNTRL_RESR {
        let bits = ((self.bits >> 1) & 1) != 0;
        USB_LPMCNTRL_RESR { bits }
    }
    #[doc = "Bits 2:3 - LPM Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en(&self) -> USB_LPMCNTRL_ENR {
        USB_LPMCNTRL_ENR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - LPM NAK"]
    #[inline(always)]
    pub fn usb_lpmcntrl_nak(&self) -> USB_LPMCNTRL_NAKR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_LPMCNTRL_NAKR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Transmit LPM Transaction Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_txlpm(&mut self) -> _USB_LPMCNTRL_TXLPMW {
        _USB_LPMCNTRL_TXLPMW { w: self }
    }
    #[doc = "Bit 1 - LPM Resume"]
    #[inline(always)]
    pub fn usb_lpmcntrl_res(&mut self) -> _USB_LPMCNTRL_RESW {
        _USB_LPMCNTRL_RESW { w: self }
    }
    #[doc = "Bits 2:3 - LPM Enable"]
    #[inline(always)]
    pub fn usb_lpmcntrl_en(&mut self) -> _USB_LPMCNTRL_ENW {
        _USB_LPMCNTRL_ENW { w: self }
    }
    #[doc = "Bit 4 - LPM NAK"]
    #[inline(always)]
    pub fn usb_lpmcntrl_nak(&mut self) -> _USB_LPMCNTRL_NAKW {
        _USB_LPMCNTRL_NAKW { w: self }
    }
}
