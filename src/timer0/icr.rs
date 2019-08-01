#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ICR {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_TATOCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_TATOCINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_CAMCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_CAMCINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_CAECINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_CAECINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_RTCCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_RTCCINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_TAMCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_TAMCINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_DMAAINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_DMAAINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_TBTOCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_TBTOCINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_CBMCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_CBMCINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_CBECINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_CBECINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_TBMCINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_TBMCINTW<'a> {
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
#[doc = r"Proxy"]
pub struct _TIMER_ICR_DMABINTW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ICR_DMABINTW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPTM Timer A Time-Out Raw Interrupt"]
    #[inline(always)]
    pub fn timer_icr_tatocint(&mut self) -> _TIMER_ICR_TATOCINTW {
        _TIMER_ICR_TATOCINTW { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_camcint(&mut self) -> _TIMER_ICR_CAMCINTW {
        _TIMER_ICR_CAMCINTW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_caecint(&mut self) -> _TIMER_ICR_CAECINTW {
        _TIMER_ICR_CAECINTW { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_rtccint(&mut self) -> _TIMER_ICR_RTCCINTW {
        _TIMER_ICR_RTCCINTW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_tamcint(&mut self) -> _TIMER_ICR_TAMCINTW {
        _TIMER_ICR_TAMCINTW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_dmaaint(&mut self) -> _TIMER_ICR_DMAAINTW {
        _TIMER_ICR_DMAAINTW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_tbtocint(&mut self) -> _TIMER_ICR_TBTOCINTW {
        _TIMER_ICR_TBTOCINTW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_cbmcint(&mut self) -> _TIMER_ICR_CBMCINTW {
        _TIMER_ICR_CBMCINTW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_cbecint(&mut self) -> _TIMER_ICR_CBECINTW {
        _TIMER_ICR_CBECINTW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_tbmcint(&mut self) -> _TIMER_ICR_TBMCINTW {
        _TIMER_ICR_TBMCINTW { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Interrupt Clear"]
    #[inline(always)]
    pub fn timer_icr_dmabint(&mut self) -> _TIMER_ICR_DMABINTW {
        _TIMER_ICR_DMABINTW { w: self }
    }
}
