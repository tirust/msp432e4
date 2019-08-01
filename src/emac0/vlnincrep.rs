#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::VLNINCREP {
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
pub struct EMAC_VLNINCREP_VLTR {
    bits: u16,
}
impl EMAC_VLNINCREP_VLTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_VLNINCREP_VLTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_VLNINCREP_VLTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 0);
        self.w.bits |= ((value as u32) & 65535) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_VLNINCREP_VLC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_VLNINCREP_VLCR {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    EMAC_VLNINCREP_VLC_NONE,
    #[doc = "VLAN tag deletion"]
    EMAC_VLNINCREP_VLC_TAGDEL,
    #[doc = "VLAN tag insertion"]
    EMAC_VLNINCREP_VLC_TAGINS,
    #[doc = "VLAN tag replacement"]
    EMAC_VLNINCREP_VLC_TAGREP,
}
impl EMAC_VLNINCREP_VLCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_NONE => 0,
            EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGDEL => 1,
            EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGINS => 2,
            EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGREP => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_VLNINCREP_VLCR {
        match value {
            0 => EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_NONE,
            1 => EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGDEL,
            2 => EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGINS,
            3 => EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGREP,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_NONE`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_none(&self) -> bool {
        *self == EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_NONE
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_TAGDEL`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_tagdel(&self) -> bool {
        *self == EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGDEL
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_TAGINS`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_tagins(&self) -> bool {
        *self == EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGINS
    }
    #[doc = "Checks if the value of the field is `EMAC_VLNINCREP_VLC_TAGREP`"]
    #[inline(always)]
    pub fn is_emac_vlnincrep_vlc_tagrep(&self) -> bool {
        *self == EMAC_VLNINCREP_VLCR::EMAC_VLNINCREP_VLC_TAGREP
    }
}
#[doc = "Values that can be written to the field `EMAC_VLNINCREP_VLC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_VLNINCREP_VLCW {
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    EMAC_VLNINCREP_VLC_NONE,
    #[doc = "VLAN tag deletion"]
    EMAC_VLNINCREP_VLC_TAGDEL,
    #[doc = "VLAN tag insertion"]
    EMAC_VLNINCREP_VLC_TAGINS,
    #[doc = "VLAN tag replacement"]
    EMAC_VLNINCREP_VLC_TAGREP,
}
impl EMAC_VLNINCREP_VLCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_NONE => 0,
            EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_TAGDEL => 1,
            EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_TAGINS => 2,
            EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_TAGREP => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_VLNINCREP_VLCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_VLNINCREP_VLCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_VLNINCREP_VLCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No VLAN tag deletion, insertion, or replacement"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_none(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_NONE)
    }
    #[doc = "VLAN tag deletion"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_tagdel(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_TAGDEL)
    }
    #[doc = "VLAN tag insertion"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_tagins(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_TAGINS)
    }
    #[doc = "VLAN tag replacement"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc_tagrep(self) -> &'a mut W {
        self.variant(EMAC_VLNINCREP_VLCW::EMAC_VLNINCREP_VLC_TAGREP)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 16);
        self.w.bits |= ((value as u32) & 3) << 16;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_VLNINCREP_VLPR {
    bits: bool,
}
impl EMAC_VLNINCREP_VLPR {
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
pub struct _EMAC_VLNINCREP_VLPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_VLNINCREP_VLPW<'a> {
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
        self.w.bits &= !(1 << 18);
        self.w.bits |= ((value as u32) & 1) << 18;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_VLNINCREP_CSVLR {
    bits: bool,
}
impl EMAC_VLNINCREP_CSVLR {
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
pub struct _EMAC_VLNINCREP_CSVLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_VLNINCREP_CSVLW<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlt(&self) -> EMAC_VLNINCREP_VLTR {
        let bits = ((self.bits >> 0) & 65535) as u16;
        EMAC_VLNINCREP_VLTR { bits }
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc(&self) -> EMAC_VLNINCREP_VLCR {
        EMAC_VLNINCREP_VLCR::_from(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlp(&self) -> EMAC_VLNINCREP_VLPR {
        let bits = ((self.bits >> 18) & 1) != 0;
        EMAC_VLNINCREP_VLPR { bits }
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn emac_vlnincrep_csvl(&self) -> EMAC_VLNINCREP_CSVLR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EMAC_VLNINCREP_CSVLR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:15 - VLAN Tag for Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlt(&mut self) -> _EMAC_VLNINCREP_VLTW {
        _EMAC_VLNINCREP_VLTW { w: self }
    }
    #[doc = "Bits 16:17 - VLAN Tag Control in Transmit Frames"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlc(&mut self) -> _EMAC_VLNINCREP_VLCW {
        _EMAC_VLNINCREP_VLCW { w: self }
    }
    #[doc = "Bit 18 - VLAN Priority Control"]
    #[inline(always)]
    pub fn emac_vlnincrep_vlp(&mut self) -> _EMAC_VLNINCREP_VLPW {
        _EMAC_VLNINCREP_VLPW { w: self }
    }
    #[doc = "Bit 19 - C-VLAN or S-VLAN"]
    #[inline(always)]
    pub fn emac_vlnincrep_csvl(&mut self) -> _EMAC_VLNINCREP_CSVLW {
        _EMAC_VLNINCREP_CSVLW { w: self }
    }
}
