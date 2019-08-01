#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIS {
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
pub struct TIMER_MIS_TATOMISR {
    bits: bool,
}
impl TIMER_MIS_TATOMISR {
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
pub struct _TIMER_MIS_TATOMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_TATOMISW<'a> {
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
pub struct TIMER_MIS_CAMMISR {
    bits: bool,
}
impl TIMER_MIS_CAMMISR {
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
pub struct _TIMER_MIS_CAMMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_CAMMISW<'a> {
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
pub struct TIMER_MIS_CAEMISR {
    bits: bool,
}
impl TIMER_MIS_CAEMISR {
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
pub struct _TIMER_MIS_CAEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_CAEMISW<'a> {
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
pub struct TIMER_MIS_RTCMISR {
    bits: bool,
}
impl TIMER_MIS_RTCMISR {
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
pub struct _TIMER_MIS_RTCMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_RTCMISW<'a> {
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
pub struct TIMER_MIS_TAMMISR {
    bits: bool,
}
impl TIMER_MIS_TAMMISR {
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
pub struct _TIMER_MIS_TAMMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_TAMMISW<'a> {
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
pub struct TIMER_MIS_DMAAMISR {
    bits: bool,
}
impl TIMER_MIS_DMAAMISR {
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
pub struct _TIMER_MIS_DMAAMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_DMAAMISW<'a> {
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
pub struct TIMER_MIS_TBTOMISR {
    bits: bool,
}
impl TIMER_MIS_TBTOMISR {
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
pub struct _TIMER_MIS_TBTOMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_TBTOMISW<'a> {
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
pub struct TIMER_MIS_CBMMISR {
    bits: bool,
}
impl TIMER_MIS_CBMMISR {
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
pub struct _TIMER_MIS_CBMMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_CBMMISW<'a> {
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
pub struct TIMER_MIS_CBEMISR {
    bits: bool,
}
impl TIMER_MIS_CBEMISR {
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
pub struct _TIMER_MIS_CBEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_CBEMISW<'a> {
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
pub struct TIMER_MIS_TBMMISR {
    bits: bool,
}
impl TIMER_MIS_TBMMISR {
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
pub struct _TIMER_MIS_TBMMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_TBMMISW<'a> {
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
pub struct TIMER_MIS_DMABMISR {
    bits: bool,
}
impl TIMER_MIS_DMABMISR {
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
pub struct _TIMER_MIS_DMABMISW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_MIS_DMABMISW<'a> {
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
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tatomis(&self) -> TIMER_MIS_TATOMISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        TIMER_MIS_TATOMISR { bits }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cammis(&self) -> TIMER_MIS_CAMMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        TIMER_MIS_CAMMISR { bits }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_caemis(&self) -> TIMER_MIS_CAEMISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        TIMER_MIS_CAEMISR { bits }
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_rtcmis(&self) -> TIMER_MIS_RTCMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        TIMER_MIS_RTCMISR { bits }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tammis(&self) -> TIMER_MIS_TAMMISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_MIS_TAMMISR { bits }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmaamis(&self) -> TIMER_MIS_DMAAMISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        TIMER_MIS_DMAAMISR { bits }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbtomis(&self) -> TIMER_MIS_TBTOMISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_MIS_TBTOMISR { bits }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbmmis(&self) -> TIMER_MIS_CBMMISR {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_MIS_CBMMISR { bits }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbemis(&self) -> TIMER_MIS_CBEMISR {
        let bits = ((self.bits >> 10) & 1) != 0;
        TIMER_MIS_CBEMISR { bits }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbmmis(&self) -> TIMER_MIS_TBMMISR {
        let bits = ((self.bits >> 11) & 1) != 0;
        TIMER_MIS_TBMMISR { bits }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmabmis(&self) -> TIMER_MIS_DMABMISR {
        let bits = ((self.bits >> 13) & 1) != 0;
        TIMER_MIS_DMABMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPTM Timer A Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tatomis(&mut self) -> _TIMER_MIS_TATOMISW {
        _TIMER_MIS_TATOMISW { w: self }
    }
    #[doc = "Bit 1 - GPTM Timer A Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cammis(&mut self) -> _TIMER_MIS_CAMMISW {
        _TIMER_MIS_CAMMISW { w: self }
    }
    #[doc = "Bit 2 - GPTM Timer A Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_caemis(&mut self) -> _TIMER_MIS_CAEMISW {
        _TIMER_MIS_CAEMISW { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_rtcmis(&mut self) -> _TIMER_MIS_RTCMISW {
        _TIMER_MIS_RTCMISW { w: self }
    }
    #[doc = "Bit 4 - GPTM Timer A Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tammis(&mut self) -> _TIMER_MIS_TAMMISW {
        _TIMER_MIS_TAMMISW { w: self }
    }
    #[doc = "Bit 5 - GPTM Timer A DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmaamis(&mut self) -> _TIMER_MIS_DMAAMISW {
        _TIMER_MIS_DMAAMISW { w: self }
    }
    #[doc = "Bit 8 - GPTM Timer B Time-Out Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbtomis(&mut self) -> _TIMER_MIS_TBTOMISW {
        _TIMER_MIS_TBTOMISW { w: self }
    }
    #[doc = "Bit 9 - GPTM Timer B Capture Mode Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbmmis(&mut self) -> _TIMER_MIS_CBMMISW {
        _TIMER_MIS_CBMMISW { w: self }
    }
    #[doc = "Bit 10 - GPTM Timer B Capture Mode Event Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_cbemis(&mut self) -> _TIMER_MIS_CBEMISW {
        _TIMER_MIS_CBEMISW { w: self }
    }
    #[doc = "Bit 11 - GPTM Timer B Match Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_tbmmis(&mut self) -> _TIMER_MIS_TBMMISW {
        _TIMER_MIS_TBMMISW { w: self }
    }
    #[doc = "Bit 13 - GPTM Timer B DMA Done Masked Interrupt"]
    #[inline(always)]
    pub fn timer_mis_dmabmis(&mut self) -> _TIMER_MIS_DMABMISW {
        _TIMER_MIS_DMABMISW { w: self }
    }
}
