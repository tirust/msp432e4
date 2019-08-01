#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACTSS {
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
pub struct ADC_ACTSS_ASEN0R {
    bits: bool,
}
impl ADC_ACTSS_ASEN0R {
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
pub struct _ADC_ACTSS_ASEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ASEN0W<'a> {
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
pub struct ADC_ACTSS_ASEN1R {
    bits: bool,
}
impl ADC_ACTSS_ASEN1R {
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
pub struct _ADC_ACTSS_ASEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ASEN1W<'a> {
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
pub struct ADC_ACTSS_ASEN2R {
    bits: bool,
}
impl ADC_ACTSS_ASEN2R {
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
pub struct _ADC_ACTSS_ASEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ASEN2W<'a> {
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
pub struct ADC_ACTSS_ASEN3R {
    bits: bool,
}
impl ADC_ACTSS_ASEN3R {
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
pub struct _ADC_ACTSS_ASEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ASEN3W<'a> {
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
pub struct ADC_ACTSS_ADEN0R {
    bits: bool,
}
impl ADC_ACTSS_ADEN0R {
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
pub struct _ADC_ACTSS_ADEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ADEN0W<'a> {
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
pub struct ADC_ACTSS_ADEN1R {
    bits: bool,
}
impl ADC_ACTSS_ADEN1R {
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
pub struct _ADC_ACTSS_ADEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ADEN1W<'a> {
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
pub struct ADC_ACTSS_ADEN2R {
    bits: bool,
}
impl ADC_ACTSS_ADEN2R {
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
pub struct _ADC_ACTSS_ADEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ADEN2W<'a> {
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
pub struct ADC_ACTSS_ADEN3R {
    bits: bool,
}
impl ADC_ACTSS_ADEN3R {
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
pub struct _ADC_ACTSS_ADEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_ADEN3W<'a> {
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
pub struct ADC_ACTSS_BUSYR {
    bits: bool,
}
impl ADC_ACTSS_BUSYR {
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
pub struct _ADC_ACTSS_BUSYW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ACTSS_BUSYW<'a> {
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
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen0(&self) -> ADC_ACTSS_ASEN0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_ACTSS_ASEN0R { bits }
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen1(&self) -> ADC_ACTSS_ASEN1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_ACTSS_ASEN1R { bits }
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen2(&self) -> ADC_ACTSS_ASEN2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_ACTSS_ASEN2R { bits }
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen3(&self) -> ADC_ACTSS_ASEN3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_ACTSS_ASEN3R { bits }
    }
    #[doc = "Bit 8 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden0(&self) -> ADC_ACTSS_ADEN0R {
        let bits = ((self.bits >> 8) & 1) != 0;
        ADC_ACTSS_ADEN0R { bits }
    }
    #[doc = "Bit 9 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden1(&self) -> ADC_ACTSS_ADEN1R {
        let bits = ((self.bits >> 9) & 1) != 0;
        ADC_ACTSS_ADEN1R { bits }
    }
    #[doc = "Bit 10 - ADC SS2 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden2(&self) -> ADC_ACTSS_ADEN2R {
        let bits = ((self.bits >> 10) & 1) != 0;
        ADC_ACTSS_ADEN2R { bits }
    }
    #[doc = "Bit 11 - ADC SS3 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden3(&self) -> ADC_ACTSS_ADEN3R {
        let bits = ((self.bits >> 11) & 1) != 0;
        ADC_ACTSS_ADEN3R { bits }
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn adc_actss_busy(&self) -> ADC_ACTSS_BUSYR {
        let bits = ((self.bits >> 16) & 1) != 0;
        ADC_ACTSS_BUSYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - ADC SS0 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen0(&mut self) -> _ADC_ACTSS_ASEN0W {
        _ADC_ACTSS_ASEN0W { w: self }
    }
    #[doc = "Bit 1 - ADC SS1 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen1(&mut self) -> _ADC_ACTSS_ASEN1W {
        _ADC_ACTSS_ASEN1W { w: self }
    }
    #[doc = "Bit 2 - ADC SS2 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen2(&mut self) -> _ADC_ACTSS_ASEN2W {
        _ADC_ACTSS_ASEN2W { w: self }
    }
    #[doc = "Bit 3 - ADC SS3 Enable"]
    #[inline(always)]
    pub fn adc_actss_asen3(&mut self) -> _ADC_ACTSS_ASEN3W {
        _ADC_ACTSS_ASEN3W { w: self }
    }
    #[doc = "Bit 8 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden0(&mut self) -> _ADC_ACTSS_ADEN0W {
        _ADC_ACTSS_ADEN0W { w: self }
    }
    #[doc = "Bit 9 - ADC SS1 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden1(&mut self) -> _ADC_ACTSS_ADEN1W {
        _ADC_ACTSS_ADEN1W { w: self }
    }
    #[doc = "Bit 10 - ADC SS2 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden2(&mut self) -> _ADC_ACTSS_ADEN2W {
        _ADC_ACTSS_ADEN2W { w: self }
    }
    #[doc = "Bit 11 - ADC SS3 DMA Enable"]
    #[inline(always)]
    pub fn adc_actss_aden3(&mut self) -> _ADC_ACTSS_ADEN3W {
        _ADC_ACTSS_ADEN3W { w: self }
    }
    #[doc = "Bit 16 - ADC Busy"]
    #[inline(always)]
    pub fn adc_actss_busy(&mut self) -> _ADC_ACTSS_BUSYW {
        _ADC_ACTSS_BUSYW { w: self }
    }
}
