#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PPI2C {
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
pub struct SYSCTL_PPI2C_P0R {
    bits: bool,
}
impl SYSCTL_PPI2C_P0R {
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
pub struct _SYSCTL_PPI2C_P0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P0W<'a> {
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
pub struct SYSCTL_PPI2C_P1R {
    bits: bool,
}
impl SYSCTL_PPI2C_P1R {
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
pub struct _SYSCTL_PPI2C_P1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P1W<'a> {
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
pub struct SYSCTL_PPI2C_P2R {
    bits: bool,
}
impl SYSCTL_PPI2C_P2R {
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
pub struct _SYSCTL_PPI2C_P2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P2W<'a> {
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
pub struct SYSCTL_PPI2C_P3R {
    bits: bool,
}
impl SYSCTL_PPI2C_P3R {
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
pub struct _SYSCTL_PPI2C_P3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P3W<'a> {
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
pub struct SYSCTL_PPI2C_P4R {
    bits: bool,
}
impl SYSCTL_PPI2C_P4R {
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
pub struct _SYSCTL_PPI2C_P4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P4W<'a> {
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
pub struct SYSCTL_PPI2C_P5R {
    bits: bool,
}
impl SYSCTL_PPI2C_P5R {
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
pub struct _SYSCTL_PPI2C_P5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P5W<'a> {
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
pub struct SYSCTL_PPI2C_P6R {
    bits: bool,
}
impl SYSCTL_PPI2C_P6R {
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
pub struct _SYSCTL_PPI2C_P6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P6W<'a> {
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
pub struct SYSCTL_PPI2C_P7R {
    bits: bool,
}
impl SYSCTL_PPI2C_P7R {
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
pub struct _SYSCTL_PPI2C_P7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P7W<'a> {
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
pub struct SYSCTL_PPI2C_P8R {
    bits: bool,
}
impl SYSCTL_PPI2C_P8R {
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
pub struct _SYSCTL_PPI2C_P8W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P8W<'a> {
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
pub struct SYSCTL_PPI2C_P9R {
    bits: bool,
}
impl SYSCTL_PPI2C_P9R {
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
pub struct _SYSCTL_PPI2C_P9W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PPI2C_P9W<'a> {
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
    #[doc = "Bit 0 - I2C Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p0(&self) -> SYSCTL_PPI2C_P0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_PPI2C_P0R { bits }
    }
    #[doc = "Bit 1 - I2C Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p1(&self) -> SYSCTL_PPI2C_P1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_PPI2C_P1R { bits }
    }
    #[doc = "Bit 2 - I2C Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p2(&self) -> SYSCTL_PPI2C_P2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_PPI2C_P2R { bits }
    }
    #[doc = "Bit 3 - I2C Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p3(&self) -> SYSCTL_PPI2C_P3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_PPI2C_P3R { bits }
    }
    #[doc = "Bit 4 - I2C Module 4 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p4(&self) -> SYSCTL_PPI2C_P4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_PPI2C_P4R { bits }
    }
    #[doc = "Bit 5 - I2C Module 5 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p5(&self) -> SYSCTL_PPI2C_P5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_PPI2C_P5R { bits }
    }
    #[doc = "Bit 6 - I2C Module 6 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p6(&self) -> SYSCTL_PPI2C_P6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_PPI2C_P6R { bits }
    }
    #[doc = "Bit 7 - I2C Module 7 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p7(&self) -> SYSCTL_PPI2C_P7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_PPI2C_P7R { bits }
    }
    #[doc = "Bit 8 - I2C Module 8 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p8(&self) -> SYSCTL_PPI2C_P8R {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_PPI2C_P8R { bits }
    }
    #[doc = "Bit 9 - I2C Module 9 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p9(&self) -> SYSCTL_PPI2C_P9R {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_PPI2C_P9R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - I2C Module 0 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p0(&mut self) -> _SYSCTL_PPI2C_P0W {
        _SYSCTL_PPI2C_P0W { w: self }
    }
    #[doc = "Bit 1 - I2C Module 1 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p1(&mut self) -> _SYSCTL_PPI2C_P1W {
        _SYSCTL_PPI2C_P1W { w: self }
    }
    #[doc = "Bit 2 - I2C Module 2 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p2(&mut self) -> _SYSCTL_PPI2C_P2W {
        _SYSCTL_PPI2C_P2W { w: self }
    }
    #[doc = "Bit 3 - I2C Module 3 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p3(&mut self) -> _SYSCTL_PPI2C_P3W {
        _SYSCTL_PPI2C_P3W { w: self }
    }
    #[doc = "Bit 4 - I2C Module 4 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p4(&mut self) -> _SYSCTL_PPI2C_P4W {
        _SYSCTL_PPI2C_P4W { w: self }
    }
    #[doc = "Bit 5 - I2C Module 5 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p5(&mut self) -> _SYSCTL_PPI2C_P5W {
        _SYSCTL_PPI2C_P5W { w: self }
    }
    #[doc = "Bit 6 - I2C Module 6 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p6(&mut self) -> _SYSCTL_PPI2C_P6W {
        _SYSCTL_PPI2C_P6W { w: self }
    }
    #[doc = "Bit 7 - I2C Module 7 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p7(&mut self) -> _SYSCTL_PPI2C_P7W {
        _SYSCTL_PPI2C_P7W { w: self }
    }
    #[doc = "Bit 8 - I2C Module 8 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p8(&mut self) -> _SYSCTL_PPI2C_P8W {
        _SYSCTL_PPI2C_P8W { w: self }
    }
    #[doc = "Bit 9 - I2C Module 9 Present"]
    #[inline(always)]
    pub fn sysctl_ppi2c_p9(&mut self) -> _SYSCTL_PPI2C_P9W {
        _SYSCTL_PPI2C_P9W { w: self }
    }
}
