#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::_0_FLTSTAT0 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Value of the field"]
pub struct PWM_0_FLTSTAT0_FAULT0R {
    bits: bool,
}
impl PWM_0_FLTSTAT0_FAULT0R {
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
#[doc = r"Value of the field"]
pub struct PWM_0_FLTSTAT0_FAULT1R {
    bits: bool,
}
impl PWM_0_FLTSTAT0_FAULT1R {
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
#[doc = r"Value of the field"]
pub struct PWM_0_FLTSTAT0_FAULT2R {
    bits: bool,
}
impl PWM_0_FLTSTAT0_FAULT2R {
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
#[doc = r"Value of the field"]
pub struct PWM_0_FLTSTAT0_FAULT3R {
    bits: bool,
}
impl PWM_0_FLTSTAT0_FAULT3R {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Fault Input 0"]
    #[inline(always)]
    pub fn pwm_0_fltstat0_fault0(&self) -> PWM_0_FLTSTAT0_FAULT0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_0_FLTSTAT0_FAULT0R { bits }
    }
    #[doc = "Bit 1 - Fault Input 1"]
    #[inline(always)]
    pub fn pwm_0_fltstat0_fault1(&self) -> PWM_0_FLTSTAT0_FAULT1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_0_FLTSTAT0_FAULT1R { bits }
    }
    #[doc = "Bit 2 - Fault Input 2"]
    #[inline(always)]
    pub fn pwm_0_fltstat0_fault2(&self) -> PWM_0_FLTSTAT0_FAULT2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_0_FLTSTAT0_FAULT2R { bits }
    }
    #[doc = "Bit 3 - Fault Input 3"]
    #[inline(always)]
    pub fn pwm_0_fltstat0_fault3(&self) -> PWM_0_FLTSTAT0_FAULT3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_0_FLTSTAT0_FAULT3R { bits }
    }
}
