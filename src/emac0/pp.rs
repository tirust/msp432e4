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
#[doc = "Possible values of the field `EMAC_PP_PHYTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PP_PHYTYPER {
    #[doc = "No PHY"]
    EMAC_PP_PHYTYPE_NONE,
    #[doc = "Snowflake class PHY"]
    EMAC_PP_PHYTYPE_1,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_PP_PHYTYPER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_PP_PHYTYPER::EMAC_PP_PHYTYPE_NONE => 0,
            EMAC_PP_PHYTYPER::EMAC_PP_PHYTYPE_1 => 3,
            EMAC_PP_PHYTYPER::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_PP_PHYTYPER {
        match value {
            0 => EMAC_PP_PHYTYPER::EMAC_PP_PHYTYPE_NONE,
            3 => EMAC_PP_PHYTYPER::EMAC_PP_PHYTYPE_1,
            i => EMAC_PP_PHYTYPER::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PP_PHYTYPE_NONE`"]
    #[inline(always)]
    pub fn is_emac_pp_phytype_none(&self) -> bool {
        *self == EMAC_PP_PHYTYPER::EMAC_PP_PHYTYPE_NONE
    }
    #[doc = "Checks if the value of the field is `EMAC_PP_PHYTYPE_1`"]
    #[inline(always)]
    pub fn is_emac_pp_phytype_1(&self) -> bool {
        *self == EMAC_PP_PHYTYPER::EMAC_PP_PHYTYPE_1
    }
}
#[doc = "Values that can be written to the field `EMAC_PP_PHYTYPE`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PP_PHYTYPEW {
    #[doc = "No PHY"]
    EMAC_PP_PHYTYPE_NONE,
    #[doc = "Snowflake class PHY"]
    EMAC_PP_PHYTYPE_1,
}
impl EMAC_PP_PHYTYPEW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_PP_PHYTYPEW::EMAC_PP_PHYTYPE_NONE => 0,
            EMAC_PP_PHYTYPEW::EMAC_PP_PHYTYPE_1 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PP_PHYTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PP_PHYTYPEW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_PP_PHYTYPEW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "No PHY"]
    #[inline(always)]
    pub fn emac_pp_phytype_none(self) -> &'a mut W {
        self.variant(EMAC_PP_PHYTYPEW::EMAC_PP_PHYTYPE_NONE)
    }
    #[doc = "Snowflake class PHY"]
    #[inline(always)]
    pub fn emac_pp_phytype_1(self) -> &'a mut W {
        self.variant(EMAC_PP_PHYTYPEW::EMAC_PP_PHYTYPE_1)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PP_MACTYPER {
    bits: u8,
}
impl EMAC_PP_MACTYPER {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PP_MACTYPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PP_MACTYPEW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 8);
        self.w.bits |= ((value as u32) & 7) << 8;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:2 - Ethernet PHY Type"]
    #[inline(always)]
    pub fn emac_pp_phytype(&self) -> EMAC_PP_PHYTYPER {
        EMAC_PP_PHYTYPER::_from(((self.bits >> 0) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Ethernet MAC Type"]
    #[inline(always)]
    pub fn emac_pp_mactype(&self) -> EMAC_PP_MACTYPER {
        let bits = ((self.bits >> 8) & 7) as u8;
        EMAC_PP_MACTYPER { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Ethernet PHY Type"]
    #[inline(always)]
    pub fn emac_pp_phytype(&mut self) -> _EMAC_PP_PHYTYPEW {
        _EMAC_PP_PHYTYPEW { w: self }
    }
    #[doc = "Bits 8:10 - Ethernet MAC Type"]
    #[inline(always)]
    pub fn emac_pp_mactype(&mut self) -> _EMAC_PP_MACTYPEW {
        _EMAC_PP_MACTYPEW { w: self }
    }
}
