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
pub struct HIB_IM_RTCALT0R {
    bits: bool,
}
impl HIB_IM_RTCALT0R {
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
pub struct _HIB_IM_RTCALT0W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_IM_RTCALT0W<'a> {
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
pub struct HIB_IM_LOWBATR {
    bits: bool,
}
impl HIB_IM_LOWBATR {
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
pub struct _HIB_IM_LOWBATW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_IM_LOWBATW<'a> {
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
pub struct HIB_IM_EXTWR {
    bits: bool,
}
impl HIB_IM_EXTWR {
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
pub struct _HIB_IM_EXTWW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_IM_EXTWW<'a> {
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
pub struct HIB_IM_WCR {
    bits: bool,
}
impl HIB_IM_WCR {
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
pub struct _HIB_IM_WCW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_IM_WCW<'a> {
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
pub struct HIB_IM_PADIOWKR {
    bits: bool,
}
impl HIB_IM_PADIOWKR {
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
pub struct _HIB_IM_PADIOWKW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_IM_PADIOWKW<'a> {
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
pub struct HIB_IM_RSTWKR {
    bits: bool,
}
impl HIB_IM_RSTWKR {
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
pub struct _HIB_IM_RSTWKW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_IM_RSTWKW<'a> {
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
pub struct HIB_IM_VDDFAILR {
    bits: bool,
}
impl HIB_IM_VDDFAILR {
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
pub struct _HIB_IM_VDDFAILW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_IM_VDDFAILW<'a> {
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
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rtcalt0(&self) -> HIB_IM_RTCALT0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        HIB_IM_RTCALT0R { bits }
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_lowbat(&self) -> HIB_IM_LOWBATR {
        let bits = ((self.bits >> 2) & 1) != 0;
        HIB_IM_LOWBATR { bits }
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_extw(&self) -> HIB_IM_EXTWR {
        let bits = ((self.bits >> 3) & 1) != 0;
        HIB_IM_EXTWR { bits }
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_wc(&self) -> HIB_IM_WCR {
        let bits = ((self.bits >> 4) & 1) != 0;
        HIB_IM_WCR { bits }
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_padiowk(&self) -> HIB_IM_PADIOWKR {
        let bits = ((self.bits >> 5) & 1) != 0;
        HIB_IM_PADIOWKR { bits }
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rstwk(&self) -> HIB_IM_RSTWKR {
        let bits = ((self.bits >> 6) & 1) != 0;
        HIB_IM_RSTWKR { bits }
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_vddfail(&self) -> HIB_IM_VDDFAILR {
        let bits = ((self.bits >> 7) & 1) != 0;
        HIB_IM_VDDFAILR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RTC Alert 0 Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rtcalt0(&mut self) -> _HIB_IM_RTCALT0W {
        _HIB_IM_RTCALT0W { w: self }
    }
    #[doc = "Bit 2 - Low Battery Voltage Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_lowbat(&mut self) -> _HIB_IM_LOWBATW {
        _HIB_IM_LOWBATW { w: self }
    }
    #[doc = "Bit 3 - External Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_extw(&mut self) -> _HIB_IM_EXTWW {
        _HIB_IM_EXTWW { w: self }
    }
    #[doc = "Bit 4 - External Write Complete/Capable Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_wc(&mut self) -> _HIB_IM_WCW {
        _HIB_IM_WCW { w: self }
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_padiowk(&mut self) -> _HIB_IM_PADIOWKW {
        _HIB_IM_PADIOWKW { w: self }
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_rstwk(&mut self) -> _HIB_IM_RSTWKW {
        _HIB_IM_RSTWKW { w: self }
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_im_vddfail(&mut self) -> _HIB_IM_VDDFAILW {
        _HIB_IM_VDDFAILW { w: self }
    }
}
