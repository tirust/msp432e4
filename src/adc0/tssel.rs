#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TSSEL {
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
#[doc = "Possible values of the field `ADC_TSSEL_PS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS0R {
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS0_0,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_TSSEL_PS0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS0R::ADC_TSSEL_PS0_0 => 0,
            ADC_TSSEL_PS0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_TSSEL_PS0R {
        match value {
            0 => ADC_TSSEL_PS0R::ADC_TSSEL_PS0_0,
            i => ADC_TSSEL_PS0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS0_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps0_0(&self) -> bool {
        *self == ADC_TSSEL_PS0R::ADC_TSSEL_PS0_0
    }
}
#[doc = "Values that can be written to the field `ADC_TSSEL_PS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS0W {
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS0_0,
}
impl ADC_TSSEL_PS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS0W::ADC_TSSEL_PS0_0 => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_TSSEL_PS0W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_TSSEL_PS0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_TSSEL_PS0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 0 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps0_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS0W::ADC_TSSEL_PS0_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_TSSEL_PS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS1R {
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS1_0,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_TSSEL_PS1R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS1R::ADC_TSSEL_PS1_0 => 0,
            ADC_TSSEL_PS1R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_TSSEL_PS1R {
        match value {
            0 => ADC_TSSEL_PS1R::ADC_TSSEL_PS1_0,
            i => ADC_TSSEL_PS1R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS1_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps1_0(&self) -> bool {
        *self == ADC_TSSEL_PS1R::ADC_TSSEL_PS1_0
    }
}
#[doc = "Values that can be written to the field `ADC_TSSEL_PS1`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS1W {
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS1_0,
}
impl ADC_TSSEL_PS1W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS1W::ADC_TSSEL_PS1_0 => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_TSSEL_PS1W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_TSSEL_PS1W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_TSSEL_PS1W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 1 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps1_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS1W::ADC_TSSEL_PS1_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 12);
        self.w.bits |= ((value as u32) & 3) << 12;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_TSSEL_PS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS2R {
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS2_0,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_TSSEL_PS2R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS2R::ADC_TSSEL_PS2_0 => 0,
            ADC_TSSEL_PS2R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_TSSEL_PS2R {
        match value {
            0 => ADC_TSSEL_PS2R::ADC_TSSEL_PS2_0,
            i => ADC_TSSEL_PS2R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS2_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps2_0(&self) -> bool {
        *self == ADC_TSSEL_PS2R::ADC_TSSEL_PS2_0
    }
}
#[doc = "Values that can be written to the field `ADC_TSSEL_PS2`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS2W {
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS2_0,
}
impl ADC_TSSEL_PS2W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS2W::ADC_TSSEL_PS2_0 => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_TSSEL_PS2W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_TSSEL_PS2W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_TSSEL_PS2W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 2 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps2_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS2W::ADC_TSSEL_PS2_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 20);
        self.w.bits |= ((value as u32) & 3) << 20;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_TSSEL_PS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS3R {
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS3_0,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_TSSEL_PS3R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS3R::ADC_TSSEL_PS3_0 => 0,
            ADC_TSSEL_PS3R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_TSSEL_PS3R {
        match value {
            0 => ADC_TSSEL_PS3R::ADC_TSSEL_PS3_0,
            i => ADC_TSSEL_PS3R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_TSSEL_PS3_0`"]
    #[inline(always)]
    pub fn is_adc_tssel_ps3_0(&self) -> bool {
        *self == ADC_TSSEL_PS3R::ADC_TSSEL_PS3_0
    }
}
#[doc = "Values that can be written to the field `ADC_TSSEL_PS3`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_TSSEL_PS3W {
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    ADC_TSSEL_PS3_0,
}
impl ADC_TSSEL_PS3W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_TSSEL_PS3W::ADC_TSSEL_PS3_0 => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_TSSEL_PS3W<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_TSSEL_PS3W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_TSSEL_PS3W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Use Generator 3 (and its trigger) in PWM module 0"]
    #[inline(always)]
    pub fn adc_tssel_ps3_0(self) -> &'a mut W {
        self.variant(ADC_TSSEL_PS3W::ADC_TSSEL_PS3_0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 28);
        self.w.bits |= ((value as u32) & 3) << 28;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps0(&self) -> ADC_TSSEL_PS0R {
        ADC_TSSEL_PS0R::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps1(&self) -> ADC_TSSEL_PS1R {
        ADC_TSSEL_PS1R::_from(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps2(&self) -> ADC_TSSEL_PS2R {
        ADC_TSSEL_PS2R::_from(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps3(&self) -> ADC_TSSEL_PS3R {
        ADC_TSSEL_PS3R::_from(((self.bits >> 28) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 4:5 - Generator 0 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps0(&mut self) -> _ADC_TSSEL_PS0W {
        _ADC_TSSEL_PS0W { w: self }
    }
    #[doc = "Bits 12:13 - Generator 1 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps1(&mut self) -> _ADC_TSSEL_PS1W {
        _ADC_TSSEL_PS1W { w: self }
    }
    #[doc = "Bits 20:21 - Generator 2 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps2(&mut self) -> _ADC_TSSEL_PS2W {
        _ADC_TSSEL_PS2W { w: self }
    }
    #[doc = "Bits 28:29 - Generator 3 PWM Module Trigger Select"]
    #[inline(always)]
    pub fn adc_tssel_ps3(&mut self) -> _ADC_TSSEL_PS3W {
        _ADC_TSSEL_PS3W { w: self }
    }
}
