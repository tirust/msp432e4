#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSMUX0 {
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
pub struct ADC_SSMUX0_MUX0R {
    bits: u8,
}
impl ADC_SSMUX0_MUX0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX0W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSMUX0_MUX1R {
    bits: u8,
}
impl ADC_SSMUX0_MUX1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX1W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSMUX0_MUX2R {
    bits: u8,
}
impl ADC_SSMUX0_MUX2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX2W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSMUX0_MUX3R {
    bits: u8,
}
impl ADC_SSMUX0_MUX3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX3W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSMUX0_MUX4R {
    bits: u8,
}
impl ADC_SSMUX0_MUX4R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX4W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 16);
        self.w.bits |= ((value as u32) & 15) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSMUX0_MUX5R {
    bits: u8,
}
impl ADC_SSMUX0_MUX5R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX5W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 20);
        self.w.bits |= ((value as u32) & 15) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSMUX0_MUX6R {
    bits: u8,
}
impl ADC_SSMUX0_MUX6R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX6W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 24);
        self.w.bits |= ((value as u32) & 15) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSMUX0_MUX7R {
    bits: u8,
}
impl ADC_SSMUX0_MUX7R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SSMUX0_MUX7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSMUX0_MUX7W<'a> {
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
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux0(&self) -> ADC_SSMUX0_MUX0R {
        let bits = ((self.bits >> 0) & 15) as u8;
        ADC_SSMUX0_MUX0R { bits }
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux1(&self) -> ADC_SSMUX0_MUX1R {
        let bits = ((self.bits >> 4) & 15) as u8;
        ADC_SSMUX0_MUX1R { bits }
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux2(&self) -> ADC_SSMUX0_MUX2R {
        let bits = ((self.bits >> 8) & 15) as u8;
        ADC_SSMUX0_MUX2R { bits }
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux3(&self) -> ADC_SSMUX0_MUX3R {
        let bits = ((self.bits >> 12) & 15) as u8;
        ADC_SSMUX0_MUX3R { bits }
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux4(&self) -> ADC_SSMUX0_MUX4R {
        let bits = ((self.bits >> 16) & 15) as u8;
        ADC_SSMUX0_MUX4R { bits }
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux5(&self) -> ADC_SSMUX0_MUX5R {
        let bits = ((self.bits >> 20) & 15) as u8;
        ADC_SSMUX0_MUX5R { bits }
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux6(&self) -> ADC_SSMUX0_MUX6R {
        let bits = ((self.bits >> 24) & 15) as u8;
        ADC_SSMUX0_MUX6R { bits }
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux7(&self) -> ADC_SSMUX0_MUX7R {
        let bits = ((self.bits >> 28) & 15) as u8;
        ADC_SSMUX0_MUX7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - 1st Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux0(&mut self) -> _ADC_SSMUX0_MUX0W {
        _ADC_SSMUX0_MUX0W { w: self }
    }
    #[doc = "Bits 4:7 - 2nd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux1(&mut self) -> _ADC_SSMUX0_MUX1W {
        _ADC_SSMUX0_MUX1W { w: self }
    }
    #[doc = "Bits 8:11 - 3rd Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux2(&mut self) -> _ADC_SSMUX0_MUX2W {
        _ADC_SSMUX0_MUX2W { w: self }
    }
    #[doc = "Bits 12:15 - 4th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux3(&mut self) -> _ADC_SSMUX0_MUX3W {
        _ADC_SSMUX0_MUX3W { w: self }
    }
    #[doc = "Bits 16:19 - 5th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux4(&mut self) -> _ADC_SSMUX0_MUX4W {
        _ADC_SSMUX0_MUX4W { w: self }
    }
    #[doc = "Bits 20:23 - 6th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux5(&mut self) -> _ADC_SSMUX0_MUX5W {
        _ADC_SSMUX0_MUX5W { w: self }
    }
    #[doc = "Bits 24:27 - 7th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux6(&mut self) -> _ADC_SSMUX0_MUX6W {
        _ADC_SSMUX0_MUX6W { w: self }
    }
    #[doc = "Bits 28:31 - 8th Sample Input Select"]
    #[inline(always)]
    pub fn adc_ssmux0_mux7(&mut self) -> _ADC_SSMUX0_MUX7W {
        _ADC_SSMUX0_MUX7W { w: self }
    }
}
