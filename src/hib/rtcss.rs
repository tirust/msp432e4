#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RTCSS {
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
pub struct HIB_RTCSS_RTCSSCR {
    bits: u16,
}
impl HIB_RTCSS_RTCSSCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_RTCSS_RTCSSCW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_RTCSS_RTCSSCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(32767 << 0);
        self.w.bits |= ((value as u32) & 32767) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_RTCSS_RTCSSMR {
    bits: u16,
}
impl HIB_RTCSS_RTCSSMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_RTCSS_RTCSSMW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_RTCSS_RTCSSMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(32767 << 16);
        self.w.bits |= ((value as u32) & 32767) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssc(&self) -> HIB_RTCSS_RTCSSCR {
        let bits = ((self.bits >> 0) & 32767) as u16;
        HIB_RTCSS_RTCSSCR { bits }
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssm(&self) -> HIB_RTCSS_RTCSSMR {
        let bits = ((self.bits >> 16) & 32767) as u16;
        HIB_RTCSS_RTCSSMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:14 - RTC Sub Seconds Count"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssc(&mut self) -> _HIB_RTCSS_RTCSSCW {
        _HIB_RTCSS_RTCSSCW { w: self }
    }
    #[doc = "Bits 16:30 - RTC Sub Seconds Match"]
    #[inline(always)]
    pub fn hib_rtcss_rtcssm(&mut self) -> _HIB_RTCSS_RTCSSMW {
        _HIB_RTCSS_RTCSSMW { w: self }
    }
}
