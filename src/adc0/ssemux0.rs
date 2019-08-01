#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SSEMUX0 {
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
pub struct ADC_SSEMUX0_EMUX0R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX0R {
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
pub struct _ADC_SSEMUX0_EMUX0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX0W<'a> {
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
pub struct ADC_SSEMUX0_EMUX1R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX1R {
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
pub struct _ADC_SSEMUX0_EMUX1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX1W<'a> {
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
pub struct ADC_SSEMUX0_EMUX2R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX2R {
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
pub struct _ADC_SSEMUX0_EMUX2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX2W<'a> {
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
pub struct ADC_SSEMUX0_EMUX3R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX3R {
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
pub struct _ADC_SSEMUX0_EMUX3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX3W<'a> {
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
pub struct ADC_SSEMUX0_EMUX4R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX4R {
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
pub struct _ADC_SSEMUX0_EMUX4W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX4W<'a> {
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
pub struct ADC_SSEMUX0_EMUX5R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX5R {
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
pub struct _ADC_SSEMUX0_EMUX5W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX5W<'a> {
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
pub struct ADC_SSEMUX0_EMUX6R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX6R {
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
pub struct _ADC_SSEMUX0_EMUX6W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX6W<'a> {
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
pub struct ADC_SSEMUX0_EMUX7R {
    bits: bool,
}
impl ADC_SSEMUX0_EMUX7R {
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
pub struct _ADC_SSEMUX0_EMUX7W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SSEMUX0_EMUX7W<'a> {
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
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux0(&self) -> ADC_SSEMUX0_EMUX0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_SSEMUX0_EMUX0R { bits }
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux1(&self) -> ADC_SSEMUX0_EMUX1R {
        let bits = ((self.bits >> 4) & 1) != 0;
        ADC_SSEMUX0_EMUX1R { bits }
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux2(&self) -> ADC_SSEMUX0_EMUX2R {
        let bits = ((self.bits >> 8) & 1) != 0;
        ADC_SSEMUX0_EMUX2R { bits }
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux3(&self) -> ADC_SSEMUX0_EMUX3R {
        let bits = ((self.bits >> 12) & 1) != 0;
        ADC_SSEMUX0_EMUX3R { bits }
    }
    #[doc = "Bit 16 - 5th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux4(&self) -> ADC_SSEMUX0_EMUX4R {
        let bits = ((self.bits >> 16) & 1) != 0;
        ADC_SSEMUX0_EMUX4R { bits }
    }
    #[doc = "Bit 20 - 6th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux5(&self) -> ADC_SSEMUX0_EMUX5R {
        let bits = ((self.bits >> 20) & 1) != 0;
        ADC_SSEMUX0_EMUX5R { bits }
    }
    #[doc = "Bit 24 - 7th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux6(&self) -> ADC_SSEMUX0_EMUX6R {
        let bits = ((self.bits >> 24) & 1) != 0;
        ADC_SSEMUX0_EMUX6R { bits }
    }
    #[doc = "Bit 28 - 8th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux7(&self) -> ADC_SSEMUX0_EMUX7R {
        let bits = ((self.bits >> 28) & 1) != 0;
        ADC_SSEMUX0_EMUX7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 1st Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux0(&mut self) -> _ADC_SSEMUX0_EMUX0W {
        _ADC_SSEMUX0_EMUX0W { w: self }
    }
    #[doc = "Bit 4 - 2th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux1(&mut self) -> _ADC_SSEMUX0_EMUX1W {
        _ADC_SSEMUX0_EMUX1W { w: self }
    }
    #[doc = "Bit 8 - 3rd Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux2(&mut self) -> _ADC_SSEMUX0_EMUX2W {
        _ADC_SSEMUX0_EMUX2W { w: self }
    }
    #[doc = "Bit 12 - 4th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux3(&mut self) -> _ADC_SSEMUX0_EMUX3W {
        _ADC_SSEMUX0_EMUX3W { w: self }
    }
    #[doc = "Bit 16 - 5th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux4(&mut self) -> _ADC_SSEMUX0_EMUX4W {
        _ADC_SSEMUX0_EMUX4W { w: self }
    }
    #[doc = "Bit 20 - 6th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux5(&mut self) -> _ADC_SSEMUX0_EMUX5W {
        _ADC_SSEMUX0_EMUX5W { w: self }
    }
    #[doc = "Bit 24 - 7th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux6(&mut self) -> _ADC_SSEMUX0_EMUX6W {
        _ADC_SSEMUX0_EMUX6W { w: self }
    }
    #[doc = "Bit 28 - 8th Sample Input Select (Upper Bit)"]
    #[inline(always)]
    pub fn adc_ssemux0_emux7(&mut self) -> _ADC_SSEMUX0_EMUX7W {
        _ADC_SSEMUX0_EMUX7W { w: self }
    }
}
