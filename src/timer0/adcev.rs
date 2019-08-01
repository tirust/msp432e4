#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ADCEV {
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
pub struct TIMER_ADCEV_TATOADCENR {
    bits: bool,
}
impl TIMER_ADCEV_TATOADCENR {
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
pub struct _TIMER_ADCEV_TATOADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_TATOADCENW<'a> {
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
pub struct TIMER_ADCEV_CAMADCENR {
    bits: bool,
}
impl TIMER_ADCEV_CAMADCENR {
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
pub struct _TIMER_ADCEV_CAMADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_CAMADCENW<'a> {
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
pub struct TIMER_ADCEV_CAEADCENR {
    bits: bool,
}
impl TIMER_ADCEV_CAEADCENR {
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
pub struct _TIMER_ADCEV_CAEADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_CAEADCENW<'a> {
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
pub struct TIMER_ADCEV_RTCADCENR {
    bits: bool,
}
impl TIMER_ADCEV_RTCADCENR {
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
pub struct _TIMER_ADCEV_RTCADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_RTCADCENW<'a> {
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
pub struct TIMER_ADCEV_TAMADCENR {
    bits: bool,
}
impl TIMER_ADCEV_TAMADCENR {
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
pub struct _TIMER_ADCEV_TAMADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_TAMADCENW<'a> {
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
pub struct TIMER_ADCEV_TBTOADCENR {
    bits: bool,
}
impl TIMER_ADCEV_TBTOADCENR {
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
pub struct _TIMER_ADCEV_TBTOADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_TBTOADCENW<'a> {
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
pub struct TIMER_ADCEV_CBMADCENR {
    bits: bool,
}
impl TIMER_ADCEV_CBMADCENR {
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
pub struct _TIMER_ADCEV_CBMADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_CBMADCENW<'a> {
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
pub struct TIMER_ADCEV_CBEADCENR {
    bits: bool,
}
impl TIMER_ADCEV_CBEADCENR {
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
pub struct _TIMER_ADCEV_CBEADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_CBEADCENW<'a> {
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
pub struct TIMER_ADCEV_TBMADCENR {
    bits: bool,
}
impl TIMER_ADCEV_TBMADCENR {
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
pub struct _TIMER_ADCEV_TBMADCENW<'a> {
    w: &'a mut W,
}
impl<'a> _TIMER_ADCEV_TBMADCENW<'a> {
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
    #[doc = "Bit 0 - GPTM A Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tatoadcen(&self) -> TIMER_ADCEV_TATOADCENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        TIMER_ADCEV_TATOADCENR { bits }
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_camadcen(&self) -> TIMER_ADCEV_CAMADCENR {
        let bits = ((self.bits >> 1) & 1) != 0;
        TIMER_ADCEV_CAMADCENR { bits }
    }
    #[doc = "Bit 2 - GPTM A Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_caeadcen(&self) -> TIMER_ADCEV_CAEADCENR {
        let bits = ((self.bits >> 2) & 1) != 0;
        TIMER_ADCEV_CAEADCENR { bits }
    }
    #[doc = "Bit 3 - GPTM RTC Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_rtcadcen(&self) -> TIMER_ADCEV_RTCADCENR {
        let bits = ((self.bits >> 3) & 1) != 0;
        TIMER_ADCEV_RTCADCENR { bits }
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tamadcen(&self) -> TIMER_ADCEV_TAMADCENR {
        let bits = ((self.bits >> 4) & 1) != 0;
        TIMER_ADCEV_TAMADCENR { bits }
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbtoadcen(&self) -> TIMER_ADCEV_TBTOADCENR {
        let bits = ((self.bits >> 8) & 1) != 0;
        TIMER_ADCEV_TBTOADCENR { bits }
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbmadcen(&self) -> TIMER_ADCEV_CBMADCENR {
        let bits = ((self.bits >> 9) & 1) != 0;
        TIMER_ADCEV_CBMADCENR { bits }
    }
    #[doc = "Bit 10 - GPTM B Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbeadcen(&self) -> TIMER_ADCEV_CBEADCENR {
        let bits = ((self.bits >> 10) & 1) != 0;
        TIMER_ADCEV_CBEADCENR { bits }
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbmadcen(&self) -> TIMER_ADCEV_TBMADCENR {
        let bits = ((self.bits >> 11) & 1) != 0;
        TIMER_ADCEV_TBMADCENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPTM A Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tatoadcen(&mut self) -> _TIMER_ADCEV_TATOADCENW {
        _TIMER_ADCEV_TATOADCENW { w: self }
    }
    #[doc = "Bit 1 - GPTM A Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_camadcen(&mut self) -> _TIMER_ADCEV_CAMADCENW {
        _TIMER_ADCEV_CAMADCENW { w: self }
    }
    #[doc = "Bit 2 - GPTM A Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_caeadcen(&mut self) -> _TIMER_ADCEV_CAEADCENW {
        _TIMER_ADCEV_CAEADCENW { w: self }
    }
    #[doc = "Bit 3 - GPTM RTC Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_rtcadcen(&mut self) -> _TIMER_ADCEV_RTCADCENW {
        _TIMER_ADCEV_RTCADCENW { w: self }
    }
    #[doc = "Bit 4 - GPTM A Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tamadcen(&mut self) -> _TIMER_ADCEV_TAMADCENW {
        _TIMER_ADCEV_TAMADCENW { w: self }
    }
    #[doc = "Bit 8 - GPTM B Time-Out Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbtoadcen(&mut self) -> _TIMER_ADCEV_TBTOADCENW {
        _TIMER_ADCEV_TBTOADCENW { w: self }
    }
    #[doc = "Bit 9 - GPTM B Capture Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbmadcen(&mut self) -> _TIMER_ADCEV_CBMADCENW {
        _TIMER_ADCEV_CBMADCENW { w: self }
    }
    #[doc = "Bit 10 - GPTM B Capture Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_cbeadcen(&mut self) -> _TIMER_ADCEV_CBEADCENW {
        _TIMER_ADCEV_CBEADCENW { w: self }
    }
    #[doc = "Bit 11 - GPTM B Mode Match Event ADC Trigger Enable"]
    #[inline(always)]
    pub fn timer_adcev_tbmadcen(&mut self) -> _TIMER_ADCEV_TBMADCENW {
        _TIMER_ADCEV_TBMADCENW { w: self }
    }
}
