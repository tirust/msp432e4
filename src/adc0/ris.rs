#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RIS {
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
pub struct ADC_RIS_INR0R {
    bits: bool,
}
impl ADC_RIS_INR0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_INR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_INR0W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_INR1R {
    bits: bool,
}
impl ADC_RIS_INR1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_INR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_INR1W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_INR2R {
    bits: bool,
}
impl ADC_RIS_INR2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_INR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_INR2W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_INR3R {
    bits: bool,
}
impl ADC_RIS_INR3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_INR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_INR3W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_DMAINR0R {
    bits: bool,
}
impl ADC_RIS_DMAINR0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_DMAINR0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_DMAINR0W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_DMAINR1R {
    bits: bool,
}
impl ADC_RIS_DMAINR1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_DMAINR1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_DMAINR1W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_DMAINR2R {
    bits: bool,
}
impl ADC_RIS_DMAINR2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_DMAINR2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_DMAINR2W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_DMAINR3R {
    bits: bool,
}
impl ADC_RIS_DMAINR3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_DMAINR3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_DMAINR3W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_RIS_INRDCR {
    bits: bool,
}
impl ADC_RIS_INRDCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bit(&self) -> bool {
        self.bits
    }
    #[doc = r"Returns `true` if the bit is clear (0)"]
    #[inline(always)]
    pub fn bit_is_clear(&self) -> bool {
        !self.bit()
    }
    #[doc = r"Returns `true` if the bit is set (1)"]
    #[inline(always)]
    pub fn bit_is_set(&self) -> bool {
        self.bit()
    }
}
#[doc = r"Proxy"]
pub struct _ADC_RIS_INRDCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_RIS_INRDCW<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr0(&self) -> ADC_RIS_INR0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_RIS_INR0R { bits }
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr1(&self) -> ADC_RIS_INR1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_RIS_INR1R { bits }
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr2(&self) -> ADC_RIS_INR2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_RIS_INR2R { bits }
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr3(&self) -> ADC_RIS_INR3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_RIS_INR3R { bits }
    }
    #[doc = "Bit 8 - SS0 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr0(&self) -> ADC_RIS_DMAINR0R {
        let bits = ((self.bits >> 8) & 1) != 0;
        ADC_RIS_DMAINR0R { bits }
    }
    #[doc = "Bit 9 - SS1 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr1(&self) -> ADC_RIS_DMAINR1R {
        let bits = ((self.bits >> 9) & 1) != 0;
        ADC_RIS_DMAINR1R { bits }
    }
    #[doc = "Bit 10 - SS2 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr2(&self) -> ADC_RIS_DMAINR2R {
        let bits = ((self.bits >> 10) & 1) != 0;
        ADC_RIS_DMAINR2R { bits }
    }
    #[doc = "Bit 11 - SS3 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr3(&self) -> ADC_RIS_DMAINR3R {
        let bits = ((self.bits >> 11) & 1) != 0;
        ADC_RIS_DMAINR3R { bits }
    }
    #[doc = "Bit 16 - Digital Comparator Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inrdc(&self) -> ADC_RIS_INRDCR {
        let bits = ((self.bits >> 16) & 1) != 0;
        ADC_RIS_INRDCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SS0 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr0(&mut self) -> _ADC_RIS_INR0W {
        _ADC_RIS_INR0W { w: self }
    }
    #[doc = "Bit 1 - SS1 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr1(&mut self) -> _ADC_RIS_INR1W {
        _ADC_RIS_INR1W { w: self }
    }
    #[doc = "Bit 2 - SS2 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr2(&mut self) -> _ADC_RIS_INR2W {
        _ADC_RIS_INR2W { w: self }
    }
    #[doc = "Bit 3 - SS3 Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inr3(&mut self) -> _ADC_RIS_INR3W {
        _ADC_RIS_INR3W { w: self }
    }
    #[doc = "Bit 8 - SS0 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr0(&mut self) -> _ADC_RIS_DMAINR0W {
        _ADC_RIS_DMAINR0W { w: self }
    }
    #[doc = "Bit 9 - SS1 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr1(&mut self) -> _ADC_RIS_DMAINR1W {
        _ADC_RIS_DMAINR1W { w: self }
    }
    #[doc = "Bit 10 - SS2 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr2(&mut self) -> _ADC_RIS_DMAINR2W {
        _ADC_RIS_DMAINR2W { w: self }
    }
    #[doc = "Bit 11 - SS3 DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_dmainr3(&mut self) -> _ADC_RIS_DMAINR3W {
        _ADC_RIS_DMAINR3W { w: self }
    }
    #[doc = "Bit 16 - Digital Comparator Raw Interrupt Status"]
    #[inline(always)]
    pub fn adc_ris_inrdc(&mut self) -> _ADC_RIS_INRDCW {
        _ADC_RIS_INRDCW { w: self }
    }
}
