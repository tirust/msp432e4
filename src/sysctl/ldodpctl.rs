#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::LDODPCTL {
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
#[doc = "Possible values of the field `SYSCTL_LDODPCTL_VLDO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_LDODPCTL_VLDOR {
    #[doc = "0.90 V"]
    SYSCTL_LDODPCTL_VLDO_0_90V,
    #[doc = "0.95 V"]
    SYSCTL_LDODPCTL_VLDO_0_95V,
    #[doc = "1.00 V"]
    SYSCTL_LDODPCTL_VLDO_1_00V,
    #[doc = "1.05 V"]
    SYSCTL_LDODPCTL_VLDO_1_05V,
    #[doc = "1.10 V"]
    SYSCTL_LDODPCTL_VLDO_1_10V,
    #[doc = "1.15 V"]
    SYSCTL_LDODPCTL_VLDO_1_15V,
    #[doc = "1.20 V"]
    SYSCTL_LDODPCTL_VLDO_1_20V,
    #[doc = "1.25 V"]
    SYSCTL_LDODPCTL_VLDO_1_25V,
    #[doc = "1.30 V"]
    SYSCTL_LDODPCTL_VLDO_1_30V,
    #[doc = "1.35 V"]
    SYSCTL_LDODPCTL_VLDO_1_35V,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_LDODPCTL_VLDOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_0_90V => 18,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_0_95V => 19,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_00V => 20,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_05V => 21,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_10V => 22,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_15V => 23,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_20V => 24,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_25V => 25,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_30V => 26,
            SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_35V => 27,
            SYSCTL_LDODPCTL_VLDOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_LDODPCTL_VLDOR {
        match value {
            18 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_0_90V,
            19 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_0_95V,
            20 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_00V,
            21 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_05V,
            22 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_10V,
            23 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_15V,
            24 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_20V,
            25 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_25V,
            26 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_30V,
            27 => SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_35V,
            i => SYSCTL_LDODPCTL_VLDOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_0_90V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_0_90v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_0_90V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_0_95V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_0_95v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_0_95V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_00V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_00v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_00V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_05V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_05v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_05V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_10V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_10v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_10V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_15V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_15v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_15V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_20V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_20v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_20V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_25V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_25v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_25V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_30V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_30v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_30V
    }
    #[doc = "Checks if the value of the field is `SYSCTL_LDODPCTL_VLDO_1_35V`"]
    #[inline(always)]
    pub fn is_sysctl_ldodpctl_vldo_1_35v(&self) -> bool {
        *self == SYSCTL_LDODPCTL_VLDOR::SYSCTL_LDODPCTL_VLDO_1_35V
    }
}
#[doc = "Values that can be written to the field `SYSCTL_LDODPCTL_VLDO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_LDODPCTL_VLDOW {
    #[doc = "0.90 V"]
    SYSCTL_LDODPCTL_VLDO_0_90V,
    #[doc = "0.95 V"]
    SYSCTL_LDODPCTL_VLDO_0_95V,
    #[doc = "1.00 V"]
    SYSCTL_LDODPCTL_VLDO_1_00V,
    #[doc = "1.05 V"]
    SYSCTL_LDODPCTL_VLDO_1_05V,
    #[doc = "1.10 V"]
    SYSCTL_LDODPCTL_VLDO_1_10V,
    #[doc = "1.15 V"]
    SYSCTL_LDODPCTL_VLDO_1_15V,
    #[doc = "1.20 V"]
    SYSCTL_LDODPCTL_VLDO_1_20V,
    #[doc = "1.25 V"]
    SYSCTL_LDODPCTL_VLDO_1_25V,
    #[doc = "1.30 V"]
    SYSCTL_LDODPCTL_VLDO_1_30V,
    #[doc = "1.35 V"]
    SYSCTL_LDODPCTL_VLDO_1_35V,
}
impl SYSCTL_LDODPCTL_VLDOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_0_90V => 18,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_0_95V => 19,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_00V => 20,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_05V => 21,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_10V => 22,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_15V => 23,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_20V => 24,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_25V => 25,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_30V => 26,
            SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_35V => 27,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_LDODPCTL_VLDOW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_LDODPCTL_VLDOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_LDODPCTL_VLDOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "0.90 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_0_90v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_0_90V)
    }
    #[doc = "0.95 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_0_95v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_0_95V)
    }
    #[doc = "1.00 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_00v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_00V)
    }
    #[doc = "1.05 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_05v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_05V)
    }
    #[doc = "1.10 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_10v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_10V)
    }
    #[doc = "1.15 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_15v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_15V)
    }
    #[doc = "1.20 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_20v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_20V)
    }
    #[doc = "1.25 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_25v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_25V)
    }
    #[doc = "1.30 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_30v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_30V)
    }
    #[doc = "1.35 V"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo_1_35v(self) -> &'a mut W {
        self.variant(SYSCTL_LDODPCTL_VLDOW::SYSCTL_LDODPCTL_VLDO_1_35V)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(255 << 0);
        self.w.bits |= ((value as u32) & 255) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_LDODPCTL_VADJENR {
    bits: bool,
}
impl SYSCTL_LDODPCTL_VADJENR {
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
pub struct _SYSCTL_LDODPCTL_VADJENW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_LDODPCTL_VADJENW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo(&self) -> SYSCTL_LDODPCTL_VLDOR {
        SYSCTL_LDODPCTL_VLDOR::_from(((self.bits >> 0) & 255) as u8)
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vadjen(&self) -> SYSCTL_LDODPCTL_VADJENR {
        let bits = ((self.bits >> 31) & 1) != 0;
        SYSCTL_LDODPCTL_VADJENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - LDO Output Voltage"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vldo(&mut self) -> _SYSCTL_LDODPCTL_VLDOW {
        _SYSCTL_LDODPCTL_VLDOW { w: self }
    }
    #[doc = "Bit 31 - Voltage Adjust Enable"]
    #[inline(always)]
    pub fn sysctl_ldodpctl_vadjen(&mut self) -> _SYSCTL_LDODPCTL_VADJENW {
        _SYSCTL_LDODPCTL_VADJENW { w: self }
    }
}
