#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RXFIFOSZ {
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
#[doc = "Possible values of the field `USB_RXFIFOSZ_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RXFIFOSZ_SIZER {
    #[doc = "8"]
    USB_RXFIFOSZ_SIZE_8,
    #[doc = "16"]
    USB_RXFIFOSZ_SIZE_16,
    #[doc = "32"]
    USB_RXFIFOSZ_SIZE_32,
    #[doc = "64"]
    USB_RXFIFOSZ_SIZE_64,
    #[doc = "128"]
    USB_RXFIFOSZ_SIZE_128,
    #[doc = "256"]
    USB_RXFIFOSZ_SIZE_256,
    #[doc = "512"]
    USB_RXFIFOSZ_SIZE_512,
    #[doc = "1024"]
    USB_RXFIFOSZ_SIZE_1024,
    #[doc = "2048"]
    USB_RXFIFOSZ_SIZE_2048,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl USB_RXFIFOSZ_SIZER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_8 => 0,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_16 => 1,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_32 => 2,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_64 => 3,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_128 => 4,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_256 => 5,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_512 => 6,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_1024 => 7,
            USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_2048 => 8,
            USB_RXFIFOSZ_SIZER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> USB_RXFIFOSZ_SIZER {
        match value {
            0 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_8,
            1 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_16,
            2 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_32,
            3 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_64,
            4 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_128,
            5 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_256,
            6 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_512,
            7 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_1024,
            8 => USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_2048,
            i => USB_RXFIFOSZ_SIZER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_8`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_8(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_8
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_16`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_16(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_16
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_32`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_32(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_32
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_64`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_64(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_64
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_128`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_128(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_128
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_256`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_256(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_256
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_512`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_512(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_512
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_1024`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_1024(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_1024
    }
    #[doc = "Checks if the value of the field is `USB_RXFIFOSZ_SIZE_2048`"]
    #[inline(always)]
    pub fn is_usb_rxfifosz_size_2048(&self) -> bool {
        *self == USB_RXFIFOSZ_SIZER::USB_RXFIFOSZ_SIZE_2048
    }
}
#[doc = "Values that can be written to the field `USB_RXFIFOSZ_SIZE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum USB_RXFIFOSZ_SIZEW {
    #[doc = "8"]
    USB_RXFIFOSZ_SIZE_8,
    #[doc = "16"]
    USB_RXFIFOSZ_SIZE_16,
    #[doc = "32"]
    USB_RXFIFOSZ_SIZE_32,
    #[doc = "64"]
    USB_RXFIFOSZ_SIZE_64,
    #[doc = "128"]
    USB_RXFIFOSZ_SIZE_128,
    #[doc = "256"]
    USB_RXFIFOSZ_SIZE_256,
    #[doc = "512"]
    USB_RXFIFOSZ_SIZE_512,
    #[doc = "1024"]
    USB_RXFIFOSZ_SIZE_1024,
    #[doc = "2048"]
    USB_RXFIFOSZ_SIZE_2048,
}
impl USB_RXFIFOSZ_SIZEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_8 => 0,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_16 => 1,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_32 => 2,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_64 => 3,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_128 => 4,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_256 => 5,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_512 => 6,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_1024 => 7,
            USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_2048 => 8,
        }
    }
}
#[doc = r"Proxy"]
pub struct _USB_RXFIFOSZ_SIZEW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXFIFOSZ_SIZEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USB_RXFIFOSZ_SIZEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "8"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_8(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_8)
    }
    #[doc = "16"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_16(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_16)
    }
    #[doc = "32"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_32(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_32)
    }
    #[doc = "64"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_64(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_64)
    }
    #[doc = "128"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_128(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_128)
    }
    #[doc = "256"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_256(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_256)
    }
    #[doc = "512"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_512(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_512)
    }
    #[doc = "1024"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_1024(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_1024)
    }
    #[doc = "2048"]
    #[inline(always)]
    pub fn usb_rxfifosz_size_2048(self) -> &'a mut W {
        self.variant(USB_RXFIFOSZ_SIZEW::USB_RXFIFOSZ_SIZE_2048)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u8) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RXFIFOSZ_DPBR {
    bits: bool,
}
impl USB_RXFIFOSZ_DPBR {
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
pub struct _USB_RXFIFOSZ_DPBW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RXFIFOSZ_DPBW<'a> {
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
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline(always)]
    pub fn usb_rxfifosz_size(&self) -> USB_RXFIFOSZ_SIZER {
        USB_RXFIFOSZ_SIZER::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline(always)]
    pub fn usb_rxfifosz_dpb(&self) -> USB_RXFIFOSZ_DPBR {
        let bits = ((self.bits >> 4) & 1) != 0;
        USB_RXFIFOSZ_DPBR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Max Packet Size"]
    #[inline(always)]
    pub fn usb_rxfifosz_size(&mut self) -> _USB_RXFIFOSZ_SIZEW {
        _USB_RXFIFOSZ_SIZEW { w: self }
    }
    #[doc = "Bit 4 - Double Packet Buffer Support"]
    #[inline(always)]
    pub fn usb_rxfifosz_dpb(&mut self) -> _USB_RXFIFOSZ_DPBW {
        _USB_RXFIFOSZ_DPBW { w: self }
    }
}
