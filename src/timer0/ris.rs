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
pub struct TIMER_RIS_TATORISR {
    bits: bool,
}
impl TIMER_RIS_TATORISR {
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
pub struct _TIMER_RIS_TATORISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_TATORISW<'a> {
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
pub struct TIMER_RIS_CAMRISR {
    bits: bool,
}
impl TIMER_RIS_CAMRISR {
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
pub struct _TIMER_RIS_CAMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_CAMRISW<'a> {
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
pub struct TIMER_RIS_CAERISR {
    bits: bool,
}
impl TIMER_RIS_CAERISR {
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
pub struct _TIMER_RIS_CAERISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_CAERISW<'a> {
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
pub struct TIMER_RIS_RTCRISR {
    bits: bool,
}
impl TIMER_RIS_RTCRISR {
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
pub struct _TIMER_RIS_RTCRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_RTCRISW<'a> {
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
pub struct TIMER_RIS_TAMRISR {
    bits: bool,
}
impl TIMER_RIS_TAMRISR {
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
pub struct _TIMER_RIS_TAMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_TAMRISW<'a> {
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
pub struct TIMER_RIS_DMAARISR {
    bits: bool,
}
impl TIMER_RIS_DMAARISR {
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
pub struct _TIMER_RIS_DMAARISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_DMAARISW<'a> {
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
pub struct TIMER_RIS_TBTORISR {
    bits: bool,
}
impl TIMER_RIS_TBTORISR {
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
pub struct _TIMER_RIS_TBTORISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_TBTORISW<'a> {
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
pub struct TIMER_RIS_CBMRISR {
    bits: bool,
}
impl TIMER_RIS_CBMRISR {
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
pub struct _TIMER_RIS_CBMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_CBMRISW<'a> {
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
pub struct TIMER_RIS_CBERISR {
    bits: bool,
}
impl TIMER_RIS_CBERISR {
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
pub struct _TIMER_RIS_CBERISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_CBERISW<'a> {
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
pub struct TIMER_RIS_TBMRISR {
    bits: bool,
}
impl TIMER_RIS_TBMRISR {
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
pub struct _TIMER_RIS_TBMRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_TBMRISW<'a> {
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
pub struct TIMER_RIS_DMABRISR {
    bits: bool,
}
impl TIMER_RIS_DMABRISR {
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
pub struct _TIMER_RIS_DMABRISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_RIS_DMABRISW<'a> {
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
        self.w.bits &= !(1 << 13);
        self.w.bits |= ((value as u32) & 1) << 13;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tatoris(&self) -> TIMER_RIS_TATORISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        TIMER_RIS_TATORISR { bits }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_camris(&self) -> TIMER_RIS_CAMRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        TIMER_RIS_CAMRISR { bits }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_caeris(&self) -> TIMER_RIS_CAERISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        TIMER_RIS_CAERISR { bits }
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_rtcris(&self) -> TIMER_RIS_RTCRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        TIMER_RIS_RTCRISR { bits }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tamris(&self) -> TIMER_RIS_TAMRISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_RIS_TAMRISR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmaaris(&self) -> TIMER_RIS_DMAARISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        TIMER_RIS_DMAARISR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbtoris(&self) -> TIMER_RIS_TBTORISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_RIS_TBTORISR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cbmris(&self) -> TIMER_RIS_CBMRISR {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_RIS_CBMRISR { bits }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cberis(&self) -> TIMER_RIS_CBERISR {
        let bits = ((self.bits >> 10) & 1) != 0;
        TIMER_RIS_CBERISR { bits }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbmris(&self) -> TIMER_RIS_TBMRISR {
        let bits = ((self.bits >> 11) & 1) != 0;
        TIMER_RIS_TBMRISR { bits }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmabris(&self) -> TIMER_RIS_DMABRISR {
        let bits = ((self.bits >> 13) & 1) != 0;
        TIMER_RIS_DMABRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tatoris(&mut self) -> _TIMER_RIS_TATORISW {
        _TIMER_RIS_TATORISW { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_camris(&mut self) -> _TIMER_RIS_CAMRISW {
        _TIMER_RIS_CAMRISW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_caeris(&mut self) -> _TIMER_RIS_CAERISW {
        _TIMER_RIS_CAERISW { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_rtcris(&mut self) -> _TIMER_RIS_RTCRISW {
        _TIMER_RIS_RTCRISW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tamris(&mut self) -> _TIMER_RIS_TAMRISW {
        _TIMER_RIS_TAMRISW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmaaris(&mut self) -> _TIMER_RIS_DMAARISW {
        _TIMER_RIS_DMAARISW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbtoris(&mut self) -> _TIMER_RIS_TBTORISW {
        _TIMER_RIS_TBTORISW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cbmris(&mut self) -> _TIMER_RIS_CBMRISW {
        _TIMER_RIS_CBMRISW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_cberis(&mut self) -> _TIMER_RIS_CBERISW {
        _TIMER_RIS_CBERISW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Raw Interrupt"]
    #[inline(always)]
    pub fn timer_ris_tbmris(&mut self) -> _TIMER_RIS_TBMRISW {
        _TIMER_RIS_TBMRISW { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Raw Interrupt Status"]
    #[inline(always)]
    pub fn timer_ris_dmabris(&mut self) -> _TIMER_RIS_DMABRISW {
        _TIMER_RIS_DMABRISW { w: self }
    }
}
