#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::ENUPD {
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
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD0R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD0_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD0_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD0_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_IMM => 0,
            PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_LSYNC => 2,
            PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_GSYNC => 3,
            PWM_ENUPD_ENUPD0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD0R {
        match value {
            0 => PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_IMM,
            2 => PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_LSYNC,
            3 => PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_GSYNC,
            i => PWM_ENUPD_ENUPD0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD0_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD0_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD0_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd0_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD0R::PWM_ENUPD_ENUPD0_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD0W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD0_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD0_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD0_GSYNC,
}
impl PWM_ENUPD_ENUPD0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD0W::PWM_ENUPD_ENUPD0_IMM => 0,
            PWM_ENUPD_ENUPD0W::PWM_ENUPD_ENUPD0_LSYNC => 2,
            PWM_ENUPD_ENUPD0W::PWM_ENUPD_ENUPD0_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD0W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD0W::PWM_ENUPD_ENUPD0_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD0W::PWM_ENUPD_ENUPD0_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD0W::PWM_ENUPD_ENUPD0_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD1R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD1_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD1_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD1_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_IMM => 0,
            PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_LSYNC => 2,
            PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_GSYNC => 3,
            PWM_ENUPD_ENUPD1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD1R {
        match value {
            0 => PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_IMM,
            2 => PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_LSYNC,
            3 => PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_GSYNC,
            i => PWM_ENUPD_ENUPD1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD1_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD1_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD1_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd1_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD1R::PWM_ENUPD_ENUPD1_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD1W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD1_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD1_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD1_GSYNC,
}
impl PWM_ENUPD_ENUPD1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD1W::PWM_ENUPD_ENUPD1_IMM => 0,
            PWM_ENUPD_ENUPD1W::PWM_ENUPD_ENUPD1_LSYNC => 2,
            PWM_ENUPD_ENUPD1W::PWM_ENUPD_ENUPD1_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD1W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD1W::PWM_ENUPD_ENUPD1_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD1W::PWM_ENUPD_ENUPD1_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD1W::PWM_ENUPD_ENUPD1_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD2R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD2_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD2_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD2_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_IMM => 0,
            PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_LSYNC => 2,
            PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_GSYNC => 3,
            PWM_ENUPD_ENUPD2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD2R {
        match value {
            0 => PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_IMM,
            2 => PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_LSYNC,
            3 => PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_GSYNC,
            i => PWM_ENUPD_ENUPD2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD2_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD2_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD2_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd2_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD2R::PWM_ENUPD_ENUPD2_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD2W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD2_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD2_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD2_GSYNC,
}
impl PWM_ENUPD_ENUPD2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD2W::PWM_ENUPD_ENUPD2_IMM => 0,
            PWM_ENUPD_ENUPD2W::PWM_ENUPD_ENUPD2_LSYNC => 2,
            PWM_ENUPD_ENUPD2W::PWM_ENUPD_ENUPD2_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD2W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD2W::PWM_ENUPD_ENUPD2_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD2W::PWM_ENUPD_ENUPD2_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD2W::PWM_ENUPD_ENUPD2_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD3R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD3_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD3_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD3_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_IMM => 0,
            PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_LSYNC => 2,
            PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_GSYNC => 3,
            PWM_ENUPD_ENUPD3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD3R {
        match value {
            0 => PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_IMM,
            2 => PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_LSYNC,
            3 => PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_GSYNC,
            i => PWM_ENUPD_ENUPD3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD3_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD3_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD3_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd3_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD3R::PWM_ENUPD_ENUPD3_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD3W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD3_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD3_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD3_GSYNC,
}
impl PWM_ENUPD_ENUPD3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD3W::PWM_ENUPD_ENUPD3_IMM => 0,
            PWM_ENUPD_ENUPD3W::PWM_ENUPD_ENUPD3_LSYNC => 2,
            PWM_ENUPD_ENUPD3W::PWM_ENUPD_ENUPD3_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD3W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD3W::PWM_ENUPD_ENUPD3_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD3W::PWM_ENUPD_ENUPD3_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD3W::PWM_ENUPD_ENUPD3_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD4R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD4_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD4_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD4_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD4R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_IMM => 0,
            PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_LSYNC => 2,
            PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_GSYNC => 3,
            PWM_ENUPD_ENUPD4R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD4R {
        match value {
            0 => PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_IMM,
            2 => PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_LSYNC,
            3 => PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_GSYNC,
            i => PWM_ENUPD_ENUPD4R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD4_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD4_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD4_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd4_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD4R::PWM_ENUPD_ENUPD4_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD4`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD4W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD4_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD4_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD4_GSYNC,
}
impl PWM_ENUPD_ENUPD4W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD4W::PWM_ENUPD_ENUPD4_IMM => 0,
            PWM_ENUPD_ENUPD4W::PWM_ENUPD_ENUPD4_LSYNC => 2,
            PWM_ENUPD_ENUPD4W::PWM_ENUPD_ENUPD4_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD4W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD4W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD4W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD4W::PWM_ENUPD_ENUPD4_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD4W::PWM_ENUPD_ENUPD4_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD4W::PWM_ENUPD_ENUPD4_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD5R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD5_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD5_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD5_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD5R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_IMM => 0,
            PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_LSYNC => 2,
            PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_GSYNC => 3,
            PWM_ENUPD_ENUPD5R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD5R {
        match value {
            0 => PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_IMM,
            2 => PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_LSYNC,
            3 => PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_GSYNC,
            i => PWM_ENUPD_ENUPD5R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD5_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD5_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD5_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd5_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD5R::PWM_ENUPD_ENUPD5_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD5`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD5W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD5_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD5_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD5_GSYNC,
}
impl PWM_ENUPD_ENUPD5W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD5W::PWM_ENUPD_ENUPD5_IMM => 0,
            PWM_ENUPD_ENUPD5W::PWM_ENUPD_ENUPD5_LSYNC => 2,
            PWM_ENUPD_ENUPD5W::PWM_ENUPD_ENUPD5_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD5W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD5W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD5W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD5W::PWM_ENUPD_ENUPD5_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD5W::PWM_ENUPD_ENUPD5_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD5W::PWM_ENUPD_ENUPD5_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD6R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD6_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD6_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD6_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD6R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_IMM => 0,
            PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_LSYNC => 2,
            PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_GSYNC => 3,
            PWM_ENUPD_ENUPD6R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD6R {
        match value {
            0 => PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_IMM,
            2 => PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_LSYNC,
            3 => PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_GSYNC,
            i => PWM_ENUPD_ENUPD6R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD6_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD6_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD6_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd6_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD6R::PWM_ENUPD_ENUPD6_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD6`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD6W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD6_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD6_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD6_GSYNC,
}
impl PWM_ENUPD_ENUPD6W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD6W::PWM_ENUPD_ENUPD6_IMM => 0,
            PWM_ENUPD_ENUPD6W::PWM_ENUPD_ENUPD6_LSYNC => 2,
            PWM_ENUPD_ENUPD6W::PWM_ENUPD_ENUPD6_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD6W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD6W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD6W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD6W::PWM_ENUPD_ENUPD6_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD6W::PWM_ENUPD_ENUPD6_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD6W::PWM_ENUPD_ENUPD6_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_ENUPD_ENUPD7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD7R {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD7_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD7_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD7_GSYNC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_ENUPD_ENUPD7R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_IMM => 0,
            PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_LSYNC => 2,
            PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_GSYNC => 3,
            PWM_ENUPD_ENUPD7R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_ENUPD_ENUPD7R {
        match value {
            0 => PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_IMM,
            2 => PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_LSYNC,
            3 => PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_GSYNC,
            i => PWM_ENUPD_ENUPD7R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD7_IMM`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_imm(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_IMM
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD7_LSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_lsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_LSYNC
    }
    #[doc = "Checks if the value of the field is `PWM_ENUPD_ENUPD7_GSYNC`"]
    #[inline(always)]
    pub fn is_pwm_enupd_enupd7_gsync(&self) -> bool {
        *self == PWM_ENUPD_ENUPD7R::PWM_ENUPD_ENUPD7_GSYNC
    }
}
#[doc = "Values that can be written to the field `PWM_ENUPD_ENUPD7`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_ENUPD_ENUPD7W {
    #[doc = "Immediate"]
    PWM_ENUPD_ENUPD7_IMM,
    #[doc = "Locally Synchronized"]
    PWM_ENUPD_ENUPD7_LSYNC,
    #[doc = "Globally Synchronized"]
    PWM_ENUPD_ENUPD7_GSYNC,
}
impl PWM_ENUPD_ENUPD7W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_ENUPD_ENUPD7W::PWM_ENUPD_ENUPD7_IMM => 0,
            PWM_ENUPD_ENUPD7W::PWM_ENUPD_ENUPD7_LSYNC => 2,
            PWM_ENUPD_ENUPD7W::PWM_ENUPD_ENUPD7_GSYNC => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_ENUPD_ENUPD7W<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_ENUPD_ENUPD7W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_ENUPD_ENUPD7W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_imm(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD7W::PWM_ENUPD_ENUPD7_IMM)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_lsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD7W::PWM_ENUPD_ENUPD7_LSYNC)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7_gsync(self) -> &'a mut W {
        self.variant(PWM_ENUPD_ENUPD7W::PWM_ENUPD_ENUPD7_GSYNC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 14);
        self.w.bits |= ((value as u32) & 3) << 14;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0(&self) -> PWM_ENUPD_ENUPD0R {
        PWM_ENUPD_ENUPD0R::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1(&self) -> PWM_ENUPD_ENUPD1R {
        PWM_ENUPD_ENUPD1R::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2(&self) -> PWM_ENUPD_ENUPD2R {
        PWM_ENUPD_ENUPD2R::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3(&self) -> PWM_ENUPD_ENUPD3R {
        PWM_ENUPD_ENUPD3R::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4(&self) -> PWM_ENUPD_ENUPD4R {
        PWM_ENUPD_ENUPD4R::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5(&self) -> PWM_ENUPD_ENUPD5R {
        PWM_ENUPD_ENUPD5R::_from(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6(&self) -> PWM_ENUPD_ENUPD6R {
        PWM_ENUPD_ENUPD6R::_from(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7(&self) -> PWM_ENUPD_ENUPD7R {
        PWM_ENUPD_ENUPD7R::_from(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - MnPWM0 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd0(&mut self) -> _PWM_ENUPD_ENUPD0W {
        _PWM_ENUPD_ENUPD0W { w: self }
    }
    #[doc = "Bits 2:3 - MnPWM1 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd1(&mut self) -> _PWM_ENUPD_ENUPD1W {
        _PWM_ENUPD_ENUPD1W { w: self }
    }
    #[doc = "Bits 4:5 - MnPWM2 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd2(&mut self) -> _PWM_ENUPD_ENUPD2W {
        _PWM_ENUPD_ENUPD2W { w: self }
    }
    #[doc = "Bits 6:7 - MnPWM3 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd3(&mut self) -> _PWM_ENUPD_ENUPD3W {
        _PWM_ENUPD_ENUPD3W { w: self }
    }
    #[doc = "Bits 8:9 - MnPWM4 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd4(&mut self) -> _PWM_ENUPD_ENUPD4W {
        _PWM_ENUPD_ENUPD4W { w: self }
    }
    #[doc = "Bits 10:11 - MnPWM5 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd5(&mut self) -> _PWM_ENUPD_ENUPD5W {
        _PWM_ENUPD_ENUPD5W { w: self }
    }
    #[doc = "Bits 12:13 - MnPWM6 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd6(&mut self) -> _PWM_ENUPD_ENUPD6W {
        _PWM_ENUPD_ENUPD6W { w: self }
    }
    #[doc = "Bits 14:15 - MnPWM7 Enable Update Mode"]
    #[inline(always)]
    pub fn pwm_enupd_enupd7(&mut self) -> _PWM_ENUPD_ENUPD7W {
        _PWM_ENUPD_ENUPD7W { w: self }
    }
}
