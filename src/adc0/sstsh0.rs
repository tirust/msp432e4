#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSTSH0 {
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
pub struct ADC_SSTSH0_TSH0R {
    bits: u8,
}
impl ADC_SSTSH0_TSH0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSTSH0_TSH1R {
    bits: u8,
}
impl ADC_SSTSH0_TSH1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSTSH0_TSH2R {
    bits: u8,
}
impl ADC_SSTSH0_TSH2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSTSH0_TSH3R {
    bits: u8,
}
impl ADC_SSTSH0_TSH3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSTSH0_TSH4R {
    bits: u8,
}
impl ADC_SSTSH0_TSH4R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH4W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSTSH0_TSH5R {
    bits: u8,
}
impl ADC_SSTSH0_TSH5R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH5W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSTSH0_TSH6R {
    bits: u8,
}
impl ADC_SSTSH0_TSH6R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH6W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSTSH0_TSH7R {
    bits: u8,
}
impl ADC_SSTSH0_TSH7R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSTSH0_TSH7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSTSH0_TSH7W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 28);
        self.w.bits |= ((value as u32) & 15) << 28;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh0(&self) -> ADC_SSTSH0_TSH0R {
        let bits = ((self.bits >> 0) & 15) as u8;
        ADC_SSTSH0_TSH0R { bits }
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh1(&self) -> ADC_SSTSH0_TSH1R {
        let bits = ((self.bits >> 4) & 15) as u8;
        ADC_SSTSH0_TSH1R { bits }
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh2(&self) -> ADC_SSTSH0_TSH2R {
        let bits = ((self.bits >> 8) & 15) as u8;
        ADC_SSTSH0_TSH2R { bits }
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh3(&self) -> ADC_SSTSH0_TSH3R {
        let bits = ((self.bits >> 12) & 15) as u8;
        ADC_SSTSH0_TSH3R { bits }
    }
    #[doc = "Bits 16:19 - 5th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh4(&self) -> ADC_SSTSH0_TSH4R {
        let bits = ((self.bits >> 16) & 15) as u8;
        ADC_SSTSH0_TSH4R { bits }
    }
    #[doc = "Bits 20:23 - 6th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh5(&self) -> ADC_SSTSH0_TSH5R {
        let bits = ((self.bits >> 20) & 15) as u8;
        ADC_SSTSH0_TSH5R { bits }
    }
    #[doc = "Bits 24:27 - 7th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh6(&self) -> ADC_SSTSH0_TSH6R {
        let bits = ((self.bits >> 24) & 15) as u8;
        ADC_SSTSH0_TSH6R { bits }
    }
    #[doc = "Bits 28:31 - 8th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh7(&self) -> ADC_SSTSH0_TSH7R {
        let bits = ((self.bits >> 28) & 15) as u8;
        ADC_SSTSH0_TSH7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - 1st Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh0(&mut self) -> _ADC_SSTSH0_TSH0W {
        _ADC_SSTSH0_TSH0W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh1(&mut self) -> _ADC_SSTSH0_TSH1W {
        _ADC_SSTSH0_TSH1W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh2(&mut self) -> _ADC_SSTSH0_TSH2W {
        _ADC_SSTSH0_TSH2W { w: self }
    }
    #[doc = "Bits 12:15 - 4th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh3(&mut self) -> _ADC_SSTSH0_TSH3W {
        _ADC_SSTSH0_TSH3W { w: self }
    }
    #[doc = "Bits 16:19 - 5th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh4(&mut self) -> _ADC_SSTSH0_TSH4W {
        _ADC_SSTSH0_TSH4W { w: self }
    }
    #[doc = "Bits 20:23 - 6th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh5(&mut self) -> _ADC_SSTSH0_TSH5W {
        _ADC_SSTSH0_TSH5W { w: self }
    }
    #[doc = "Bits 24:27 - 7th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh6(&mut self) -> _ADC_SSTSH0_TSH6W {
        _ADC_SSTSH0_TSH6W { w: self }
    }
    #[doc = "Bits 28:31 - 8th Sample and Hold Period Select"]
    #[inline(always)]
    pub fn adc_sstsh0_tsh7(&mut self) -> _ADC_SSTSH0_TSH7W {
        _ADC_SSTSH0_TSH7W { w: self }
    }
}
