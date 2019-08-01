#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PC {
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
#[doc = "Possible values of the field `ADC_PC_MCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PC_MCRR {
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_8,
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_4,
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_2,
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    ADC_PC_MCR_FULL,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl ADC_PC_MCRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            ADC_PC_MCRR::ADC_PC_MCR_1_8 => 1,
            ADC_PC_MCRR::ADC_PC_MCR_1_4 => 3,
            ADC_PC_MCRR::ADC_PC_MCR_1_2 => 5,
            ADC_PC_MCRR::ADC_PC_MCR_FULL => 7,
            ADC_PC_MCRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> ADC_PC_MCRR {
        match value {
            1 => ADC_PC_MCRR::ADC_PC_MCR_1_8,
            3 => ADC_PC_MCRR::ADC_PC_MCR_1_4,
            5 => ADC_PC_MCRR::ADC_PC_MCR_1_2,
            7 => ADC_PC_MCRR::ADC_PC_MCR_FULL,
            i => ADC_PC_MCRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_1_8`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_1_8(&self) -> bool {
        *self == ADC_PC_MCRR::ADC_PC_MCR_1_8
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_1_4`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_1_4(&self) -> bool {
        *self == ADC_PC_MCRR::ADC_PC_MCR_1_4
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_1_2`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_1_2(&self) -> bool {
        *self == ADC_PC_MCRR::ADC_PC_MCR_1_2
    }
    #[doc = "Checks if the value of the field is `ADC_PC_MCR_FULL`"]
    #[inline(always)]
    pub fn is_adc_pc_mcr_full(&self) -> bool {
        *self == ADC_PC_MCRR::ADC_PC_MCR_FULL
    }
}
#[doc = "Values that can be written to the field `ADC_PC_MCR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ADC_PC_MCRW {
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_8,
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_4,
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    ADC_PC_MCR_1_2,
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    ADC_PC_MCR_FULL,
}
impl ADC_PC_MCRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            ADC_PC_MCRW::ADC_PC_MCR_1_8 => 1,
            ADC_PC_MCRW::ADC_PC_MCR_1_4 => 3,
            ADC_PC_MCRW::ADC_PC_MCR_1_2 => 5,
            ADC_PC_MCRW::ADC_PC_MCR_FULL => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _ADC_PC_MCRW<'a> {
    w: &'a mut W,
}
impl<'a> _ADC_PC_MCRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: ADC_PC_MCRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Eighth conversion rate. After a conversion completes, the logic pauses for 112 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn adc_pc_mcr_1_8(self) -> &'a mut W {
        self.variant(ADC_PC_MCRW::ADC_PC_MCR_1_8)
    }
    #[doc = "Quarter conversion rate. After a conversion completes, the logic pauses for 48 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn adc_pc_mcr_1_4(self) -> &'a mut W {
        self.variant(ADC_PC_MCRW::ADC_PC_MCR_1_4)
    }
    #[doc = "Half conversion rate. After a conversion completes, the logic pauses for 16 TADC periods before starting the next conversion"]
    #[inline(always)]
    pub fn adc_pc_mcr_1_2(self) -> &'a mut W {
        self.variant(ADC_PC_MCRW::ADC_PC_MCR_1_2)
    }
    #[doc = "Full conversion rate (FCONV) as defined by TADC and NSH"]
    #[inline(always)]
    pub fn adc_pc_mcr_full(self) -> &'a mut W {
        self.variant(ADC_PC_MCRW::ADC_PC_MCR_FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
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
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline(always)]
    pub fn adc_pc_mcr(&self) -> ADC_PC_MCRR {
        ADC_PC_MCRR::_from(((self.bits >> 0) & 15) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Conversion Rate"]
    #[inline(always)]
    pub fn adc_pc_mcr(&mut self) -> _ADC_PC_MCRW {
        _ADC_PC_MCRW { w: self }
    }
}
