#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PP {
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
#[doc = "Possible values of the field `ADC_PP_MCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PP_MCRR {
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    ADC_PP_MCR_FULL,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_PP_MCRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_PP_MCRR::ADC_PP_MCR_FULL => 7,
            ADC_PP_MCRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_PP_MCRR {
        match value {
            7 => ADC_PP_MCRR::ADC_PP_MCR_FULL,
            i => ADC_PP_MCRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_PP_MCR_FULL`"]
    #[inline(always)]
    pub fn is_adc_pp_mcr_full(&self) -> bool {
        *self == ADC_PP_MCRR::ADC_PP_MCR_FULL
    }
}
#[doc = "Values that can be written to the field `ADC_PP_MCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PP_MCRW {
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    ADC_PP_MCR_FULL,
}
impl ADC_PP_MCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_PP_MCRW::ADC_PP_MCR_FULL => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_PP_MCRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PP_MCRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_PP_MCRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    #[inline(always)]
    pub fn adc_pp_mcr_full(self) -> &'a mut W {
        self.variant(ADC_PP_MCRW::ADC_PP_MCR_FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_PP_CHR {
    bits: u8,
}
impl ADC_PP_CHR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_PP_CHW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PP_CHW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 4);
        self.w.bits |= ((value as u32) & 63) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_PP_DCR {
    bits: u8,
}
impl ADC_PP_DCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_PP_DCW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PP_DCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 10);
        self.w.bits |= ((value as u32) & 63) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `ADC_PP_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PP_TYPER {
    #[doc = "SAR"]
    ADC_PP_TYPE_SAR,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_PP_TYPER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_PP_TYPER::ADC_PP_TYPE_SAR => 0,
            ADC_PP_TYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_PP_TYPER {
        match value {
            0 => ADC_PP_TYPER::ADC_PP_TYPE_SAR,
            i => ADC_PP_TYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_PP_TYPE_SAR`"]
    #[inline(always)]
    pub fn is_adc_pp_type_sar(&self) -> bool {
        *self == ADC_PP_TYPER::ADC_PP_TYPE_SAR
    }
}
#[doc = "Values that can be written to the field `ADC_PP_TYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PP_TYPEW {
    #[doc = "SAR"]
    ADC_PP_TYPE_SAR,
}
impl ADC_PP_TYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_PP_TYPEW::ADC_PP_TYPE_SAR => 0,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_PP_TYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PP_TYPEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_PP_TYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "SAR"]
    #[inline(always)]
    pub fn adc_pp_type_sar(self) -> &'a mut W {
        self.variant(ADC_PP_TYPEW::ADC_PP_TYPE_SAR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 16);
        self.w.bits |= ((value as u32) & 3) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_PP_RSLR {
    bits: u8,
}
impl ADC_PP_RSLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_PP_RSLW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PP_RSLW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(31 << 18);
        self.w.bits |= ((value as u32) & 31) << 18;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_PP_TSR {
    bits: bool,
}
impl ADC_PP_TSR {
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
pub struct _ADC_PP_TSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PP_TSW<'a> {
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
        self.w.bits &= !(1 << 23);
        self.w.bits |= ((value as u32) & 1) << 23;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct ADC_PP_APSHTR {
    bits: bool,
}
impl ADC_PP_APSHTR {
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
pub struct _ADC_PP_APSHTW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PP_APSHTW<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Maximum Conversion Rate"]
    #[inline(always)]
    pub fn adc_pp_mcr(&self) -> ADC_PP_MCRR {
        ADC_PP_MCRR::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline(always)]
    pub fn adc_pp_ch(&self) -> ADC_PP_CHR {
        let bits = ((self.bits >> 4) & 63) as u8;
        ADC_PP_CHR { bits }
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline(always)]
    pub fn adc_pp_dc(&self) -> ADC_PP_DCR {
        let bits = ((self.bits >> 10) & 63) as u8;
        ADC_PP_DCR { bits }
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline(always)]
    pub fn adc_pp_type(&self) -> ADC_PP_TYPER {
        ADC_PP_TYPER::_from(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline(always)]
    pub fn adc_pp_rsl(&self) -> ADC_PP_RSLR {
        let bits = ((self.bits >> 18) & 31) as u8;
        ADC_PP_RSLR { bits }
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline(always)]
    pub fn adc_pp_ts(&self) -> ADC_PP_TSR {
        let bits = ((self.bits >> 23) & 1) != 0;
        ADC_PP_TSR { bits }
    }
    #[doc = "Bit 24 - Application-Programmable Sample-and-Hold Time"]
    #[inline(always)]
    pub fn adc_pp_apsht(&self) -> ADC_PP_APSHTR {
        let bits = ((self.bits >> 24) & 1) != 0;
        ADC_PP_APSHTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Maximum Conversion Rate"]
    #[inline(always)]
    pub fn adc_pp_mcr(&mut self) -> _ADC_PP_MCRW {
        _ADC_PP_MCRW { w: self }
    }
    #[doc = "Bits 4:9 - ADC Channel Count"]
    #[inline(always)]
    pub fn adc_pp_ch(&mut self) -> _ADC_PP_CHW {
        _ADC_PP_CHW { w: self }
    }
    #[doc = "Bits 10:15 - Digital Comparator Count"]
    #[inline(always)]
    pub fn adc_pp_dc(&mut self) -> _ADC_PP_DCW {
        _ADC_PP_DCW { w: self }
    }
    #[doc = "Bits 16:17 - ADC Architecture"]
    #[inline(always)]
    pub fn adc_pp_type(&mut self) -> _ADC_PP_TYPEW {
        _ADC_PP_TYPEW { w: self }
    }
    #[doc = "Bits 18:22 - Resolution"]
    #[inline(always)]
    pub fn adc_pp_rsl(&mut self) -> _ADC_PP_RSLW {
        _ADC_PP_RSLW { w: self }
    }
    #[doc = "Bit 23 - Temperature Sensor"]
    #[inline(always)]
    pub fn adc_pp_ts(&mut self) -> _ADC_PP_TSW {
        _ADC_PP_TSW { w: self }
    }
    #[doc = "Bit 24 - Application-Programmable Sample-and-Hold Time"]
    #[inline(always)]
    pub fn adc_pp_apsht(&mut self) -> _ADC_PP_APSHTW {
        _ADC_PP_APSHTW { w: self }
    }
}
