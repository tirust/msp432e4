#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CAL0 {
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
pub struct HIB_CAL0_SECR {
    bits: u8,
}
impl HIB_CAL0_SECR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CAL0_SECW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL0_SECW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 0);
        self.w.bits |= ((value as u32) & 63) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL0_MINR {
    bits: u8,
}
impl HIB_CAL0_MINR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CAL0_MINW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL0_MINW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 8);
        self.w.bits |= ((value as u32) & 63) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL0_HRR {
    bits: u8,
}
impl HIB_CAL0_HRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CAL0_HRW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL0_HRW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 16);
        self.w.bits |= ((value as u32) & 31) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL0_AMPMR {
    bits: bool,
}
impl HIB_CAL0_AMPMR {
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
pub struct _HIB_CAL0_AMPMW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL0_AMPMW<'a> {
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
        self.w.bits &= !(1 << 22);
        self.w.bits |= ((value as u32) & 1) << 22;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL0_VALIDR {
    bits: bool,
}
impl HIB_CAL0_VALIDR {
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
pub struct _HIB_CAL0_VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL0_VALIDW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn hib_cal0_sec(&self) -> HIB_CAL0_SECR {
        let bits = ((self.bits >> 0) & 63) as u8;
        HIB_CAL0_SECR { bits }
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn hib_cal0_min(&self) -> HIB_CAL0_MINR {
        let bits = ((self.bits >> 8) & 63) as u8;
        HIB_CAL0_MINR { bits }
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hib_cal0_hr(&self) -> HIB_CAL0_HRR {
        let bits = ((self.bits >> 16) & 31) as u8;
        HIB_CAL0_HRR { bits }
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn hib_cal0_ampm(&self) -> HIB_CAL0_AMPMR {
        let bits = ((self.bits >> 22) & 1) != 0;
        HIB_CAL0_AMPMR { bits }
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn hib_cal0_valid(&self) -> HIB_CAL0_VALIDR {
        let bits = ((self.bits >> 31) & 1) != 0;
        HIB_CAL0_VALIDR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:5 - Seconds"]
    #[inline(always)]
    pub fn hib_cal0_sec(&mut self) -> _HIB_CAL0_SECW {
        _HIB_CAL0_SECW { w: self }
    }
    #[doc = "Bits 8:13 - Minutes"]
    #[inline(always)]
    pub fn hib_cal0_min(&mut self) -> _HIB_CAL0_MINW {
        _HIB_CAL0_MINW { w: self }
    }
    #[doc = "Bits 16:20 - Hours"]
    #[inline(always)]
    pub fn hib_cal0_hr(&mut self) -> _HIB_CAL0_HRW {
        _HIB_CAL0_HRW { w: self }
    }
    #[doc = "Bit 22 - AM/PM Designation"]
    #[inline(always)]
    pub fn hib_cal0_ampm(&mut self) -> _HIB_CAL0_AMPMW {
        _HIB_CAL0_AMPMW { w: self }
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn hib_cal0_valid(&mut self) -> _HIB_CAL0_VALIDW {
        _HIB_CAL0_VALIDW { w: self }
    }
}
