#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SLPPWRCFG {
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
#[doc = "Possible values of the field `SYSCTL_SLPPWRCFG_SRAMPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_SLPPWRCFG_SRAMPMR {
    #[doc = "Active Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_NRM,
    #[doc = "Standby Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_SBY,
    #[doc = "Low Power Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_LP,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_SLPPWRCFG_SRAMPMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_NRM => 0,
            SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_SBY => 1,
            SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_LP => 3,
            SYSCTL_SLPPWRCFG_SRAMPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_SLPPWRCFG_SRAMPMR {
        match value {
            0 => SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_NRM,
            1 => SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_SBY,
            3 => SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_LP,
            i => SYSCTL_SLPPWRCFG_SRAMPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_SRAMPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_srampm_nrm(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_SRAMPM_SBY`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_srampm_sby(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_SBY
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_SRAMPM_LP`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_srampm_lp(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_SRAMPMR::SYSCTL_SLPPWRCFG_SRAMPM_LP
    }
}
#[doc = "Values that can be written to the field `SYSCTL_SLPPWRCFG_SRAMPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_SLPPWRCFG_SRAMPMW {
    #[doc = "Active Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_NRM,
    #[doc = "Standby Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_SBY,
    #[doc = "Low Power Mode"]
    SYSCTL_SLPPWRCFG_SRAMPM_LP,
}
impl SYSCTL_SLPPWRCFG_SRAMPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_SLPPWRCFG_SRAMPMW::SYSCTL_SLPPWRCFG_SRAMPM_NRM => 0,
            SYSCTL_SLPPWRCFG_SRAMPMW::SYSCTL_SLPPWRCFG_SRAMPM_SBY => 1,
            SYSCTL_SLPPWRCFG_SRAMPMW::SYSCTL_SLPPWRCFG_SRAMPM_LP => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_SLPPWRCFG_SRAMPMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SLPPWRCFG_SRAMPMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_SLPPWRCFG_SRAMPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_SRAMPMW::SYSCTL_SLPPWRCFG_SRAMPM_NRM)
    }
    #[doc = "Standby Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm_sby(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_SRAMPMW::SYSCTL_SLPPWRCFG_SRAMPM_SBY)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_srampm_lp(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_SRAMPMW::SYSCTL_SLPPWRCFG_SRAMPM_LP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_SLPPWRCFG_FLASHPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_SLPPWRCFG_FLASHPMR {
    #[doc = "Active Mode"]
    SYSCTL_SLPPWRCFG_FLASHPM_NRM,
    #[doc = "Low Power Mode"]
    SYSCTL_SLPPWRCFG_FLASHPM_SLP,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl SYSCTL_SLPPWRCFG_FLASHPMR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_SLPPWRCFG_FLASHPMR::SYSCTL_SLPPWRCFG_FLASHPM_NRM => 0,
            SYSCTL_SLPPWRCFG_FLASHPMR::SYSCTL_SLPPWRCFG_FLASHPM_SLP => 2,
            SYSCTL_SLPPWRCFG_FLASHPMR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_SLPPWRCFG_FLASHPMR {
        match value {
            0 => SYSCTL_SLPPWRCFG_FLASHPMR::SYSCTL_SLPPWRCFG_FLASHPM_NRM,
            2 => SYSCTL_SLPPWRCFG_FLASHPMR::SYSCTL_SLPPWRCFG_FLASHPM_SLP,
            i => SYSCTL_SLPPWRCFG_FLASHPMR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_FLASHPM_NRM`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_flashpm_nrm(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_FLASHPMR::SYSCTL_SLPPWRCFG_FLASHPM_NRM
    }
    #[doc = "Checks if the value of the field is `SYSCTL_SLPPWRCFG_FLASHPM_SLP`"]
    #[inline(always)]
    pub fn is_sysctl_slppwrcfg_flashpm_slp(&self) -> bool {
        *self == SYSCTL_SLPPWRCFG_FLASHPMR::SYSCTL_SLPPWRCFG_FLASHPM_SLP
    }
}
#[doc = "Values that can be written to the field `SYSCTL_SLPPWRCFG_FLASHPM`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_SLPPWRCFG_FLASHPMW {
    #[doc = "Active Mode"]
    SYSCTL_SLPPWRCFG_FLASHPM_NRM,
    #[doc = "Low Power Mode"]
    SYSCTL_SLPPWRCFG_FLASHPM_SLP,
}
impl SYSCTL_SLPPWRCFG_FLASHPMW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_SLPPWRCFG_FLASHPMW::SYSCTL_SLPPWRCFG_FLASHPM_NRM => 0,
            SYSCTL_SLPPWRCFG_FLASHPMW::SYSCTL_SLPPWRCFG_FLASHPM_SLP => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_SLPPWRCFG_FLASHPMW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_SLPPWRCFG_FLASHPMW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_SLPPWRCFG_FLASHPMW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Active Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm_nrm(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_FLASHPMW::SYSCTL_SLPPWRCFG_FLASHPM_NRM)
    }
    #[doc = "Low Power Mode"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm_slp(self) -> &'a mut W {
        self.variant(SYSCTL_SLPPWRCFG_FLASHPMW::SYSCTL_SLPPWRCFG_FLASHPM_SLP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
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
    pub fn sysctl_slppwrcfg_srampm(&self) -> SYSCTL_SLPPWRCFG_SRAMPMR {
        SYSCTL_SLPPWRCFG_SRAMPMR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm(&self) -> SYSCTL_SLPPWRCFG_FLASHPMR {
        SYSCTL_SLPPWRCFG_FLASHPMR::_from(((self.bits >> 4) & 3) as u8)
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
    pub fn sysctl_slppwrcfg_srampm(&mut self) -> _SYSCTL_SLPPWRCFG_SRAMPMW {
        _SYSCTL_SLPPWRCFG_SRAMPMW { w: self }
    }
    #[doc = "Bits 4:5 - Flash Power Modes"]
    #[inline(always)]
    pub fn sysctl_slppwrcfg_flashpm(&mut self) -> _SYSCTL_SLPPWRCFG_FLASHPMW {
        _SYSCTL_SLPPWRCFG_FLASHPMW { w: self }
    }
}
