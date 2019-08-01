#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::EPINFO {
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
pub struct USB_EPINFO_TXEPR {
    bits: u8,
}
impl USB_EPINFO_TXEPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_EPINFO_TXEPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPINFO_TXEPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u8) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_EPINFO_RXEPR {
    bits: u8,
}
impl USB_EPINFO_RXEPR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_EPINFO_RXEPW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_EPINFO_RXEPW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u8) & 15) << 4;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:3 - TX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_txep(&self) -> USB_EPINFO_TXEPR {
        let bits = ((self.bits >> 0) & 15) as u8;
        USB_EPINFO_TXEPR { bits }
    }
    #[doc = "Bits 4:7 - RX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_rxep(&self) -> USB_EPINFO_RXEPR {
        let bits = ((self.bits >> 4) & 15) as u8;
        USB_EPINFO_RXEPR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - TX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_txep(&mut self) -> _USB_EPINFO_TXEPW {
        _USB_EPINFO_TXEPW { w: self }
    }
    #[doc = "Bits 4:7 - RX Endpoints"]
    #[inline(always)]
    pub fn usb_epinfo_rxep(&mut self) -> _USB_EPINFO_RXEPW {
        _USB_EPINFO_RXEPW { w: self }
    }
}