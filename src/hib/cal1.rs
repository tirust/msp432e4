#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CAL1 {
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
pub struct HIB_CAL1_DOMR {
    bits: u8,
}
impl HIB_CAL1_DOMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CAL1_DOMW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL1_DOMW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 0);
        self.w.bits |= ((value as u32) & 31) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL1_MONR {
    bits: u8,
}
impl HIB_CAL1_MONR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CAL1_MONW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL1_MONW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL1_YEARR {
    bits: u8,
}
impl HIB_CAL1_YEARR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CAL1_YEARW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL1_YEARW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(127 << 16);
        self.w.bits |= ((value as u32) & 127) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL1_DOWR {
    bits: u8,
}
impl HIB_CAL1_DOWR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _HIB_CAL1_DOWW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL1_DOWW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 24);
        self.w.bits |= ((value as u32) & 7) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_CAL1_VALIDR {
    bits: bool,
}
impl HIB_CAL1_VALIDR {
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
pub struct _HIB_CAL1_VALIDW<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_CAL1_VALIDW<'a> {
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
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_cal1_dom(&self) -> HIB_CAL1_DOMR {
        let bits = ((self.bits >> 0) & 31) as u8;
        HIB_CAL1_DOMR { bits }
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn hib_cal1_mon(&self) -> HIB_CAL1_MONR {
        let bits = ((self.bits >> 8) & 15) as u8;
        HIB_CAL1_MONR { bits }
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn hib_cal1_year(&self) -> HIB_CAL1_YEARR {
        let bits = ((self.bits >> 16) & 127) as u8;
        HIB_CAL1_YEARR { bits }
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn hib_cal1_dow(&self) -> HIB_CAL1_DOWR {
        let bits = ((self.bits >> 24) & 7) as u8;
        HIB_CAL1_DOWR { bits }
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn hib_cal1_valid(&self) -> HIB_CAL1_VALIDR {
        let bits = ((self.bits >> 31) & 1) != 0;
        HIB_CAL1_VALIDR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:4 - Day of Month"]
    #[inline(always)]
    pub fn hib_cal1_dom(&mut self) -> _HIB_CAL1_DOMW {
        _HIB_CAL1_DOMW { w: self }
    }
    #[doc = "Bits 8:11 - Month"]
    #[inline(always)]
    pub fn hib_cal1_mon(&mut self) -> _HIB_CAL1_MONW {
        _HIB_CAL1_MONW { w: self }
    }
    #[doc = "Bits 16:22 - Year Value"]
    #[inline(always)]
    pub fn hib_cal1_year(&mut self) -> _HIB_CAL1_YEARW {
        _HIB_CAL1_YEARW { w: self }
    }
    #[doc = "Bits 24:26 - Day of Week"]
    #[inline(always)]
    pub fn hib_cal1_dow(&mut self) -> _HIB_CAL1_DOWW {
        _HIB_CAL1_DOWW { w: self }
    }
    #[doc = "Bit 31 - Valid Calendar Load"]
    #[inline(always)]
    pub fn hib_cal1_valid(&mut self) -> _HIB_CAL1_VALIDW {
        _HIB_CAL1_VALIDW { w: self }
    }
}
