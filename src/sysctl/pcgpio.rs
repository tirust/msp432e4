#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PCGPIO {
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
pub struct SYSCTL_PCGPIO_P0R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P0R {
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
pub struct _SYSCTL_PCGPIO_P0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P0W<'a> {
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
pub struct SYSCTL_PCGPIO_P1R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P1R {
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
pub struct _SYSCTL_PCGPIO_P1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P1W<'a> {
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
pub struct SYSCTL_PCGPIO_P2R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P2R {
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
pub struct _SYSCTL_PCGPIO_P2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P2W<'a> {
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
pub struct SYSCTL_PCGPIO_P3R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P3R {
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
pub struct _SYSCTL_PCGPIO_P3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P3W<'a> {
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
pub struct SYSCTL_PCGPIO_P4R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P4R {
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
pub struct _SYSCTL_PCGPIO_P4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P4W<'a> {
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
pub struct SYSCTL_PCGPIO_P5R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P5R {
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
pub struct _SYSCTL_PCGPIO_P5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P5W<'a> {
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
pub struct SYSCTL_PCGPIO_P6R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P6R {
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
pub struct _SYSCTL_PCGPIO_P6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P6W<'a> {
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
pub struct SYSCTL_PCGPIO_P7R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P7R {
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
pub struct _SYSCTL_PCGPIO_P7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P7W<'a> {
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
pub struct SYSCTL_PCGPIO_P8R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P8R {
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
pub struct _SYSCTL_PCGPIO_P8W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P8W<'a> {
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
pub struct SYSCTL_PCGPIO_P9R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P9R {
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
pub struct _SYSCTL_PCGPIO_P9W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P9W<'a> {
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
pub struct SYSCTL_PCGPIO_P10R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P10R {
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
pub struct _SYSCTL_PCGPIO_P10W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P10W<'a> {
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
pub struct SYSCTL_PCGPIO_P11R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P11R {
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
pub struct _SYSCTL_PCGPIO_P11W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P11W<'a> {
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
pub struct SYSCTL_PCGPIO_P12R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P12R {
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
pub struct _SYSCTL_PCGPIO_P12W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P12W<'a> {
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
pub struct SYSCTL_PCGPIO_P13R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P13R {
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
pub struct _SYSCTL_PCGPIO_P13W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P13W<'a> {
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
pub struct SYSCTL_PCGPIO_P14R {
    bits: bool,
}
impl SYSCTL_PCGPIO_P14R {
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
pub struct _SYSCTL_PCGPIO_P14W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PCGPIO_P14W<'a> {
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
    #[doc = "Bit 0 - GPIO Port A Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p0(&self) -> SYSCTL_PCGPIO_P0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_PCGPIO_P0R { bits }
    }
    #[doc = "Bit 1 - GPIO Port B Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p1(&self) -> SYSCTL_PCGPIO_P1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_PCGPIO_P1R { bits }
    }
    #[doc = "Bit 2 - GPIO Port C Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p2(&self) -> SYSCTL_PCGPIO_P2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_PCGPIO_P2R { bits }
    }
    #[doc = "Bit 3 - GPIO Port D Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p3(&self) -> SYSCTL_PCGPIO_P3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_PCGPIO_P3R { bits }
    }
    #[doc = "Bit 4 - GPIO Port E Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p4(&self) -> SYSCTL_PCGPIO_P4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_PCGPIO_P4R { bits }
    }
    #[doc = "Bit 5 - GPIO Port F Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p5(&self) -> SYSCTL_PCGPIO_P5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_PCGPIO_P5R { bits }
    }
    #[doc = "Bit 6 - GPIO Port G Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p6(&self) -> SYSCTL_PCGPIO_P6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_PCGPIO_P6R { bits }
    }
    #[doc = "Bit 7 - GPIO Port H Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p7(&self) -> SYSCTL_PCGPIO_P7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_PCGPIO_P7R { bits }
    }
    #[doc = "Bit 8 - GPIO Port J Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p8(&self) -> SYSCTL_PCGPIO_P8R {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_PCGPIO_P8R { bits }
    }
    #[doc = "Bit 9 - GPIO Port K Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p9(&self) -> SYSCTL_PCGPIO_P9R {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_PCGPIO_P9R { bits }
    }
    #[doc = "Bit 10 - GPIO Port L Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p10(&self) -> SYSCTL_PCGPIO_P10R {
        let bits = ((self.bits >> 10) & 1) != 0;
        SYSCTL_PCGPIO_P10R { bits }
    }
    #[doc = "Bit 11 - GPIO Port M Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p11(&self) -> SYSCTL_PCGPIO_P11R {
        let bits = ((self.bits >> 11) & 1) != 0;
        SYSCTL_PCGPIO_P11R { bits }
    }
    #[doc = "Bit 12 - GPIO Port N Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p12(&self) -> SYSCTL_PCGPIO_P12R {
        let bits = ((self.bits >> 12) & 1) != 0;
        SYSCTL_PCGPIO_P12R { bits }
    }
    #[doc = "Bit 13 - GPIO Port P Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p13(&self) -> SYSCTL_PCGPIO_P13R {
        let bits = ((self.bits >> 13) & 1) != 0;
        SYSCTL_PCGPIO_P13R { bits }
    }
    #[doc = "Bit 14 - GPIO Port Q Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p14(&self) -> SYSCTL_PCGPIO_P14R {
        let bits = ((self.bits >> 14) & 1) != 0;
        SYSCTL_PCGPIO_P14R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPIO Port A Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p0(&mut self) -> _SYSCTL_PCGPIO_P0W {
        _SYSCTL_PCGPIO_P0W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p1(&mut self) -> _SYSCTL_PCGPIO_P1W {
        _SYSCTL_PCGPIO_P1W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p2(&mut self) -> _SYSCTL_PCGPIO_P2W {
        _SYSCTL_PCGPIO_P2W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p3(&mut self) -> _SYSCTL_PCGPIO_P3W {
        _SYSCTL_PCGPIO_P3W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p4(&mut self) -> _SYSCTL_PCGPIO_P4W {
        _SYSCTL_PCGPIO_P4W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p5(&mut self) -> _SYSCTL_PCGPIO_P5W {
        _SYSCTL_PCGPIO_P5W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p6(&mut self) -> _SYSCTL_PCGPIO_P6W {
        _SYSCTL_PCGPIO_P6W { w: self }
    }
    #[doc = "Bit 7 - GPIO Port H Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p7(&mut self) -> _SYSCTL_PCGPIO_P7W {
        _SYSCTL_PCGPIO_P7W { w: self }
    }
    #[doc = "Bit 8 - GPIO Port J Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p8(&mut self) -> _SYSCTL_PCGPIO_P8W {
        _SYSCTL_PCGPIO_P8W { w: self }
    }
    #[doc = "Bit 9 - GPIO Port K Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p9(&mut self) -> _SYSCTL_PCGPIO_P9W {
        _SYSCTL_PCGPIO_P9W { w: self }
    }
    #[doc = "Bit 10 - GPIO Port L Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p10(&mut self) -> _SYSCTL_PCGPIO_P10W {
        _SYSCTL_PCGPIO_P10W { w: self }
    }
    #[doc = "Bit 11 - GPIO Port M Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p11(&mut self) -> _SYSCTL_PCGPIO_P11W {
        _SYSCTL_PCGPIO_P11W { w: self }
    }
    #[doc = "Bit 12 - GPIO Port N Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p12(&mut self) -> _SYSCTL_PCGPIO_P12W {
        _SYSCTL_PCGPIO_P12W { w: self }
    }
    #[doc = "Bit 13 - GPIO Port P Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p13(&mut self) -> _SYSCTL_PCGPIO_P13W {
        _SYSCTL_PCGPIO_P13W { w: self }
    }
    #[doc = "Bit 14 - GPIO Port Q Power Control"]
    #[inline(always)]
    pub fn sysctl_pcgpio_p14(&mut self) -> _SYSCTL_PCGPIO_P14W {
        _SYSCTL_PCGPIO_P14W { w: self }
    }
}
