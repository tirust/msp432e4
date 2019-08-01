#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISC {
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
pub struct ADC_ISC_IN0R {
    bits: bool,
}
impl ADC_ISC_IN0R {
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
pub struct _ADC_ISC_IN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_IN0W<'a> {
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
pub struct ADC_ISC_IN1R {
    bits: bool,
}
impl ADC_ISC_IN1R {
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
pub struct _ADC_ISC_IN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_IN1W<'a> {
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
pub struct ADC_ISC_IN2R {
    bits: bool,
}
impl ADC_ISC_IN2R {
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
pub struct _ADC_ISC_IN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_IN2W<'a> {
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
pub struct ADC_ISC_IN3R {
    bits: bool,
}
impl ADC_ISC_IN3R {
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
pub struct _ADC_ISC_IN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_IN3W<'a> {
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
pub struct ADC_ISC_DMAIN0R {
    bits: bool,
}
impl ADC_ISC_DMAIN0R {
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
pub struct _ADC_ISC_DMAIN0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DMAIN0W<'a> {
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
pub struct ADC_ISC_DMAIN1R {
    bits: bool,
}
impl ADC_ISC_DMAIN1R {
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
pub struct _ADC_ISC_DMAIN1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DMAIN1W<'a> {
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
pub struct ADC_ISC_DMAIN2R {
    bits: bool,
}
impl ADC_ISC_DMAIN2R {
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
pub struct _ADC_ISC_DMAIN2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DMAIN2W<'a> {
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
pub struct ADC_ISC_DMAIN3R {
    bits: bool,
}
impl ADC_ISC_DMAIN3R {
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
pub struct _ADC_ISC_DMAIN3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DMAIN3W<'a> {
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
pub struct ADC_ISC_DCINSS0R {
    bits: bool,
}
impl ADC_ISC_DCINSS0R {
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
pub struct _ADC_ISC_DCINSS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DCINSS0W<'a> {
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
pub struct ADC_ISC_DCINSS1R {
    bits: bool,
}
impl ADC_ISC_DCINSS1R {
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
pub struct _ADC_ISC_DCINSS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DCINSS1W<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_ISC_DCINSS2R {
    bits: bool,
}
impl ADC_ISC_DCINSS2R {
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
pub struct _ADC_ISC_DCINSS2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DCINSS2W<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_ISC_DCINSS3R {
    bits: bool,
}
impl ADC_ISC_DCINSS3R {
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
pub struct _ADC_ISC_DCINSS3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_ISC_DCINSS3W<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in0(&self) -> ADC_ISC_IN0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_ISC_IN0R { bits }
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in1(&self) -> ADC_ISC_IN1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_ISC_IN1R { bits }
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in2(&self) -> ADC_ISC_IN2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_ISC_IN2R { bits }
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in3(&self) -> ADC_ISC_IN3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_ISC_IN3R { bits }
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain0(&self) -> ADC_ISC_DMAIN0R {
        let bits = ((self.bits >> 8) & 1) != 0;
        ADC_ISC_DMAIN0R { bits }
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain1(&self) -> ADC_ISC_DMAIN1R {
        let bits = ((self.bits >> 9) & 1) != 0;
        ADC_ISC_DMAIN1R { bits }
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain2(&self) -> ADC_ISC_DMAIN2R {
        let bits = ((self.bits >> 10) & 1) != 0;
        ADC_ISC_DMAIN2R { bits }
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain3(&self) -> ADC_ISC_DMAIN3R {
        let bits = ((self.bits >> 11) & 1) != 0;
        ADC_ISC_DMAIN3R { bits }
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    pub fn adc_isc_dcinss0(&self) -> ADC_ISC_DCINSS0R {
        let bits = ((self.bits >> 16) & 1) != 0;
        ADC_ISC_DCINSS0R { bits }
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    pub fn adc_isc_dcinss1(&self) -> ADC_ISC_DCINSS1R {
        let bits = ((self.bits >> 17) & 1) != 0;
        ADC_ISC_DCINSS1R { bits }
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    pub fn adc_isc_dcinss2(&self) -> ADC_ISC_DCINSS2R {
        let bits = ((self.bits >> 18) & 1) != 0;
        ADC_ISC_DCINSS2R { bits }
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    pub fn adc_isc_dcinss3(&self) -> ADC_ISC_DCINSS3R {
        let bits = ((self.bits >> 19) & 1) != 0;
        ADC_ISC_DCINSS3R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SS0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in0(&mut self) -> _ADC_ISC_IN0W {
        _ADC_ISC_IN0W { w: self }
    }
    #[doc = "Bit 1 - SS1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in1(&mut self) -> _ADC_ISC_IN1W {
        _ADC_ISC_IN1W { w: self }
    }
    #[doc = "Bit 2 - SS2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in2(&mut self) -> _ADC_ISC_IN2W {
        _ADC_ISC_IN2W { w: self }
    }
    #[doc = "Bit 3 - SS3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_in3(&mut self) -> _ADC_ISC_IN3W {
        _ADC_ISC_IN3W { w: self }
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain0(&mut self) -> _ADC_ISC_DMAIN0W {
        _ADC_ISC_DMAIN0W { w: self }
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain1(&mut self) -> _ADC_ISC_DMAIN1W {
        _ADC_ISC_DMAIN1W { w: self }
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain2(&mut self) -> _ADC_ISC_DMAIN2W {
        _ADC_ISC_DMAIN2W { w: self }
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_isc_dmain3(&mut self) -> _ADC_ISC_DMAIN3W {
        _ADC_ISC_DMAIN3W { w: self }
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt Status on SS0"]
    #[inline(always)]
    pub fn adc_isc_dcinss0(&mut self) -> _ADC_ISC_DCINSS0W {
        _ADC_ISC_DCINSS0W { w: self }
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt Status on SS1"]
    #[inline(always)]
    pub fn adc_isc_dcinss1(&mut self) -> _ADC_ISC_DCINSS1W {
        _ADC_ISC_DCINSS1W { w: self }
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt Status on SS2"]
    #[inline(always)]
    pub fn adc_isc_dcinss2(&mut self) -> _ADC_ISC_DCINSS2W {
        _ADC_ISC_DCINSS2W { w: self }
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt Status on SS3"]
    #[inline(always)]
    pub fn adc_isc_dcinss3(&mut self) -> _ADC_ISC_DCINSS3W {
        _ADC_ISC_DCINSS3W { w: self }
    }
}
