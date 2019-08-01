#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCTIMER {
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
pub struct SYSCTL_PCTIMER_P0R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P0R {
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
pub struct _SYSCTL_PCTIMER_P0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P0W<'a> {
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
pub struct SYSCTL_PCTIMER_P1R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P1R {
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
pub struct _SYSCTL_PCTIMER_P1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P1W<'a> {
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
pub struct SYSCTL_PCTIMER_P2R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P2R {
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
pub struct _SYSCTL_PCTIMER_P2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P2W<'a> {
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
pub struct SYSCTL_PCTIMER_P3R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P3R {
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
pub struct _SYSCTL_PCTIMER_P3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P3W<'a> {
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
pub struct SYSCTL_PCTIMER_P4R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P4R {
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
pub struct _SYSCTL_PCTIMER_P4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P4W<'a> {
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
pub struct SYSCTL_PCTIMER_P5R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P5R {
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
pub struct _SYSCTL_PCTIMER_P5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P5W<'a> {
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
pub struct SYSCTL_PCTIMER_P6R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P6R {
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
pub struct _SYSCTL_PCTIMER_P6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P6W<'a> {
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
pub struct SYSCTL_PCTIMER_P7R {
    bits: bool,
}
impl SYSCTL_PCTIMER_P7R {
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
pub struct _SYSCTL_PCTIMER_P7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCTIMER_P7W<'a> {
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
    #[doc = "Bit 0 - General-Purpose Timer 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p0(&self) -> SYSCTL_PCTIMER_P0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_PCTIMER_P0R { bits }
    }
    #[doc = "Bit 1 - General-Purpose Timer 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p1(&self) -> SYSCTL_PCTIMER_P1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_PCTIMER_P1R { bits }
    }
    #[doc = "Bit 2 - General-Purpose Timer 2 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p2(&self) -> SYSCTL_PCTIMER_P2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_PCTIMER_P2R { bits }
    }
    #[doc = "Bit 3 - General-Purpose Timer 3 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p3(&self) -> SYSCTL_PCTIMER_P3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_PCTIMER_P3R { bits }
    }
    #[doc = "Bit 4 - General-Purpose Timer 4 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p4(&self) -> SYSCTL_PCTIMER_P4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_PCTIMER_P4R { bits }
    }
    #[doc = "Bit 5 - General-Purpose Timer 5 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p5(&self) -> SYSCTL_PCTIMER_P5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_PCTIMER_P5R { bits }
    }
    #[doc = "Bit 6 - General-Purpose Timer 6 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p6(&self) -> SYSCTL_PCTIMER_P6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_PCTIMER_P6R { bits }
    }
    #[doc = "Bit 7 - General-Purpose Timer 7 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p7(&self) -> SYSCTL_PCTIMER_P7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_PCTIMER_P7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - General-Purpose Timer 0 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p0(&mut self) -> _SYSCTL_PCTIMER_P0W {
        _SYSCTL_PCTIMER_P0W { w: self }
    }
    #[doc = "Bit 1 - General-Purpose Timer 1 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p1(&mut self) -> _SYSCTL_PCTIMER_P1W {
        _SYSCTL_PCTIMER_P1W { w: self }
    }
    #[doc = "Bit 2 - General-Purpose Timer 2 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p2(&mut self) -> _SYSCTL_PCTIMER_P2W {
        _SYSCTL_PCTIMER_P2W { w: self }
    }
    #[doc = "Bit 3 - General-Purpose Timer 3 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p3(&mut self) -> _SYSCTL_PCTIMER_P3W {
        _SYSCTL_PCTIMER_P3W { w: self }
    }
    #[doc = "Bit 4 - General-Purpose Timer 4 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p4(&mut self) -> _SYSCTL_PCTIMER_P4W {
        _SYSCTL_PCTIMER_P4W { w: self }
    }
    #[doc = "Bit 5 - General-Purpose Timer 5 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p5(&mut self) -> _SYSCTL_PCTIMER_P5W {
        _SYSCTL_PCTIMER_P5W { w: self }
    }
    #[doc = "Bit 6 - General-Purpose Timer 6 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p6(&mut self) -> _SYSCTL_PCTIMER_P6W {
        _SYSCTL_PCTIMER_P6W { w: self }
    }
    #[doc = "Bit 7 - General-Purpose Timer 7 Power Control"]
    #[inline(always)]
    pub fn sysctl_pctimer_p7(&mut self) -> _SYSCTL_PCTIMER_P7W {
        _SYSCTL_PCTIMER_P7W { w: self }
    }
}
