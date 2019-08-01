#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSDC2 {
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
pub struct ADC_SSDC2_S0DCSELR {
    bits: u8,
}
impl ADC_SSDC2_S0DCSELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSDC2_S0DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSDC2_S0DCSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSDC2_S1DCSELR {
    bits: u8,
}
impl ADC_SSDC2_S1DCSELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSDC2_S1DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSDC2_S1DCSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSDC2_S2DCSELR {
    bits: u8,
}
impl ADC_SSDC2_S2DCSELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSDC2_S2DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSDC2_S2DCSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSDC2_S3DCSELR {
    bits: u8,
}
impl ADC_SSDC2_S3DCSELR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSDC2_S3DCSELW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSDC2_S3DCSELW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s0dcsel(&self) -> ADC_SSDC2_S0DCSELR {
        let bits = ((self.bits >> 0) & 15) as u8;
        ADC_SSDC2_S0DCSELR { bits }
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s1dcsel(&self) -> ADC_SSDC2_S1DCSELR {
        let bits = ((self.bits >> 4) & 15) as u8;
        ADC_SSDC2_S1DCSELR { bits }
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s2dcsel(&self) -> ADC_SSDC2_S2DCSELR {
        let bits = ((self.bits >> 8) & 15) as u8;
        ADC_SSDC2_S2DCSELR { bits }
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s3dcsel(&self) -> ADC_SSDC2_S3DCSELR {
        let bits = ((self.bits >> 12) & 15) as u8;
        ADC_SSDC2_S3DCSELR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Sample 0 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s0dcsel(&mut self) -> _ADC_SSDC2_S0DCSELW {
        _ADC_SSDC2_S0DCSELW { w: self }
    }
    #[doc = "Bits 4:7 - Sample 1 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s1dcsel(&mut self) -> _ADC_SSDC2_S1DCSELW {
        _ADC_SSDC2_S1DCSELW { w: self }
    }
    #[doc = "Bits 8:11 - Sample 2 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s2dcsel(&mut self) -> _ADC_SSDC2_S2DCSELW {
        _ADC_SSDC2_S2DCSELW { w: self }
    }
    #[doc = "Bits 12:15 - Sample 3 Digital Comparator Select"]
    #[inline(always)]
    pub fn adc_ssdc2_s3dcsel(&mut self) -> _ADC_SSDC2_S3DCSELW {
        _ADC_SSDC2_S3DCSELW { w: self }
    }
}
