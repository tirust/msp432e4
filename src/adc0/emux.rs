#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::EMUX {
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
#[doc = "Possible values of the field `ADC_EMUX_EM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM0R {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM0_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM0_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM0_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM0_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM0_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM0_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM0_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM0_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM0_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM0_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM0_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM0_ALWAYS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_EMUX_EM0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM0R::ADC_EMUX_EM0_PROCESSOR => 0,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP0 => 1,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP1 => 2,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP2 => 3,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_EXTERNAL => 4,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_TIMER => 5,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM0 => 6,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM1 => 7,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM2 => 8,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM3 => 9,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_NEVER => 14,
            ADC_EMUX_EM0R::ADC_EMUX_EM0_ALWAYS => 15,
            ADC_EMUX_EM0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_EMUX_EM0R {
        match value {
            0 => ADC_EMUX_EM0R::ADC_EMUX_EM0_PROCESSOR,
            1 => ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP0,
            2 => ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP1,
            3 => ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP2,
            4 => ADC_EMUX_EM0R::ADC_EMUX_EM0_EXTERNAL,
            5 => ADC_EMUX_EM0R::ADC_EMUX_EM0_TIMER,
            6 => ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM0,
            7 => ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM1,
            8 => ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM2,
            9 => ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM3,
            14 => ADC_EMUX_EM0R::ADC_EMUX_EM0_NEVER,
            15 => ADC_EMUX_EM0R::ADC_EMUX_EM0_ALWAYS,
            i => ADC_EMUX_EM0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_processor(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp0(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp1(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_comp2(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_external(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_timer(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_never(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM0_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em0_always(&self) -> bool {
        *self == ADC_EMUX_EM0R::ADC_EMUX_EM0_ALWAYS
    }
}
#[doc = "Values that can be written to the field `ADC_EMUX_EM0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM0W {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM0_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM0_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM0_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM0_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM0_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM0_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM0_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM0_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM0_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM0_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM0_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM0_ALWAYS,
}
impl ADC_EMUX_EM0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM0W::ADC_EMUX_EM0_PROCESSOR => 0,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_COMP0 => 1,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_COMP1 => 2,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_COMP2 => 3,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_EXTERNAL => 4,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_TIMER => 5,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM0 => 6,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM1 => 7,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM2 => 8,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM3 => 9,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_NEVER => 14,
            ADC_EMUX_EM0W::ADC_EMUX_EM0_ALWAYS => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_EMUX_EM0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_EMUX_EM0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em0_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em0_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em0_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em0_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em0_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em0_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em0_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em0_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em0_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM0W::ADC_EMUX_EM0_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_EMUX_EM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM1R {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM1_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM1_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM1_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM1_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM1_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM1_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM1_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM1_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM1_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM1_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM1_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM1_ALWAYS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_EMUX_EM1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM1R::ADC_EMUX_EM1_PROCESSOR => 0,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP0 => 1,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP1 => 2,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP2 => 3,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_EXTERNAL => 4,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_TIMER => 5,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM0 => 6,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM1 => 7,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM2 => 8,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM3 => 9,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_NEVER => 14,
            ADC_EMUX_EM1R::ADC_EMUX_EM1_ALWAYS => 15,
            ADC_EMUX_EM1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_EMUX_EM1R {
        match value {
            0 => ADC_EMUX_EM1R::ADC_EMUX_EM1_PROCESSOR,
            1 => ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP0,
            2 => ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP1,
            3 => ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP2,
            4 => ADC_EMUX_EM1R::ADC_EMUX_EM1_EXTERNAL,
            5 => ADC_EMUX_EM1R::ADC_EMUX_EM1_TIMER,
            6 => ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM0,
            7 => ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM1,
            8 => ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM2,
            9 => ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM3,
            14 => ADC_EMUX_EM1R::ADC_EMUX_EM1_NEVER,
            15 => ADC_EMUX_EM1R::ADC_EMUX_EM1_ALWAYS,
            i => ADC_EMUX_EM1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_processor(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp0(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp1(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_comp2(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_external(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_timer(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_never(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM1_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em1_always(&self) -> bool {
        *self == ADC_EMUX_EM1R::ADC_EMUX_EM1_ALWAYS
    }
}
#[doc = "Values that can be written to the field `ADC_EMUX_EM1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM1W {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM1_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM1_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM1_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM1_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM1_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM1_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM1_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM1_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM1_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM1_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM1_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM1_ALWAYS,
}
impl ADC_EMUX_EM1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM1W::ADC_EMUX_EM1_PROCESSOR => 0,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_COMP0 => 1,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_COMP1 => 2,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_COMP2 => 3,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_EXTERNAL => 4,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_TIMER => 5,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM0 => 6,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM1 => 7,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM2 => 8,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM3 => 9,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_NEVER => 14,
            ADC_EMUX_EM1W::ADC_EMUX_EM1_ALWAYS => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_EMUX_EM1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_EMUX_EM1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em1_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em1_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em1_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em1_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em1_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em1_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em1_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em1_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em1_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM1W::ADC_EMUX_EM1_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 4);
        self.w.bits |= ((value as u32) & 15) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_EMUX_EM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM2R {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM2_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM2_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM2_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM2_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM2_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM2_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM2_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM2_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM2_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM2_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM2_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM2_ALWAYS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_EMUX_EM2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM2R::ADC_EMUX_EM2_PROCESSOR => 0,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP0 => 1,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP1 => 2,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP2 => 3,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_EXTERNAL => 4,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_TIMER => 5,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM0 => 6,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM1 => 7,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM2 => 8,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM3 => 9,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_NEVER => 14,
            ADC_EMUX_EM2R::ADC_EMUX_EM2_ALWAYS => 15,
            ADC_EMUX_EM2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_EMUX_EM2R {
        match value {
            0 => ADC_EMUX_EM2R::ADC_EMUX_EM2_PROCESSOR,
            1 => ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP0,
            2 => ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP1,
            3 => ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP2,
            4 => ADC_EMUX_EM2R::ADC_EMUX_EM2_EXTERNAL,
            5 => ADC_EMUX_EM2R::ADC_EMUX_EM2_TIMER,
            6 => ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM0,
            7 => ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM1,
            8 => ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM2,
            9 => ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM3,
            14 => ADC_EMUX_EM2R::ADC_EMUX_EM2_NEVER,
            15 => ADC_EMUX_EM2R::ADC_EMUX_EM2_ALWAYS,
            i => ADC_EMUX_EM2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_processor(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp0(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp1(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_comp2(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_external(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_timer(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_never(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM2_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em2_always(&self) -> bool {
        *self == ADC_EMUX_EM2R::ADC_EMUX_EM2_ALWAYS
    }
}
#[doc = "Values that can be written to the field `ADC_EMUX_EM2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM2W {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM2_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM2_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM2_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM2_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM2_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM2_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM2_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM2_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM2_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM2_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM2_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM2_ALWAYS,
}
impl ADC_EMUX_EM2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM2W::ADC_EMUX_EM2_PROCESSOR => 0,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_COMP0 => 1,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_COMP1 => 2,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_COMP2 => 3,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_EXTERNAL => 4,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_TIMER => 5,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM0 => 6,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM1 => 7,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM2 => 8,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM3 => 9,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_NEVER => 14,
            ADC_EMUX_EM2W::ADC_EMUX_EM2_ALWAYS => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_EMUX_EM2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_EMUX_EM2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em2_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em2_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em2_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em2_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em2_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em2_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em2_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em2_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em2_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM2W::ADC_EMUX_EM2_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 8);
        self.w.bits |= ((value as u32) & 15) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_EMUX_EM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM3R {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM3_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM3_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM3_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM3_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM3_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM3_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM3_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM3_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM3_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM3_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM3_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM3_ALWAYS,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_EMUX_EM3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM3R::ADC_EMUX_EM3_PROCESSOR => 0,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP0 => 1,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP1 => 2,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP2 => 3,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_EXTERNAL => 4,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_TIMER => 5,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM0 => 6,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM1 => 7,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM2 => 8,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM3 => 9,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_NEVER => 14,
            ADC_EMUX_EM3R::ADC_EMUX_EM3_ALWAYS => 15,
            ADC_EMUX_EM3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_EMUX_EM3R {
        match value {
            0 => ADC_EMUX_EM3R::ADC_EMUX_EM3_PROCESSOR,
            1 => ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP0,
            2 => ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP1,
            3 => ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP2,
            4 => ADC_EMUX_EM3R::ADC_EMUX_EM3_EXTERNAL,
            5 => ADC_EMUX_EM3R::ADC_EMUX_EM3_TIMER,
            6 => ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM0,
            7 => ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM1,
            8 => ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM2,
            9 => ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM3,
            14 => ADC_EMUX_EM3R::ADC_EMUX_EM3_NEVER,
            15 => ADC_EMUX_EM3R::ADC_EMUX_EM3_ALWAYS,
            i => ADC_EMUX_EM3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PROCESSOR`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_processor(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_PROCESSOR
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP0`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp0(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP1`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp1(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_COMP2`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_comp2(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_COMP2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_EXTERNAL`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_external(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_EXTERNAL
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_TIMER`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_timer(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_TIMER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM0`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm0(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM0
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM1`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm1(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM1
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM2`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm2(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM2
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_PWM3`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_pwm3(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_PWM3
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_NEVER`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_never(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_NEVER
    }
    #[doc = "Checks if the value of the field is `ADC_EMUX_EM3_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_emux_em3_always(&self) -> bool {
        *self == ADC_EMUX_EM3R::ADC_EMUX_EM3_ALWAYS
    }
}
#[doc = "Values that can be written to the field `ADC_EMUX_EM3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_EMUX_EM3W {
    #[doc = "Processor (default)"]
    ADC_EMUX_EM3_PROCESSOR,
    #[doc = "Analog Comparator 0"]
    ADC_EMUX_EM3_COMP0,
    #[doc = "Analog Comparator 1"]
    ADC_EMUX_EM3_COMP1,
    #[doc = "Analog Comparator 2"]
    ADC_EMUX_EM3_COMP2,
    #[doc = "External (GPIO Pins)"]
    ADC_EMUX_EM3_EXTERNAL,
    #[doc = "Timer"]
    ADC_EMUX_EM3_TIMER,
    #[doc = "PWM generator 0"]
    ADC_EMUX_EM3_PWM0,
    #[doc = "PWM generator 1"]
    ADC_EMUX_EM3_PWM1,
    #[doc = "PWM generator 2"]
    ADC_EMUX_EM3_PWM2,
    #[doc = "PWM generator 3"]
    ADC_EMUX_EM3_PWM3,
    #[doc = "Never Trigger"]
    ADC_EMUX_EM3_NEVER,
    #[doc = "Always (continuously sample)"]
    ADC_EMUX_EM3_ALWAYS,
}
impl ADC_EMUX_EM3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_EMUX_EM3W::ADC_EMUX_EM3_PROCESSOR => 0,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_COMP0 => 1,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_COMP1 => 2,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_COMP2 => 3,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_EXTERNAL => 4,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_TIMER => 5,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM0 => 6,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM1 => 7,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM2 => 8,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM3 => 9,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_NEVER => 14,
            ADC_EMUX_EM3W::ADC_EMUX_EM3_ALWAYS => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_EMUX_EM3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_EMUX_EM3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_EMUX_EM3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Processor (default)"]
    #[inline(always)]
    pub fn adc_emux_em3_processor(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_PROCESSOR)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn adc_emux_em3_comp0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_COMP0)
    }
    #[doc = "Analog Comparator 1"]
    #[inline(always)]
    pub fn adc_emux_em3_comp1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_COMP1)
    }
    #[doc = "Analog Comparator 2"]
    #[inline(always)]
    pub fn adc_emux_em3_comp2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_COMP2)
    }
    #[doc = "External (GPIO Pins)"]
    #[inline(always)]
    pub fn adc_emux_em3_external(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_EXTERNAL)
    }
    #[doc = "Timer"]
    #[inline(always)]
    pub fn adc_emux_em3_timer(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_TIMER)
    }
    #[doc = "PWM generator 0"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm0(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM0)
    }
    #[doc = "PWM generator 1"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm1(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM1)
    }
    #[doc = "PWM generator 2"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm2(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM2)
    }
    #[doc = "PWM generator 3"]
    #[inline(always)]
    pub fn adc_emux_em3_pwm3(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_PWM3)
    }
    #[doc = "Never Trigger"]
    #[inline(always)]
    pub fn adc_emux_em3_never(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_NEVER)
    }
    #[doc = "Always (continuously sample)"]
    #[inline(always)]
    pub fn adc_emux_em3_always(self) -> &'a mut W {
        self.variant(ADC_EMUX_EM3W::ADC_EMUX_EM3_ALWAYS)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 12);
        self.w.bits |= ((value as u32) & 15) << 12;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em0(&self) -> ADC_EMUX_EM0R {
        ADC_EMUX_EM0R::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em1(&self) -> ADC_EMUX_EM1R {
        ADC_EMUX_EM1R::_from(((self.bits >> 4) & 15) as u8)
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em2(&self) -> ADC_EMUX_EM2R {
        ADC_EMUX_EM2R::_from(((self.bits >> 8) & 15) as u8)
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em3(&self) -> ADC_EMUX_EM3R {
        ADC_EMUX_EM3R::_from(((self.bits >> 12) & 15) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - SS0 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em0(&mut self) -> _ADC_EMUX_EM0W {
        _ADC_EMUX_EM0W { w: self }
    }
    #[doc = "Bits 4:7 - SS1 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em1(&mut self) -> _ADC_EMUX_EM1W {
        _ADC_EMUX_EM1W { w: self }
    }
    #[doc = "Bits 8:11 - SS2 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em2(&mut self) -> _ADC_EMUX_EM2W {
        _ADC_EMUX_EM2W { w: self }
    }
    #[doc = "Bits 12:15 - SS3 Trigger Select"]
    #[inline(always)]
    pub fn adc_emux_em3(&mut self) -> _ADC_EMUX_EM3W {
        _ADC_EMUX_EM3W { w: self }
    }
}
