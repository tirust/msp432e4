#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CFG {
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
#[doc = "Possible values of the field `EMAC_CFG_PRELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_CFG_PRELENR {
    #[doc = "7 bytes of preamble"]
    EMAC_CFG_PRELEN_7,
    #[doc = "5 bytes of preamble"]
    EMAC_CFG_PRELEN_5,
    #[doc = "3 bytes of preamble"]
    EMAC_CFG_PRELEN_3,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_CFG_PRELENR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_7 => 0,
            EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_5 => 1,
            EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_3 => 2,
            EMAC_CFG_PRELENR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_CFG_PRELENR {
        match value {
            0 => EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_7,
            1 => EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_5,
            2 => EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_3,
            i => EMAC_CFG_PRELENR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_PRELEN_7`"]
    #[inline(always)]
    pub fn is_emac_cfg_prelen_7(&self) -> bool {
        *self == EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_7
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_PRELEN_5`"]
    #[inline(always)]
    pub fn is_emac_cfg_prelen_5(&self) -> bool {
        *self == EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_5
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_PRELEN_3`"]
    #[inline(always)]
    pub fn is_emac_cfg_prelen_3(&self) -> bool {
        *self == EMAC_CFG_PRELENR::EMAC_CFG_PRELEN_3
    }
}
#[doc = "Values that can be written to the field `EMAC_CFG_PRELEN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_CFG_PRELENW {
    #[doc = "7 bytes of preamble"]
    EMAC_CFG_PRELEN_7,
    #[doc = "5 bytes of preamble"]
    EMAC_CFG_PRELEN_5,
    #[doc = "3 bytes of preamble"]
    EMAC_CFG_PRELEN_3,
}
impl EMAC_CFG_PRELENW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_CFG_PRELENW::EMAC_CFG_PRELEN_7 => 0,
            EMAC_CFG_PRELENW::EMAC_CFG_PRELEN_5 => 1,
            EMAC_CFG_PRELENW::EMAC_CFG_PRELEN_3 => 2,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_CFG_PRELENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_PRELENW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_CFG_PRELENW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "7 bytes of preamble"]
    #[inline(always)]
    pub fn emac_cfg_prelen_7(self) -> &'a mut W {
        self.variant(EMAC_CFG_PRELENW::EMAC_CFG_PRELEN_7)
    }
    #[doc = "5 bytes of preamble"]
    #[inline(always)]
    pub fn emac_cfg_prelen_5(self) -> &'a mut W {
        self.variant(EMAC_CFG_PRELENW::EMAC_CFG_PRELEN_5)
    }
    #[doc = "3 bytes of preamble"]
    #[inline(always)]
    pub fn emac_cfg_prelen_3(self) -> &'a mut W {
        self.variant(EMAC_CFG_PRELENW::EMAC_CFG_PRELEN_3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 0);
        self.w.bits |= ((value as u32) & 3) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_RER {
    bits: bool,
}
impl EMAC_CFG_RER {
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
pub struct _EMAC_CFG_REW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_REW<'a> {
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
pub struct EMAC_CFG_TER {
    bits: bool,
}
impl EMAC_CFG_TER {
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
pub struct _EMAC_CFG_TEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_TEW<'a> {
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
#[doc = r"Value of the field"]
pub struct EMAC_CFG_DCR {
    bits: bool,
}
impl EMAC_CFG_DCR {
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
pub struct _EMAC_CFG_DCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_DCW<'a> {
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
        self.w.bits &= !(1 << 4);
        self.w.bits |= ((value as u32) & 1) << 4;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_CFG_BL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_CFG_BLR {
    #[doc = "k = min (n,10)"]
    EMAC_CFG_BL_1024,
    #[doc = "k = min (n,8)"]
    EMAC_CFG_BL_256,
    #[doc = "k = min (n,4)"]
    EMAC_CFG_BL_8,
    #[doc = "k = min (n,1)"]
    EMAC_CFG_BL_2,
}
impl EMAC_CFG_BLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_CFG_BLR::EMAC_CFG_BL_1024 => 0,
            EMAC_CFG_BLR::EMAC_CFG_BL_256 => 1,
            EMAC_CFG_BLR::EMAC_CFG_BL_8 => 2,
            EMAC_CFG_BLR::EMAC_CFG_BL_2 => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_CFG_BLR {
        match value {
            0 => EMAC_CFG_BLR::EMAC_CFG_BL_1024,
            1 => EMAC_CFG_BLR::EMAC_CFG_BL_256,
            2 => EMAC_CFG_BLR::EMAC_CFG_BL_8,
            3 => EMAC_CFG_BLR::EMAC_CFG_BL_2,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_1024`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_1024(&self) -> bool {
        *self == EMAC_CFG_BLR::EMAC_CFG_BL_1024
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_256`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_256(&self) -> bool {
        *self == EMAC_CFG_BLR::EMAC_CFG_BL_256
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_8`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_8(&self) -> bool {
        *self == EMAC_CFG_BLR::EMAC_CFG_BL_8
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_BL_2`"]
    #[inline(always)]
    pub fn is_emac_cfg_bl_2(&self) -> bool {
        *self == EMAC_CFG_BLR::EMAC_CFG_BL_2
    }
}
#[doc = "Values that can be written to the field `EMAC_CFG_BL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_CFG_BLW {
    #[doc = "k = min (n,10)"]
    EMAC_CFG_BL_1024,
    #[doc = "k = min (n,8)"]
    EMAC_CFG_BL_256,
    #[doc = "k = min (n,4)"]
    EMAC_CFG_BL_8,
    #[doc = "k = min (n,1)"]
    EMAC_CFG_BL_2,
}
impl EMAC_CFG_BLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_CFG_BLW::EMAC_CFG_BL_1024 => 0,
            EMAC_CFG_BLW::EMAC_CFG_BL_256 => 1,
            EMAC_CFG_BLW::EMAC_CFG_BL_8 => 2,
            EMAC_CFG_BLW::EMAC_CFG_BL_2 => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_CFG_BLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_BLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_CFG_BLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "k = min (n,10)"]
    #[inline(always)]
    pub fn emac_cfg_bl_1024(self) -> &'a mut W {
        self.variant(EMAC_CFG_BLW::EMAC_CFG_BL_1024)
    }
    #[doc = "k = min (n,8)"]
    #[inline(always)]
    pub fn emac_cfg_bl_256(self) -> &'a mut W {
        self.variant(EMAC_CFG_BLW::EMAC_CFG_BL_256)
    }
    #[doc = "k = min (n,4)"]
    #[inline(always)]
    pub fn emac_cfg_bl_8(self) -> &'a mut W {
        self.variant(EMAC_CFG_BLW::EMAC_CFG_BL_8)
    }
    #[doc = "k = min (n,1)"]
    #[inline(always)]
    pub fn emac_cfg_bl_2(self) -> &'a mut W {
        self.variant(EMAC_CFG_BLW::EMAC_CFG_BL_2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 5);
        self.w.bits |= ((value as u32) & 3) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_ACSR {
    bits: bool,
}
impl EMAC_CFG_ACSR {
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
pub struct _EMAC_CFG_ACSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_ACSW<'a> {
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
pub struct EMAC_CFG_DRR {
    bits: bool,
}
impl EMAC_CFG_DRR {
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
pub struct _EMAC_CFG_DRW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_DRW<'a> {
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
#[doc = r"Value of the field"]
pub struct EMAC_CFG_IPCR {
    bits: bool,
}
impl EMAC_CFG_IPCR {
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
pub struct _EMAC_CFG_IPCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_IPCW<'a> {
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
        self.w.bits &= !(1 << 10);
        self.w.bits |= ((value as u32) & 1) << 10;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_DUPMR {
    bits: bool,
}
impl EMAC_CFG_DUPMR {
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
pub struct _EMAC_CFG_DUPMW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_DUPMW<'a> {
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
        self.w.bits &= !(1 << 11);
        self.w.bits |= ((value as u32) & 1) << 11;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_LOOPBMR {
    bits: bool,
}
impl EMAC_CFG_LOOPBMR {
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
pub struct _EMAC_CFG_LOOPBMW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_LOOPBMW<'a> {
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
        self.w.bits &= !(1 << 12);
        self.w.bits |= ((value as u32) & 1) << 12;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_DROR {
    bits: bool,
}
impl EMAC_CFG_DROR {
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
pub struct _EMAC_CFG_DROW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_DROW<'a> {
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
        self.w.bits &= !(1 << 13);
        self.w.bits |= ((value as u32) & 1) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_FESR {
    bits: bool,
}
impl EMAC_CFG_FESR {
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
pub struct _EMAC_CFG_FESW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_FESW<'a> {
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
        self.w.bits &= !(1 << 14);
        self.w.bits |= ((value as u32) & 1) << 14;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_PSR {
    bits: bool,
}
impl EMAC_CFG_PSR {
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
pub struct _EMAC_CFG_PSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_PSW<'a> {
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
        self.w.bits &= !(1 << 15);
        self.w.bits |= ((value as u32) & 1) << 15;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_DISCRSR {
    bits: bool,
}
impl EMAC_CFG_DISCRSR {
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
pub struct _EMAC_CFG_DISCRSW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_DISCRSW<'a> {
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
        self.w.bits &= !(1 << 16);
        self.w.bits |= ((value as u32) & 1) << 16;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_CFG_IFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_CFG_IFGR {
    #[doc = "96 bit times"]
    EMAC_CFG_IFG_96,
    #[doc = "88 bit times"]
    EMAC_CFG_IFG_88,
    #[doc = "80 bit times"]
    EMAC_CFG_IFG_80,
    #[doc = "72 bit times"]
    EMAC_CFG_IFG_72,
    #[doc = "64 bit times"]
    EMAC_CFG_IFG_64,
    #[doc = "56 bit times"]
    EMAC_CFG_IFG_56,
    #[doc = "48 bit times"]
    EMAC_CFG_IFG_48,
    #[doc = "40 bit times"]
    EMAC_CFG_IFG_40,
}
impl EMAC_CFG_IFGR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_CFG_IFGR::EMAC_CFG_IFG_96 => 0,
            EMAC_CFG_IFGR::EMAC_CFG_IFG_88 => 1,
            EMAC_CFG_IFGR::EMAC_CFG_IFG_80 => 2,
            EMAC_CFG_IFGR::EMAC_CFG_IFG_72 => 3,
            EMAC_CFG_IFGR::EMAC_CFG_IFG_64 => 4,
            EMAC_CFG_IFGR::EMAC_CFG_IFG_56 => 5,
            EMAC_CFG_IFGR::EMAC_CFG_IFG_48 => 6,
            EMAC_CFG_IFGR::EMAC_CFG_IFG_40 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_CFG_IFGR {
        match value {
            0 => EMAC_CFG_IFGR::EMAC_CFG_IFG_96,
            1 => EMAC_CFG_IFGR::EMAC_CFG_IFG_88,
            2 => EMAC_CFG_IFGR::EMAC_CFG_IFG_80,
            3 => EMAC_CFG_IFGR::EMAC_CFG_IFG_72,
            4 => EMAC_CFG_IFGR::EMAC_CFG_IFG_64,
            5 => EMAC_CFG_IFGR::EMAC_CFG_IFG_56,
            6 => EMAC_CFG_IFGR::EMAC_CFG_IFG_48,
            7 => EMAC_CFG_IFGR::EMAC_CFG_IFG_40,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_96`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_96(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_96
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_88`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_88(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_88
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_80`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_80(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_80
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_72`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_72(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_72
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_64`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_64(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_64
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_56`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_56(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_56
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_48`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_48(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_48
    }
    #[doc = "Checks if the value of the field is `EMAC_CFG_IFG_40`"]
    #[inline(always)]
    pub fn is_emac_cfg_ifg_40(&self) -> bool {
        *self == EMAC_CFG_IFGR::EMAC_CFG_IFG_40
    }
}
#[doc = "Values that can be written to the field `EMAC_CFG_IFG`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_CFG_IFGW {
    #[doc = "96 bit times"]
    EMAC_CFG_IFG_96,
    #[doc = "88 bit times"]
    EMAC_CFG_IFG_88,
    #[doc = "80 bit times"]
    EMAC_CFG_IFG_80,
    #[doc = "72 bit times"]
    EMAC_CFG_IFG_72,
    #[doc = "64 bit times"]
    EMAC_CFG_IFG_64,
    #[doc = "56 bit times"]
    EMAC_CFG_IFG_56,
    #[doc = "48 bit times"]
    EMAC_CFG_IFG_48,
    #[doc = "40 bit times"]
    EMAC_CFG_IFG_40,
}
impl EMAC_CFG_IFGW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_CFG_IFGW::EMAC_CFG_IFG_96 => 0,
            EMAC_CFG_IFGW::EMAC_CFG_IFG_88 => 1,
            EMAC_CFG_IFGW::EMAC_CFG_IFG_80 => 2,
            EMAC_CFG_IFGW::EMAC_CFG_IFG_72 => 3,
            EMAC_CFG_IFGW::EMAC_CFG_IFG_64 => 4,
            EMAC_CFG_IFGW::EMAC_CFG_IFG_56 => 5,
            EMAC_CFG_IFGW::EMAC_CFG_IFG_48 => 6,
            EMAC_CFG_IFGW::EMAC_CFG_IFG_40 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_CFG_IFGW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_IFGW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_CFG_IFGW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "96 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_96(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_96)
    }
    #[doc = "88 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_88(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_88)
    }
    #[doc = "80 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_80(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_80)
    }
    #[doc = "72 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_72(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_72)
    }
    #[doc = "64 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_64(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_64)
    }
    #[doc = "56 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_56(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_56)
    }
    #[doc = "48 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_48(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_48)
    }
    #[doc = "40 bit times"]
    #[inline(always)]
    pub fn emac_cfg_ifg_40(self) -> &'a mut W {
        self.variant(EMAC_CFG_IFGW::EMAC_CFG_IFG_40)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 17);
        self.w.bits |= ((value as u32) & 7) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_JFENR {
    bits: bool,
}
impl EMAC_CFG_JFENR {
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
pub struct _EMAC_CFG_JFENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_JFENW<'a> {
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
        self.w.bits &= !(1 << 20);
        self.w.bits |= ((value as u32) & 1) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_JDR {
    bits: bool,
}
impl EMAC_CFG_JDR {
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
pub struct _EMAC_CFG_JDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_JDW<'a> {
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
        self.w.bits &= !(1 << 22);
        self.w.bits |= ((value as u32) & 1) << 22;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_WDDISR {
    bits: bool,
}
impl EMAC_CFG_WDDISR {
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
pub struct _EMAC_CFG_WDDISW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_WDDISW<'a> {
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
        self.w.bits &= !(1 << 23);
        self.w.bits |= ((value as u32) & 1) << 23;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_CSTR {
    bits: bool,
}
impl EMAC_CFG_CSTR {
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
pub struct _EMAC_CFG_CSTW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_CSTW<'a> {
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
        self.w.bits &= !(1 << 25);
        self.w.bits |= ((value as u32) & 1) << 25;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_CFG_TWOKPENR {
    bits: bool,
}
impl EMAC_CFG_TWOKPENR {
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
pub struct _EMAC_CFG_TWOKPENW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_CFG_TWOKPENW<'a> {
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
        self.w.bits &= !(1 << 27);
        self.w.bits |= ((value as u32) & 1) << 27;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn emac_cfg_prelen(&self) -> EMAC_CFG_PRELENR {
        EMAC_CFG_PRELENR::_from(((self.bits >> 0) & 3) as u8)
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn emac_cfg_re(&self) -> EMAC_CFG_RER {
        let bits = ((self.bits >> 2) & 1) != 0;
        EMAC_CFG_RER { bits }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn emac_cfg_te(&self) -> EMAC_CFG_TER {
        let bits = ((self.bits >> 3) & 1) != 0;
        EMAC_CFG_TER { bits }
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn emac_cfg_dc(&self) -> EMAC_CFG_DCR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EMAC_CFG_DCR { bits }
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn emac_cfg_bl(&self) -> EMAC_CFG_BLR {
        EMAC_CFG_BLR::_from(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn emac_cfg_acs(&self) -> EMAC_CFG_ACSR {
        let bits = ((self.bits >> 7) & 1) != 0;
        EMAC_CFG_ACSR { bits }
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn emac_cfg_dr(&self) -> EMAC_CFG_DRR {
        let bits = ((self.bits >> 9) & 1) != 0;
        EMAC_CFG_DRR { bits }
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn emac_cfg_ipc(&self) -> EMAC_CFG_IPCR {
        let bits = ((self.bits >> 10) & 1) != 0;
        EMAC_CFG_IPCR { bits }
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn emac_cfg_dupm(&self) -> EMAC_CFG_DUPMR {
        let bits = ((self.bits >> 11) & 1) != 0;
        EMAC_CFG_DUPMR { bits }
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn emac_cfg_loopbm(&self) -> EMAC_CFG_LOOPBMR {
        let bits = ((self.bits >> 12) & 1) != 0;
        EMAC_CFG_LOOPBMR { bits }
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn emac_cfg_dro(&self) -> EMAC_CFG_DROR {
        let bits = ((self.bits >> 13) & 1) != 0;
        EMAC_CFG_DROR { bits }
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn emac_cfg_fes(&self) -> EMAC_CFG_FESR {
        let bits = ((self.bits >> 14) & 1) != 0;
        EMAC_CFG_FESR { bits }
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn emac_cfg_ps(&self) -> EMAC_CFG_PSR {
        let bits = ((self.bits >> 15) & 1) != 0;
        EMAC_CFG_PSR { bits }
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn emac_cfg_discrs(&self) -> EMAC_CFG_DISCRSR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_CFG_DISCRSR { bits }
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline(always)]
    pub fn emac_cfg_ifg(&self) -> EMAC_CFG_IFGR {
        EMAC_CFG_IFGR::_from(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn emac_cfg_jfen(&self) -> EMAC_CFG_JFENR {
        let bits = ((self.bits >> 20) & 1) != 0;
        EMAC_CFG_JFENR { bits }
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn emac_cfg_jd(&self) -> EMAC_CFG_JDR {
        let bits = ((self.bits >> 22) & 1) != 0;
        EMAC_CFG_JDR { bits }
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn emac_cfg_wddis(&self) -> EMAC_CFG_WDDISR {
        let bits = ((self.bits >> 23) & 1) != 0;
        EMAC_CFG_WDDISR { bits }
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline(always)]
    pub fn emac_cfg_cst(&self) -> EMAC_CFG_CSTR {
        let bits = ((self.bits >> 25) & 1) != 0;
        EMAC_CFG_CSTR { bits }
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline(always)]
    pub fn emac_cfg_twokpen(&self) -> EMAC_CFG_TWOKPENR {
        let bits = ((self.bits >> 27) & 1) != 0;
        EMAC_CFG_TWOKPENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:1 - Preamble Length for Transmit Frames"]
    #[inline(always)]
    pub fn emac_cfg_prelen(&mut self) -> _EMAC_CFG_PRELENW {
        _EMAC_CFG_PRELENW { w: self }
    }
    #[doc = "Bit 2 - Receiver Enable"]
    #[inline(always)]
    pub fn emac_cfg_re(&mut self) -> _EMAC_CFG_REW {
        _EMAC_CFG_REW { w: self }
    }
    #[doc = "Bit 3 - Transmitter Enable"]
    #[inline(always)]
    pub fn emac_cfg_te(&mut self) -> _EMAC_CFG_TEW {
        _EMAC_CFG_TEW { w: self }
    }
    #[doc = "Bit 4 - Deferral Check"]
    #[inline(always)]
    pub fn emac_cfg_dc(&mut self) -> _EMAC_CFG_DCW {
        _EMAC_CFG_DCW { w: self }
    }
    #[doc = "Bits 5:6 - Back-Off Limit"]
    #[inline(always)]
    pub fn emac_cfg_bl(&mut self) -> _EMAC_CFG_BLW {
        _EMAC_CFG_BLW { w: self }
    }
    #[doc = "Bit 7 - Automatic Pad or CRC Stripping"]
    #[inline(always)]
    pub fn emac_cfg_acs(&mut self) -> _EMAC_CFG_ACSW {
        _EMAC_CFG_ACSW { w: self }
    }
    #[doc = "Bit 9 - Disable Retry"]
    #[inline(always)]
    pub fn emac_cfg_dr(&mut self) -> _EMAC_CFG_DRW {
        _EMAC_CFG_DRW { w: self }
    }
    #[doc = "Bit 10 - Checksum Offload"]
    #[inline(always)]
    pub fn emac_cfg_ipc(&mut self) -> _EMAC_CFG_IPCW {
        _EMAC_CFG_IPCW { w: self }
    }
    #[doc = "Bit 11 - Duplex Mode"]
    #[inline(always)]
    pub fn emac_cfg_dupm(&mut self) -> _EMAC_CFG_DUPMW {
        _EMAC_CFG_DUPMW { w: self }
    }
    #[doc = "Bit 12 - Loopback Mode"]
    #[inline(always)]
    pub fn emac_cfg_loopbm(&mut self) -> _EMAC_CFG_LOOPBMW {
        _EMAC_CFG_LOOPBMW { w: self }
    }
    #[doc = "Bit 13 - Disable Receive Own"]
    #[inline(always)]
    pub fn emac_cfg_dro(&mut self) -> _EMAC_CFG_DROW {
        _EMAC_CFG_DROW { w: self }
    }
    #[doc = "Bit 14 - Speed"]
    #[inline(always)]
    pub fn emac_cfg_fes(&mut self) -> _EMAC_CFG_FESW {
        _EMAC_CFG_FESW { w: self }
    }
    #[doc = "Bit 15 - Port Select"]
    #[inline(always)]
    pub fn emac_cfg_ps(&mut self) -> _EMAC_CFG_PSW {
        _EMAC_CFG_PSW { w: self }
    }
    #[doc = "Bit 16 - Disable Carrier Sense During Transmission"]
    #[inline(always)]
    pub fn emac_cfg_discrs(&mut self) -> _EMAC_CFG_DISCRSW {
        _EMAC_CFG_DISCRSW { w: self }
    }
    #[doc = "Bits 17:19 - Inter-Frame Gap (IFG)"]
    #[inline(always)]
    pub fn emac_cfg_ifg(&mut self) -> _EMAC_CFG_IFGW {
        _EMAC_CFG_IFGW { w: self }
    }
    #[doc = "Bit 20 - Jumbo Frame Enable"]
    #[inline(always)]
    pub fn emac_cfg_jfen(&mut self) -> _EMAC_CFG_JFENW {
        _EMAC_CFG_JFENW { w: self }
    }
    #[doc = "Bit 22 - Jabber Disable"]
    #[inline(always)]
    pub fn emac_cfg_jd(&mut self) -> _EMAC_CFG_JDW {
        _EMAC_CFG_JDW { w: self }
    }
    #[doc = "Bit 23 - Watchdog Disable"]
    #[inline(always)]
    pub fn emac_cfg_wddis(&mut self) -> _EMAC_CFG_WDDISW {
        _EMAC_CFG_WDDISW { w: self }
    }
    #[doc = "Bit 25 - CRC Stripping for Type Frames"]
    #[inline(always)]
    pub fn emac_cfg_cst(&mut self) -> _EMAC_CFG_CSTW {
        _EMAC_CFG_CSTW { w: self }
    }
    #[doc = "Bit 27 - IEEE 802"]
    #[inline(always)]
    pub fn emac_cfg_twokpen(&mut self) -> _EMAC_CFG_TWOKPENW {
        _EMAC_CFG_TWOKPENW { w: self }
    }
}
