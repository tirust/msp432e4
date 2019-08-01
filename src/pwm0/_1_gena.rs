#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::_1_GENA {
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
#[doc = "Possible values of the field `PWM_1_GENA_ACTZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTZEROR {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTZERO_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTZERO_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTZERO_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTZERO_ONE,
}
impl PWM_1_GENA_ACTZEROR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_NONE => 0,
            PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_INV => 1,
            PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_ZERO => 2,
            PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_1_GENA_ACTZEROR {
        match value {
            0 => PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_NONE,
            1 => PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_INV,
            2 => PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_ZERO,
            3 => PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTZERO_NONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actzero_none(&self) -> bool {
        *self == PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTZERO_INV`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actzero_inv(&self) -> bool {
        *self == PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_INV
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTZERO_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actzero_zero(&self) -> bool {
        *self == PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTZERO_ONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actzero_one(&self) -> bool {
        *self == PWM_1_GENA_ACTZEROR::PWM_1_GENA_ACTZERO_ONE
    }
}
#[doc = "Values that can be written to the field `PWM_1_GENA_ACTZERO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTZEROW {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTZERO_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTZERO_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTZERO_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTZERO_ONE,
}
impl PWM_1_GENA_ACTZEROW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_NONE => 0,
            PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_INV => 1,
            PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_ZERO => 2,
            PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_ONE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_1_GENA_ACTZEROW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_GENA_ACTZEROW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_1_GENA_ACTZEROW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_1_gena_actzero_none(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_1_gena_actzero_inv(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_1_gena_actzero_zero(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_1_gena_actzero_one(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTZEROW::PWM_1_GENA_ACTZERO_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_1_GENA_ACTLOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTLOADR {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTLOAD_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTLOAD_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTLOAD_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTLOAD_ONE,
}
impl PWM_1_GENA_ACTLOADR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_NONE => 0,
            PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_INV => 1,
            PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_ZERO => 2,
            PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_1_GENA_ACTLOADR {
        match value {
            0 => PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_NONE,
            1 => PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_INV,
            2 => PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_ZERO,
            3 => PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTLOAD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actload_none(&self) -> bool {
        *self == PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTLOAD_INV`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actload_inv(&self) -> bool {
        *self == PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTLOAD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actload_zero(&self) -> bool {
        *self == PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTLOAD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actload_one(&self) -> bool {
        *self == PWM_1_GENA_ACTLOADR::PWM_1_GENA_ACTLOAD_ONE
    }
}
#[doc = "Values that can be written to the field `PWM_1_GENA_ACTLOAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTLOADW {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTLOAD_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTLOAD_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTLOAD_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTLOAD_ONE,
}
impl PWM_1_GENA_ACTLOADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_NONE => 0,
            PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_INV => 1,
            PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_ZERO => 2,
            PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_ONE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_1_GENA_ACTLOADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_GENA_ACTLOADW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_1_GENA_ACTLOADW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_1_gena_actload_none(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_1_gena_actload_inv(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_1_gena_actload_zero(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_1_gena_actload_one(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTLOADW::PWM_1_GENA_ACTLOAD_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_1_GENA_ACTCMPAU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPAUR {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPAU_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPAU_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPAU_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPAU_ONE,
}
impl PWM_1_GENA_ACTCMPAUR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_NONE => 0,
            PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_INV => 1,
            PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_ZERO => 2,
            PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_1_GENA_ACTCMPAUR {
        match value {
            0 => PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_NONE,
            1 => PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_INV,
            2 => PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_ZERO,
            3 => PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAU_NONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpau_none(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAU_INV`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpau_inv(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_INV
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAU_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpau_zero(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAU_ONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpau_one(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPAUR::PWM_1_GENA_ACTCMPAU_ONE
    }
}
#[doc = "Values that can be written to the field `PWM_1_GENA_ACTCMPAU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPAUW {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPAU_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPAU_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPAU_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPAU_ONE,
}
impl PWM_1_GENA_ACTCMPAUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_NONE => 0,
            PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_INV => 1,
            PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_ZERO => 2,
            PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_ONE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_1_GENA_ACTCMPAUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_GENA_ACTCMPAUW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_1_GENA_ACTCMPAUW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpau_none(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpau_inv(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpau_zero(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpau_one(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPAUW::PWM_1_GENA_ACTCMPAU_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_1_GENA_ACTCMPAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPADR {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPAD_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPAD_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPAD_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPAD_ONE,
}
impl PWM_1_GENA_ACTCMPADR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_NONE => 0,
            PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_INV => 1,
            PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_ZERO => 2,
            PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_1_GENA_ACTCMPADR {
        match value {
            0 => PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_NONE,
            1 => PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_INV,
            2 => PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_ZERO,
            3 => PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpad_none(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAD_INV`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpad_inv(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpad_zero(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPAD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpad_one(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPADR::PWM_1_GENA_ACTCMPAD_ONE
    }
}
#[doc = "Values that can be written to the field `PWM_1_GENA_ACTCMPAD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPADW {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPAD_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPAD_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPAD_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPAD_ONE,
}
impl PWM_1_GENA_ACTCMPADW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_NONE => 0,
            PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_INV => 1,
            PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_ZERO => 2,
            PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_ONE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_1_GENA_ACTCMPADW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_GENA_ACTCMPADW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_1_GENA_ACTCMPADW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpad_none(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpad_inv(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpad_zero(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpad_one(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPADW::PWM_1_GENA_ACTCMPAD_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 6);
        self.w.bits |= ((value as u32) & 3) << 6;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_1_GENA_ACTCMPBU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPBUR {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPBU_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPBU_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPBU_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPBU_ONE,
}
impl PWM_1_GENA_ACTCMPBUR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_NONE => 0,
            PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_INV => 1,
            PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_ZERO => 2,
            PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_1_GENA_ACTCMPBUR {
        match value {
            0 => PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_NONE,
            1 => PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_INV,
            2 => PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_ZERO,
            3 => PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBU_NONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbu_none(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBU_INV`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbu_inv(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_INV
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBU_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbu_zero(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBU_ONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbu_one(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBUR::PWM_1_GENA_ACTCMPBU_ONE
    }
}
#[doc = "Values that can be written to the field `PWM_1_GENA_ACTCMPBU`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPBUW {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPBU_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPBU_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPBU_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPBU_ONE,
}
impl PWM_1_GENA_ACTCMPBUW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_NONE => 0,
            PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_INV => 1,
            PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_ZERO => 2,
            PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_ONE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_1_GENA_ACTCMPBUW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_GENA_ACTCMPBUW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_1_GENA_ACTCMPBUW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbu_none(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbu_inv(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbu_zero(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbu_one(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBUW::PWM_1_GENA_ACTCMPBU_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `PWM_1_GENA_ACTCMPBD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPBDR {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPBD_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPBD_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPBD_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPBD_ONE,
}
impl PWM_1_GENA_ACTCMPBDR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_NONE => 0,
            PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_INV => 1,
            PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_ZERO => 2,
            PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_ONE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> PWM_1_GENA_ACTCMPBDR {
        match value {
            0 => PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_NONE,
            1 => PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_INV,
            2 => PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_ZERO,
            3 => PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_ONE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBD_NONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbd_none(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_NONE
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBD_INV`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbd_inv(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_INV
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBD_ZERO`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbd_zero(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_ZERO
    }
    #[doc = "Checks if the value of the field is `PWM_1_GENA_ACTCMPBD_ONE`"]
    #[inline(always)]
    pub fn is_pwm_1_gena_actcmpbd_one(&self) -> bool {
        *self == PWM_1_GENA_ACTCMPBDR::PWM_1_GENA_ACTCMPBD_ONE
    }
}
#[doc = "Values that can be written to the field `PWM_1_GENA_ACTCMPBD`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PWM_1_GENA_ACTCMPBDW {
    #[doc = "Do nothing"]
    PWM_1_GENA_ACTCMPBD_NONE,
    #[doc = "Invert pwmA"]
    PWM_1_GENA_ACTCMPBD_INV,
    #[doc = "Drive pwmA Low"]
    PWM_1_GENA_ACTCMPBD_ZERO,
    #[doc = "Drive pwmA High"]
    PWM_1_GENA_ACTCMPBD_ONE,
}
impl PWM_1_GENA_ACTCMPBDW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_NONE => 0,
            PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_INV => 1,
            PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_ZERO => 2,
            PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_ONE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _PWM_1_GENA_ACTCMPBDW<'a> {
    w: &'a mut W,
}
impl<'a> _PWM_1_GENA_ACTCMPBDW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PWM_1_GENA_ACTCMPBDW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Do nothing"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbd_none(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_NONE)
    }
    #[doc = "Invert pwmA"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbd_inv(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_INV)
    }
    #[doc = "Drive pwmA Low"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbd_zero(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_ZERO)
    }
    #[doc = "Drive pwmA High"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbd_one(self) -> &'a mut W {
        self.variant(PWM_1_GENA_ACTCMPBDW::PWM_1_GENA_ACTCMPBD_ONE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn pwm_1_gena_actzero(&self) -> PWM_1_GENA_ACTZEROR {
        PWM_1_GENA_ACTZEROR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn pwm_1_gena_actload(&self) -> PWM_1_GENA_ACTLOADR {
        PWM_1_GENA_ACTLOADR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpau(&self) -> PWM_1_GENA_ACTCMPAUR {
        PWM_1_GENA_ACTCMPAUR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpad(&self) -> PWM_1_GENA_ACTCMPADR {
        PWM_1_GENA_ACTCMPADR::_from(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbu(&self) -> PWM_1_GENA_ACTCMPBUR {
        PWM_1_GENA_ACTCMPBUR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbd(&self) -> PWM_1_GENA_ACTCMPBDR {
        PWM_1_GENA_ACTCMPBDR::_from(((self.bits >> 10) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Action for Counter=0"]
    #[inline(always)]
    pub fn pwm_1_gena_actzero(&mut self) -> _PWM_1_GENA_ACTZEROW {
        _PWM_1_GENA_ACTZEROW { w: self }
    }
    #[doc = "Bits 2:3 - Action for Counter=LOAD"]
    #[inline(always)]
    pub fn pwm_1_gena_actload(&mut self) -> _PWM_1_GENA_ACTLOADW {
        _PWM_1_GENA_ACTLOADW { w: self }
    }
    #[doc = "Bits 4:5 - Action for Comparator A Up"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpau(&mut self) -> _PWM_1_GENA_ACTCMPAUW {
        _PWM_1_GENA_ACTCMPAUW { w: self }
    }
    #[doc = "Bits 6:7 - Action for Comparator A Down"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpad(&mut self) -> _PWM_1_GENA_ACTCMPADW {
        _PWM_1_GENA_ACTCMPADW { w: self }
    }
    #[doc = "Bits 8:9 - Action for Comparator B Up"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbu(&mut self) -> _PWM_1_GENA_ACTCMPBUW {
        _PWM_1_GENA_ACTCMPBUW { w: self }
    }
    #[doc = "Bits 10:11 - Action for Comparator B Down"]
    #[inline(always)]
    pub fn pwm_1_gena_actcmpbd(&mut self) -> _PWM_1_GENA_ACTCMPBDW {
        _PWM_1_GENA_ACTCMPBDW { w: self }
    }
}
