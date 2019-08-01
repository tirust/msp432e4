#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IM {
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
pub struct ADC_IM_MASK0R {
    bits: bool,
}
impl ADC_IM_MASK0R {
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
pub struct _ADC_IM_MASK0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_MASK0W<'a> {
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
pub struct ADC_IM_MASK1R {
    bits: bool,
}
impl ADC_IM_MASK1R {
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
pub struct _ADC_IM_MASK1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_MASK1W<'a> {
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
pub struct ADC_IM_MASK2R {
    bits: bool,
}
impl ADC_IM_MASK2R {
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
pub struct _ADC_IM_MASK2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_MASK2W<'a> {
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
pub struct ADC_IM_MASK3R {
    bits: bool,
}
impl ADC_IM_MASK3R {
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
pub struct _ADC_IM_MASK3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_MASK3W<'a> {
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
pub struct ADC_IM_DMAMASK0R {
    bits: bool,
}
impl ADC_IM_DMAMASK0R {
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
pub struct _ADC_IM_DMAMASK0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DMAMASK0W<'a> {
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
pub struct ADC_IM_DMAMASK1R {
    bits: bool,
}
impl ADC_IM_DMAMASK1R {
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
pub struct _ADC_IM_DMAMASK1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DMAMASK1W<'a> {
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
pub struct ADC_IM_DMAMASK2R {
    bits: bool,
}
impl ADC_IM_DMAMASK2R {
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
pub struct _ADC_IM_DMAMASK2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DMAMASK2W<'a> {
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
pub struct ADC_IM_DMAMASK3R {
    bits: bool,
}
impl ADC_IM_DMAMASK3R {
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
pub struct _ADC_IM_DMAMASK3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DMAMASK3W<'a> {
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
pub struct ADC_IM_DCONSS0R {
    bits: bool,
}
impl ADC_IM_DCONSS0R {
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
pub struct _ADC_IM_DCONSS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DCONSS0W<'a> {
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
pub struct ADC_IM_DCONSS1R {
    bits: bool,
}
impl ADC_IM_DCONSS1R {
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
pub struct _ADC_IM_DCONSS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DCONSS1W<'a> {
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
pub struct ADC_IM_DCONSS2R {
    bits: bool,
}
impl ADC_IM_DCONSS2R {
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
pub struct _ADC_IM_DCONSS2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DCONSS2W<'a> {
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
pub struct ADC_IM_DCONSS3R {
    bits: bool,
}
impl ADC_IM_DCONSS3R {
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
pub struct _ADC_IM_DCONSS3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_IM_DCONSS3W<'a> {
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
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask0(&self) -> ADC_IM_MASK0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        ADC_IM_MASK0R { bits }
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask1(&self) -> ADC_IM_MASK1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        ADC_IM_MASK1R { bits }
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask2(&self) -> ADC_IM_MASK2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        ADC_IM_MASK2R { bits }
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask3(&self) -> ADC_IM_MASK3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        ADC_IM_MASK3R { bits }
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask0(&self) -> ADC_IM_DMAMASK0R {
        let bits = ((self.bits >> 8) & 1) != 0;
        ADC_IM_DMAMASK0R { bits }
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask1(&self) -> ADC_IM_DMAMASK1R {
        let bits = ((self.bits >> 9) & 1) != 0;
        ADC_IM_DMAMASK1R { bits }
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask2(&self) -> ADC_IM_DMAMASK2R {
        let bits = ((self.bits >> 10) & 1) != 0;
        ADC_IM_DMAMASK2R { bits }
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask3(&self) -> ADC_IM_DMAMASK3R {
        let bits = ((self.bits >> 11) & 1) != 0;
        ADC_IM_DMAMASK3R { bits }
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    pub fn adc_im_dconss0(&self) -> ADC_IM_DCONSS0R {
        let bits = ((self.bits >> 16) & 1) != 0;
        ADC_IM_DCONSS0R { bits }
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    pub fn adc_im_dconss1(&self) -> ADC_IM_DCONSS1R {
        let bits = ((self.bits >> 17) & 1) != 0;
        ADC_IM_DCONSS1R { bits }
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    pub fn adc_im_dconss2(&self) -> ADC_IM_DCONSS2R {
        let bits = ((self.bits >> 18) & 1) != 0;
        ADC_IM_DCONSS2R { bits }
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    pub fn adc_im_dconss3(&self) -> ADC_IM_DCONSS3R {
        let bits = ((self.bits >> 19) & 1) != 0;
        ADC_IM_DCONSS3R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - SS0 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask0(&mut self) -> _ADC_IM_MASK0W {
        _ADC_IM_MASK0W { w: self }
    }
    #[doc = "Bit 1 - SS1 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask1(&mut self) -> _ADC_IM_MASK1W {
        _ADC_IM_MASK1W { w: self }
    }
    #[doc = "Bit 2 - SS2 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask2(&mut self) -> _ADC_IM_MASK2W {
        _ADC_IM_MASK2W { w: self }
    }
    #[doc = "Bit 3 - SS3 Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_mask3(&mut self) -> _ADC_IM_MASK3W {
        _ADC_IM_MASK3W { w: self }
    }
    #[doc = "Bit 8 - SS0 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask0(&mut self) -> _ADC_IM_DMAMASK0W {
        _ADC_IM_DMAMASK0W { w: self }
    }
    #[doc = "Bit 9 - SS1 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask1(&mut self) -> _ADC_IM_DMAMASK1W {
        _ADC_IM_DMAMASK1W { w: self }
    }
    #[doc = "Bit 10 - SS2 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask2(&mut self) -> _ADC_IM_DMAMASK2W {
        _ADC_IM_DMAMASK2W { w: self }
    }
    #[doc = "Bit 11 - SS3 DMA Interrupt Mask"]
    #[inline(always)]
    pub fn adc_im_dmamask3(&mut self) -> _ADC_IM_DMAMASK3W {
        _ADC_IM_DMAMASK3W { w: self }
    }
    #[doc = "Bit 16 - Digital Comparator Interrupt on SS0"]
    #[inline(always)]
    pub fn adc_im_dconss0(&mut self) -> _ADC_IM_DCONSS0W {
        _ADC_IM_DCONSS0W { w: self }
    }
    #[doc = "Bit 17 - Digital Comparator Interrupt on SS1"]
    #[inline(always)]
    pub fn adc_im_dconss1(&mut self) -> _ADC_IM_DCONSS1W {
        _ADC_IM_DCONSS1W { w: self }
    }
    #[doc = "Bit 18 - Digital Comparator Interrupt on SS2"]
    #[inline(always)]
    pub fn adc_im_dconss2(&mut self) -> _ADC_IM_DCONSS2W {
        _ADC_IM_DCONSS2W { w: self }
    }
    #[doc = "Bit 19 - Digital Comparator Interrupt on SS3"]
    #[inline(always)]
    pub fn adc_im_dconss3(&mut self) -> _ADC_IM_DCONSS3W {
        _ADC_IM_DCONSS3W { w: self }
    }
}
