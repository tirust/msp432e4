#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCCMP7 {
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
pub struct ADC_DCCMP7_COMP0R {
    bits: u16,
}
impl ADC_DCCMP7_COMP0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCCMP7_COMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCMP7_COMP0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(4095 << 0);
        self.w.bits |= ((value as u32) & 4095) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_DCCMP7_COMP1R {
    bits: u16,
}
impl ADC_DCCMP7_COMP1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCCMP7_COMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCMP7_COMP1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(4095 << 16);
        self.w.bits |= ((value as u32) & 4095) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    pub fn adc_dccmp7_comp0(&self) -> ADC_DCCMP7_COMP0R {
        let bits = ((self.bits >> 0) & 4095) as u16;
        ADC_DCCMP7_COMP0R { bits }
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    pub fn adc_dccmp7_comp1(&self) -> ADC_DCCMP7_COMP1R {
        let bits = ((self.bits >> 16) & 4095) as u16;
        ADC_DCCMP7_COMP1R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:11 - Compare 0"]
    #[inline(always)]
    pub fn adc_dccmp7_comp0(&mut self) -> _ADC_DCCMP7_COMP0W {
        _ADC_DCCMP7_COMP0W { w: self }
    }
    #[doc = "Bits 16:27 - Compare 1"]
    #[inline(always)]
    pub fn adc_dccmp7_comp1(&mut self) -> _ADC_DCCMP7_COMP1W {
        _ADC_DCCMP7_COMP1W { w: self }
    }
}
