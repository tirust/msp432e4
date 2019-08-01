#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_2_CTL {
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
pub struct PWM_2_CTL_ENABLER {
    bits: bool,
}
impl PWM_2_CTL_ENABLER {
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
pub struct _PWM_2_CTL_ENABLEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_ENABLEW<'a> {
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
pub struct PWM_2_CTL_MODER {
    bits: bool,
}
impl PWM_2_CTL_MODER {
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
pub struct _PWM_2_CTL_MODEW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_MODEW<'a> {
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
pub struct PWM_2_CTL_DEBUGR {
    bits: bool,
}
impl PWM_2_CTL_DEBUGR {
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
pub struct _PWM_2_CTL_DEBUGW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_DEBUGW<'a> {
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
pub struct PWM_2_CTL_LOADUPDR {
    bits: bool,
}
impl PWM_2_CTL_LOADUPDR {
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
pub struct _PWM_2_CTL_LOADUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_LOADUPDW<'a> {
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
pub struct PWM_2_CTL_CMPAUPDR {
    bits: bool,
}
impl PWM_2_CTL_CMPAUPDR {
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
pub struct _PWM_2_CTL_CMPAUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_CMPAUPDW<'a> {
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
pub struct PWM_2_CTL_CMPBUPDR {
    bits: bool,
}
impl PWM_2_CTL_CMPBUPDR {
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
pub struct _PWM_2_CTL_CMPBUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_CMPBUPDW<'a> {
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
#[doc = "Possible values of the field `PWM_2_CTL_GENAUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_GENAUPDR {
    #[doc = "Immediate"]
    PWM_2_CTL_GENAUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_GENAUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_GENAUPD_GS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_2_CTL_GENAUPDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_I => 0,
            PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_LS => 2,
            PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_GS => 3,
            PWM_2_CTL_GENAUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_2_CTL_GENAUPDR {
        match value {
            0 => PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_I,
            2 => PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_LS,
            3 => PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_GS,
            i => PWM_2_CTL_GENAUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENAUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genaupd_i(&self) -> bool {
        *self == PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENAUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genaupd_ls(&self) -> bool {
        *self == PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENAUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genaupd_gs(&self) -> bool {
        *self == PWM_2_CTL_GENAUPDR::PWM_2_CTL_GENAUPD_GS
    }
}
#[doc = "Values that can be written to the field `PWM_2_CTL_GENAUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_GENAUPDW {
    #[doc = "Immediate"]
    PWM_2_CTL_GENAUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_GENAUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_GENAUPD_GS,
}
impl PWM_2_CTL_GENAUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_GENAUPDW::PWM_2_CTL_GENAUPD_I => 0,
            PWM_2_CTL_GENAUPDW::PWM_2_CTL_GENAUPD_LS => 2,
            PWM_2_CTL_GENAUPDW::PWM_2_CTL_GENAUPD_GS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_2_CTL_GENAUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_GENAUPDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_2_CTL_GENAUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENAUPDW::PWM_2_CTL_GENAUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENAUPDW::PWM_2_CTL_GENAUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENAUPDW::PWM_2_CTL_GENAUPD_GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_2_CTL_GENBUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_GENBUPDR {
    #[doc = "Immediate"]
    PWM_2_CTL_GENBUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_GENBUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_GENBUPD_GS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_2_CTL_GENBUPDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_I => 0,
            PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_LS => 2,
            PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_GS => 3,
            PWM_2_CTL_GENBUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_2_CTL_GENBUPDR {
        match value {
            0 => PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_I,
            2 => PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_LS,
            3 => PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_GS,
            i => PWM_2_CTL_GENBUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENBUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genbupd_i(&self) -> bool {
        *self == PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENBUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genbupd_ls(&self) -> bool {
        *self == PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_GENBUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_genbupd_gs(&self) -> bool {
        *self == PWM_2_CTL_GENBUPDR::PWM_2_CTL_GENBUPD_GS
    }
}
#[doc = "Values that can be written to the field `PWM_2_CTL_GENBUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_GENBUPDW {
    #[doc = "Immediate"]
    PWM_2_CTL_GENBUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_GENBUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_GENBUPD_GS,
}
impl PWM_2_CTL_GENBUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_GENBUPDW::PWM_2_CTL_GENBUPD_I => 0,
            PWM_2_CTL_GENBUPDW::PWM_2_CTL_GENBUPD_LS => 2,
            PWM_2_CTL_GENBUPDW::PWM_2_CTL_GENBUPD_GS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_2_CTL_GENBUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_GENBUPDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_2_CTL_GENBUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENBUPDW::PWM_2_CTL_GENBUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENBUPDW::PWM_2_CTL_GENBUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_GENBUPDW::PWM_2_CTL_GENBUPD_GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_2_CTL_DBCTLUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_DBCTLUPDR {
    #[doc = "Immediate"]
    PWM_2_CTL_DBCTLUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_DBCTLUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_DBCTLUPD_GS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_2_CTL_DBCTLUPDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_I => 0,
            PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_LS => 2,
            PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_GS => 3,
            PWM_2_CTL_DBCTLUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_2_CTL_DBCTLUPDR {
        match value {
            0 => PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_I,
            2 => PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_LS,
            3 => PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_GS,
            i => PWM_2_CTL_DBCTLUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBCTLUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbctlupd_i(&self) -> bool {
        *self == PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBCTLUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbctlupd_ls(&self) -> bool {
        *self == PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBCTLUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbctlupd_gs(&self) -> bool {
        *self == PWM_2_CTL_DBCTLUPDR::PWM_2_CTL_DBCTLUPD_GS
    }
}
#[doc = "Values that can be written to the field `PWM_2_CTL_DBCTLUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_DBCTLUPDW {
    #[doc = "Immediate"]
    PWM_2_CTL_DBCTLUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_DBCTLUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_DBCTLUPD_GS,
}
impl PWM_2_CTL_DBCTLUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_DBCTLUPDW::PWM_2_CTL_DBCTLUPD_I => 0,
            PWM_2_CTL_DBCTLUPDW::PWM_2_CTL_DBCTLUPD_LS => 2,
            PWM_2_CTL_DBCTLUPDW::PWM_2_CTL_DBCTLUPD_GS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_2_CTL_DBCTLUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_DBCTLUPDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_2_CTL_DBCTLUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBCTLUPDW::PWM_2_CTL_DBCTLUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBCTLUPDW::PWM_2_CTL_DBCTLUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBCTLUPDW::PWM_2_CTL_DBCTLUPD_GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_2_CTL_DBRISEUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_DBRISEUPDR {
    #[doc = "Immediate"]
    PWM_2_CTL_DBRISEUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_DBRISEUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_DBRISEUPD_GS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_2_CTL_DBRISEUPDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_I => 0,
            PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_LS => 2,
            PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_GS => 3,
            PWM_2_CTL_DBRISEUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_2_CTL_DBRISEUPDR {
        match value {
            0 => PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_I,
            2 => PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_LS,
            3 => PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_GS,
            i => PWM_2_CTL_DBRISEUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBRISEUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbriseupd_i(&self) -> bool {
        *self == PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBRISEUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbriseupd_ls(&self) -> bool {
        *self == PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBRISEUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbriseupd_gs(&self) -> bool {
        *self == PWM_2_CTL_DBRISEUPDR::PWM_2_CTL_DBRISEUPD_GS
    }
}
#[doc = "Values that can be written to the field `PWM_2_CTL_DBRISEUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_DBRISEUPDW {
    #[doc = "Immediate"]
    PWM_2_CTL_DBRISEUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_DBRISEUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_DBRISEUPD_GS,
}
impl PWM_2_CTL_DBRISEUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_DBRISEUPDW::PWM_2_CTL_DBRISEUPD_I => 0,
            PWM_2_CTL_DBRISEUPDW::PWM_2_CTL_DBRISEUPD_LS => 2,
            PWM_2_CTL_DBRISEUPDW::PWM_2_CTL_DBRISEUPD_GS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_2_CTL_DBRISEUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_DBRISEUPDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_2_CTL_DBRISEUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBRISEUPDW::PWM_2_CTL_DBRISEUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBRISEUPDW::PWM_2_CTL_DBRISEUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBRISEUPDW::PWM_2_CTL_DBRISEUPD_GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_2_CTL_DBFALLUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_DBFALLUPDR {
    #[doc = "Immediate"]
    PWM_2_CTL_DBFALLUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_DBFALLUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_DBFALLUPD_GS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl PWM_2_CTL_DBFALLUPDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_I => 0,
            PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_LS => 2,
            PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_GS => 3,
            PWM_2_CTL_DBFALLUPDR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_2_CTL_DBFALLUPDR {
        match value {
            0 => PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_I,
            2 => PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_LS,
            3 => PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_GS,
            i => PWM_2_CTL_DBFALLUPDR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBFALLUPD_I`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbfallupd_i(&self) -> bool {
        *self == PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_I
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBFALLUPD_LS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbfallupd_ls(&self) -> bool {
        *self == PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_LS
    }
    #[doc = "Checks if the value of the field is `PWM_2_CTL_DBFALLUPD_GS`"]
    #[inline(always)]
    pub fn is_pwm_2_ctl_dbfallupd_gs(&self) -> bool {
        *self == PWM_2_CTL_DBFALLUPDR::PWM_2_CTL_DBFALLUPD_GS
    }
}
#[doc = "Values that can be written to the field `PWM_2_CTL_DBFALLUPD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_2_CTL_DBFALLUPDW {
    #[doc = "Immediate"]
    PWM_2_CTL_DBFALLUPD_I,
    #[doc = "Locally Synchronized"]
    PWM_2_CTL_DBFALLUPD_LS,
    #[doc = "Globally Synchronized"]
    PWM_2_CTL_DBFALLUPD_GS,
}
impl PWM_2_CTL_DBFALLUPDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_2_CTL_DBFALLUPDW::PWM_2_CTL_DBFALLUPD_I => 0,
            PWM_2_CTL_DBFALLUPDW::PWM_2_CTL_DBFALLUPD_LS => 2,
            PWM_2_CTL_DBFALLUPDW::PWM_2_CTL_DBFALLUPD_GS => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_2_CTL_DBFALLUPDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_DBFALLUPDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_2_CTL_DBFALLUPDW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Immediate"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd_i(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBFALLUPDW::PWM_2_CTL_DBFALLUPD_I)
    }
    #[doc = "Locally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd_ls(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBFALLUPDW::PWM_2_CTL_DBFALLUPD_LS)
    }
    #[doc = "Globally Synchronized"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd_gs(self) -> &'a mut W {
        self.variant(PWM_2_CTL_DBFALLUPDW::PWM_2_CTL_DBFALLUPD_GS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 14);
        self.w.bits |= ((value as u32) & 3) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct PWM_2_CTL_FLTSRCR {
    bits: bool,
}
impl PWM_2_CTL_FLTSRCR {
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
pub struct _PWM_2_CTL_FLTSRCW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_FLTSRCW<'a> {
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
pub struct PWM_2_CTL_MINFLTPERR {
    bits: bool,
}
impl PWM_2_CTL_MINFLTPERR {
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
pub struct _PWM_2_CTL_MINFLTPERW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_MINFLTPERW<'a> {
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
pub struct PWM_2_CTL_LATCHR {
    bits: bool,
}
impl PWM_2_CTL_LATCHR {
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
pub struct _PWM_2_CTL_LATCHW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_2_CTL_LATCHW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn pwm_2_ctl_enable(&self) -> PWM_2_CTL_ENABLER {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_2_CTL_ENABLER { bits }
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_mode(&self) -> PWM_2_CTL_MODER {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_2_CTL_MODER { bits }
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_debug(&self) -> PWM_2_CTL_DEBUGR {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_2_CTL_DEBUGR { bits }
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_loadupd(&self) -> PWM_2_CTL_LOADUPDR {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_2_CTL_LOADUPDR { bits }
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpaupd(&self) -> PWM_2_CTL_CMPAUPDR {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_2_CTL_CMPAUPDR { bits }
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpbupd(&self) -> PWM_2_CTL_CMPBUPDR {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_2_CTL_CMPBUPDR { bits }
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd(&self) -> PWM_2_CTL_GENAUPDR {
        PWM_2_CTL_GENAUPDR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd(&self) -> PWM_2_CTL_GENBUPDR {
        PWM_2_CTL_GENBUPDR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd(&self) -> PWM_2_CTL_DBCTLUPDR {
        PWM_2_CTL_DBCTLUPDR::_from(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd(&self) -> PWM_2_CTL_DBRISEUPDR {
        PWM_2_CTL_DBRISEUPDR::_from(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd(&self) -> PWM_2_CTL_DBFALLUPDR {
        PWM_2_CTL_DBFALLUPDR::_from(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    pub fn pwm_2_ctl_fltsrc(&self) -> PWM_2_CTL_FLTSRCR {
        let bits = ((self.bits >> 16) & 1) != 0;
        PWM_2_CTL_FLTSRCR { bits }
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_2_ctl_minfltper(&self) -> PWM_2_CTL_MINFLTPERR {
        let bits = ((self.bits >> 17) & 1) != 0;
        PWM_2_CTL_MINFLTPERR { bits }
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    pub fn pwm_2_ctl_latch(&self) -> PWM_2_CTL_LATCHR {
        let bits = ((self.bits >> 18) & 1) != 0;
        PWM_2_CTL_LATCHR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - PWM Block Enable"]
    #[inline(always)]
    pub fn pwm_2_ctl_enable(&mut self) -> _PWM_2_CTL_ENABLEW {
        _PWM_2_CTL_ENABLEW { w: self }
    }
    #[doc = "Bit 1 - Counter Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_mode(&mut self) -> _PWM_2_CTL_MODEW {
        _PWM_2_CTL_MODEW { w: self }
    }
    #[doc = "Bit 2 - Debug Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_debug(&mut self) -> _PWM_2_CTL_DEBUGW {
        _PWM_2_CTL_DEBUGW { w: self }
    }
    #[doc = "Bit 3 - Load Register Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_loadupd(&mut self) -> _PWM_2_CTL_LOADUPDW {
        _PWM_2_CTL_LOADUPDW { w: self }
    }
    #[doc = "Bit 4 - Comparator A Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpaupd(&mut self) -> _PWM_2_CTL_CMPAUPDW {
        _PWM_2_CTL_CMPAUPDW { w: self }
    }
    #[doc = "Bit 5 - Comparator B Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_cmpbupd(&mut self) -> _PWM_2_CTL_CMPBUPDW {
        _PWM_2_CTL_CMPBUPDW { w: self }
    }
    #[doc = "Bits 6:7 - PWMnGENA Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genaupd(&mut self) -> _PWM_2_CTL_GENAUPDW {
        _PWM_2_CTL_GENAUPDW { w: self }
    }
    #[doc = "Bits 8:9 - PWMnGENB Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_genbupd(&mut self) -> _PWM_2_CTL_GENBUPDW {
        _PWM_2_CTL_GENBUPDW { w: self }
    }
    #[doc = "Bits 10:11 - PWMnDBCTL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbctlupd(&mut self) -> _PWM_2_CTL_DBCTLUPDW {
        _PWM_2_CTL_DBCTLUPDW { w: self }
    }
    #[doc = "Bits 12:13 - PWMnDBRISE Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbriseupd(&mut self) -> _PWM_2_CTL_DBRISEUPDW {
        _PWM_2_CTL_DBRISEUPDW { w: self }
    }
    #[doc = "Bits 14:15 - PWMnDBFALL Update Mode"]
    #[inline(always)]
    pub fn pwm_2_ctl_dbfallupd(&mut self) -> _PWM_2_CTL_DBFALLUPDW {
        _PWM_2_CTL_DBFALLUPDW { w: self }
    }
    #[doc = "Bit 16 - Fault Condition Source"]
    #[inline(always)]
    pub fn pwm_2_ctl_fltsrc(&mut self) -> _PWM_2_CTL_FLTSRCW {
        _PWM_2_CTL_FLTSRCW { w: self }
    }
    #[doc = "Bit 17 - Minimum Fault Period"]
    #[inline(always)]
    pub fn pwm_2_ctl_minfltper(&mut self) -> _PWM_2_CTL_MINFLTPERW {
        _PWM_2_CTL_MINFLTPERW { w: self }
    }
    #[doc = "Bit 18 - Latch Fault Input"]
    #[inline(always)]
    pub fn pwm_2_ctl_latch(&mut self) -> _PWM_2_CTL_LATCHW {
        _PWM_2_CTL_LATCHW { w: self }
    }
}
