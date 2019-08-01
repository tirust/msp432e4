#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCGCGPIO {
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
pub struct SYSCTL_DCGCGPIO_D0R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D0R {
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
pub struct _SYSCTL_DCGCGPIO_D0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D0W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D1R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D1R {
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
pub struct _SYSCTL_DCGCGPIO_D1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D1W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D2R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D2R {
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
pub struct _SYSCTL_DCGCGPIO_D2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D2W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D3R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D3R {
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
pub struct _SYSCTL_DCGCGPIO_D3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D3W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D4R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D4R {
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
pub struct _SYSCTL_DCGCGPIO_D4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D4W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D5R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D5R {
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
pub struct _SYSCTL_DCGCGPIO_D5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D5W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D6R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D6R {
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
pub struct _SYSCTL_DCGCGPIO_D6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D6W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D7R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D7R {
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
pub struct _SYSCTL_DCGCGPIO_D7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D7W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D8R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D8R {
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
pub struct _SYSCTL_DCGCGPIO_D8W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D8W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D9R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D9R {
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
pub struct _SYSCTL_DCGCGPIO_D9W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D9W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D10R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D10R {
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
pub struct _SYSCTL_DCGCGPIO_D10W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D10W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D11R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D11R {
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
pub struct _SYSCTL_DCGCGPIO_D11W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D11W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D12R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D12R {
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
pub struct _SYSCTL_DCGCGPIO_D12W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D12W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D13R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D13R {
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
pub struct _SYSCTL_DCGCGPIO_D13W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D13W<'a> {
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
pub struct SYSCTL_DCGCGPIO_D14R {
    bits: bool,
}
impl SYSCTL_DCGCGPIO_D14R {
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
pub struct _SYSCTL_DCGCGPIO_D14W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DCGCGPIO_D14W<'a> {
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
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d0(&self) -> SYSCTL_DCGCGPIO_D0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_DCGCGPIO_D0R { bits }
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d1(&self) -> SYSCTL_DCGCGPIO_D1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_DCGCGPIO_D1R { bits }
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d2(&self) -> SYSCTL_DCGCGPIO_D2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_DCGCGPIO_D2R { bits }
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d3(&self) -> SYSCTL_DCGCGPIO_D3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_DCGCGPIO_D3R { bits }
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d4(&self) -> SYSCTL_DCGCGPIO_D4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_DCGCGPIO_D4R { bits }
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d5(&self) -> SYSCTL_DCGCGPIO_D5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_DCGCGPIO_D5R { bits }
    }
    #[doc = "Bit 6 - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d6(&self) -> SYSCTL_DCGCGPIO_D6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_DCGCGPIO_D6R { bits }
    }
    #[doc = "Bit 7 - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d7(&self) -> SYSCTL_DCGCGPIO_D7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_DCGCGPIO_D7R { bits }
    }
    #[doc = "Bit 8 - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d8(&self) -> SYSCTL_DCGCGPIO_D8R {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_DCGCGPIO_D8R { bits }
    }
    #[doc = "Bit 9 - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d9(&self) -> SYSCTL_DCGCGPIO_D9R {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_DCGCGPIO_D9R { bits }
    }
    #[doc = "Bit 10 - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d10(&self) -> SYSCTL_DCGCGPIO_D10R {
        let bits = ((self.bits >> 10) & 1) != 0;
        SYSCTL_DCGCGPIO_D10R { bits }
    }
    #[doc = "Bit 11 - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d11(&self) -> SYSCTL_DCGCGPIO_D11R {
        let bits = ((self.bits >> 11) & 1) != 0;
        SYSCTL_DCGCGPIO_D11R { bits }
    }
    #[doc = "Bit 12 - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d12(&self) -> SYSCTL_DCGCGPIO_D12R {
        let bits = ((self.bits >> 12) & 1) != 0;
        SYSCTL_DCGCGPIO_D12R { bits }
    }
    #[doc = "Bit 13 - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d13(&self) -> SYSCTL_DCGCGPIO_D13R {
        let bits = ((self.bits >> 13) & 1) != 0;
        SYSCTL_DCGCGPIO_D13R { bits }
    }
    #[doc = "Bit 14 - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d14(&self) -> SYSCTL_DCGCGPIO_D14R {
        let bits = ((self.bits >> 14) & 1) != 0;
        SYSCTL_DCGCGPIO_D14R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - GPIO Port A Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d0(&mut self) -> _SYSCTL_DCGCGPIO_D0W {
        _SYSCTL_DCGCGPIO_D0W { w: self }
    }
    #[doc = "Bit 1 - GPIO Port B Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d1(&mut self) -> _SYSCTL_DCGCGPIO_D1W {
        _SYSCTL_DCGCGPIO_D1W { w: self }
    }
    #[doc = "Bit 2 - GPIO Port C Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d2(&mut self) -> _SYSCTL_DCGCGPIO_D2W {
        _SYSCTL_DCGCGPIO_D2W { w: self }
    }
    #[doc = "Bit 3 - GPIO Port D Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d3(&mut self) -> _SYSCTL_DCGCGPIO_D3W {
        _SYSCTL_DCGCGPIO_D3W { w: self }
    }
    #[doc = "Bit 4 - GPIO Port E Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d4(&mut self) -> _SYSCTL_DCGCGPIO_D4W {
        _SYSCTL_DCGCGPIO_D4W { w: self }
    }
    #[doc = "Bit 5 - GPIO Port F Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d5(&mut self) -> _SYSCTL_DCGCGPIO_D5W {
        _SYSCTL_DCGCGPIO_D5W { w: self }
    }
    #[doc = "Bit 6 - GPIO Port G Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d6(&mut self) -> _SYSCTL_DCGCGPIO_D6W {
        _SYSCTL_DCGCGPIO_D6W { w: self }
    }
    #[doc = "Bit 7 - GPIO Port H Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d7(&mut self) -> _SYSCTL_DCGCGPIO_D7W {
        _SYSCTL_DCGCGPIO_D7W { w: self }
    }
    #[doc = "Bit 8 - GPIO Port J Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d8(&mut self) -> _SYSCTL_DCGCGPIO_D8W {
        _SYSCTL_DCGCGPIO_D8W { w: self }
    }
    #[doc = "Bit 9 - GPIO Port K Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d9(&mut self) -> _SYSCTL_DCGCGPIO_D9W {
        _SYSCTL_DCGCGPIO_D9W { w: self }
    }
    #[doc = "Bit 10 - GPIO Port L Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d10(&mut self) -> _SYSCTL_DCGCGPIO_D10W {
        _SYSCTL_DCGCGPIO_D10W { w: self }
    }
    #[doc = "Bit 11 - GPIO Port M Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d11(&mut self) -> _SYSCTL_DCGCGPIO_D11W {
        _SYSCTL_DCGCGPIO_D11W { w: self }
    }
    #[doc = "Bit 12 - GPIO Port N Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d12(&mut self) -> _SYSCTL_DCGCGPIO_D12W {
        _SYSCTL_DCGCGPIO_D12W { w: self }
    }
    #[doc = "Bit 13 - GPIO Port P Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d13(&mut self) -> _SYSCTL_DCGCGPIO_D13W {
        _SYSCTL_DCGCGPIO_D13W { w: self }
    }
    #[doc = "Bit 14 - GPIO Port Q Deep-Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_dcgcgpio_d14(&mut self) -> _SYSCTL_DCGCGPIO_D14W {
        _SYSCTL_DCGCGPIO_D14W { w: self }
    }
}
