#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::TXINTERVAL4 {
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
pub struct USB_TXINTERVAL4_TXPOLLR {
    bits: u8,
}
impl USB_TXINTERVAL4_TXPOLLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_TXINTERVAL4_TXPOLLW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXINTERVAL4_TXPOLLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u8) & 255) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_TXINTERVAL4_NAKLMTR {
    bits: u8,
}
impl USB_TXINTERVAL4_NAKLMTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_TXINTERVAL4_NAKLMTW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXINTERVAL4_NAKLMTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u8) & 255) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
    #[doc = "Bits 0:7 - TX Polling"]
    #[inline(always)]
    pub fn usb_txinterval4_txpoll(&self) -> USB_TXINTERVAL4_TXPOLLR {
        let bits = ((self.bits >> 0) & 255) as u8;
        USB_TXINTERVAL4_TXPOLLR { bits }
    }
    #[doc = "Bits 0:7 - NAK Limit"]
    #[inline(always)]
    pub fn usb_txinterval4_naklmt(&self) -> USB_TXINTERVAL4_NAKLMTR {
        let bits = ((self.bits >> 0) & 255) as u8;
        USB_TXINTERVAL4_NAKLMTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - TX Polling"]
    #[inline(always)]
    pub fn usb_txinterval4_txpoll(&mut self) -> _USB_TXINTERVAL4_TXPOLLW {
        _USB_TXINTERVAL4_TXPOLLW { w: self }
    }
    #[doc = "Bits 0:7 - NAK Limit"]
    #[inline(always)]
    pub fn usb_txinterval4_naklmt(&mut self) -> _USB_TXINTERVAL4_NAKLMTW {
        _USB_TXINTERVAL4_NAKLMTW { w: self }
    }
}
