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
pub struct HIB_MIS_RTCALT0R {
    bits: bool,
}
impl HIB_MIS_RTCALT0R {
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
pub struct _HIB_MIS_RTCALT0W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_MIS_RTCALT0W<'a> {
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
pub struct HIB_MIS_LOWBATR {
    bits: bool,
}
impl HIB_MIS_LOWBATR {
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
pub struct _HIB_MIS_LOWBATW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_MIS_LOWBATW<'a> {
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
pub struct HIB_MIS_EXTWR {
    bits: bool,
}
impl HIB_MIS_EXTWR {
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
pub struct _HIB_MIS_EXTWW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_MIS_EXTWW<'a> {
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
pub struct HIB_MIS_WCR {
    bits: bool,
}
impl HIB_MIS_WCR {
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
pub struct _HIB_MIS_WCW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_MIS_WCW<'a> {
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
pub struct HIB_MIS_PADIOWKR {
    bits: bool,
}
impl HIB_MIS_PADIOWKR {
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
pub struct _HIB_MIS_PADIOWKW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_MIS_PADIOWKW<'a> {
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
pub struct HIB_MIS_RSTWKR {
    bits: bool,
}
impl HIB_MIS_RSTWKR {
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
pub struct _HIB_MIS_RSTWKW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_MIS_RSTWKW<'a> {
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
pub struct HIB_MIS_VDDFAILR {
    bits: bool,
}
impl HIB_MIS_VDDFAILR {
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
pub struct _HIB_MIS_VDDFAILW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_MIS_VDDFAILW<'a> {
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
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt0(&self) -> HIB_MIS_RTCALT0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        HIB_MIS_RTCALT0R { bits }
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_lowbat(&self) -> HIB_MIS_LOWBATR {
        let bits = ((self.bits >> 2) & 1) != 0;
        HIB_MIS_LOWBATR { bits }
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_extw(&self) -> HIB_MIS_EXTWR {
        let bits = ((self.bits >> 3) & 1) != 0;
        HIB_MIS_EXTWR { bits }
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_wc(&self) -> HIB_MIS_WCR {
        let bits = ((self.bits >> 4) & 1) != 0;
        HIB_MIS_WCR { bits }
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_padiowk(&self) -> HIB_MIS_PADIOWKR {
        let bits = ((self.bits >> 5) & 1) != 0;
        HIB_MIS_PADIOWKR { bits }
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_rstwk(&self) -> HIB_MIS_RSTWKR {
        let bits = ((self.bits >> 6) & 1) != 0;
        HIB_MIS_RSTWKR { bits }
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_vddfail(&self) -> HIB_MIS_VDDFAILR {
        let bits = ((self.bits >> 7) & 1) != 0;
        HIB_MIS_VDDFAILR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - RTC Alert 0 Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_rtcalt0(&mut self) -> _HIB_MIS_RTCALT0W {
        _HIB_MIS_RTCALT0W { w: self }
    }
    #[doc = "Bit 2 - Low Battery Voltage Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_lowbat(&mut self) -> _HIB_MIS_LOWBATW {
        _HIB_MIS_LOWBATW { w: self }
    }
    #[doc = "Bit 3 - External Wake-Up Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_extw(&mut self) -> _HIB_MIS_EXTWW {
        _HIB_MIS_EXTWW { w: self }
    }
    #[doc = "Bit 4 - Write Complete/Capable Masked Interrupt Status"]
    #[inline(always)]
    pub fn hib_mis_wc(&mut self) -> _HIB_MIS_WCW {
        _HIB_MIS_WCW { w: self }
    }
    #[doc = "Bit 5 - Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_padiowk(&mut self) -> _HIB_MIS_PADIOWKW {
        _HIB_MIS_PADIOWKW { w: self }
    }
    #[doc = "Bit 6 - Reset Pad I/O Wake-Up Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_rstwk(&mut self) -> _HIB_MIS_RSTWKW {
        _HIB_MIS_RSTWKW { w: self }
    }
    #[doc = "Bit 7 - VDD Fail Interrupt Mask"]
    #[inline(always)]
    pub fn hib_mis_vddfail(&mut self) -> _HIB_MIS_VDDFAILW {
        _HIB_MIS_VDDFAILW { w: self }
    }
}
