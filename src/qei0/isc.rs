#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ISC {
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
pub struct QEI_ISC_INDEXR {
    bits: bool,
}
impl QEI_ISC_INDEXR {
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
pub struct _QEI_ISC_INDEXW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_ISC_INDEXW<'a> {
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
pub struct QEI_ISC_TIMERR {
    bits: bool,
}
impl QEI_ISC_TIMERR {
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
pub struct _QEI_ISC_TIMERW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_ISC_TIMERW<'a> {
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
pub struct QEI_ISC_DIRR {
    bits: bool,
}
impl QEI_ISC_DIRR {
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
pub struct _QEI_ISC_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_ISC_DIRW<'a> {
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
pub struct QEI_ISC_ERRORR {
    bits: bool,
}
impl QEI_ISC_ERRORR {
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
pub struct _QEI_ISC_ERRORW<'a> {
    w: &'a mut W,
}
impl<'a> _QEI_ISC_ERRORW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    pub fn qei_isc_index(&self) -> QEI_ISC_INDEXR {
        let bits = ((self.bits >> 0) & 1) != 0;
        QEI_ISC_INDEXR { bits }
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    pub fn qei_isc_timer(&self) -> QEI_ISC_TIMERR {
        let bits = ((self.bits >> 1) & 1) != 0;
        QEI_ISC_TIMERR { bits }
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    pub fn qei_isc_dir(&self) -> QEI_ISC_DIRR {
        let bits = ((self.bits >> 2) & 1) != 0;
        QEI_ISC_DIRR { bits }
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    pub fn qei_isc_error(&self) -> QEI_ISC_ERRORR {
        let bits = ((self.bits >> 3) & 1) != 0;
        QEI_ISC_ERRORR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Index Pulse Interrupt"]
    #[inline(always)]
    pub fn qei_isc_index(&mut self) -> _QEI_ISC_INDEXW {
        _QEI_ISC_INDEXW { w: self }
    }
    #[doc = "Bit 1 - Velocity Timer Expired Interrupt"]
    #[inline(always)]
    pub fn qei_isc_timer(&mut self) -> _QEI_ISC_TIMERW {
        _QEI_ISC_TIMERW { w: self }
    }
    #[doc = "Bit 2 - Direction Change Interrupt"]
    #[inline(always)]
    pub fn qei_isc_dir(&mut self) -> _QEI_ISC_DIRW {
        _QEI_ISC_DIRW { w: self }
    }
    #[doc = "Bit 3 - Phase Error Interrupt"]
    #[inline(always)]
    pub fn qei_isc_error(&mut self) -> _QEI_ISC_ERRORW {
        _QEI_ISC_ERRORW { w: self }
    }
}
