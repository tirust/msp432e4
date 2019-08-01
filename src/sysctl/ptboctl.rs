#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PTBOCTL {
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
#[doc = "Possible values of the field `SYSCTL_PTBOCTL_VDD_UBOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_PTBOCTL_VDD_UBORR {
    #[doc = "No Action"]
    SYSCTL_PTBOCTL_VDD_UBOR_NONE,
    #[doc = "System control interrupt"]
    SYSCTL_PTBOCTL_VDD_UBOR_SYSINT,
    #[doc = "NMI"]
    SYSCTL_PTBOCTL_VDD_UBOR_NMI,
    #[doc = "Reset"]
    SYSCTL_PTBOCTL_VDD_UBOR_RST,
}
impl SYSCTL_PTBOCTL_VDD_UBORR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_NONE => 0,
            SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT => 1,
            SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_NMI => 2,
            SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_RST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_PTBOCTL_VDD_UBORR {
        match value {
            0 => SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_NONE,
            1 => SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT,
            2 => SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_NMI,
            3 => SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_NONE`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_none(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_NONE
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_SYSINT`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_sysint(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_NMI`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_nmi(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_NMI
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDD_UBOR_RST`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdd_ubor_rst(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDD_UBORR::SYSCTL_PTBOCTL_VDD_UBOR_RST
    }
}
#[doc = "Values that can be written to the field `SYSCTL_PTBOCTL_VDD_UBOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_PTBOCTL_VDD_UBORW {
    #[doc = "No Action"]
    SYSCTL_PTBOCTL_VDD_UBOR_NONE,
    #[doc = "System control interrupt"]
    SYSCTL_PTBOCTL_VDD_UBOR_SYSINT,
    #[doc = "NMI"]
    SYSCTL_PTBOCTL_VDD_UBOR_NMI,
    #[doc = "Reset"]
    SYSCTL_PTBOCTL_VDD_UBOR_RST,
}
impl SYSCTL_PTBOCTL_VDD_UBORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_NONE => 0,
            SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT => 1,
            SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_NMI => 2,
            SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_RST => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PTBOCTL_VDD_UBORW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PTBOCTL_VDD_UBORW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_PTBOCTL_VDD_UBORW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_none(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_NONE)
    }
    #[doc = "System control interrupt"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_sysint(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_SYSINT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_nmi(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_NMI)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor_rst(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDD_UBORW::SYSCTL_PTBOCTL_VDD_UBOR_RST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `SYSCTL_PTBOCTL_VDDA_UBOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_PTBOCTL_VDDA_UBORR {
    #[doc = "No Action"]
    SYSCTL_PTBOCTL_VDDA_UBOR_NONE,
    #[doc = "System control interrupt"]
    SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT,
    #[doc = "NMI"]
    SYSCTL_PTBOCTL_VDDA_UBOR_NMI,
    #[doc = "Reset"]
    SYSCTL_PTBOCTL_VDDA_UBOR_RST,
}
impl SYSCTL_PTBOCTL_VDDA_UBORR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_NONE => 0,
            SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT => 1,
            SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_NMI => 2,
            SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_RST => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> SYSCTL_PTBOCTL_VDDA_UBORR {
        match value {
            0 => SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_NONE,
            1 => SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT,
            2 => SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_NMI,
            3 => SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_RST,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_NONE`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_none(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_NONE
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_sysint(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_NMI`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_nmi(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_NMI
    }
    #[doc = "Checks if the value of the field is `SYSCTL_PTBOCTL_VDDA_UBOR_RST`"]
    #[inline(always)]
    pub fn is_sysctl_ptboctl_vdda_ubor_rst(&self) -> bool {
        *self == SYSCTL_PTBOCTL_VDDA_UBORR::SYSCTL_PTBOCTL_VDDA_UBOR_RST
    }
}
#[doc = "Values that can be written to the field `SYSCTL_PTBOCTL_VDDA_UBOR`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum SYSCTL_PTBOCTL_VDDA_UBORW {
    #[doc = "No Action"]
    SYSCTL_PTBOCTL_VDDA_UBOR_NONE,
    #[doc = "System control interrupt"]
    SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT,
    #[doc = "NMI"]
    SYSCTL_PTBOCTL_VDDA_UBOR_NMI,
    #[doc = "Reset"]
    SYSCTL_PTBOCTL_VDDA_UBOR_RST,
}
impl SYSCTL_PTBOCTL_VDDA_UBORW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_NONE => 0,
            SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT => 1,
            SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_NMI => 2,
            SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_RST => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _SYSCTL_PTBOCTL_VDDA_UBORW<'a> {
    w: &'a mut W,
}
impl<'a> _SYSCTL_PTBOCTL_VDDA_UBORW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SYSCTL_PTBOCTL_VDDA_UBORW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_none(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_NONE)
    }
    #[doc = "System control interrupt"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_sysint(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_SYSINT)
    }
    #[doc = "NMI"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_nmi(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_NMI)
    }
    #[doc = "Reset"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor_rst(self) -> &'a mut W {
        self.variant(SYSCTL_PTBOCTL_VDDA_UBORW::SYSCTL_PTBOCTL_VDDA_UBOR_RST)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor(&self) -> SYSCTL_PTBOCTL_VDD_UBORR {
        SYSCTL_PTBOCTL_VDD_UBORR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor(&self) -> SYSCTL_PTBOCTL_VDDA_UBORR {
        SYSCTL_PTBOCTL_VDDA_UBORR::_from(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - VDD (VDDS) under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdd_ubor(&mut self) -> _SYSCTL_PTBOCTL_VDD_UBORW {
        _SYSCTL_PTBOCTL_VDD_UBORW { w: self }
    }
    #[doc = "Bits 8:9 - VDDA under BOR Event Action"]
    #[inline(always)]
    pub fn sysctl_ptboctl_vdda_ubor(&mut self) -> _SYSCTL_PTBOCTL_VDDA_UBORW {
        _SYSCTL_PTBOCTL_VDDA_UBORW { w: self }
    }
}
