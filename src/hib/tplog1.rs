#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TPLOG1 {
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
pub struct HIB_TPLOG1_TRIG0R {
    bits: bool,
}
impl HIB_TPLOG1_TRIG0R {
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
pub struct _HIB_TPLOG1_TRIG0W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPLOG1_TRIG0W<'a> {
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
pub struct HIB_TPLOG1_TRIG1R {
    bits: bool,
}
impl HIB_TPLOG1_TRIG1R {
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
pub struct _HIB_TPLOG1_TRIG1W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPLOG1_TRIG1W<'a> {
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
pub struct HIB_TPLOG1_TRIG2R {
    bits: bool,
}
impl HIB_TPLOG1_TRIG2R {
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
pub struct _HIB_TPLOG1_TRIG2W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPLOG1_TRIG2W<'a> {
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
pub struct HIB_TPLOG1_TRIG3R {
    bits: bool,
}
impl HIB_TPLOG1_TRIG3R {
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
pub struct _HIB_TPLOG1_TRIG3W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPLOG1_TRIG3W<'a> {
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
pub struct HIB_TPLOG1_XOSCR {
    bits: bool,
}
impl HIB_TPLOG1_XOSCR {
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
pub struct _HIB_TPLOG1_XOSCW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPLOG1_XOSCW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Status of TMPR\\[0\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig0(&self) -> HIB_TPLOG1_TRIG0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        HIB_TPLOG1_TRIG0R { bits }
    }
    #[doc = "Bit 1 - Status of TMPR\\[1\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig1(&self) -> HIB_TPLOG1_TRIG1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        HIB_TPLOG1_TRIG1R { bits }
    }
    #[doc = "Bit 2 - Status of TMPR\\[2\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig2(&self) -> HIB_TPLOG1_TRIG2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        HIB_TPLOG1_TRIG2R { bits }
    }
    #[doc = "Bit 3 - Status of TMPR\\[3\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig3(&self) -> HIB_TPLOG1_TRIG3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        HIB_TPLOG1_TRIG3R { bits }
    }
    #[doc = "Bit 16 - Status of external 32"]
    #[inline(always)]
    pub fn hib_tplog1_xosc(&self) -> HIB_TPLOG1_XOSCR {
        let bits = ((self.bits >> 16) & 1) != 0;
        HIB_TPLOG1_XOSCR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Status of TMPR\\[0\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig0(&mut self) -> _HIB_TPLOG1_TRIG0W {
        _HIB_TPLOG1_TRIG0W { w: self }
    }
    #[doc = "Bit 1 - Status of TMPR\\[1\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig1(&mut self) -> _HIB_TPLOG1_TRIG1W {
        _HIB_TPLOG1_TRIG1W { w: self }
    }
    #[doc = "Bit 2 - Status of TMPR\\[2\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig2(&mut self) -> _HIB_TPLOG1_TRIG2W {
        _HIB_TPLOG1_TRIG2W { w: self }
    }
    #[doc = "Bit 3 - Status of TMPR\\[3\\] Trigger"]
    #[inline(always)]
    pub fn hib_tplog1_trig3(&mut self) -> _HIB_TPLOG1_TRIG3W {
        _HIB_TPLOG1_TRIG3W { w: self }
    }
    #[doc = "Bit 16 - Status of external 32"]
    #[inline(always)]
    pub fn hib_tplog1_xosc(&mut self) -> _HIB_TPLOG1_XOSCW {
        _HIB_TPLOG1_XOSCW { w: self }
    }
}
