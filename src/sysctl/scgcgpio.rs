#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCGCGPIO {
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
pub struct SYSCTL_SCGCGPIO_S0R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S0R {
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
pub struct _SYSCTL_SCGCGPIO_S0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S0W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S1R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S1R {
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
pub struct _SYSCTL_SCGCGPIO_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S1W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S2R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S2R {
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
pub struct _SYSCTL_SCGCGPIO_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S2W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S3R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S3R {
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
pub struct _SYSCTL_SCGCGPIO_S3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S3W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S4R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S4R {
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
pub struct _SYSCTL_SCGCGPIO_S4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S4W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S5R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S5R {
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
pub struct _SYSCTL_SCGCGPIO_S5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S5W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S6R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S6R {
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
pub struct _SYSCTL_SCGCGPIO_S6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S6W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S7R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S7R {
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
pub struct _SYSCTL_SCGCGPIO_S7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S7W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S8R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S8R {
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
pub struct _SYSCTL_SCGCGPIO_S8W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S8W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S9R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S9R {
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
pub struct _SYSCTL_SCGCGPIO_S9W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S9W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S10R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S10R {
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
pub struct _SYSCTL_SCGCGPIO_S10W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S10W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S11R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S11R {
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
pub struct _SYSCTL_SCGCGPIO_S11W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S11W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S12R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S12R {
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
pub struct _SYSCTL_SCGCGPIO_S12W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S12W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S13R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S13R {
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
pub struct _SYSCTL_SCGCGPIO_S13W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S13W<'a> {
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
pub struct SYSCTL_SCGCGPIO_S14R {
    bits: bool,
}
impl SYSCTL_SCGCGPIO_S14R {
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
pub struct _SYSCTL_SCGCGPIO_S14W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCGPIO_S14W<'a> {
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
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s0(&self) -> SYSCTL_SCGCGPIO_S0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_SCGCGPIO_S0R { bits }
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s1(&self) -> SYSCTL_SCGCGPIO_S1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_SCGCGPIO_S1R { bits }
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s2(&self) -> SYSCTL_SCGCGPIO_S2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_SCGCGPIO_S2R { bits }
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s3(&self) -> SYSCTL_SCGCGPIO_S3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_SCGCGPIO_S3R { bits }
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s4(&self) -> SYSCTL_SCGCGPIO_S4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_SCGCGPIO_S4R { bits }
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s5(&self) -> SYSCTL_SCGCGPIO_S5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_SCGCGPIO_S5R { bits }
    }
    #[doc = "Bit 6 - GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s6(&self) -> SYSCTL_SCGCGPIO_S6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_SCGCGPIO_S6R { bits }
    }
    #[doc = "Bit 7 - GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s7(&self) -> SYSCTL_SCGCGPIO_S7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_SCGCGPIO_S7R { bits }
    }
    #[doc = "Bit 8 - GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s8(&self) -> SYSCTL_SCGCGPIO_S8R {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_SCGCGPIO_S8R { bits }
    }
    #[doc = "Bit 9 - GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s9(&self) -> SYSCTL_SCGCGPIO_S9R {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_SCGCGPIO_S9R { bits }
    }
    #[doc = "Bit 10 - GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s10(&self) -> SYSCTL_SCGCGPIO_S10R {
        let bits = ((self.bits >> 10) & 1) != 0;
        SYSCTL_SCGCGPIO_S10R { bits }
    }
    #[doc = "Bit 11 - GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s11(&self) -> SYSCTL_SCGCGPIO_S11R {
        let bits = ((self.bits >> 11) & 1) != 0;
        SYSCTL_SCGCGPIO_S11R { bits }
    }
    #[doc = "Bit 12 - GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s12(&self) -> SYSCTL_SCGCGPIO_S12R {
        let bits = ((self.bits >> 12) & 1) != 0;
        SYSCTL_SCGCGPIO_S12R { bits }
    }
    #[doc = "Bit 13 - GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s13(&self) -> SYSCTL_SCGCGPIO_S13R {
        let bits = ((self.bits >> 13) & 1) != 0;
        SYSCTL_SCGCGPIO_S13R { bits }
    }
    #[doc = "Bit 14 - GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s14(&self) -> SYSCTL_SCGCGPIO_S14R {
        let bits = ((self.bits >> 14) & 1) != 0;
        SYSCTL_SCGCGPIO_S14R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPIO Port A Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s0(&mut self) -> _SYSCTL_SCGCGPIO_S0W {
        _SYSCTL_SCGCGPIO_S0W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s1(&mut self) -> _SYSCTL_SCGCGPIO_S1W {
        _SYSCTL_SCGCGPIO_S1W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s2(&mut self) -> _SYSCTL_SCGCGPIO_S2W {
        _SYSCTL_SCGCGPIO_S2W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s3(&mut self) -> _SYSCTL_SCGCGPIO_S3W {
        _SYSCTL_SCGCGPIO_S3W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s4(&mut self) -> _SYSCTL_SCGCGPIO_S4W {
        _SYSCTL_SCGCGPIO_S4W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s5(&mut self) -> _SYSCTL_SCGCGPIO_S5W {
        _SYSCTL_SCGCGPIO_S5W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s6(&mut self) -> _SYSCTL_SCGCGPIO_S6W {
        _SYSCTL_SCGCGPIO_S6W { w: self }
    }
    #[doc = "Bit 7 - GPIO Port H Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s7(&mut self) -> _SYSCTL_SCGCGPIO_S7W {
        _SYSCTL_SCGCGPIO_S7W { w: self }
    }
    #[doc = "Bit 8 - GPIO Port J Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s8(&mut self) -> _SYSCTL_SCGCGPIO_S8W {
        _SYSCTL_SCGCGPIO_S8W { w: self }
    }
    #[doc = "Bit 9 - GPIO Port K Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s9(&mut self) -> _SYSCTL_SCGCGPIO_S9W {
        _SYSCTL_SCGCGPIO_S9W { w: self }
    }
    #[doc = "Bit 10 - GPIO Port L Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s10(&mut self) -> _SYSCTL_SCGCGPIO_S10W {
        _SYSCTL_SCGCGPIO_S10W { w: self }
    }
    #[doc = "Bit 11 - GPIO Port M Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s11(&mut self) -> _SYSCTL_SCGCGPIO_S11W {
        _SYSCTL_SCGCGPIO_S11W { w: self }
    }
    #[doc = "Bit 12 - GPIO Port N Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s12(&mut self) -> _SYSCTL_SCGCGPIO_S12W {
        _SYSCTL_SCGCGPIO_S12W { w: self }
    }
    #[doc = "Bit 13 - GPIO Port P Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s13(&mut self) -> _SYSCTL_SCGCGPIO_S13W {
        _SYSCTL_SCGCGPIO_S13W { w: self }
    }
    #[doc = "Bit 14 - GPIO Port Q Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgcgpio_s14(&mut self) -> _SYSCTL_SCGCGPIO_S14W {
        _SYSCTL_SCGCGPIO_S14W { w: self }
    }
}
