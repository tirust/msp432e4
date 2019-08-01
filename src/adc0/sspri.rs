#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSPRI {
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
pub struct ADC_SSPRI_SS0R {
    bits: u8,
}
impl ADC_SSPRI_SS0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSPRI_SS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSPRI_SS0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSPRI_SS1R {
    bits: u8,
}
impl ADC_SSPRI_SS1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSPRI_SS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSPRI_SS1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSPRI_SS2R {
    bits: u8,
}
impl ADC_SSPRI_SS2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSPRI_SS2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSPRI_SS2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSPRI_SS3R {
    bits: u8,
}
impl ADC_SSPRI_SS3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSPRI_SS3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSPRI_SS3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0(&self) -> ADC_SSPRI_SS0R {
        let bits = ((self.bits >> 0) & 3) as u8;
        ADC_SSPRI_SS0R { bits }
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1(&self) -> ADC_SSPRI_SS1R {
        let bits = ((self.bits >> 4) & 3) as u8;
        ADC_SSPRI_SS1R { bits }
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2(&self) -> ADC_SSPRI_SS2R {
        let bits = ((self.bits >> 8) & 3) as u8;
        ADC_SSPRI_SS2R { bits }
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3(&self) -> ADC_SSPRI_SS3R {
        let bits = ((self.bits >> 12) & 3) as u8;
        ADC_SSPRI_SS3R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - SS0 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss0(&mut self) -> _ADC_SSPRI_SS0W {
        _ADC_SSPRI_SS0W { w: self }
    }
    #[doc = "Bits 4:5 - SS1 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss1(&mut self) -> _ADC_SSPRI_SS1W {
        _ADC_SSPRI_SS1W { w: self }
    }
    #[doc = "Bits 8:9 - SS2 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss2(&mut self) -> _ADC_SSPRI_SS2W {
        _ADC_SSPRI_SS2W { w: self }
    }
    #[doc = "Bits 12:13 - SS3 Priority"]
    #[inline(always)]
    pub fn adc_sspri_ss3(&mut self) -> _ADC_SSPRI_SS3W {
        _ADC_SSPRI_SS3W { w: self }
    }
}
