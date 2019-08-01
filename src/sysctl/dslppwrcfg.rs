#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::DSLPPWRCFG {
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
#[doc = "Possible values of the field `SYSCTL_DSLPPWRCFG_SRAMPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DSLPPWRCFG_SRAMPMR {
    #[doc = "Active Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_NRM,
    #[doc = "Standby Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_SBY,
    #[doc = "Low Power Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_LP,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DSLPPWRCFG_SRAMPMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_NRM => 0,
            SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_SBY => 1,
            SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_LP => 3,
            SYSCTL_DSLPPWRCFG_SRAMPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DSLPPWRCFG_SRAMPMR {
        match value {
            0 => SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_NRM,
            1 => SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_SBY,
            3 => SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_LP,
            i => SYSCTL_DSLPPWRCFG_SRAMPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_SRAMPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_srampm_nrm(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_SRAMPM_SBY`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_srampm_sby(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_SBY
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_SRAMPM_LP`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_srampm_lp(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_SRAMPMR::SYSCTL_DSLPPWRCFG_SRAMPM_LP
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DSLPPWRCFG_SRAMPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DSLPPWRCFG_SRAMPMW {
    #[doc = "Active Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_NRM,
    #[doc = "Standby Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_SBY,
    #[doc = "Low Power Mode"]
    SYSCTL_DSLPPWRCFG_SRAMPM_LP,
}
impl SYSCTL_DSLPPWRCFG_SRAMPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DSLPPWRCFG_SRAMPMW::SYSCTL_DSLPPWRCFG_SRAMPM_NRM => 0,
            SYSCTL_DSLPPWRCFG_SRAMPMW::SYSCTL_DSLPPWRCFG_SRAMPM_SBY => 1,
            SYSCTL_DSLPPWRCFG_SRAMPMW::SYSCTL_DSLPPWRCFG_SRAMPM_LP => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DSLPPWRCFG_SRAMPMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSLPPWRCFG_SRAMPMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DSLPPWRCFG_SRAMPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_SRAMPMW::SYSCTL_DSLPPWRCFG_SRAMPM_NRM)
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm_sby(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_SRAMPMW::SYSCTL_DSLPPWRCFG_SRAMPM_SBY)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm_lp(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_SRAMPMW::SYSCTL_DSLPPWRCFG_SRAMPM_LP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_DSLPPWRCFG_FLASHPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DSLPPWRCFG_FLASHPMR {
    #[doc = "Active Mode"]
    SYSCTL_DSLPPWRCFG_FLASHPM_NRM,
    #[doc = "Low Power Mode"]
    SYSCTL_DSLPPWRCFG_FLASHPM_SLP,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_DSLPPWRCFG_FLASHPMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_DSLPPWRCFG_FLASHPMR::SYSCTL_DSLPPWRCFG_FLASHPM_NRM => 0,
            SYSCTL_DSLPPWRCFG_FLASHPMR::SYSCTL_DSLPPWRCFG_FLASHPM_SLP => 2,
            SYSCTL_DSLPPWRCFG_FLASHPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_DSLPPWRCFG_FLASHPMR {
        match value {
            0 => SYSCTL_DSLPPWRCFG_FLASHPMR::SYSCTL_DSLPPWRCFG_FLASHPM_NRM,
            2 => SYSCTL_DSLPPWRCFG_FLASHPMR::SYSCTL_DSLPPWRCFG_FLASHPM_SLP,
            i => SYSCTL_DSLPPWRCFG_FLASHPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_FLASHPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_flashpm_nrm(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_FLASHPMR::SYSCTL_DSLPPWRCFG_FLASHPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_DSLPPWRCFG_FLASHPM_SLP`"]
    #[inline(always)]
    pub fn is_sysctl_dslppwrcfg_flashpm_slp(&self) -> bool {
        *self == SYSCTL_DSLPPWRCFG_FLASHPMR::SYSCTL_DSLPPWRCFG_FLASHPM_SLP
    }
}
#[doc = "Values that can be written to the field `SYSCTL_DSLPPWRCFG_FLASHPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_DSLPPWRCFG_FLASHPMW {
    #[doc = "Active Mode"]
    SYSCTL_DSLPPWRCFG_FLASHPM_NRM,
    #[doc = "Low Power Mode"]
    SYSCTL_DSLPPWRCFG_FLASHPM_SLP,
}
impl SYSCTL_DSLPPWRCFG_FLASHPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_DSLPPWRCFG_FLASHPMW::SYSCTL_DSLPPWRCFG_FLASHPM_NRM => 0,
            SYSCTL_DSLPPWRCFG_FLASHPMW::SYSCTL_DSLPPWRCFG_FLASHPM_SLP => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_DSLPPWRCFG_FLASHPMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSLPPWRCFG_FLASHPMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_DSLPPWRCFG_FLASHPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_FLASHPMW::SYSCTL_DSLPPWRCFG_FLASHPM_NRM)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm_slp(self) -> &'a mut W {
        self.variant(SYSCTL_DSLPPWRCFG_FLASHPMW::SYSCTL_DSLPPWRCFG_FLASHPM_SLP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DSLPPWRCFG_TSPDR {
    bits: bool,
}
impl SYSCTL_DSLPPWRCFG_TSPDR {
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
pub struct _SYSCTL_DSLPPWRCFG_TSPDW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSLPPWRCFG_TSPDW<'a> {
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
        self.w.bits &= !(1 << 8);
        self.w.bits |= ((value as u32) & 1) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct SYSCTL_DSLPPWRCFG_LDOSMR {
    bits: bool,
}
impl SYSCTL_DSLPPWRCFG_LDOSMR {
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
pub struct _SYSCTL_DSLPPWRCFG_LDOSMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_DSLPPWRCFG_LDOSMW<'a> {
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
        self.w.bits &= !(1 << 9);
        self.w.bits |= ((value as u32) & 1) << 9;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm(&self) -> SYSCTL_DSLPPWRCFG_SRAMPMR {
        SYSCTL_DSLPPWRCFG_SRAMPMR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm(&self) -> SYSCTL_DSLPPWRCFG_FLASHPMR {
        SYSCTL_DSLPPWRCFG_FLASHPMR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - Temperature Sense Power Down"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_tspd(&self) -> SYSCTL_DSLPPWRCFG_TSPDR {
        let bits = ((self.bits >> 8) & 1) != 0;
        SYSCTL_DSLPPWRCFG_TSPDR { bits }
    }
    #[doc = "Bit 9 - LDO Sleep Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_ldosm(&self) -> SYSCTL_DSLPPWRCFG_LDOSMR {
        let bits = ((self.bits >> 9) & 1) != 0;
        SYSCTL_DSLPPWRCFG_LDOSMR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - SRAM Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_srampm(&mut self) -> _SYSCTL_DSLPPWRCFG_SRAMPMW {
        _SYSCTL_DSLPPWRCFG_SRAMPMW { w: self }
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_flashpm(&mut self) -> _SYSCTL_DSLPPWRCFG_FLASHPMW {
        _SYSCTL_DSLPPWRCFG_FLASHPMW { w: self }
    }
    #[doc = "Bit 8 - Temperature Sense Power Down"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_tspd(&mut self) -> _SYSCTL_DSLPPWRCFG_TSPDW {
        _SYSCTL_DSLPPWRCFG_TSPDW { w: self }
    }
    #[doc = "Bit 9 - LDO Sleep Mode"]
    #[inline(always)]
    pub fn sysctl_dslppwrcfg_ldosm(&mut self) -> _SYSCTL_DSLPPWRCFG_LDOSMW {
        _SYSCTL_DSLPPWRCFG_LDOSMW { w: self }
    }
}
