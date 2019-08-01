#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
impl super::_3_FLTSTAT1 {
    #[doc = r"Reads the contents of the register"]
    #[inline(always)]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
}
#[doc = r"Value of the field"]
pub struct PWM_3_FLTSTAT1_DCMP0R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP0R {
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
pub struct PWM_3_FLTSTAT1_DCMP1R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP1R {
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
pub struct PWM_3_FLTSTAT1_DCMP2R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP2R {
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
pub struct PWM_3_FLTSTAT1_DCMP3R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP3R {
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
pub struct PWM_3_FLTSTAT1_DCMP4R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP4R {
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
pub struct PWM_3_FLTSTAT1_DCMP5R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP5R {
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
pub struct PWM_3_FLTSTAT1_DCMP6R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP6R {
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
pub struct PWM_3_FLTSTAT1_DCMP7R {
    bits: bool,
}
impl PWM_3_FLTSTAT1_DCMP7R {
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
    #[doc = "Bit 0 - Digital Comparator 0 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp0(&self) -> PWM_3_FLTSTAT1_DCMP0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP0R { bits }
    }
    #[doc = "Bit 1 - Digital Comparator 1 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp1(&self) -> PWM_3_FLTSTAT1_DCMP1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP1R { bits }
    }
    #[doc = "Bit 2 - Digital Comparator 2 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp2(&self) -> PWM_3_FLTSTAT1_DCMP2R {
        let bits = ((self.bits >> 2) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP2R { bits }
    }
    #[doc = "Bit 3 - Digital Comparator 3 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp3(&self) -> PWM_3_FLTSTAT1_DCMP3R {
        let bits = ((self.bits >> 3) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP3R { bits }
    }
    #[doc = "Bit 4 - Digital Comparator 4 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp4(&self) -> PWM_3_FLTSTAT1_DCMP4R {
        let bits = ((self.bits >> 4) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP4R { bits }
    }
    #[doc = "Bit 5 - Digital Comparator 5 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp5(&self) -> PWM_3_FLTSTAT1_DCMP5R {
        let bits = ((self.bits >> 5) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP5R { bits }
    }
    #[doc = "Bit 6 - Digital Comparator 6 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp6(&self) -> PWM_3_FLTSTAT1_DCMP6R {
        let bits = ((self.bits >> 6) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP6R { bits }
    }
    #[doc = "Bit 7 - Digital Comparator 7 Trigger"]
    #[inline(always)]
    pub fn pwm_3_fltstat1_dcmp7(&self) -> PWM_3_FLTSTAT1_DCMP7R {
        let bits = ((self.bits >> 7) & 1) != 0;
        PWM_3_FLTSTAT1_DCMP7R { bits }
    }
}
