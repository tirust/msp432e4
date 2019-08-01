#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCGCI2C {
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
pub struct SYSCTL_SCGCI2C_S0R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S0R {
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
pub struct _SYSCTL_SCGCI2C_S0W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S0W<'a> {
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
pub struct SYSCTL_SCGCI2C_S1R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S1R {
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
pub struct _SYSCTL_SCGCI2C_S1W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S1W<'a> {
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
pub struct SYSCTL_SCGCI2C_S2R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S2R {
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
pub struct _SYSCTL_SCGCI2C_S2W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S2W<'a> {
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
pub struct SYSCTL_SCGCI2C_S3R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S3R {
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
pub struct _SYSCTL_SCGCI2C_S3W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S3W<'a> {
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
pub struct SYSCTL_SCGCI2C_S4R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S4R {
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
pub struct _SYSCTL_SCGCI2C_S4W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S4W<'a> {
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
pub struct SYSCTL_SCGCI2C_S5R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S5R {
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
pub struct _SYSCTL_SCGCI2C_S5W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S5W<'a> {
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
pub struct SYSCTL_SCGCI2C_S6R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S6R {
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
pub struct _SYSCTL_SCGCI2C_S6W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S6W<'a> {
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
pub struct SYSCTL_SCGCI2C_S7R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S7R {
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
pub struct _SYSCTL_SCGCI2C_S7W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S7W<'a> {
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
pub struct SYSCTL_SCGCI2C_S8R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S8R {
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
pub struct _SYSCTL_SCGCI2C_S8W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S8W<'a> {
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
pub struct SYSCTL_SCGCI2C_S9R {
    bits: bool,
}
impl SYSCTL_SCGCI2C_S9R {
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
pub struct _SYSCTL_SCGCI2C_S9W<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SCGCI2C_S9W<'a> {
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
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s0(&self) -> SYSCTL_SCGCI2C_S0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        SYSCTL_SCGCI2C_S0R { bits }
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s1(&self) -> SYSCTL_SCGCI2C_S1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        SYSCTL_SCGCI2C_S1R { bits }
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s2(&self) -> SYSCTL_SCGCI2C_S2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        SYSCTL_SCGCI2C_S2R { bits }
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s3(&self) -> SYSCTL_SCGCI2C_S3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        SYSCTL_SCGCI2C_S3R { bits }
    }
    #[doc = "Bit 4 - I2C Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s4(&self) -> SYSCTL_SCGCI2C_S4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        SYSCTL_SCGCI2C_S4R { bits }
    }
    #[doc = "Bit 5 - I2C Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s5(&self) -> SYSCTL_SCGCI2C_S5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        SYSCTL_SCGCI2C_S5R { bits }
    }
    #[doc = "Bit 6 - I2C Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s6(&self) -> SYSCTL_SCGCI2C_S6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        SYSCTL_SCGCI2C_S6R { bits }
    }
    #[doc = "Bit 7 - I2C Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s7(&self) -> SYSCTL_SCGCI2C_S7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        SYSCTL_SCGCI2C_S7R { bits }
    }
    #[doc = "Bit 8 - I2C Module 8 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s8(&self) -> SYSCTL_SCGCI2C_S8R {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_SCGCI2C_S8R { bits }
    }
    #[doc = "Bit 9 - I2C Module 9 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s9(&self) -> SYSCTL_SCGCI2C_S9R {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_SCGCI2C_S9R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - I2C Module 0 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s0(&mut self) -> _SYSCTL_SCGCI2C_S0W {
        _SYSCTL_SCGCI2C_S0W { w: self }
    }
    #[doc = "Bit 1 - I2C Module 1 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s1(&mut self) -> _SYSCTL_SCGCI2C_S1W {
        _SYSCTL_SCGCI2C_S1W { w: self }
    }
    #[doc = "Bit 2 - I2C Module 2 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s2(&mut self) -> _SYSCTL_SCGCI2C_S2W {
        _SYSCTL_SCGCI2C_S2W { w: self }
    }
    #[doc = "Bit 3 - I2C Module 3 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s3(&mut self) -> _SYSCTL_SCGCI2C_S3W {
        _SYSCTL_SCGCI2C_S3W { w: self }
    }
    #[doc = "Bit 4 - I2C Module 4 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s4(&mut self) -> _SYSCTL_SCGCI2C_S4W {
        _SYSCTL_SCGCI2C_S4W { w: self }
    }
    #[doc = "Bit 5 - I2C Module 5 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s5(&mut self) -> _SYSCTL_SCGCI2C_S5W {
        _SYSCTL_SCGCI2C_S5W { w: self }
    }
    #[doc = "Bit 6 - I2C Module 6 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s6(&mut self) -> _SYSCTL_SCGCI2C_S6W {
        _SYSCTL_SCGCI2C_S6W { w: self }
    }
    #[doc = "Bit 7 - I2C Module 7 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s7(&mut self) -> _SYSCTL_SCGCI2C_S7W {
        _SYSCTL_SCGCI2C_S7W { w: self }
    }
    #[doc = "Bit 8 - I2C Module 8 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s8(&mut self) -> _SYSCTL_SCGCI2C_S8W {
        _SYSCTL_SCGCI2C_S8W { w: self }
    }
    #[doc = "Bit 9 - I2C Module 9 Sleep Mode Clock Gating Control"]
    #[inline(always)]
    pub fn sysctl_scgci2c_s9(&mut self) -> _SYSCTL_SCGCI2C_S9W {
        _SYSCTL_SCGCI2C_S9W { w: self }
    }
}
