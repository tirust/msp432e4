#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC {
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
#[doc = "Possible values of the field `ADC_CC_CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CC_CSR {
    #[doc = "PLL VCO divided by CLKDIV"]
    ADC_CC_CS_SYSPLL,
    #[doc = "PIOSC"]
    ADC_CC_CS_PIOSC,
    #[doc = "MOSC"]
    ADC_CC_CS_MOSC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_CC_CSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_CC_CSR::ADC_CC_CS_SYSPLL => 0,
            ADC_CC_CSR::ADC_CC_CS_PIOSC => 1,
            ADC_CC_CSR::ADC_CC_CS_MOSC => 2,
            ADC_CC_CSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_CC_CSR {
        match value {
            0 => ADC_CC_CSR::ADC_CC_CS_SYSPLL,
            1 => ADC_CC_CSR::ADC_CC_CS_PIOSC,
            2 => ADC_CC_CSR::ADC_CC_CS_MOSC,
            i => ADC_CC_CSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_CC_CS_SYSPLL`"]
    #[inline(always)]
    pub fn is_adc_cc_cs_syspll(&self) -> bool {
        *self == ADC_CC_CSR::ADC_CC_CS_SYSPLL
    }
    #[doc = "Checks if the value of the field is `ADC_CC_CS_PIOSC`"]
    #[inline(always)]
    pub fn is_adc_cc_cs_piosc(&self) -> bool {
        *self == ADC_CC_CSR::ADC_CC_CS_PIOSC
    }
    #[doc = "Checks if the value of the field is `ADC_CC_CS_MOSC`"]
    #[inline(always)]
    pub fn is_adc_cc_cs_mosc(&self) -> bool {
        *self == ADC_CC_CSR::ADC_CC_CS_MOSC
    }
}
#[doc = "Values that can be written to the field `ADC_CC_CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_CC_CSW {
    #[doc = "PLL VCO divided by CLKDIV"]
    ADC_CC_CS_SYSPLL,
    #[doc = "PIOSC"]
    ADC_CC_CS_PIOSC,
    #[doc = "MOSC"]
    ADC_CC_CS_MOSC,
}
impl ADC_CC_CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_CC_CSW::ADC_CC_CS_SYSPLL => 0,
            ADC_CC_CSW::ADC_CC_CS_PIOSC => 1,
            ADC_CC_CSW::ADC_CC_CS_MOSC => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_CC_CSW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_CC_CSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_CC_CSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "PLL VCO divided by CLKDIV"]
    #[inline(always)]
    pub fn adc_cc_cs_syspll(self) -> &'a mut W {
        self.variant(ADC_CC_CSW::ADC_CC_CS_SYSPLL)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn adc_cc_cs_piosc(self) -> &'a mut W {
        self.variant(ADC_CC_CSW::ADC_CC_CS_PIOSC)
    }
    #[doc = "MOSC"]
    #[inline(always)]
    pub fn adc_cc_cs_mosc(self) -> &'a mut W {
        self.variant(ADC_CC_CSW::ADC_CC_CS_MOSC)
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
pub struct ADC_CC_CLKDIVR {
    bits: u8,
}
impl ADC_CC_CLKDIVR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _ADC_CC_CLKDIVW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_CC_CLKDIVW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(63 << 4);
        self.w.bits |= ((value as u32) & 63) << 4;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - ADC Clock Source"]
    #[inline(always)]
    pub fn adc_cc_cs(&self) -> ADC_CC_CSR {
        ADC_CC_CSR::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bits 4:9 - PLL VCO Clock Divisor"]
    #[inline(always)]
    pub fn adc_cc_clkdiv(&self) -> ADC_CC_CLKDIVR {
        let bits = ((self.bits >> 4) & 63) as u8;
        ADC_CC_CLKDIVR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - ADC Clock Source"]
    #[inline(always)]
    pub fn adc_cc_cs(&mut self) -> _ADC_CC_CSW {
        _ADC_CC_CSW { w: self }
    }
    #[doc = "Bits 4:9 - PLL VCO Clock Divisor"]
    #[inline(always)]
    pub fn adc_cc_clkdiv(&mut self) -> _ADC_CC_CLKDIVW {
        _ADC_CC_CLKDIVW { w: self }
    }
}
