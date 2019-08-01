#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SPC {
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
#[doc = "Possible values of the field `ADC_SPC_PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SPC_PHASER {
    #[doc = "ADC sample lags by 0.0"]
    ADC_SPC_PHASE_0,
    #[doc = "ADC sample lags by 22.5"]
    ADC_SPC_PHASE_22_5,
    #[doc = "ADC sample lags by 45.0"]
    ADC_SPC_PHASE_45,
    #[doc = "ADC sample lags by 67.5"]
    ADC_SPC_PHASE_67_5,
    #[doc = "ADC sample lags by 90.0"]
    ADC_SPC_PHASE_90,
    #[doc = "ADC sample lags by 112.5"]
    ADC_SPC_PHASE_112_5,
    #[doc = "ADC sample lags by 135.0"]
    ADC_SPC_PHASE_135,
    #[doc = "ADC sample lags by 157.5"]
    ADC_SPC_PHASE_157_5,
    #[doc = "ADC sample lags by 180.0"]
    ADC_SPC_PHASE_180,
    #[doc = "ADC sample lags by 202.5"]
    ADC_SPC_PHASE_202_5,
    #[doc = "ADC sample lags by 225.0"]
    ADC_SPC_PHASE_225,
    #[doc = "ADC sample lags by 247.5"]
    ADC_SPC_PHASE_247_5,
    #[doc = "ADC sample lags by 270.0"]
    ADC_SPC_PHASE_270,
    #[doc = "ADC sample lags by 292.5"]
    ADC_SPC_PHASE_292_5,
    #[doc = "ADC sample lags by 315.0"]
    ADC_SPC_PHASE_315,
    #[doc = "ADC sample lags by 337.5"]
    ADC_SPC_PHASE_337_5,
}
impl ADC_SPC_PHASER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_SPC_PHASER::ADC_SPC_PHASE_0 => 0,
            ADC_SPC_PHASER::ADC_SPC_PHASE_22_5 => 1,
            ADC_SPC_PHASER::ADC_SPC_PHASE_45 => 2,
            ADC_SPC_PHASER::ADC_SPC_PHASE_67_5 => 3,
            ADC_SPC_PHASER::ADC_SPC_PHASE_90 => 4,
            ADC_SPC_PHASER::ADC_SPC_PHASE_112_5 => 5,
            ADC_SPC_PHASER::ADC_SPC_PHASE_135 => 6,
            ADC_SPC_PHASER::ADC_SPC_PHASE_157_5 => 7,
            ADC_SPC_PHASER::ADC_SPC_PHASE_180 => 8,
            ADC_SPC_PHASER::ADC_SPC_PHASE_202_5 => 9,
            ADC_SPC_PHASER::ADC_SPC_PHASE_225 => 10,
            ADC_SPC_PHASER::ADC_SPC_PHASE_247_5 => 11,
            ADC_SPC_PHASER::ADC_SPC_PHASE_270 => 12,
            ADC_SPC_PHASER::ADC_SPC_PHASE_292_5 => 13,
            ADC_SPC_PHASER::ADC_SPC_PHASE_315 => 14,
            ADC_SPC_PHASER::ADC_SPC_PHASE_337_5 => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_SPC_PHASER {
        match value {
            0 => ADC_SPC_PHASER::ADC_SPC_PHASE_0,
            1 => ADC_SPC_PHASER::ADC_SPC_PHASE_22_5,
            2 => ADC_SPC_PHASER::ADC_SPC_PHASE_45,
            3 => ADC_SPC_PHASER::ADC_SPC_PHASE_67_5,
            4 => ADC_SPC_PHASER::ADC_SPC_PHASE_90,
            5 => ADC_SPC_PHASER::ADC_SPC_PHASE_112_5,
            6 => ADC_SPC_PHASER::ADC_SPC_PHASE_135,
            7 => ADC_SPC_PHASER::ADC_SPC_PHASE_157_5,
            8 => ADC_SPC_PHASER::ADC_SPC_PHASE_180,
            9 => ADC_SPC_PHASER::ADC_SPC_PHASE_202_5,
            10 => ADC_SPC_PHASER::ADC_SPC_PHASE_225,
            11 => ADC_SPC_PHASER::ADC_SPC_PHASE_247_5,
            12 => ADC_SPC_PHASER::ADC_SPC_PHASE_270,
            13 => ADC_SPC_PHASER::ADC_SPC_PHASE_292_5,
            14 => ADC_SPC_PHASER::ADC_SPC_PHASE_315,
            15 => ADC_SPC_PHASER::ADC_SPC_PHASE_337_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_0`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_0(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_0
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_22_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_22_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_22_5
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_45`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_45(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_45
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_67_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_67_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_67_5
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_90`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_90(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_90
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_112_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_112_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_112_5
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_135`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_135(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_135
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_157_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_157_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_157_5
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_180`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_180(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_180
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_202_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_202_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_202_5
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_225`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_225(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_225
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_247_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_247_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_247_5
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_270`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_270(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_270
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_292_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_292_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_292_5
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_315`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_315(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_315
    }
    #[doc = "Checks if the value of the field is `ADC_SPC_PHASE_337_5`"]
    #[inline(always)]
    pub fn is_adc_spc_phase_337_5(&self) -> bool {
        *self == ADC_SPC_PHASER::ADC_SPC_PHASE_337_5
    }
}
#[doc = "Values that can be written to the field `ADC_SPC_PHASE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SPC_PHASEW {
    #[doc = "ADC sample lags by 0.0"]
    ADC_SPC_PHASE_0,
    #[doc = "ADC sample lags by 22.5"]
    ADC_SPC_PHASE_22_5,
    #[doc = "ADC sample lags by 45.0"]
    ADC_SPC_PHASE_45,
    #[doc = "ADC sample lags by 67.5"]
    ADC_SPC_PHASE_67_5,
    #[doc = "ADC sample lags by 90.0"]
    ADC_SPC_PHASE_90,
    #[doc = "ADC sample lags by 112.5"]
    ADC_SPC_PHASE_112_5,
    #[doc = "ADC sample lags by 135.0"]
    ADC_SPC_PHASE_135,
    #[doc = "ADC sample lags by 157.5"]
    ADC_SPC_PHASE_157_5,
    #[doc = "ADC sample lags by 180.0"]
    ADC_SPC_PHASE_180,
    #[doc = "ADC sample lags by 202.5"]
    ADC_SPC_PHASE_202_5,
    #[doc = "ADC sample lags by 225.0"]
    ADC_SPC_PHASE_225,
    #[doc = "ADC sample lags by 247.5"]
    ADC_SPC_PHASE_247_5,
    #[doc = "ADC sample lags by 270.0"]
    ADC_SPC_PHASE_270,
    #[doc = "ADC sample lags by 292.5"]
    ADC_SPC_PHASE_292_5,
    #[doc = "ADC sample lags by 315.0"]
    ADC_SPC_PHASE_315,
    #[doc = "ADC sample lags by 337.5"]
    ADC_SPC_PHASE_337_5,
}
impl ADC_SPC_PHASEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_SPC_PHASEW::ADC_SPC_PHASE_0 => 0,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_22_5 => 1,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_45 => 2,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_67_5 => 3,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_90 => 4,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_112_5 => 5,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_135 => 6,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_157_5 => 7,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_180 => 8,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_202_5 => 9,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_225 => 10,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_247_5 => 11,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_270 => 12,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_292_5 => 13,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_315 => 14,
            ADC_SPC_PHASEW::ADC_SPC_PHASE_337_5 => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SPC_PHASEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SPC_PHASEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SPC_PHASEW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "ADC sample lags by 0.0"]
    #[inline(always)]
    pub fn adc_spc_phase_0(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_0)
    }
    #[doc = "ADC sample lags by 22.5"]
    #[inline(always)]
    pub fn adc_spc_phase_22_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_22_5)
    }
    #[doc = "ADC sample lags by 45.0"]
    #[inline(always)]
    pub fn adc_spc_phase_45(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_45)
    }
    #[doc = "ADC sample lags by 67.5"]
    #[inline(always)]
    pub fn adc_spc_phase_67_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_67_5)
    }
    #[doc = "ADC sample lags by 90.0"]
    #[inline(always)]
    pub fn adc_spc_phase_90(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_90)
    }
    #[doc = "ADC sample lags by 112.5"]
    #[inline(always)]
    pub fn adc_spc_phase_112_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_112_5)
    }
    #[doc = "ADC sample lags by 135.0"]
    #[inline(always)]
    pub fn adc_spc_phase_135(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_135)
    }
    #[doc = "ADC sample lags by 157.5"]
    #[inline(always)]
    pub fn adc_spc_phase_157_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_157_5)
    }
    #[doc = "ADC sample lags by 180.0"]
    #[inline(always)]
    pub fn adc_spc_phase_180(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_180)
    }
    #[doc = "ADC sample lags by 202.5"]
    #[inline(always)]
    pub fn adc_spc_phase_202_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_202_5)
    }
    #[doc = "ADC sample lags by 225.0"]
    #[inline(always)]
    pub fn adc_spc_phase_225(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_225)
    }
    #[doc = "ADC sample lags by 247.5"]
    #[inline(always)]
    pub fn adc_spc_phase_247_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_247_5)
    }
    #[doc = "ADC sample lags by 270.0"]
    #[inline(always)]
    pub fn adc_spc_phase_270(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_270)
    }
    #[doc = "ADC sample lags by 292.5"]
    #[inline(always)]
    pub fn adc_spc_phase_292_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_292_5)
    }
    #[doc = "ADC sample lags by 315.0"]
    #[inline(always)]
    pub fn adc_spc_phase_315(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_315)
    }
    #[doc = "ADC sample lags by 337.5"]
    #[inline(always)]
    pub fn adc_spc_phase_337_5(self) -> &'a mut W {
        self.variant(ADC_SPC_PHASEW::ADC_SPC_PHASE_337_5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline(always)]
    pub fn adc_spc_phase(&self) -> ADC_SPC_PHASER {
        ADC_SPC_PHASER::_from(((self.bits >> 0) & 15) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Phase Difference"]
    #[inline(always)]
    pub fn adc_spc_phase(&mut self) -> _ADC_SPC_PHASEW {
        _ADC_SPC_PHASEW { w: self }
    }
}
