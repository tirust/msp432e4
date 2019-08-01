#[doc = r"Value read from the register"]
pub struct R {
    bits: u8,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u8,
}
impl super::RAMINFO {
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
pub struct USB_RAMINFO_RAMBITSR {
    bits: u8,
}
impl USB_RAMINFO_RAMBITSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_RAMINFO_RAMBITSW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RAMINFO_RAMBITSW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u8) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct USB_RAMINFO_DMACHANR {
    bits: u8,
}
impl USB_RAMINFO_DMACHANR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _USB_RAMINFO_DMACHANW<'a> {
    w: &'a mut W,
}
impl<'a> _USB_RAMINFO_DMACHANW<'a> {
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
    #[doc = "Bits 0:3 - RAM Address Bus Width"]
    #[inline(always)]
    pub fn usb_raminfo_rambits(&self) -> USB_RAMINFO_RAMBITSR {
        let bits = ((self.bits >> 0) & 15) as u8;
        USB_RAMINFO_RAMBITSR { bits }
    }
    #[doc = "Bits 4:7 - DMA Channels"]
    #[inline(always)]
    pub fn usb_raminfo_dmachan(&self) -> USB_RAMINFO_DMACHANR {
        let bits = ((self.bits >> 4) & 15) as u8;
        USB_RAMINFO_DMACHANR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u8) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - RAM Address Bus Width"]
    #[inline(always)]
    pub fn usb_raminfo_rambits(&mut self) -> _USB_RAMINFO_RAMBITSW {
        _USB_RAMINFO_RAMBITSW { w: self }
    }
    #[doc = "Bits 4:7 - DMA Channels"]
    #[inline(always)]
    pub fn usb_raminfo_dmachan(&mut self) -> _USB_RAMINFO_DMACHANW {
        _USB_RAMINFO_DMACHANW { w: self }
    }
}
