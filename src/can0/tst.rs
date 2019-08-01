#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::TST {
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
pub struct CAN_TST_BASICR {
    bits: bool,
}
impl CAN_TST_BASICR {
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
pub struct _CAN_TST_BASICW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_TST_BASICW<'a> {
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
pub struct CAN_TST_SILENTR {
    bits: bool,
}
impl CAN_TST_SILENTR {
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
pub struct _CAN_TST_SILENTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_TST_SILENTW<'a> {
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
pub struct CAN_TST_LBACKR {
    bits: bool,
}
impl CAN_TST_LBACKR {
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
pub struct _CAN_TST_LBACKW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_TST_LBACKW<'a> {
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
#[doc = "Possible values of the field `CAN_TST_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_TST_TXR {
    #[doc = "CAN Module Control"]
    CAN_TST_TX_CANCTL,
    #[doc = "Sample Point"]
    CAN_TST_TX_SAMPLE,
    #[doc = "Driven Low"]
    CAN_TST_TX_DOMINANT,
    #[doc = "Driven High"]
    CAN_TST_TX_RECESSIVE,
}
impl CAN_TST_TXR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            CAN_TST_TXR::CAN_TST_TX_CANCTL => 0,
            CAN_TST_TXR::CAN_TST_TX_SAMPLE => 1,
            CAN_TST_TXR::CAN_TST_TX_DOMINANT => 2,
            CAN_TST_TXR::CAN_TST_TX_RECESSIVE => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> CAN_TST_TXR {
        match value {
            0 => CAN_TST_TXR::CAN_TST_TX_CANCTL,
            1 => CAN_TST_TXR::CAN_TST_TX_SAMPLE,
            2 => CAN_TST_TXR::CAN_TST_TX_DOMINANT,
            3 => CAN_TST_TXR::CAN_TST_TX_RECESSIVE,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_CANCTL`"]
    #[inline(always)]
    pub fn is_can_tst_tx_canctl(&self) -> bool {
        *self == CAN_TST_TXR::CAN_TST_TX_CANCTL
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_SAMPLE`"]
    #[inline(always)]
    pub fn is_can_tst_tx_sample(&self) -> bool {
        *self == CAN_TST_TXR::CAN_TST_TX_SAMPLE
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_DOMINANT`"]
    #[inline(always)]
    pub fn is_can_tst_tx_dominant(&self) -> bool {
        *self == CAN_TST_TXR::CAN_TST_TX_DOMINANT
    }
    #[doc = "Checks if the value of the field is `CAN_TST_TX_RECESSIVE`"]
    #[inline(always)]
    pub fn is_can_tst_tx_recessive(&self) -> bool {
        *self == CAN_TST_TXR::CAN_TST_TX_RECESSIVE
    }
}
#[doc = "Values that can be written to the field `CAN_TST_TX`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_TST_TXW {
    #[doc = "CAN Module Control"]
    CAN_TST_TX_CANCTL,
    #[doc = "Sample Point"]
    CAN_TST_TX_SAMPLE,
    #[doc = "Driven Low"]
    CAN_TST_TX_DOMINANT,
    #[doc = "Driven High"]
    CAN_TST_TX_RECESSIVE,
}
impl CAN_TST_TXW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAN_TST_TXW::CAN_TST_TX_CANCTL => 0,
            CAN_TST_TXW::CAN_TST_TX_SAMPLE => 1,
            CAN_TST_TXW::CAN_TST_TX_DOMINANT => 2,
            CAN_TST_TXW::CAN_TST_TX_RECESSIVE => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAN_TST_TXW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_TST_TXW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN_TST_TXW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "CAN Module Control"]
    #[inline(always)]
    pub fn can_tst_tx_canctl(self) -> &'a mut W {
        self.variant(CAN_TST_TXW::CAN_TST_TX_CANCTL)
    }
    #[doc = "Sample Point"]
    #[inline(always)]
    pub fn can_tst_tx_sample(self) -> &'a mut W {
        self.variant(CAN_TST_TXW::CAN_TST_TX_SAMPLE)
    }
    #[doc = "Driven Low"]
    #[inline(always)]
    pub fn can_tst_tx_dominant(self) -> &'a mut W {
        self.variant(CAN_TST_TXW::CAN_TST_TX_DOMINANT)
    }
    #[doc = "Driven High"]
    #[inline(always)]
    pub fn can_tst_tx_recessive(self) -> &'a mut W {
        self.variant(CAN_TST_TXW::CAN_TST_TX_RECESSIVE)
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
pub struct CAN_TST_RXR {
    bits: bool,
}
impl CAN_TST_RXR {
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
pub struct _CAN_TST_RXW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_TST_RXW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn can_tst_basic(&self) -> CAN_TST_BASICR {
        let bits = ((self.bits >> 2) & 1) != 0;
        CAN_TST_BASICR { bits }
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn can_tst_silent(&self) -> CAN_TST_SILENTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        CAN_TST_SILENTR { bits }
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn can_tst_lback(&self) -> CAN_TST_LBACKR {
        let bits = ((self.bits >> 4) & 1) != 0;
        CAN_TST_LBACKR { bits }
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    pub fn can_tst_tx(&self) -> CAN_TST_TXR {
        CAN_TST_TXR::_from(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    pub fn can_tst_rx(&self) -> CAN_TST_RXR {
        let bits = ((self.bits >> 7) & 1) != 0;
        CAN_TST_RXR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 2 - Basic Mode"]
    #[inline(always)]
    pub fn can_tst_basic(&mut self) -> _CAN_TST_BASICW {
        _CAN_TST_BASICW { w: self }
    }
    #[doc = "Bit 3 - Silent Mode"]
    #[inline(always)]
    pub fn can_tst_silent(&mut self) -> _CAN_TST_SILENTW {
        _CAN_TST_SILENTW { w: self }
    }
    #[doc = "Bit 4 - Loopback Mode"]
    #[inline(always)]
    pub fn can_tst_lback(&mut self) -> _CAN_TST_LBACKW {
        _CAN_TST_LBACKW { w: self }
    }
    #[doc = "Bits 5:6 - Transmit Control"]
    #[inline(always)]
    pub fn can_tst_tx(&mut self) -> _CAN_TST_TXW {
        _CAN_TST_TXW { w: self }
    }
    #[doc = "Bit 7 - Receive Observation"]
    #[inline(always)]
    pub fn can_tst_rx(&mut self) -> _CAN_TST_RXW {
        _CAN_TST_RXW { w: self }
    }
}
