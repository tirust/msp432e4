#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::USTAT {
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
pub struct ADC_USTAT_UV0R {
    bits: bool,
}
impl ADC_USTAT_UV0R {
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
pub struct _ADC_USTAT_UV0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_USTAT_UV0W<'a> {
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
pub struct ADC_USTAT_UV1R {
    bits: bool,
}
impl ADC_USTAT_UV1R {
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
pub struct _ADC_USTAT_UV1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_USTAT_UV1W<'a> {
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
pub struct ADC_USTAT_UV2R {
    bits: bool,
}
impl ADC_USTAT_UV2R {
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
pub struct _ADC_USTAT_UV2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_USTAT_UV2W<'a> {
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
pub struct ADC_USTAT_UV3R {
    bits: bool,
}
impl ADC_USTAT_UV3R {
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
pub struct _ADC_USTAT_UV3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_USTAT_UV3W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv0(&self) -> ADC_USTAT_UV0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_USTAT_UV0R { bits }
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv1(&self) -> ADC_USTAT_UV1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_USTAT_UV1R { bits }
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv2(&self) -> ADC_USTAT_UV2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_USTAT_UV2R { bits }
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv3(&self) -> ADC_USTAT_UV3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_USTAT_UV3R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SS0 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv0(&mut self) -> _ADC_USTAT_UV0W {
        _ADC_USTAT_UV0W { w: self }
    }
    #[doc = "Bit 1 - SS1 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv1(&mut self) -> _ADC_USTAT_UV1W {
        _ADC_USTAT_UV1W { w: self }
    }
    #[doc = "Bit 2 - SS2 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv2(&mut self) -> _ADC_USTAT_UV2W {
        _ADC_USTAT_UV2W { w: self }
    }
    #[doc = "Bit 3 - SS3 FIFO Underflow"]
    #[inline(always)]
    pub fn adc_ustat_uv3(&mut self) -> _ADC_USTAT_UV3W {
        _ADC_USTAT_UV3W { w: self }
    }
}