#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCISC {
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
pub struct ADC_DCISC_DCINT0R {
    bits: bool,
}
impl ADC_DCISC_DCINT0R {
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
pub struct _ADC_DCISC_DCINT0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT0W<'a> {
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
pub struct ADC_DCISC_DCINT1R {
    bits: bool,
}
impl ADC_DCISC_DCINT1R {
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
pub struct _ADC_DCISC_DCINT1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT1W<'a> {
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
pub struct ADC_DCISC_DCINT2R {
    bits: bool,
}
impl ADC_DCISC_DCINT2R {
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
pub struct _ADC_DCISC_DCINT2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT2W<'a> {
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
pub struct ADC_DCISC_DCINT3R {
    bits: bool,
}
impl ADC_DCISC_DCINT3R {
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
pub struct _ADC_DCISC_DCINT3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT3W<'a> {
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
pub struct ADC_DCISC_DCINT4R {
    bits: bool,
}
impl ADC_DCISC_DCINT4R {
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
pub struct _ADC_DCISC_DCINT4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT4W<'a> {
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
pub struct ADC_DCISC_DCINT5R {
    bits: bool,
}
impl ADC_DCISC_DCINT5R {
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
pub struct _ADC_DCISC_DCINT5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT5W<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u32) & 1) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_DCISC_DCINT6R {
    bits: bool,
}
impl ADC_DCISC_DCINT6R {
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
pub struct _ADC_DCISC_DCINT6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT6W<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_DCISC_DCINT7R {
    bits: bool,
}
impl ADC_DCISC_DCINT7R {
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
pub struct _ADC_DCISC_DCINT7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCISC_DCINT7W<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Digital Comparator 0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint0(&self) -> ADC_DCISC_DCINT0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_DCISC_DCINT0R { bits }
    }
    #[doc = "Bit 1 - Digital Comparator 1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint1(&self) -> ADC_DCISC_DCINT1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_DCISC_DCINT1R { bits }
    }
    #[doc = "Bit 2 - Digital Comparator 2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint2(&self) -> ADC_DCISC_DCINT2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_DCISC_DCINT2R { bits }
    }
    #[doc = "Bit 3 - Digital Comparator 3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint3(&self) -> ADC_DCISC_DCINT3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_DCISC_DCINT3R { bits }
    }
    #[doc = "Bit 4 - Digital Comparator 4 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint4(&self) -> ADC_DCISC_DCINT4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        ADC_DCISC_DCINT4R { bits }
    }
    #[doc = "Bit 5 - Digital Comparator 5 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint5(&self) -> ADC_DCISC_DCINT5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        ADC_DCISC_DCINT5R { bits }
    }
    #[doc = "Bit 6 - Digital Comparator 6 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint6(&self) -> ADC_DCISC_DCINT6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        ADC_DCISC_DCINT6R { bits }
    }
    #[doc = "Bit 7 - Digital Comparator 7 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint7(&self) -> ADC_DCISC_DCINT7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        ADC_DCISC_DCINT7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Digital Comparator 0 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint0(&mut self) -> _ADC_DCISC_DCINT0W {
        _ADC_DCISC_DCINT0W { w: self }
    }
    #[doc = "Bit 1 - Digital Comparator 1 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint1(&mut self) -> _ADC_DCISC_DCINT1W {
        _ADC_DCISC_DCINT1W { w: self }
    }
    #[doc = "Bit 2 - Digital Comparator 2 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint2(&mut self) -> _ADC_DCISC_DCINT2W {
        _ADC_DCISC_DCINT2W { w: self }
    }
    #[doc = "Bit 3 - Digital Comparator 3 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint3(&mut self) -> _ADC_DCISC_DCINT3W {
        _ADC_DCISC_DCINT3W { w: self }
    }
    #[doc = "Bit 4 - Digital Comparator 4 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint4(&mut self) -> _ADC_DCISC_DCINT4W {
        _ADC_DCISC_DCINT4W { w: self }
    }
    #[doc = "Bit 5 - Digital Comparator 5 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint5(&mut self) -> _ADC_DCISC_DCINT5W {
        _ADC_DCISC_DCINT5W { w: self }
    }
    #[doc = "Bit 6 - Digital Comparator 6 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint6(&mut self) -> _ADC_DCISC_DCINT6W {
        _ADC_DCISC_DCINT6W { w: self }
    }
    #[doc = "Bit 7 - Digital Comparator 7 Interrupt Status and Clear"]
    #[inline(always)]
    pub fn adc_dcisc_dcint7(&mut self) -> _ADC_DCISC_DCINT7W {
        _ADC_DCISC_DCINT7W { w: self }
    }
}
