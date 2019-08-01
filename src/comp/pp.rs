#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PP {
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
pub struct COMP_PP_CMP0R {
    bits: bool,
}
impl COMP_PP_CMP0R {
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
pub struct _COMP_PP_CMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_PP_CMP0W<'a> {
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
pub struct COMP_PP_CMP1R {
    bits: bool,
}
impl COMP_PP_CMP1R {
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
pub struct _COMP_PP_CMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_PP_CMP1W<'a> {
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
pub struct COMP_PP_CMP2R {
    bits: bool,
}
impl COMP_PP_CMP2R {
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
pub struct _COMP_PP_CMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_PP_CMP2W<'a> {
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
pub struct COMP_PP_C0OR {
    bits: bool,
}
impl COMP_PP_C0OR {
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
pub struct _COMP_PP_C0OW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_PP_C0OW<'a> {
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
#[doc = r"Value of the field"]
pub struct COMP_PP_C1OR {
    bits: bool,
}
impl COMP_PP_C1OR {
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
pub struct _COMP_PP_C1OW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_PP_C1OW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct COMP_PP_C2OR {
    bits: bool,
}
impl COMP_PP_C2OR {
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
pub struct _COMP_PP_C2OW<'a> {
    w: &'a mut W,
}
impl<'a> _COMP_PP_C2OW<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Comparator 0 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp0(&self) -> COMP_PP_CMP0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        COMP_PP_CMP0R { bits }
    }
    #[doc = "Bit 1 - Comparator 1 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp1(&self) -> COMP_PP_CMP1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        COMP_PP_CMP1R { bits }
    }
    #[doc = "Bit 2 - Comparator 2 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp2(&self) -> COMP_PP_CMP2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        COMP_PP_CMP2R { bits }
    }
    #[doc = "Bit 16 - Comparator Output 0 Present"]
    #[inline(always)]
    pub fn comp_pp_c0o(&self) -> COMP_PP_C0OR {
        let bits = ((self.bits >> 16) & 1) != 0;
        COMP_PP_C0OR { bits }
    }
    #[doc = "Bit 17 - Comparator Output 1 Present"]
    #[inline(always)]
    pub fn comp_pp_c1o(&self) -> COMP_PP_C1OR {
        let bits = ((self.bits >> 17) & 1) != 0;
        COMP_PP_C1OR { bits }
    }
    #[doc = "Bit 18 - Comparator Output 2 Present"]
    #[inline(always)]
    pub fn comp_pp_c2o(&self) -> COMP_PP_C2OR {
        let bits = ((self.bits >> 18) & 1) != 0;
        COMP_PP_C2OR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Comparator 0 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp0(&mut self) -> _COMP_PP_CMP0W {
        _COMP_PP_CMP0W { w: self }
    }
    #[doc = "Bit 1 - Comparator 1 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp1(&mut self) -> _COMP_PP_CMP1W {
        _COMP_PP_CMP1W { w: self }
    }
    #[doc = "Bit 2 - Comparator 2 Present"]
    #[inline(always)]
    pub fn comp_pp_cmp2(&mut self) -> _COMP_PP_CMP2W {
        _COMP_PP_CMP2W { w: self }
    }
    #[doc = "Bit 16 - Comparator Output 0 Present"]
    #[inline(always)]
    pub fn comp_pp_c0o(&mut self) -> _COMP_PP_C0OW {
        _COMP_PP_C0OW { w: self }
    }
    #[doc = "Bit 17 - Comparator Output 1 Present"]
    #[inline(always)]
    pub fn comp_pp_c1o(&mut self) -> _COMP_PP_C1OW {
        _COMP_PP_C1OW { w: self }
    }
    #[doc = "Bit 18 - Comparator Output 2 Present"]
    #[inline(always)]
    pub fn comp_pp_c2o(&mut self) -> _COMP_PP_C2OW {
        _COMP_PP_C2OW { w: self }
    }
}
