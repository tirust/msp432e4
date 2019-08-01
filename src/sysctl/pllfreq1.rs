#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PLLFREQ1 {
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
pub struct SYSCTL_PLLFREQ1_NR {
    bits: u8,
}
impl SYSCTL_PLLFREQ1_NR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PLLFREQ1_NW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PLLFREQ1_NW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 0);
        self.w.bits |= ((value as u32) & 31) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PLLFREQ1_QR {
    bits: u8,
}
impl SYSCTL_PLLFREQ1_QR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PLLFREQ1_QW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PLLFREQ1_QW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 8);
        self.w.bits |= ((value as u32) & 31) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_n(&self) -> SYSCTL_PLLFREQ1_NR {
        let bits = ((self.bits >> 0) & 31) as u8;
        SYSCTL_PLLFREQ1_NR { bits }
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_q(&self) -> SYSCTL_PLLFREQ1_QR {
        let bits = ((self.bits >> 8) & 31) as u8;
        SYSCTL_PLLFREQ1_QR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - PLL N Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_n(&mut self) -> _SYSCTL_PLLFREQ1_NW {
        _SYSCTL_PLLFREQ1_NW { w: self }
    }
    #[doc = "Bits 8:12 - PLL Q Value"]
    #[inline(always)]
    pub fn sysctl_pllfreq1_q(&mut self) -> _SYSCTL_PLLFREQ1_QW {
        _SYSCTL_PLLFREQ1_QW { w: self }
    }
}
