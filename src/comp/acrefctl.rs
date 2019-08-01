#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ACREFCTL {
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
pub struct COMP_ACREFCTL_VREFR {
    bits: u8,
}
impl COMP_ACREFCTL_VREFR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _COMP_ACREFCTL_VREFW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACREFCTL_VREFW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct COMP_ACREFCTL_RNGR {
    bits: bool,
}
impl COMP_ACREFCTL_RNGR {
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
pub struct _COMP_ACREFCTL_RNGW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACREFCTL_RNGW<'a> {
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
pub struct COMP_ACREFCTL_ENR {
    bits: bool,
}
impl COMP_ACREFCTL_ENR {
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
pub struct _COMP_ACREFCTL_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_ACREFCTL_ENW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Resistor Ladder Voltage Ref"]
    #[inline(always)]
    pub fn comp_acrefctl_vref(&self) -> COMP_ACREFCTL_VREFR {
        let bits = ((self.bits >> 0) & 15) as u8;
        COMP_ACREFCTL_VREFR { bits }
    }
    #[doc = "Bit 8 - Resistor Ladder Range"]
    #[inline(always)]
    pub fn comp_acrefctl_rng(&self) -> COMP_ACREFCTL_RNGR {
        let bits = ((self.bits >> 8) & 1) != 0;
        COMP_ACREFCTL_RNGR { bits }
    }
    #[doc = "Bit 9 - Resistor Ladder Enable"]
    #[inline(always)]
    pub fn comp_acrefctl_en(&self) -> COMP_ACREFCTL_ENR {
        let bits = ((self.bits >> 9) & 1) != 0;
        COMP_ACREFCTL_ENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Resistor Ladder Voltage Ref"]
    #[inline(always)]
    pub fn comp_acrefctl_vref(&mut self) -> _COMP_ACREFCTL_VREFW {
        _COMP_ACREFCTL_VREFW { w: self }
    }
    #[doc = "Bit 8 - Resistor Ladder Range"]
    #[inline(always)]
    pub fn comp_acrefctl_rng(&mut self) -> _COMP_ACREFCTL_RNGW {
        _COMP_ACREFCTL_RNGW { w: self }
    }
    #[doc = "Bit 9 - Resistor Ladder Enable"]
    #[inline(always)]
    pub fn comp_acrefctl_en(&mut self) -> _COMP_ACREFCTL_ENW {
        _COMP_ACREFCTL_ENW { w: self }
    }
}
