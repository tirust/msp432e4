#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PIOSCSTAT {
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
#[doc = r"Value of the field"]
pub struct SYSCTL_PIOSCSTAT_CTR {
    bits: u8,
}
impl SYSCTL_PIOSCSTAT_CTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PIOSCSTAT_CTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PIOSCSTAT_CTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(127 << 0);
        self.w.bits |= ((value as u32) & 127) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_PIOSCSTAT_CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_PIOSCSTAT_CRR {
    #[doc = "Calibration has not been attempted"]
    SYSCTL_PIOSCSTAT_CRNONE,
    #[doc = "The last calibration operation completed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRPASS,
    #[doc = "The last calibration operation failed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRFAIL,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_PIOSCSTAT_CRR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRNONE => 0,
            SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRPASS => 1,
            SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRFAIL => 2,
            SYSCTL_PIOSCSTAT_CRR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_PIOSCSTAT_CRR {
        match value {
            0 => SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRNONE,
            1 => SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRPASS,
            2 => SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRFAIL,
            i => SYSCTL_PIOSCSTAT_CRR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PIOSCSTAT_CRNONE`"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crnone(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRNONE
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PIOSCSTAT_CRPASS`"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crpass(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRPASS
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PIOSCSTAT_CRFAIL`"]
    #[inline(always)]
    pub fn is_sysctl_pioscstat_crfail(&self) -> bool {
        *self == SYSCTL_PIOSCSTAT_CRR::SYSCTL_PIOSCSTAT_CRFAIL
    }
}
#[doc = "Values that can be written to the field `SYSCTL_PIOSCSTAT_CR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_PIOSCSTAT_CRW {
    #[doc = "Calibration has not been attempted"]
    SYSCTL_PIOSCSTAT_CRNONE,
    #[doc = "The last calibration operation completed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRPASS,
    #[doc = "The last calibration operation failed to meet 1% accuracy"]
    SYSCTL_PIOSCSTAT_CRFAIL,
}
impl SYSCTL_PIOSCSTAT_CRW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_PIOSCSTAT_CRW::SYSCTL_PIOSCSTAT_CRNONE => 0,
            SYSCTL_PIOSCSTAT_CRW::SYSCTL_PIOSCSTAT_CRPASS => 1,
            SYSCTL_PIOSCSTAT_CRW::SYSCTL_PIOSCSTAT_CRFAIL => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PIOSCSTAT_CRW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PIOSCSTAT_CRW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_PIOSCSTAT_CRW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Calibration has not been attempted"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crnone(self) -> &'a mut W {
        self.variant(SYSCTL_PIOSCSTAT_CRW::SYSCTL_PIOSCSTAT_CRNONE)
    }
    #[doc = "The last calibration operation completed to meet 1% accuracy"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crpass(self) -> &'a mut W {
        self.variant(SYSCTL_PIOSCSTAT_CRW::SYSCTL_PIOSCSTAT_CRPASS)
    }
    #[doc = "The last calibration operation failed to meet 1% accuracy"]
    #[inline(always)]
    pub fn sysctl_pioscstat_crfail(self) -> &'a mut W {
        self.variant(SYSCTL_PIOSCSTAT_CRW::SYSCTL_PIOSCSTAT_CRFAIL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_PIOSCSTAT_DTR {
    bits: u8,
}
impl SYSCTL_PIOSCSTAT_DTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PIOSCSTAT_DTW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PIOSCSTAT_DTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(127 << 16);
        self.w.bits |= ((value as u32) & 127) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_ct(&self) -> SYSCTL_PIOSCSTAT_CTR {
        let bits = ((self.bits >> 0) & 127) as u8;
        SYSCTL_PIOSCSTAT_CTR { bits }
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline(always)]
    pub fn sysctl_pioscstat_cr(&self) -> SYSCTL_PIOSCSTAT_CRR {
        SYSCTL_PIOSCSTAT_CRR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_dt(&self) -> SYSCTL_PIOSCSTAT_DTR {
        let bits = ((self.bits >> 16) & 127) as u8;
        SYSCTL_PIOSCSTAT_DTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:6 - Calibration Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_ct(&mut self) -> _SYSCTL_PIOSCSTAT_CTW {
        _SYSCTL_PIOSCSTAT_CTW { w: self }
    }
    #[doc = "Bits 8:9 - Calibration Result"]
    #[inline(always)]
    pub fn sysctl_pioscstat_cr(&mut self) -> _SYSCTL_PIOSCSTAT_CRW {
        _SYSCTL_PIOSCSTAT_CRW { w: self }
    }
    #[doc = "Bits 16:22 - Default Trim Value"]
    #[inline(always)]
    pub fn sysctl_pioscstat_dt(&mut self) -> _SYSCTL_PIOSCSTAT_DTW {
        _SYSCTL_PIOSCSTAT_DTW { w: self }
    }
}
