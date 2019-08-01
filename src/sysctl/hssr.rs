#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::HSSR {
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
    pub const fn reset_value() -> u32 {
        0
    }
    #[doc = r"Writes the reset value to the register"]
    #[inline(always)]
    pub fn reset(&self) {
        self.register.set(Self::reset_value())
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_HSSR_CDOFFR {
    bits: u32,
}
impl SYSCTL_HSSR_CDOFFR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_HSSR_CDOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_HSSR_CDOFFW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits &= !(16777215 << 0);
        self.w.bits |= ((value as u32) & 16777215) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_HSSR_KEYR {
    bits: u8,
}
impl SYSCTL_HSSR_KEYR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_HSSR_KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_HSSR_KEYW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 24);
        self.w.bits |= ((value as u32) & 255) << 24;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:23 - Command Descriptor Pointer"]
    #[inline(always)]
    pub fn sysctl_hssr_cdoff(&self) -> SYSCTL_HSSR_CDOFFR {
        let bits = ((self.bits >> 0) & 16777215) as u32;
        SYSCTL_HSSR_CDOFFR { bits }
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn sysctl_hssr_key(&self) -> SYSCTL_HSSR_KEYR {
        let bits = ((self.bits >> 24) & 255) as u8;
        SYSCTL_HSSR_KEYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:23 - Command Descriptor Pointer"]
    #[inline(always)]
    pub fn sysctl_hssr_cdoff(&mut self) -> _SYSCTL_HSSR_CDOFFW {
        _SYSCTL_HSSR_CDOFFW { w: self }
    }
    #[doc = "Bits 24:31 - Write Key"]
    #[inline(always)]
    pub fn sysctl_hssr_key(&mut self) -> _SYSCTL_HSSR_KEYW {
        _SYSCTL_HSSR_KEYW { w: self }
    }
}
