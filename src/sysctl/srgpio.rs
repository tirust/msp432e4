#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SRGPIO {
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
pub struct SYSCTL_SRGPIO_R0R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R0R {
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
pub struct _SYSCTL_SRGPIO_R0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R0W<'a> {
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
pub struct SYSCTL_SRGPIO_R1R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R1R {
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
pub struct _SYSCTL_SRGPIO_R1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R1W<'a> {
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
pub struct SYSCTL_SRGPIO_R2R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R2R {
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
pub struct _SYSCTL_SRGPIO_R2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R2W<'a> {
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
pub struct SYSCTL_SRGPIO_R3R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R3R {
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
pub struct _SYSCTL_SRGPIO_R3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R3W<'a> {
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
pub struct SYSCTL_SRGPIO_R4R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R4R {
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
pub struct _SYSCTL_SRGPIO_R4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R4W<'a> {
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
pub struct SYSCTL_SRGPIO_R5R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R5R {
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
pub struct _SYSCTL_SRGPIO_R5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R5W<'a> {
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
pub struct SYSCTL_SRGPIO_R6R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R6R {
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
pub struct _SYSCTL_SRGPIO_R6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R6W<'a> {
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
pub struct SYSCTL_SRGPIO_R7R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R7R {
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
pub struct _SYSCTL_SRGPIO_R7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R7W<'a> {
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
#[doc = r"Value of the field"]
pub struct SYSCTL_SRGPIO_R8R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R8R {
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
pub struct _SYSCTL_SRGPIO_R8W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R8W<'a> {
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
pub struct SYSCTL_SRGPIO_R9R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R9R {
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
pub struct _SYSCTL_SRGPIO_R9W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R9W<'a> {
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
pub struct SYSCTL_SRGPIO_R10R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R10R {
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
pub struct _SYSCTL_SRGPIO_R10W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R10W<'a> {
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
pub struct SYSCTL_SRGPIO_R11R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R11R {
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
pub struct _SYSCTL_SRGPIO_R11W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R11W<'a> {
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
pub struct SYSCTL_SRGPIO_R12R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R12R {
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
pub struct _SYSCTL_SRGPIO_R12W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R12W<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_SRGPIO_R13R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R13R {
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
pub struct _SYSCTL_SRGPIO_R13W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R13W<'a> {
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
pub struct SYSCTL_SRGPIO_R14R {
    bits: bool,
}
impl SYSCTL_SRGPIO_R14R {
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
pub struct _SYSCTL_SRGPIO_R14W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SRGPIO_R14W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - GPIO Port A Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r0(&self) -> SYSCTL_SRGPIO_R0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_SRGPIO_R0R { bits }
    }
    #[doc = "Bit 1 - GPIO Port B Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r1(&self) -> SYSCTL_SRGPIO_R1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_SRGPIO_R1R { bits }
    }
    #[doc = "Bit 2 - GPIO Port C Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r2(&self) -> SYSCTL_SRGPIO_R2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_SRGPIO_R2R { bits }
    }
    #[doc = "Bit 3 - GPIO Port D Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r3(&self) -> SYSCTL_SRGPIO_R3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_SRGPIO_R3R { bits }
    }
    #[doc = "Bit 4 - GPIO Port E Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r4(&self) -> SYSCTL_SRGPIO_R4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_SRGPIO_R4R { bits }
    }
    #[doc = "Bit 5 - GPIO Port F Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r5(&self) -> SYSCTL_SRGPIO_R5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_SRGPIO_R5R { bits }
    }
    #[doc = "Bit 6 - GPIO Port G Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r6(&self) -> SYSCTL_SRGPIO_R6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_SRGPIO_R6R { bits }
    }
    #[doc = "Bit 7 - GPIO Port H Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r7(&self) -> SYSCTL_SRGPIO_R7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_SRGPIO_R7R { bits }
    }
    #[doc = "Bit 8 - GPIO Port J Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r8(&self) -> SYSCTL_SRGPIO_R8R {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_SRGPIO_R8R { bits }
    }
    #[doc = "Bit 9 - GPIO Port K Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r9(&self) -> SYSCTL_SRGPIO_R9R {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_SRGPIO_R9R { bits }
    }
    #[doc = "Bit 10 - GPIO Port L Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r10(&self) -> SYSCTL_SRGPIO_R10R {
        let bits = ((self.bits >> 10) & 1) != 0;
        SYSCTL_SRGPIO_R10R { bits }
    }
    #[doc = "Bit 11 - GPIO Port M Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r11(&self) -> SYSCTL_SRGPIO_R11R {
        let bits = ((self.bits >> 11) & 1) != 0;
        SYSCTL_SRGPIO_R11R { bits }
    }
    #[doc = "Bit 12 - GPIO Port N Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r12(&self) -> SYSCTL_SRGPIO_R12R {
        let bits = ((self.bits >> 12) & 1) != 0;
        SYSCTL_SRGPIO_R12R { bits }
    }
    #[doc = "Bit 13 - GPIO Port P Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r13(&self) -> SYSCTL_SRGPIO_R13R {
        let bits = ((self.bits >> 13) & 1) != 0;
        SYSCTL_SRGPIO_R13R { bits }
    }
    #[doc = "Bit 14 - GPIO Port Q Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r14(&self) -> SYSCTL_SRGPIO_R14R {
        let bits = ((self.bits >> 14) & 1) != 0;
        SYSCTL_SRGPIO_R14R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPIO Port A Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r0(&mut self) -> _SYSCTL_SRGPIO_R0W {
        _SYSCTL_SRGPIO_R0W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r1(&mut self) -> _SYSCTL_SRGPIO_R1W {
        _SYSCTL_SRGPIO_R1W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r2(&mut self) -> _SYSCTL_SRGPIO_R2W {
        _SYSCTL_SRGPIO_R2W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r3(&mut self) -> _SYSCTL_SRGPIO_R3W {
        _SYSCTL_SRGPIO_R3W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r4(&mut self) -> _SYSCTL_SRGPIO_R4W {
        _SYSCTL_SRGPIO_R4W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r5(&mut self) -> _SYSCTL_SRGPIO_R5W {
        _SYSCTL_SRGPIO_R5W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r6(&mut self) -> _SYSCTL_SRGPIO_R6W {
        _SYSCTL_SRGPIO_R6W { w: self }
    }
    #[doc = "Bit 7 - GPIO Port H Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r7(&mut self) -> _SYSCTL_SRGPIO_R7W {
        _SYSCTL_SRGPIO_R7W { w: self }
    }
    #[doc = "Bit 8 - GPIO Port J Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r8(&mut self) -> _SYSCTL_SRGPIO_R8W {
        _SYSCTL_SRGPIO_R8W { w: self }
    }
    #[doc = "Bit 9 - GPIO Port K Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r9(&mut self) -> _SYSCTL_SRGPIO_R9W {
        _SYSCTL_SRGPIO_R9W { w: self }
    }
    #[doc = "Bit 10 - GPIO Port L Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r10(&mut self) -> _SYSCTL_SRGPIO_R10W {
        _SYSCTL_SRGPIO_R10W { w: self }
    }
    #[doc = "Bit 11 - GPIO Port M Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r11(&mut self) -> _SYSCTL_SRGPIO_R11W {
        _SYSCTL_SRGPIO_R11W { w: self }
    }
    #[doc = "Bit 12 - GPIO Port N Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r12(&mut self) -> _SYSCTL_SRGPIO_R12W {
        _SYSCTL_SRGPIO_R12W { w: self }
    }
    #[doc = "Bit 13 - GPIO Port P Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r13(&mut self) -> _SYSCTL_SRGPIO_R13W {
        _SYSCTL_SRGPIO_R13W { w: self }
    }
    #[doc = "Bit 14 - GPIO Port Q Software Reset"]
    #[inline(always)]
    pub fn sysctl_srgpio_r14(&mut self) -> _SYSCTL_SRGPIO_R14W {
        _SYSCTL_SRGPIO_R14W { w: self }
    }
}
