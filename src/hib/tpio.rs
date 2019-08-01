#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TPIO {
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
pub struct HIB_TPIO_EN0R {
    bits: bool,
}
impl HIB_TPIO_EN0R {
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
pub struct _HIB_TPIO_EN0W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_EN0W<'a> {
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
pub struct HIB_TPIO_LEV0R {
    bits: bool,
}
impl HIB_TPIO_LEV0R {
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
pub struct _HIB_TPIO_LEV0W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_LEV0W<'a> {
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
pub struct HIB_TPIO_PUEN0R {
    bits: bool,
}
impl HIB_TPIO_PUEN0R {
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
pub struct _HIB_TPIO_PUEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_PUEN0W<'a> {
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
pub struct HIB_TPIO_GFLTR0R {
    bits: bool,
}
impl HIB_TPIO_GFLTR0R {
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
pub struct _HIB_TPIO_GFLTR0W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_GFLTR0W<'a> {
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
pub struct HIB_TPIO_EN1R {
    bits: bool,
}
impl HIB_TPIO_EN1R {
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
pub struct _HIB_TPIO_EN1W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_EN1W<'a> {
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
pub struct HIB_TPIO_LEV1R {
    bits: bool,
}
impl HIB_TPIO_LEV1R {
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
pub struct _HIB_TPIO_LEV1W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_LEV1W<'a> {
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
#[doc = r"Value of the field"]
pub struct HIB_TPIO_PUEN1R {
    bits: bool,
}
impl HIB_TPIO_PUEN1R {
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
pub struct _HIB_TPIO_PUEN1W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_PUEN1W<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPIO_GFLTR1R {
    bits: bool,
}
impl HIB_TPIO_GFLTR1R {
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
pub struct _HIB_TPIO_GFLTR1W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_GFLTR1W<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPIO_EN2R {
    bits: bool,
}
impl HIB_TPIO_EN2R {
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
pub struct _HIB_TPIO_EN2W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_EN2W<'a> {
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
pub struct HIB_TPIO_LEV2R {
    bits: bool,
}
impl HIB_TPIO_LEV2R {
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
pub struct _HIB_TPIO_LEV2W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_LEV2W<'a> {
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
pub struct HIB_TPIO_PUEN2R {
    bits: bool,
}
impl HIB_TPIO_PUEN2R {
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
pub struct _HIB_TPIO_PUEN2W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_PUEN2W<'a> {
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
#[doc = r"Value of the field"]
pub struct HIB_TPIO_GFLTR2R {
    bits: bool,
}
impl HIB_TPIO_GFLTR2R {
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
pub struct _HIB_TPIO_GFLTR2W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_GFLTR2W<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPIO_EN3R {
    bits: bool,
}
impl HIB_TPIO_EN3R {
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
pub struct _HIB_TPIO_EN3W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_EN3W<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPIO_LEV3R {
    bits: bool,
}
impl HIB_TPIO_LEV3R {
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
pub struct _HIB_TPIO_LEV3W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_LEV3W<'a> {
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
        self.w.bits &= !(1 << 25);
        self.w.bits |= ((value as u32) & 1) << 25;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPIO_PUEN3R {
    bits: bool,
}
impl HIB_TPIO_PUEN3R {
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
pub struct _HIB_TPIO_PUEN3W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_PUEN3W<'a> {
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
        self.w.bits &= !(1 << 26);
        self.w.bits |= ((value as u32) & 1) << 26;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct HIB_TPIO_GFLTR3R {
    bits: bool,
}
impl HIB_TPIO_GFLTR3R {
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
pub struct _HIB_TPIO_GFLTR3W<'a> {
    w: &'a mut W,
}
impl<'a> _HIB_TPIO_GFLTR3W<'a> {
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
        self.w.bits &= !(1 << 27);
        self.w.bits |= ((value as u32) & 1) << 27;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - TMPR0 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en0(&self) -> HIB_TPIO_EN0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        HIB_TPIO_EN0R { bits }
    }
    #[doc = "Bit 1 - TMPR0 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev0(&self) -> HIB_TPIO_LEV0R {
        let bits = ((self.bits >> 1) & 1) != 0;
        HIB_TPIO_LEV0R { bits }
    }
    #[doc = "Bit 2 - TMPR0 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen0(&self) -> HIB_TPIO_PUEN0R {
        let bits = ((self.bits >> 2) & 1) != 0;
        HIB_TPIO_PUEN0R { bits }
    }
    #[doc = "Bit 3 - TMPR0 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr0(&self) -> HIB_TPIO_GFLTR0R {
        let bits = ((self.bits >> 3) & 1) != 0;
        HIB_TPIO_GFLTR0R { bits }
    }
    #[doc = "Bit 8 - TMPR1Enable"]
    #[inline(always)]
    pub fn hib_tpio_en1(&self) -> HIB_TPIO_EN1R {
        let bits = ((self.bits >> 8) & 1) != 0;
        HIB_TPIO_EN1R { bits }
    }
    #[doc = "Bit 9 - TMPR1 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev1(&self) -> HIB_TPIO_LEV1R {
        let bits = ((self.bits >> 9) & 1) != 0;
        HIB_TPIO_LEV1R { bits }
    }
    #[doc = "Bit 10 - TMPR1 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen1(&self) -> HIB_TPIO_PUEN1R {
        let bits = ((self.bits >> 10) & 1) != 0;
        HIB_TPIO_PUEN1R { bits }
    }
    #[doc = "Bit 11 - TMPR1 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr1(&self) -> HIB_TPIO_GFLTR1R {
        let bits = ((self.bits >> 11) & 1) != 0;
        HIB_TPIO_GFLTR1R { bits }
    }
    #[doc = "Bit 16 - TMPR2 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en2(&self) -> HIB_TPIO_EN2R {
        let bits = ((self.bits >> 16) & 1) != 0;
        HIB_TPIO_EN2R { bits }
    }
    #[doc = "Bit 17 - TMPR2 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev2(&self) -> HIB_TPIO_LEV2R {
        let bits = ((self.bits >> 17) & 1) != 0;
        HIB_TPIO_LEV2R { bits }
    }
    #[doc = "Bit 18 - TMPR2 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen2(&self) -> HIB_TPIO_PUEN2R {
        let bits = ((self.bits >> 18) & 1) != 0;
        HIB_TPIO_PUEN2R { bits }
    }
    #[doc = "Bit 19 - TMPR2 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr2(&self) -> HIB_TPIO_GFLTR2R {
        let bits = ((self.bits >> 19) & 1) != 0;
        HIB_TPIO_GFLTR2R { bits }
    }
    #[doc = "Bit 24 - TMPR3 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en3(&self) -> HIB_TPIO_EN3R {
        let bits = ((self.bits >> 24) & 1) != 0;
        HIB_TPIO_EN3R { bits }
    }
    #[doc = "Bit 25 - TMPR3 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev3(&self) -> HIB_TPIO_LEV3R {
        let bits = ((self.bits >> 25) & 1) != 0;
        HIB_TPIO_LEV3R { bits }
    }
    #[doc = "Bit 26 - TMPR3 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen3(&self) -> HIB_TPIO_PUEN3R {
        let bits = ((self.bits >> 26) & 1) != 0;
        HIB_TPIO_PUEN3R { bits }
    }
    #[doc = "Bit 27 - TMPR3 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr3(&self) -> HIB_TPIO_GFLTR3R {
        let bits = ((self.bits >> 27) & 1) != 0;
        HIB_TPIO_GFLTR3R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - TMPR0 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en0(&mut self) -> _HIB_TPIO_EN0W {
        _HIB_TPIO_EN0W { w: self }
    }
    #[doc = "Bit 1 - TMPR0 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev0(&mut self) -> _HIB_TPIO_LEV0W {
        _HIB_TPIO_LEV0W { w: self }
    }
    #[doc = "Bit 2 - TMPR0 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen0(&mut self) -> _HIB_TPIO_PUEN0W {
        _HIB_TPIO_PUEN0W { w: self }
    }
    #[doc = "Bit 3 - TMPR0 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr0(&mut self) -> _HIB_TPIO_GFLTR0W {
        _HIB_TPIO_GFLTR0W { w: self }
    }
    #[doc = "Bit 8 - TMPR1Enable"]
    #[inline(always)]
    pub fn hib_tpio_en1(&mut self) -> _HIB_TPIO_EN1W {
        _HIB_TPIO_EN1W { w: self }
    }
    #[doc = "Bit 9 - TMPR1 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev1(&mut self) -> _HIB_TPIO_LEV1W {
        _HIB_TPIO_LEV1W { w: self }
    }
    #[doc = "Bit 10 - TMPR1 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen1(&mut self) -> _HIB_TPIO_PUEN1W {
        _HIB_TPIO_PUEN1W { w: self }
    }
    #[doc = "Bit 11 - TMPR1 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr1(&mut self) -> _HIB_TPIO_GFLTR1W {
        _HIB_TPIO_GFLTR1W { w: self }
    }
    #[doc = "Bit 16 - TMPR2 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en2(&mut self) -> _HIB_TPIO_EN2W {
        _HIB_TPIO_EN2W { w: self }
    }
    #[doc = "Bit 17 - TMPR2 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev2(&mut self) -> _HIB_TPIO_LEV2W {
        _HIB_TPIO_LEV2W { w: self }
    }
    #[doc = "Bit 18 - TMPR2 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen2(&mut self) -> _HIB_TPIO_PUEN2W {
        _HIB_TPIO_PUEN2W { w: self }
    }
    #[doc = "Bit 19 - TMPR2 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr2(&mut self) -> _HIB_TPIO_GFLTR2W {
        _HIB_TPIO_GFLTR2W { w: self }
    }
    #[doc = "Bit 24 - TMPR3 Enable"]
    #[inline(always)]
    pub fn hib_tpio_en3(&mut self) -> _HIB_TPIO_EN3W {
        _HIB_TPIO_EN3W { w: self }
    }
    #[doc = "Bit 25 - TMPR3 Trigger Level"]
    #[inline(always)]
    pub fn hib_tpio_lev3(&mut self) -> _HIB_TPIO_LEV3W {
        _HIB_TPIO_LEV3W { w: self }
    }
    #[doc = "Bit 26 - TMPR3 Internal Weak Pull-up Enable"]
    #[inline(always)]
    pub fn hib_tpio_puen3(&mut self) -> _HIB_TPIO_PUEN3W {
        _HIB_TPIO_PUEN3W { w: self }
    }
    #[doc = "Bit 27 - TMPR3 Glitch Filtering"]
    #[inline(always)]
    pub fn hib_tpio_gfltr3(&mut self) -> _HIB_TPIO_GFLTR3W {
        _HIB_TPIO_GFLTR3W { w: self }
    }
}
