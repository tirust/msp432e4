#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DMAEV {
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
pub struct TIMER_DMAEV_TATODMAENR {
    bits: bool,
}
impl TIMER_DMAEV_TATODMAENR {
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
pub struct _TIMER_DMAEV_TATODMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_TATODMAENW<'a> {
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
pub struct TIMER_DMAEV_CAMDMAENR {
    bits: bool,
}
impl TIMER_DMAEV_CAMDMAENR {
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
pub struct _TIMER_DMAEV_CAMDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_CAMDMAENW<'a> {
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
pub struct TIMER_DMAEV_CAEDMAENR {
    bits: bool,
}
impl TIMER_DMAEV_CAEDMAENR {
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
pub struct _TIMER_DMAEV_CAEDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_CAEDMAENW<'a> {
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
pub struct TIMER_DMAEV_RTCDMAENR {
    bits: bool,
}
impl TIMER_DMAEV_RTCDMAENR {
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
pub struct _TIMER_DMAEV_RTCDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_RTCDMAENW<'a> {
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
pub struct TIMER_DMAEV_TAMDMAENR {
    bits: bool,
}
impl TIMER_DMAEV_TAMDMAENR {
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
pub struct _TIMER_DMAEV_TAMDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_TAMDMAENW<'a> {
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
pub struct TIMER_DMAEV_TBTODMAENR {
    bits: bool,
}
impl TIMER_DMAEV_TBTODMAENR {
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
pub struct _TIMER_DMAEV_TBTODMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_TBTODMAENW<'a> {
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
pub struct TIMER_DMAEV_CBMDMAENR {
    bits: bool,
}
impl TIMER_DMAEV_CBMDMAENR {
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
pub struct _TIMER_DMAEV_CBMDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_CBMDMAENW<'a> {
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
pub struct TIMER_DMAEV_CBEDMAENR {
    bits: bool,
}
impl TIMER_DMAEV_CBEDMAENR {
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
pub struct _TIMER_DMAEV_CBEDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_CBEDMAENW<'a> {
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
pub struct TIMER_DMAEV_TBMDMAENR {
    bits: bool,
}
impl TIMER_DMAEV_TBMDMAENR {
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
pub struct _TIMER_DMAEV_TBMDMAENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_DMAEV_TBMDMAENW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - GPTM A Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tatodmaen(&self) -> TIMER_DMAEV_TATODMAENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        TIMER_DMAEV_TATODMAENR { bits }
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_camdmaen(&self) -> TIMER_DMAEV_CAMDMAENR {
        let bits = ((self.bits >> 1) & 1) != 0;
        TIMER_DMAEV_CAMDMAENR { bits }
    }
    #[doc = "Bit 2 - GPTM A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_caedmaen(&self) -> TIMER_DMAEV_CAEDMAENR {
        let bits = ((self.bits >> 2) & 1) != 0;
        TIMER_DMAEV_CAEDMAENR { bits }
    }
    #[doc = "Bit 3 - GPTM A RTC Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_rtcdmaen(&self) -> TIMER_DMAEV_RTCDMAENR {
        let bits = ((self.bits >> 3) & 1) != 0;
        TIMER_DMAEV_RTCDMAENR { bits }
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tamdmaen(&self) -> TIMER_DMAEV_TAMDMAENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_DMAEV_TAMDMAENR { bits }
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbtodmaen(&self) -> TIMER_DMAEV_TBTODMAENR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_DMAEV_TBTODMAENR { bits }
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbmdmaen(&self) -> TIMER_DMAEV_CBMDMAENR {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_DMAEV_CBMDMAENR { bits }
    }
    #[doc = "Bit 10 - GPTM B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbedmaen(&self) -> TIMER_DMAEV_CBEDMAENR {
        let bits = ((self.bits >> 10) & 1) != 0;
        TIMER_DMAEV_CBEDMAENR { bits }
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbmdmaen(&self) -> TIMER_DMAEV_TBMDMAENR {
        let bits = ((self.bits >> 11) & 1) != 0;
        TIMER_DMAEV_TBMDMAENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPTM A Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tatodmaen(&mut self) -> _TIMER_DMAEV_TATODMAENW {
        _TIMER_DMAEV_TATODMAENW { w: self }
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_camdmaen(&mut self) -> _TIMER_DMAEV_CAMDMAENW {
        _TIMER_DMAEV_CAMDMAENW { w: self }
    }
    #[doc = "Bit 2 - GPTM A Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_caedmaen(&mut self) -> _TIMER_DMAEV_CAEDMAENW {
        _TIMER_DMAEV_CAEDMAENW { w: self }
    }
    #[doc = "Bit 3 - GPTM A RTC Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_rtcdmaen(&mut self) -> _TIMER_DMAEV_RTCDMAENW {
        _TIMER_DMAEV_RTCDMAENW { w: self }
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tamdmaen(&mut self) -> _TIMER_DMAEV_TAMDMAENW {
        _TIMER_DMAEV_TAMDMAENW { w: self }
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbtodmaen(&mut self) -> _TIMER_DMAEV_TBTODMAENW {
        _TIMER_DMAEV_TBTODMAENW { w: self }
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbmdmaen(&mut self) -> _TIMER_DMAEV_CBMDMAENW {
        _TIMER_DMAEV_CBMDMAENW { w: self }
    }
    #[doc = "Bit 10 - GPTM B Capture Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_cbedmaen(&mut self) -> _TIMER_DMAEV_CBEDMAENW {
        _TIMER_DMAEV_CBEDMAENW { w: self }
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event DMA Trigger Enable"]
    #[inline(always)]
    pub fn timer_dmaev_tbmdmaen(&mut self) -> _TIMER_DMAEV_TBMDMAENW {
        _TIMER_DMAEV_TBMDMAENW { w: self }
    }
}
