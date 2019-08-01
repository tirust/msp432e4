#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DCCTL0 {
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
#[doc = "Possible values of the field `ADC_DCCTL0_CIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CIMR {
    #[doc = "Always"]
    ADC_DCCTL0_CIM_ALWAYS,
    #[doc = "Once"]
    ADC_DCCTL0_CIM_ONCE,
    #[doc = "Hysteresis Always"]
    ADC_DCCTL0_CIM_HALWAYS,
    #[doc = "Hysteresis Once"]
    ADC_DCCTL0_CIM_HONCE,
}
impl ADC_DCCTL0_CIMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_ALWAYS => 0,
            ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_ONCE => 1,
            ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_HALWAYS => 2,
            ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_HONCE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_DCCTL0_CIMR {
        match value {
            0 => ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_ALWAYS,
            1 => ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_ONCE,
            2 => ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_HALWAYS,
            3 => ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CIM_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_cim_always(&self) -> bool {
        *self == ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_ALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CIM_ONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_cim_once(&self) -> bool {
        *self == ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_ONCE
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CIM_HALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_cim_halways(&self) -> bool {
        *self == ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_HALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CIM_HONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_cim_honce(&self) -> bool {
        *self == ADC_DCCTL0_CIMR::ADC_DCCTL0_CIM_HONCE
    }
}
#[doc = "Values that can be written to the field `ADC_DCCTL0_CIM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CIMW {
    #[doc = "Always"]
    ADC_DCCTL0_CIM_ALWAYS,
    #[doc = "Once"]
    ADC_DCCTL0_CIM_ONCE,
    #[doc = "Hysteresis Always"]
    ADC_DCCTL0_CIM_HALWAYS,
    #[doc = "Hysteresis Once"]
    ADC_DCCTL0_CIM_HONCE,
}
impl ADC_DCCTL0_CIMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_ALWAYS => 0,
            ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_ONCE => 1,
            ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_HALWAYS => 2,
            ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_HONCE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCCTL0_CIMW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCTL0_CIMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_DCCTL0_CIMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Always"]
    #[inline(always)]
    pub fn adc_dcctl0_cim_always(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn adc_dcctl0_cim_once(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn adc_dcctl0_cim_halways(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn adc_dcctl0_cim_honce(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CIMW::ADC_DCCTL0_CIM_HONCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_DCCTL0_CIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CICR {
    #[doc = "Low Band"]
    ADC_DCCTL0_CIC_LOW,
    #[doc = "Mid Band"]
    ADC_DCCTL0_CIC_MID,
    #[doc = "High Band"]
    ADC_DCCTL0_CIC_HIGH,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_DCCTL0_CICR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_LOW => 0,
            ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_MID => 1,
            ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_HIGH => 3,
            ADC_DCCTL0_CICR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_DCCTL0_CICR {
        match value {
            0 => ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_LOW,
            1 => ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_MID,
            3 => ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_HIGH,
            i => ADC_DCCTL0_CICR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CIC_LOW`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_cic_low(&self) -> bool {
        *self == ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_LOW
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CIC_MID`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_cic_mid(&self) -> bool {
        *self == ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_MID
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CIC_HIGH`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_cic_high(&self) -> bool {
        *self == ADC_DCCTL0_CICR::ADC_DCCTL0_CIC_HIGH
    }
}
#[doc = "Values that can be written to the field `ADC_DCCTL0_CIC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CICW {
    #[doc = "Low Band"]
    ADC_DCCTL0_CIC_LOW,
    #[doc = "Mid Band"]
    ADC_DCCTL0_CIC_MID,
    #[doc = "High Band"]
    ADC_DCCTL0_CIC_HIGH,
}
impl ADC_DCCTL0_CICW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CICW::ADC_DCCTL0_CIC_LOW => 0,
            ADC_DCCTL0_CICW::ADC_DCCTL0_CIC_MID => 1,
            ADC_DCCTL0_CICW::ADC_DCCTL0_CIC_HIGH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCCTL0_CICW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCTL0_CICW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_DCCTL0_CICW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn adc_dcctl0_cic_low(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CICW::ADC_DCCTL0_CIC_LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn adc_dcctl0_cic_mid(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CICW::ADC_DCCTL0_CIC_MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn adc_dcctl0_cic_high(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CICW::ADC_DCCTL0_CIC_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 2);
        self.w.bits |= ((value as u32) & 3) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_DCCTL0_CIER {
    bits: bool,
}
impl ADC_DCCTL0_CIER {
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
pub struct _ADC_DCCTL0_CIEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCTL0_CIEW<'a> {
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
#[doc = "Possible values of the field `ADC_DCCTL0_CTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CTMR {
    #[doc = "Always"]
    ADC_DCCTL0_CTM_ALWAYS,
    #[doc = "Once"]
    ADC_DCCTL0_CTM_ONCE,
    #[doc = "Hysteresis Always"]
    ADC_DCCTL0_CTM_HALWAYS,
    #[doc = "Hysteresis Once"]
    ADC_DCCTL0_CTM_HONCE,
}
impl ADC_DCCTL0_CTMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_ALWAYS => 0,
            ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_ONCE => 1,
            ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_HALWAYS => 2,
            ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_HONCE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_DCCTL0_CTMR {
        match value {
            0 => ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_ALWAYS,
            1 => ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_ONCE,
            2 => ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_HALWAYS,
            3 => ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_HONCE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CTM_ALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_ctm_always(&self) -> bool {
        *self == ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_ALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CTM_ONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_ctm_once(&self) -> bool {
        *self == ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_ONCE
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CTM_HALWAYS`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_ctm_halways(&self) -> bool {
        *self == ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_HALWAYS
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CTM_HONCE`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_ctm_honce(&self) -> bool {
        *self == ADC_DCCTL0_CTMR::ADC_DCCTL0_CTM_HONCE
    }
}
#[doc = "Values that can be written to the field `ADC_DCCTL0_CTM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CTMW {
    #[doc = "Always"]
    ADC_DCCTL0_CTM_ALWAYS,
    #[doc = "Once"]
    ADC_DCCTL0_CTM_ONCE,
    #[doc = "Hysteresis Always"]
    ADC_DCCTL0_CTM_HALWAYS,
    #[doc = "Hysteresis Once"]
    ADC_DCCTL0_CTM_HONCE,
}
impl ADC_DCCTL0_CTMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_ALWAYS => 0,
            ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_ONCE => 1,
            ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_HALWAYS => 2,
            ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_HONCE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCCTL0_CTMW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCTL0_CTMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_DCCTL0_CTMW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Always"]
    #[inline(always)]
    pub fn adc_dcctl0_ctm_always(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_ALWAYS)
    }
    #[doc = "Once"]
    #[inline(always)]
    pub fn adc_dcctl0_ctm_once(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_ONCE)
    }
    #[doc = "Hysteresis Always"]
    #[inline(always)]
    pub fn adc_dcctl0_ctm_halways(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_HALWAYS)
    }
    #[doc = "Hysteresis Once"]
    #[inline(always)]
    pub fn adc_dcctl0_ctm_honce(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CTMW::ADC_DCCTL0_CTM_HONCE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_DCCTL0_CTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CTCR {
    #[doc = "Low Band"]
    ADC_DCCTL0_CTC_LOW,
    #[doc = "Mid Band"]
    ADC_DCCTL0_CTC_MID,
    #[doc = "High Band"]
    ADC_DCCTL0_CTC_HIGH,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_DCCTL0_CTCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_LOW => 0,
            ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_MID => 1,
            ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_HIGH => 3,
            ADC_DCCTL0_CTCR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_DCCTL0_CTCR {
        match value {
            0 => ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_LOW,
            1 => ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_MID,
            3 => ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_HIGH,
            i => ADC_DCCTL0_CTCR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CTC_LOW`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_ctc_low(&self) -> bool {
        *self == ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_LOW
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CTC_MID`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_ctc_mid(&self) -> bool {
        *self == ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_MID
    }
    #[doc = "Checks if the value of the field is `ADC_DCCTL0_CTC_HIGH`"]
    #[inline(always)]
    pub fn is_adc_dcctl0_ctc_high(&self) -> bool {
        *self == ADC_DCCTL0_CTCR::ADC_DCCTL0_CTC_HIGH
    }
}
#[doc = "Values that can be written to the field `ADC_DCCTL0_CTC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_DCCTL0_CTCW {
    #[doc = "Low Band"]
    ADC_DCCTL0_CTC_LOW,
    #[doc = "Mid Band"]
    ADC_DCCTL0_CTC_MID,
    #[doc = "High Band"]
    ADC_DCCTL0_CTC_HIGH,
}
impl ADC_DCCTL0_CTCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_DCCTL0_CTCW::ADC_DCCTL0_CTC_LOW => 0,
            ADC_DCCTL0_CTCW::ADC_DCCTL0_CTC_MID => 1,
            ADC_DCCTL0_CTCW::ADC_DCCTL0_CTC_HIGH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_DCCTL0_CTCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCTL0_CTCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_DCCTL0_CTCW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Low Band"]
    #[inline(always)]
    pub fn adc_dcctl0_ctc_low(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CTCW::ADC_DCCTL0_CTC_LOW)
    }
    #[doc = "Mid Band"]
    #[inline(always)]
    pub fn adc_dcctl0_ctc_mid(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CTCW::ADC_DCCTL0_CTC_MID)
    }
    #[doc = "High Band"]
    #[inline(always)]
    pub fn adc_dcctl0_ctc_high(self) -> &'a mut W {
        self.variant(ADC_DCCTL0_CTCW::ADC_DCCTL0_CTC_HIGH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 10);
        self.w.bits |= ((value as u32) & 3) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_DCCTL0_CTER {
    bits: bool,
}
impl ADC_DCCTL0_CTER {
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
pub struct _ADC_DCCTL0_CTEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_DCCTL0_CTEW<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    pub fn adc_dcctl0_cim(&self) -> ADC_DCCTL0_CIMR {
        ADC_DCCTL0_CIMR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    pub fn adc_dcctl0_cic(&self) -> ADC_DCCTL0_CICR {
        ADC_DCCTL0_CICR::_from(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn adc_dcctl0_cie(&self) -> ADC_DCCTL0_CIER {
        let bits = ((self.bits >> 4) & 1) != 0;
        ADC_DCCTL0_CIER { bits }
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    pub fn adc_dcctl0_ctm(&self) -> ADC_DCCTL0_CTMR {
        ADC_DCCTL0_CTMR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    pub fn adc_dcctl0_ctc(&self) -> ADC_DCCTL0_CTCR {
        ADC_DCCTL0_CTCR::_from(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    pub fn adc_dcctl0_cte(&self) -> ADC_DCCTL0_CTER {
        let bits = ((self.bits >> 12) & 1) != 0;
        ADC_DCCTL0_CTER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Comparison Interrupt Mode"]
    #[inline(always)]
    pub fn adc_dcctl0_cim(&mut self) -> _ADC_DCCTL0_CIMW {
        _ADC_DCCTL0_CIMW { w: self }
    }
    #[doc = "Bits 2:3 - Comparison Interrupt Condition"]
    #[inline(always)]
    pub fn adc_dcctl0_cic(&mut self) -> _ADC_DCCTL0_CICW {
        _ADC_DCCTL0_CICW { w: self }
    }
    #[doc = "Bit 4 - Comparison Interrupt Enable"]
    #[inline(always)]
    pub fn adc_dcctl0_cie(&mut self) -> _ADC_DCCTL0_CIEW {
        _ADC_DCCTL0_CIEW { w: self }
    }
    #[doc = "Bits 8:9 - Comparison Trigger Mode"]
    #[inline(always)]
    pub fn adc_dcctl0_ctm(&mut self) -> _ADC_DCCTL0_CTMW {
        _ADC_DCCTL0_CTMW { w: self }
    }
    #[doc = "Bits 10:11 - Comparison Trigger Condition"]
    #[inline(always)]
    pub fn adc_dcctl0_ctc(&mut self) -> _ADC_DCCTL0_CTCW {
        _ADC_DCCTL0_CTCW { w: self }
    }
    #[doc = "Bit 12 - Comparison Trigger Enable"]
    #[inline(always)]
    pub fn adc_dcctl0_cte(&mut self) -> _ADC_DCCTL0_CTEW {
        _ADC_DCCTL0_CTEW { w: self }
    }
}
