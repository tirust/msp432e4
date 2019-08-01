#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF2ARB2 {
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
pub struct CAN_IF2ARB2_IDR {
    bits: u16,
}
impl CAN_IF2ARB2_IDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _CAN_IF2ARB2_IDW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2ARB2_IDW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(8191 << 0);
        self.w.bits |= ((value as u32) & 8191) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_IF2ARB2_DIRR {
    bits: bool,
}
impl CAN_IF2ARB2_DIRR {
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
pub struct _CAN_IF2ARB2_DIRW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2ARB2_DIRW<'a> {
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
        self.w.bits &= !(1 << 13);
        self.w.bits |= ((value as u32) & 1) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_IF2ARB2_XTDR {
    bits: bool,
}
impl CAN_IF2ARB2_XTDR {
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
pub struct _CAN_IF2ARB2_XTDW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2ARB2_XTDW<'a> {
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
        self.w.bits &= !(1 << 14);
        self.w.bits |= ((value as u32) & 1) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_IF2ARB2_MSGVALR {
    bits: bool,
}
impl CAN_IF2ARB2_MSGVALR {
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
pub struct _CAN_IF2ARB2_MSGVALW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2ARB2_MSGVALW<'a> {
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
        self.w.bits &= !(1 << 15);
        self.w.bits |= ((value as u32) & 1) << 15;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:12 - Message Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_id(&self) -> CAN_IF2ARB2_IDR {
        let bits = ((self.bits >> 0) & 8191) as u16;
        CAN_IF2ARB2_IDR { bits }
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    pub fn can_if2arb2_dir(&self) -> CAN_IF2ARB2_DIRR {
        let bits = ((self.bits >> 13) & 1) != 0;
        CAN_IF2ARB2_DIRR { bits }
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_xtd(&self) -> CAN_IF2ARB2_XTDR {
        let bits = ((self.bits >> 14) & 1) != 0;
        CAN_IF2ARB2_XTDR { bits }
    }
    #[doc = "Bit 15 - Message Valid"]
    #[inline(always)]
    pub fn can_if2arb2_msgval(&self) -> CAN_IF2ARB2_MSGVALR {
        let bits = ((self.bits >> 15) & 1) != 0;
        CAN_IF2ARB2_MSGVALR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:12 - Message Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_id(&mut self) -> _CAN_IF2ARB2_IDW {
        _CAN_IF2ARB2_IDW { w: self }
    }
    #[doc = "Bit 13 - Message Direction"]
    #[inline(always)]
    pub fn can_if2arb2_dir(&mut self) -> _CAN_IF2ARB2_DIRW {
        _CAN_IF2ARB2_DIRW { w: self }
    }
    #[doc = "Bit 14 - Extended Identifier"]
    #[inline(always)]
    pub fn can_if2arb2_xtd(&mut self) -> _CAN_IF2ARB2_XTDW {
        _CAN_IF2ARB2_XTDW { w: self }
    }
    #[doc = "Bit 15 - Message Valid"]
    #[inline(always)]
    pub fn can_if2arb2_msgval(&mut self) -> _CAN_IF2ARB2_MSGVALW {
        _CAN_IF2ARB2_MSGVALW { w: self }
    }
}
