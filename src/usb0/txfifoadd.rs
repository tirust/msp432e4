#[doc = r"Value read from the register"]
pub struct R {
    bits: u16,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u16,
}
impl super::TXFIFOADD {
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
#[doc = r"Value of the field"]
pub struct USB_TXFIFOADD_ADDRR {
    bits: u16,
}
impl USB_TXFIFOADD_ADDRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_TXFIFOADD_ADDRW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_TXFIFOADD_ADDRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(511 << 0);
        self.w.bits |= ((value as u16) & 511) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn usb_txfifoadd_addr(&self) -> USB_TXFIFOADD_ADDRR {
        let bits = ((self.bits >> 0) & 511) as u16;
        USB_TXFIFOADD_ADDRR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:8 - Transmit/Receive Start Address"]
    #[inline(always)]
    pub fn usb_txfifoadd_addr(&mut self) -> _USB_TXFIFOADD_ADDRW {
        _USB_TXFIFOADD_ADDRW { w: self }
    }
}
