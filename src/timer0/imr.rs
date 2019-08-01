#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IMR {
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
pub struct TIMER_IMR_TATOIMR {
    bits: bool,
}
impl TIMER_IMR_TATOIMR {
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
pub struct _TIMER_IMR_TATOIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_TATOIMW<'a> {
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
pub struct TIMER_IMR_CAMIMR {
    bits: bool,
}
impl TIMER_IMR_CAMIMR {
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
pub struct _TIMER_IMR_CAMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_CAMIMW<'a> {
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
pub struct TIMER_IMR_CAEIMR {
    bits: bool,
}
impl TIMER_IMR_CAEIMR {
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
pub struct _TIMER_IMR_CAEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_CAEIMW<'a> {
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
pub struct TIMER_IMR_RTCIMR {
    bits: bool,
}
impl TIMER_IMR_RTCIMR {
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
pub struct _TIMER_IMR_RTCIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_RTCIMW<'a> {
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
pub struct TIMER_IMR_TAMIMR {
    bits: bool,
}
impl TIMER_IMR_TAMIMR {
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
pub struct _TIMER_IMR_TAMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_TAMIMW<'a> {
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
pub struct TIMER_IMR_DMAAIMR {
    bits: bool,
}
impl TIMER_IMR_DMAAIMR {
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
pub struct _TIMER_IMR_DMAAIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_DMAAIMW<'a> {
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
pub struct TIMER_IMR_TBTOIMR {
    bits: bool,
}
impl TIMER_IMR_TBTOIMR {
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
pub struct _TIMER_IMR_TBTOIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_TBTOIMW<'a> {
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
pub struct TIMER_IMR_CBMIMR {
    bits: bool,
}
impl TIMER_IMR_CBMIMR {
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
pub struct _TIMER_IMR_CBMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_CBMIMW<'a> {
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
pub struct TIMER_IMR_CBEIMR {
    bits: bool,
}
impl TIMER_IMR_CBEIMR {
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
pub struct _TIMER_IMR_CBEIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_CBEIMW<'a> {
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
pub struct TIMER_IMR_TBMIMR {
    bits: bool,
}
impl TIMER_IMR_TBMIMR {
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
pub struct _TIMER_IMR_TBMIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_TBMIMW<'a> {
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
pub struct TIMER_IMR_DMABIMR {
    bits: bool,
}
impl TIMER_IMR_DMABIMR {
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
pub struct _TIMER_IMR_DMABIMW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_IMR_DMABIMW<'a> {
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
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tatoim(&self) -> TIMER_IMR_TATOIMR {
        let bits = ((self.bits >> 0) & 1) != 0;
        TIMER_IMR_TATOIMR { bits }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_camim(&self) -> TIMER_IMR_CAMIMR {
        let bits = ((self.bits >> 1) & 1) != 0;
        TIMER_IMR_CAMIMR { bits }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_caeim(&self) -> TIMER_IMR_CAEIMR {
        let bits = ((self.bits >> 2) & 1) != 0;
        TIMER_IMR_CAEIMR { bits }
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_rtcim(&self) -> TIMER_IMR_RTCIMR {
        let bits = ((self.bits >> 3) & 1) != 0;
        TIMER_IMR_RTCIMR { bits }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tamim(&self) -> TIMER_IMR_TAMIMR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_IMR_TAMIMR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmaaim(&self) -> TIMER_IMR_DMAAIMR {
        let bits = ((self.bits >> 5) & 1) != 0;
        TIMER_IMR_DMAAIMR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbtoim(&self) -> TIMER_IMR_TBTOIMR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_IMR_TBTOIMR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbmim(&self) -> TIMER_IMR_CBMIMR {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_IMR_CBMIMR { bits }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbeim(&self) -> TIMER_IMR_CBEIMR {
        let bits = ((self.bits >> 10) & 1) != 0;
        TIMER_IMR_CBEIMR { bits }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbmim(&self) -> TIMER_IMR_TBMIMR {
        let bits = ((self.bits >> 11) & 1) != 0;
        TIMER_IMR_TBMIMR { bits }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmabim(&self) -> TIMER_IMR_DMABIMR {
        let bits = ((self.bits >> 13) & 1) != 0;
        TIMER_IMR_DMABIMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPTM Timer A Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tatoim(&mut self) -> _TIMER_IMR_TATOIMW {
        _TIMER_IMR_TATOIMW { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_camim(&mut self) -> _TIMER_IMR_CAMIMW {
        _TIMER_IMR_CAMIMW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_caeim(&mut self) -> _TIMER_IMR_CAEIMW {
        _TIMER_IMR_CAEIMW { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_rtcim(&mut self) -> _TIMER_IMR_RTCIMW {
        _TIMER_IMR_RTCIMW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tamim(&mut self) -> _TIMER_IMR_TAMIMW {
        _TIMER_IMR_TAMIMW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmaaim(&mut self) -> _TIMER_IMR_DMAAIMW {
        _TIMER_IMR_DMAAIMW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbtoim(&mut self) -> _TIMER_IMR_TBTOIMW {
        _TIMER_IMR_TBTOIMW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbmim(&mut self) -> _TIMER_IMR_CBMIMW {
        _TIMER_IMR_CBMIMW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_cbeim(&mut self) -> _TIMER_IMR_CBEIMW {
        _TIMER_IMR_CBEIMW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_tbmim(&mut self) -> _TIMER_IMR_TBMIMW {
        _TIMER_IMR_TBMIMW { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Mask"]
    #[inline(always)]
    pub fn timer_imr_dmabim(&mut self) -> _TIMER_IMR_DMABIMW {
        _TIMER_IMR_DMABIMW { w: self }
    }
}
