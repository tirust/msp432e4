#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_3_FLTSRC1 {
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
pub struct PWM_3_FLTSRC1_DCMP0R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP0R {
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
pub struct _PWM_3_FLTSRC1_DCMP0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP0W<'a> {
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
pub struct PWM_3_FLTSRC1_DCMP1R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP1R {
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
pub struct _PWM_3_FLTSRC1_DCMP1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP1W<'a> {
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
pub struct PWM_3_FLTSRC1_DCMP2R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP2R {
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
pub struct _PWM_3_FLTSRC1_DCMP2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP2W<'a> {
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
pub struct PWM_3_FLTSRC1_DCMP3R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP3R {
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
pub struct _PWM_3_FLTSRC1_DCMP3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP3W<'a> {
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
pub struct PWM_3_FLTSRC1_DCMP4R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP4R {
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
pub struct _PWM_3_FLTSRC1_DCMP4W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP4W<'a> {
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
pub struct PWM_3_FLTSRC1_DCMP5R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP5R {
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
pub struct _PWM_3_FLTSRC1_DCMP5W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP5W<'a> {
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
pub struct PWM_3_FLTSRC1_DCMP6R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP6R {
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
pub struct _PWM_3_FLTSRC1_DCMP6W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP6W<'a> {
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
pub struct PWM_3_FLTSRC1_DCMP7R {
    bits: bool,
}
impl PWM_3_FLTSRC1_DCMP7R {
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
pub struct _PWM_3_FLTSRC1_DCMP7W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_3_FLTSRC1_DCMP7W<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Digital Comparator 0"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp0(&self) -> PWM_3_FLTSRC1_DCMP0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_3_FLTSRC1_DCMP0R { bits }
    }
    #[doc = "Bit 1 - Digital Comparator 1"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp1(&self) -> PWM_3_FLTSRC1_DCMP1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_3_FLTSRC1_DCMP1R { bits }
    }
    #[doc = "Bit 2 - Digital Comparator 2"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp2(&self) -> PWM_3_FLTSRC1_DCMP2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_3_FLTSRC1_DCMP2R { bits }
    }
    #[doc = "Bit 3 - Digital Comparator 3"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp3(&self) -> PWM_3_FLTSRC1_DCMP3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_3_FLTSRC1_DCMP3R { bits }
    }
    #[doc = "Bit 4 - Digital Comparator 4"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp4(&self) -> PWM_3_FLTSRC1_DCMP4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_3_FLTSRC1_DCMP4R { bits }
    }
    #[doc = "Bit 5 - Digital Comparator 5"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp5(&self) -> PWM_3_FLTSRC1_DCMP5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_3_FLTSRC1_DCMP5R { bits }
    }
    #[doc = "Bit 6 - Digital Comparator 6"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp6(&self) -> PWM_3_FLTSRC1_DCMP6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        PWM_3_FLTSRC1_DCMP6R { bits }
    }
    #[doc = "Bit 7 - Digital Comparator 7"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp7(&self) -> PWM_3_FLTSRC1_DCMP7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        PWM_3_FLTSRC1_DCMP7R { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Digital Comparator 0"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp0(&mut self) -> _PWM_3_FLTSRC1_DCMP0W {
        _PWM_3_FLTSRC1_DCMP0W { w: self }
    }
    #[doc = "Bit 1 - Digital Comparator 1"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp1(&mut self) -> _PWM_3_FLTSRC1_DCMP1W {
        _PWM_3_FLTSRC1_DCMP1W { w: self }
    }
    #[doc = "Bit 2 - Digital Comparator 2"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp2(&mut self) -> _PWM_3_FLTSRC1_DCMP2W {
        _PWM_3_FLTSRC1_DCMP2W { w: self }
    }
    #[doc = "Bit 3 - Digital Comparator 3"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp3(&mut self) -> _PWM_3_FLTSRC1_DCMP3W {
        _PWM_3_FLTSRC1_DCMP3W { w: self }
    }
    #[doc = "Bit 4 - Digital Comparator 4"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp4(&mut self) -> _PWM_3_FLTSRC1_DCMP4W {
        _PWM_3_FLTSRC1_DCMP4W { w: self }
    }
    #[doc = "Bit 5 - Digital Comparator 5"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp5(&mut self) -> _PWM_3_FLTSRC1_DCMP5W {
        _PWM_3_FLTSRC1_DCMP5W { w: self }
    }
    #[doc = "Bit 6 - Digital Comparator 6"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp6(&mut self) -> _PWM_3_FLTSRC1_DCMP6W {
        _PWM_3_FLTSRC1_DCMP6W { w: self }
    }
    #[doc = "Bit 7 - Digital Comparator 7"]
    #[inline(always)]
    pub fn pwm_3_fltsrc1_dcmp7(&mut self) -> _PWM_3_FLTSRC1_DCMP7W {
        _PWM_3_FLTSRC1_DCMP7W { w: self }
    }
}
