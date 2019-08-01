#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCGCTIMER {
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
pub struct SYSCTL_DCGCTIMER_D0R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D0R {
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
pub struct _SYSCTL_DCGCTIMER_D0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D0W<'a> {
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
pub struct SYSCTL_DCGCTIMER_D1R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D1R {
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
pub struct _SYSCTL_DCGCTIMER_D1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D1W<'a> {
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
pub struct SYSCTL_DCGCTIMER_D2R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D2R {
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
pub struct _SYSCTL_DCGCTIMER_D2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D2W<'a> {
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
pub struct SYSCTL_DCGCTIMER_D3R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D3R {
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
pub struct _SYSCTL_DCGCTIMER_D3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D3W<'a> {
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
pub struct SYSCTL_DCGCTIMER_D4R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D4R {
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
pub struct _SYSCTL_DCGCTIMER_D4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D4W<'a> {
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
pub struct SYSCTL_DCGCTIMER_D5R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D5R {
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
pub struct _SYSCTL_DCGCTIMER_D5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D5W<'a> {
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
pub struct SYSCTL_DCGCTIMER_D6R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D6R {
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
pub struct _SYSCTL_DCGCTIMER_D6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D6W<'a> {
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
pub struct SYSCTL_DCGCTIMER_D7R {
    bits: bool,
}
impl SYSCTL_DCGCTIMER_D7R {
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
pub struct _SYSCTL_DCGCTIMER_D7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCTIMER_D7W<'a> {
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
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d0(&self) -> SYSCTL_DCGCTIMER_D0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_DCGCTIMER_D0R { bits }
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d1(&self) -> SYSCTL_DCGCTIMER_D1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_DCGCTIMER_D1R { bits }
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d2(&self) -> SYSCTL_DCGCTIMER_D2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_DCGCTIMER_D2R { bits }
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d3(&self) -> SYSCTL_DCGCTIMER_D3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_DCGCTIMER_D3R { bits }
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d4(&self) -> SYSCTL_DCGCTIMER_D4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_DCGCTIMER_D4R { bits }
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d5(&self) -> SYSCTL_DCGCTIMER_D5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_DCGCTIMER_D5R { bits }
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d6(&self) -> SYSCTL_DCGCTIMER_D6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_DCGCTIMER_D6R { bits }
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d7(&self) -> SYSCTL_DCGCTIMER_D7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_DCGCTIMER_D7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - 16/32-Bit General-Purpose Timer 0 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d0(&mut self) -> _SYSCTL_DCGCTIMER_D0W {
        _SYSCTL_DCGCTIMER_D0W { w: self }
    }
    #[doc = "Bit 1 - 16/32-Bit General-Purpose Timer 1 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d1(&mut self) -> _SYSCTL_DCGCTIMER_D1W {
        _SYSCTL_DCGCTIMER_D1W { w: self }
    }
    #[doc = "Bit 2 - 16/32-Bit General-Purpose Timer 2 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d2(&mut self) -> _SYSCTL_DCGCTIMER_D2W {
        _SYSCTL_DCGCTIMER_D2W { w: self }
    }
    #[doc = "Bit 3 - 16/32-Bit General-Purpose Timer 3 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d3(&mut self) -> _SYSCTL_DCGCTIMER_D3W {
        _SYSCTL_DCGCTIMER_D3W { w: self }
    }
    #[doc = "Bit 4 - 16/32-Bit General-Purpose Timer 4 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d4(&mut self) -> _SYSCTL_DCGCTIMER_D4W {
        _SYSCTL_DCGCTIMER_D4W { w: self }
    }
    #[doc = "Bit 5 - 16/32-Bit General-Purpose Timer 5 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d5(&mut self) -> _SYSCTL_DCGCTIMER_D5W {
        _SYSCTL_DCGCTIMER_D5W { w: self }
    }
    #[doc = "Bit 6 - 16/32-Bit General-Purpose Timer 6 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d6(&mut self) -> _SYSCTL_DCGCTIMER_D6W {
        _SYSCTL_DCGCTIMER_D6W { w: self }
    }
    #[doc = "Bit 7 - 16/32-Bit General-Purpose Timer 7 Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgctimer_d7(&mut self) -> _SYSCTL_DCGCTIMER_D7W {
        _SYSCTL_DCGCTIMER_D7W { w: self }
    }
}
