#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSOP0 {
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
pub struct ADC_SSOP0_S0DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S0DCOPR {
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
pub struct _ADC_SSOP0_S0DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S0DCOPW<'a> {
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
pub struct ADC_SSOP0_S1DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S1DCOPR {
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
pub struct _ADC_SSOP0_S1DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S1DCOPW<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSOP0_S2DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S2DCOPR {
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
pub struct _ADC_SSOP0_S2DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S2DCOPW<'a> {
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
pub struct ADC_SSOP0_S3DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S3DCOPR {
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
pub struct _ADC_SSOP0_S3DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S3DCOPW<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSOP0_S4DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S4DCOPR {
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
pub struct _ADC_SSOP0_S4DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S4DCOPW<'a> {
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
#[doc = r"Value of the field"]
pub struct ADC_SSOP0_S5DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S5DCOPR {
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
pub struct _ADC_SSOP0_S5DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S5DCOPW<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSOP0_S6DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S6DCOPR {
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
pub struct _ADC_SSOP0_S6DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S6DCOPW<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_SSOP0_S7DCOPR {
    bits: bool,
}
impl ADC_SSOP0_S7DCOPR {
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
pub struct _ADC_SSOP0_S7DCOPW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSOP0_S7DCOPW<'a> {
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
        self.w.bits &= !(1 << 28);
        self.w.bits |= ((value as u32) & 1) << 28;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s0dcop(&self) -> ADC_SSOP0_S0DCOPR {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_SSOP0_S0DCOPR { bits }
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s1dcop(&self) -> ADC_SSOP0_S1DCOPR {
        let bits = ((self.bits >> 4) & 1) != 0;
        ADC_SSOP0_S1DCOPR { bits }
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s2dcop(&self) -> ADC_SSOP0_S2DCOPR {
        let bits = ((self.bits >> 8) & 1) != 0;
        ADC_SSOP0_S2DCOPR { bits }
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s3dcop(&self) -> ADC_SSOP0_S3DCOPR {
        let bits = ((self.bits >> 12) & 1) != 0;
        ADC_SSOP0_S3DCOPR { bits }
    }
    #[doc = "Bit 16 - Sample 4 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s4dcop(&self) -> ADC_SSOP0_S4DCOPR {
        let bits = ((self.bits >> 16) & 1) != 0;
        ADC_SSOP0_S4DCOPR { bits }
    }
    #[doc = "Bit 20 - Sample 5 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s5dcop(&self) -> ADC_SSOP0_S5DCOPR {
        let bits = ((self.bits >> 20) & 1) != 0;
        ADC_SSOP0_S5DCOPR { bits }
    }
    #[doc = "Bit 24 - Sample 6 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s6dcop(&self) -> ADC_SSOP0_S6DCOPR {
        let bits = ((self.bits >> 24) & 1) != 0;
        ADC_SSOP0_S6DCOPR { bits }
    }
    #[doc = "Bit 28 - Sample 7 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s7dcop(&self) -> ADC_SSOP0_S7DCOPR {
        let bits = ((self.bits >> 28) & 1) != 0;
        ADC_SSOP0_S7DCOPR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Sample 0 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s0dcop(&mut self) -> _ADC_SSOP0_S0DCOPW {
        _ADC_SSOP0_S0DCOPW { w: self }
    }
    #[doc = "Bit 4 - Sample 1 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s1dcop(&mut self) -> _ADC_SSOP0_S1DCOPW {
        _ADC_SSOP0_S1DCOPW { w: self }
    }
    #[doc = "Bit 8 - Sample 2 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s2dcop(&mut self) -> _ADC_SSOP0_S2DCOPW {
        _ADC_SSOP0_S2DCOPW { w: self }
    }
    #[doc = "Bit 12 - Sample 3 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s3dcop(&mut self) -> _ADC_SSOP0_S3DCOPW {
        _ADC_SSOP0_S3DCOPW { w: self }
    }
    #[doc = "Bit 16 - Sample 4 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s4dcop(&mut self) -> _ADC_SSOP0_S4DCOPW {
        _ADC_SSOP0_S4DCOPW { w: self }
    }
    #[doc = "Bit 20 - Sample 5 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s5dcop(&mut self) -> _ADC_SSOP0_S5DCOPW {
        _ADC_SSOP0_S5DCOPW { w: self }
    }
    #[doc = "Bit 24 - Sample 6 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s6dcop(&mut self) -> _ADC_SSOP0_S6DCOPW {
        _ADC_SSOP0_S6DCOPW { w: self }
    }
    #[doc = "Bit 28 - Sample 7 Digital Comparator Operation"]
    #[inline(always)]
    pub fn adc_ssop0_s7dcop(&mut self) -> _ADC_SSOP0_S7DCOPW {
        _ADC_SSOP0_S7DCOPW { w: self }
    }
}
