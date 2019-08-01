#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SAC {
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
#[doc = "Possible values of the field `ADC_SAC_AVG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SAC_AVGR {
    #[doc = "No hardware oversampling"]
    ADC_SAC_AVG_OFF,
    #[doc = "2x hardware oversampling"]
    ADC_SAC_AVG_2X,
    #[doc = "4x hardware oversampling"]
    ADC_SAC_AVG_4X,
    #[doc = "8x hardware oversampling"]
    ADC_SAC_AVG_8X,
    #[doc = "16x hardware oversampling"]
    ADC_SAC_AVG_16X,
    #[doc = "32x hardware oversampling"]
    ADC_SAC_AVG_32X,
    #[doc = "64x hardware oversampling"]
    ADC_SAC_AVG_64X,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_SAC_AVGR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_SAC_AVGR::ADC_SAC_AVG_OFF => 0,
            ADC_SAC_AVGR::ADC_SAC_AVG_2X => 1,
            ADC_SAC_AVGR::ADC_SAC_AVG_4X => 2,
            ADC_SAC_AVGR::ADC_SAC_AVG_8X => 3,
            ADC_SAC_AVGR::ADC_SAC_AVG_16X => 4,
            ADC_SAC_AVGR::ADC_SAC_AVG_32X => 5,
            ADC_SAC_AVGR::ADC_SAC_AVG_64X => 6,
            ADC_SAC_AVGR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_SAC_AVGR {
        match value {
            0 => ADC_SAC_AVGR::ADC_SAC_AVG_OFF,
            1 => ADC_SAC_AVGR::ADC_SAC_AVG_2X,
            2 => ADC_SAC_AVGR::ADC_SAC_AVG_4X,
            3 => ADC_SAC_AVGR::ADC_SAC_AVG_8X,
            4 => ADC_SAC_AVGR::ADC_SAC_AVG_16X,
            5 => ADC_SAC_AVGR::ADC_SAC_AVG_32X,
            6 => ADC_SAC_AVGR::ADC_SAC_AVG_64X,
            i => ADC_SAC_AVGR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_OFF`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_off(&self) -> bool {
        *self == ADC_SAC_AVGR::ADC_SAC_AVG_OFF
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_2X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_2x(&self) -> bool {
        *self == ADC_SAC_AVGR::ADC_SAC_AVG_2X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_4X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_4x(&self) -> bool {
        *self == ADC_SAC_AVGR::ADC_SAC_AVG_4X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_8X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_8x(&self) -> bool {
        *self == ADC_SAC_AVGR::ADC_SAC_AVG_8X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_16X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_16x(&self) -> bool {
        *self == ADC_SAC_AVGR::ADC_SAC_AVG_16X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_32X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_32x(&self) -> bool {
        *self == ADC_SAC_AVGR::ADC_SAC_AVG_32X
    }
    #[doc = "Checks if the value of the field is `ADC_SAC_AVG_64X`"]
    #[inline(always)]
    pub fn is_adc_sac_avg_64x(&self) -> bool {
        *self == ADC_SAC_AVGR::ADC_SAC_AVG_64X
    }
}
#[doc = "Values that can be written to the field `ADC_SAC_AVG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_SAC_AVGW {
    #[doc = "No hardware oversampling"]
    ADC_SAC_AVG_OFF,
    #[doc = "2x hardware oversampling"]
    ADC_SAC_AVG_2X,
    #[doc = "4x hardware oversampling"]
    ADC_SAC_AVG_4X,
    #[doc = "8x hardware oversampling"]
    ADC_SAC_AVG_8X,
    #[doc = "16x hardware oversampling"]
    ADC_SAC_AVG_16X,
    #[doc = "32x hardware oversampling"]
    ADC_SAC_AVG_32X,
    #[doc = "64x hardware oversampling"]
    ADC_SAC_AVG_64X,
}
impl ADC_SAC_AVGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_SAC_AVGW::ADC_SAC_AVG_OFF => 0,
            ADC_SAC_AVGW::ADC_SAC_AVG_2X => 1,
            ADC_SAC_AVGW::ADC_SAC_AVG_4X => 2,
            ADC_SAC_AVGW::ADC_SAC_AVG_8X => 3,
            ADC_SAC_AVGW::ADC_SAC_AVG_16X => 4,
            ADC_SAC_AVGW::ADC_SAC_AVG_32X => 5,
            ADC_SAC_AVGW::ADC_SAC_AVG_64X => 6,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_SAC_AVGW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_SAC_AVGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_SAC_AVGW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_off(self) -> &'a mut W {
        self.variant(ADC_SAC_AVGW::ADC_SAC_AVG_OFF)
    }
    #[doc = "2x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_2x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVGW::ADC_SAC_AVG_2X)
    }
    #[doc = "4x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_4x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVGW::ADC_SAC_AVG_4X)
    }
    #[doc = "8x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_8x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVGW::ADC_SAC_AVG_8X)
    }
    #[doc = "16x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_16x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVGW::ADC_SAC_AVG_16X)
    }
    #[doc = "32x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_32x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVGW::ADC_SAC_AVG_32X)
    }
    #[doc = "64x hardware oversampling"]
    #[inline(always)]
    pub fn adc_sac_avg_64x(self) -> &'a mut W {
        self.variant(ADC_SAC_AVGW::ADC_SAC_AVG_64X)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn adc_sac_avg(&self) -> ADC_SAC_AVGR {
        ADC_SAC_AVGR::_from(((self.bits >> 0) & 7) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Hardware Averaging Control"]
    #[inline(always)]
    pub fn adc_sac_avg(&mut self) -> _ADC_SAC_AVGW {
        _ADC_SAC_AVGW { w: self }
    }
}
