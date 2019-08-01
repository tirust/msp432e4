#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FLOWCTL {
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
pub struct EMAC_FLOWCTL_FCBBPAR {
    bits: bool,
}
impl EMAC_FLOWCTL_FCBBPAR {
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
pub struct _EMAC_FLOWCTL_FCBBPAW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FLOWCTL_FCBBPAW<'a> {
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
        self.w.bits &= !(1 << 0);
        self.w.bits |= ((value as u32) & 1) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_FLOWCTL_TFER {
    bits: bool,
}
impl EMAC_FLOWCTL_TFER {
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
pub struct _EMAC_FLOWCTL_TFEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FLOWCTL_TFEW<'a> {
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
        self.w.bits &= !(1 << 1);
        self.w.bits |= ((value as u32) & 1) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_FLOWCTL_RFER {
    bits: bool,
}
impl EMAC_FLOWCTL_RFER {
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
pub struct _EMAC_FLOWCTL_RFEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FLOWCTL_RFEW<'a> {
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
        self.w.bits &= !(1 << 2);
        self.w.bits |= ((value as u32) & 1) << 2;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_FLOWCTL_UPR {
    bits: bool,
}
impl EMAC_FLOWCTL_UPR {
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
pub struct _EMAC_FLOWCTL_UPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FLOWCTL_UPW<'a> {
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
        self.w.bits &= !(1 << 3);
        self.w.bits |= ((value as u32) & 1) << 3;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_FLOWCTL_PLT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_FLOWCTL_PLTR {
    #[doc = "The threshold is Pause time minus 4 slot times (PT - 4 slot times)"]
    EMAC_FLOWCTL_PLT_4,
    #[doc = "The threshold is Pause time minus 28 slot times (PT - 28 slot times)"]
    EMAC_FLOWCTL_PLT_28,
    #[doc = "The threshold is Pause time minus 144 slot times (PT - 144 slot times)"]
    EMAC_FLOWCTL_PLT_144,
    #[doc = "The threshold is Pause time minus 256 slot times (PT - 256 slot times)"]
    EMAC_FLOWCTL_PLT_156,
}
impl EMAC_FLOWCTL_PLTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_4 => 0,
            EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_28 => 1,
            EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_144 => 2,
            EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_156 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_FLOWCTL_PLTR {
        match value {
            0 => EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_4,
            1 => EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_28,
            2 => EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_144,
            3 => EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_156,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_4`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_4(&self) -> bool {
        *self == EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_4
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_28`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_28(&self) -> bool {
        *self == EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_28
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_144`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_144(&self) -> bool {
        *self == EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_144
    }
    #[doc = "Checks if the value of the field is `EMAC_FLOWCTL_PLT_156`"]
    #[inline(always)]
    pub fn is_emac_flowctl_plt_156(&self) -> bool {
        *self == EMAC_FLOWCTL_PLTR::EMAC_FLOWCTL_PLT_156
    }
}
#[doc = "Values that can be written to the field `EMAC_FLOWCTL_PLT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_FLOWCTL_PLTW {
    #[doc = "The threshold is Pause time minus 4 slot times (PT - 4 slot times)"]
    EMAC_FLOWCTL_PLT_4,
    #[doc = "The threshold is Pause time minus 28 slot times (PT - 28 slot times)"]
    EMAC_FLOWCTL_PLT_28,
    #[doc = "The threshold is Pause time minus 144 slot times (PT - 144 slot times)"]
    EMAC_FLOWCTL_PLT_144,
    #[doc = "The threshold is Pause time minus 256 slot times (PT - 256 slot times)"]
    EMAC_FLOWCTL_PLT_156,
}
impl EMAC_FLOWCTL_PLTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_4 => 0,
            EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_28 => 1,
            EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_144 => 2,
            EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_156 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_FLOWCTL_PLTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FLOWCTL_PLTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_FLOWCTL_PLTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "The threshold is Pause time minus 4 slot times (PT - 4 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_4(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_4)
    }
    #[doc = "The threshold is Pause time minus 28 slot times (PT - 28 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_28(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_28)
    }
    #[doc = "The threshold is Pause time minus 144 slot times (PT - 144 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_144(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_144)
    }
    #[doc = "The threshold is Pause time minus 256 slot times (PT - 256 slot times)"]
    #[inline(always)]
    pub fn emac_flowctl_plt_156(self) -> &'a mut W {
        self.variant(EMAC_FLOWCTL_PLTW::EMAC_FLOWCTL_PLT_156)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 4);
        self.w.bits |= ((value as u32) & 3) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_FLOWCTL_DZQPR {
    bits: bool,
}
impl EMAC_FLOWCTL_DZQPR {
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
pub struct _EMAC_FLOWCTL_DZQPW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FLOWCTL_DZQPW<'a> {
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
        self.w.bits &= !(1 << 7);
        self.w.bits |= ((value as u32) & 1) << 7;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_FLOWCTL_PTR {
    bits: u16,
}
impl EMAC_FLOWCTL_PTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u16 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_FLOWCTL_PTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_FLOWCTL_PTW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits &= !(65535 << 16);
        self.w.bits |= ((value as u32) & 65535) << 16;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Flow Control Busy or Back-pressure Activate"]
    #[inline(always)]
    pub fn emac_flowctl_fcbbpa(&self) -> EMAC_FLOWCTL_FCBBPAR {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_FLOWCTL_FCBBPAR { bits }
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_tfe(&self) -> EMAC_FLOWCTL_TFER {
        let bits = ((self.bits >> 1) & 1) != 0;
        EMAC_FLOWCTL_TFER { bits }
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_rfe(&self) -> EMAC_FLOWCTL_RFER {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_FLOWCTL_RFER { bits }
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn emac_flowctl_up(&self) -> EMAC_FLOWCTL_UPR {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_FLOWCTL_UPR { bits }
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn emac_flowctl_plt(&self) -> EMAC_FLOWCTL_PLTR {
        EMAC_FLOWCTL_PLTR::_from(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn emac_flowctl_dzqp(&self) -> EMAC_FLOWCTL_DZQPR {
        let bits = ((self.bits >> 7) & 1) != 0;
        EMAC_FLOWCTL_DZQPR { bits }
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn emac_flowctl_pt(&self) -> EMAC_FLOWCTL_PTR {
        let bits = ((self.bits >> 16) & 65535) as u16;
        EMAC_FLOWCTL_PTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Flow Control Busy or Back-pressure Activate"]
    #[inline(always)]
    pub fn emac_flowctl_fcbbpa(&mut self) -> _EMAC_FLOWCTL_FCBBPAW {
        _EMAC_FLOWCTL_FCBBPAW { w: self }
    }
    #[doc = "Bit 1 - Transmit Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_tfe(&mut self) -> _EMAC_FLOWCTL_TFEW {
        _EMAC_FLOWCTL_TFEW { w: self }
    }
    #[doc = "Bit 2 - Receive Flow Control Enable"]
    #[inline(always)]
    pub fn emac_flowctl_rfe(&mut self) -> _EMAC_FLOWCTL_RFEW {
        _EMAC_FLOWCTL_RFEW { w: self }
    }
    #[doc = "Bit 3 - Unicast Pause Frame Detect"]
    #[inline(always)]
    pub fn emac_flowctl_up(&mut self) -> _EMAC_FLOWCTL_UPW {
        _EMAC_FLOWCTL_UPW { w: self }
    }
    #[doc = "Bits 4:5 - Pause Low Threshold"]
    #[inline(always)]
    pub fn emac_flowctl_plt(&mut self) -> _EMAC_FLOWCTL_PLTW {
        _EMAC_FLOWCTL_PLTW { w: self }
    }
    #[doc = "Bit 7 - Disable Zero-Quanta Pause"]
    #[inline(always)]
    pub fn emac_flowctl_dzqp(&mut self) -> _EMAC_FLOWCTL_DZQPW {
        _EMAC_FLOWCTL_DZQPW { w: self }
    }
    #[doc = "Bits 16:31 - Pause Time"]
    #[inline(always)]
    pub fn emac_flowctl_pt(&mut self) -> _EMAC_FLOWCTL_PTW {
        _EMAC_FLOWCTL_PTW { w: self }
    }
}
