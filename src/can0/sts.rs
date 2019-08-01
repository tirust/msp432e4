#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STS {
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
#[doc = "Possible values of the field `CAN_STS_LEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_STS_LECR {
    #[doc = "No Error"]
    CAN_STS_LEC_NONE,
    #[doc = "Stuff Error"]
    CAN_STS_LEC_STUFF,
    #[doc = "Format Error"]
    CAN_STS_LEC_FORM,
    #[doc = "ACK Error"]
    CAN_STS_LEC_ACK,
    #[doc = "Bit 1 Error"]
    CAN_STS_LEC_BIT1,
    #[doc = "Bit 0 Error"]
    CAN_STS_LEC_BIT0,
    #[doc = "CRC Error"]
    CAN_STS_LEC_CRC,
    #[doc = "No Event"]
    CAN_STS_LEC_NOEVENT,
}
impl CAN_STS_LECR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            CAN_STS_LECR::CAN_STS_LEC_NONE => 0,
            CAN_STS_LECR::CAN_STS_LEC_STUFF => 1,
            CAN_STS_LECR::CAN_STS_LEC_FORM => 2,
            CAN_STS_LECR::CAN_STS_LEC_ACK => 3,
            CAN_STS_LECR::CAN_STS_LEC_BIT1 => 4,
            CAN_STS_LECR::CAN_STS_LEC_BIT0 => 5,
            CAN_STS_LECR::CAN_STS_LEC_CRC => 6,
            CAN_STS_LECR::CAN_STS_LEC_NOEVENT => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> CAN_STS_LECR {
        match value {
            0 => CAN_STS_LECR::CAN_STS_LEC_NONE,
            1 => CAN_STS_LECR::CAN_STS_LEC_STUFF,
            2 => CAN_STS_LECR::CAN_STS_LEC_FORM,
            3 => CAN_STS_LECR::CAN_STS_LEC_ACK,
            4 => CAN_STS_LECR::CAN_STS_LEC_BIT1,
            5 => CAN_STS_LECR::CAN_STS_LEC_BIT0,
            6 => CAN_STS_LECR::CAN_STS_LEC_CRC,
            7 => CAN_STS_LECR::CAN_STS_LEC_NOEVENT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_NONE`"]
    #[inline(always)]
    pub fn is_can_sts_lec_none(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_NONE
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_STUFF`"]
    #[inline(always)]
    pub fn is_can_sts_lec_stuff(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_STUFF
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_FORM`"]
    #[inline(always)]
    pub fn is_can_sts_lec_form(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_FORM
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_ACK`"]
    #[inline(always)]
    pub fn is_can_sts_lec_ack(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_ACK
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_BIT1`"]
    #[inline(always)]
    pub fn is_can_sts_lec_bit1(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_BIT1
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_BIT0`"]
    #[inline(always)]
    pub fn is_can_sts_lec_bit0(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_BIT0
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_CRC`"]
    #[inline(always)]
    pub fn is_can_sts_lec_crc(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_CRC
    }
    #[doc = "Checks if the value of the field is `CAN_STS_LEC_NOEVENT`"]
    #[inline(always)]
    pub fn is_can_sts_lec_noevent(&self) -> bool {
        *self == CAN_STS_LECR::CAN_STS_LEC_NOEVENT
    }
}
#[doc = "Values that can be written to the field `CAN_STS_LEC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CAN_STS_LECW {
    #[doc = "No Error"]
    CAN_STS_LEC_NONE,
    #[doc = "Stuff Error"]
    CAN_STS_LEC_STUFF,
    #[doc = "Format Error"]
    CAN_STS_LEC_FORM,
    #[doc = "ACK Error"]
    CAN_STS_LEC_ACK,
    #[doc = "Bit 1 Error"]
    CAN_STS_LEC_BIT1,
    #[doc = "Bit 0 Error"]
    CAN_STS_LEC_BIT0,
    #[doc = "CRC Error"]
    CAN_STS_LEC_CRC,
    #[doc = "No Event"]
    CAN_STS_LEC_NOEVENT,
}
impl CAN_STS_LECW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            CAN_STS_LECW::CAN_STS_LEC_NONE => 0,
            CAN_STS_LECW::CAN_STS_LEC_STUFF => 1,
            CAN_STS_LECW::CAN_STS_LEC_FORM => 2,
            CAN_STS_LECW::CAN_STS_LEC_ACK => 3,
            CAN_STS_LECW::CAN_STS_LEC_BIT1 => 4,
            CAN_STS_LECW::CAN_STS_LEC_BIT0 => 5,
            CAN_STS_LECW::CAN_STS_LEC_CRC => 6,
            CAN_STS_LECW::CAN_STS_LEC_NOEVENT => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _CAN_STS_LECW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_STS_LECW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CAN_STS_LECW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "No Error"]
    #[inline(always)]
    pub fn can_sts_lec_none(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_NONE)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn can_sts_lec_stuff(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_STUFF)
    }
    #[doc = "Format Error"]
    #[inline(always)]
    pub fn can_sts_lec_form(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_FORM)
    }
    #[doc = "ACK Error"]
    #[inline(always)]
    pub fn can_sts_lec_ack(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_ACK)
    }
    #[doc = "Bit 1 Error"]
    #[inline(always)]
    pub fn can_sts_lec_bit1(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_BIT1)
    }
    #[doc = "Bit 0 Error"]
    #[inline(always)]
    pub fn can_sts_lec_bit0(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_BIT0)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn can_sts_lec_crc(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_CRC)
    }
    #[doc = "No Event"]
    #[inline(always)]
    pub fn can_sts_lec_noevent(self) -> &'a mut W {
        self.variant(CAN_STS_LECW::CAN_STS_LEC_NOEVENT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_STS_TXOKR {
    bits: bool,
}
impl CAN_STS_TXOKR {
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
pub struct _CAN_STS_TXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_STS_TXOKW<'a> {
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
pub struct CAN_STS_RXOKR {
    bits: bool,
}
impl CAN_STS_RXOKR {
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
pub struct _CAN_STS_RXOKW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_STS_RXOKW<'a> {
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
#[doc = r"Value of the field"]
pub struct CAN_STS_EPASSR {
    bits: bool,
}
impl CAN_STS_EPASSR {
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
pub struct _CAN_STS_EPASSW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_STS_EPASSW<'a> {
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
        self.w.bits &= !(1 << 5);
        self.w.bits |= ((value as u32) & 1) << 5;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_STS_EWARNR {
    bits: bool,
}
impl CAN_STS_EWARNR {
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
pub struct _CAN_STS_EWARNW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_STS_EWARNW<'a> {
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
        self.w.bits &= !(1 << 6);
        self.w.bits |= ((value as u32) & 1) << 6;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_STS_BOFFR {
    bits: bool,
}
impl CAN_STS_BOFFR {
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
pub struct _CAN_STS_BOFFW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_STS_BOFFW<'a> {
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
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn can_sts_lec(&self) -> CAN_STS_LECR {
        CAN_STS_LECR::_from(((self.bits >> 0) & 7) as u8)
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn can_sts_txok(&self) -> CAN_STS_TXOKR {
        let bits = ((self.bits >> 3) & 1) != 0;
        CAN_STS_TXOKR { bits }
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn can_sts_rxok(&self) -> CAN_STS_RXOKR {
        let bits = ((self.bits >> 4) & 1) != 0;
        CAN_STS_RXOKR { bits }
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn can_sts_epass(&self) -> CAN_STS_EPASSR {
        let bits = ((self.bits >> 5) & 1) != 0;
        CAN_STS_EPASSR { bits }
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn can_sts_ewarn(&self) -> CAN_STS_EWARNR {
        let bits = ((self.bits >> 6) & 1) != 0;
        CAN_STS_EWARNR { bits }
    }
    #[doc = "Bit 7 - Bus-Off Status"]
    #[inline(always)]
    pub fn can_sts_boff(&self) -> CAN_STS_BOFFR {
        let bits = ((self.bits >> 7) & 1) != 0;
        CAN_STS_BOFFR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Last Error Code"]
    #[inline(always)]
    pub fn can_sts_lec(&mut self) -> _CAN_STS_LECW {
        _CAN_STS_LECW { w: self }
    }
    #[doc = "Bit 3 - Transmitted a Message Successfully"]
    #[inline(always)]
    pub fn can_sts_txok(&mut self) -> _CAN_STS_TXOKW {
        _CAN_STS_TXOKW { w: self }
    }
    #[doc = "Bit 4 - Received a Message Successfully"]
    #[inline(always)]
    pub fn can_sts_rxok(&mut self) -> _CAN_STS_RXOKW {
        _CAN_STS_RXOKW { w: self }
    }
    #[doc = "Bit 5 - Error Passive"]
    #[inline(always)]
    pub fn can_sts_epass(&mut self) -> _CAN_STS_EPASSW {
        _CAN_STS_EPASSW { w: self }
    }
    #[doc = "Bit 6 - Warning Status"]
    #[inline(always)]
    pub fn can_sts_ewarn(&mut self) -> _CAN_STS_EWARNW {
        _CAN_STS_EWARNW { w: self }
    }
    #[doc = "Bit 7 - Bus-Off Status"]
    #[inline(always)]
    pub fn can_sts_boff(&mut self) -> _CAN_STS_BOFFW {
        _CAN_STS_BOFFW { w: self }
    }
}
