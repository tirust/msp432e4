#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF2CMSK {
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
pub struct CAN_IF2CMSK_DATABR {
    bits: bool,
}
impl CAN_IF2CMSK_DATABR {
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
pub struct _CAN_IF2CMSK_DATABW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_DATABW<'a> {
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
pub struct CAN_IF2CMSK_DATAAR {
    bits: bool,
}
impl CAN_IF2CMSK_DATAAR {
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
pub struct _CAN_IF2CMSK_DATAAW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_DATAAW<'a> {
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
pub struct CAN_IF2CMSK_NEWDATR {
    bits: bool,
}
impl CAN_IF2CMSK_NEWDATR {
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
pub struct _CAN_IF2CMSK_NEWDATW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_NEWDATW<'a> {
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
pub struct CAN_IF2CMSK_CLRINTPNDR {
    bits: bool,
}
impl CAN_IF2CMSK_CLRINTPNDR {
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
pub struct _CAN_IF2CMSK_CLRINTPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_CLRINTPNDW<'a> {
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
pub struct CAN_IF2CMSK_CONTROLR {
    bits: bool,
}
impl CAN_IF2CMSK_CONTROLR {
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
pub struct _CAN_IF2CMSK_CONTROLW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_CONTROLW<'a> {
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
pub struct CAN_IF2CMSK_ARBR {
    bits: bool,
}
impl CAN_IF2CMSK_ARBR {
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
pub struct _CAN_IF2CMSK_ARBW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_ARBW<'a> {
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
pub struct CAN_IF2CMSK_MASKR {
    bits: bool,
}
impl CAN_IF2CMSK_MASKR {
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
pub struct _CAN_IF2CMSK_MASKW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_MASKW<'a> {
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
pub struct CAN_IF2CMSK_WRNRDR {
    bits: bool,
}
impl CAN_IF2CMSK_WRNRDR {
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
pub struct _CAN_IF2CMSK_WRNRDW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_WRNRDW<'a> {
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
pub struct CAN_IF2CMSK_TXRQSTR {
    bits: bool,
}
impl CAN_IF2CMSK_TXRQSTR {
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
pub struct _CAN_IF2CMSK_TXRQSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF2CMSK_TXRQSTW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Access Data Byte 4 to 7"]
    #[inline(always)]
    pub fn can_if2cmsk_datab(&self) -> CAN_IF2CMSK_DATABR {
        let bits = ((self.bits >> 0) & 1) != 0;
        CAN_IF2CMSK_DATABR { bits }
    }
    #[doc = "Bit 1 - Access Data Byte 0 to 3"]
    #[inline(always)]
    pub fn can_if2cmsk_dataa(&self) -> CAN_IF2CMSK_DATAAR {
        let bits = ((self.bits >> 1) & 1) != 0;
        CAN_IF2CMSK_DATAAR { bits }
    }
    #[doc = "Bit 2 - Access New Data"]
    #[inline(always)]
    pub fn can_if2cmsk_newdat(&self) -> CAN_IF2CMSK_NEWDATR {
        let bits = ((self.bits >> 2) & 1) != 0;
        CAN_IF2CMSK_NEWDATR { bits }
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn can_if2cmsk_clrintpnd(&self) -> CAN_IF2CMSK_CLRINTPNDR {
        let bits = ((self.bits >> 3) & 1) != 0;
        CAN_IF2CMSK_CLRINTPNDR { bits }
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_control(&self) -> CAN_IF2CMSK_CONTROLR {
        let bits = ((self.bits >> 4) & 1) != 0;
        CAN_IF2CMSK_CONTROLR { bits }
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_arb(&self) -> CAN_IF2CMSK_ARBR {
        let bits = ((self.bits >> 5) & 1) != 0;
        CAN_IF2CMSK_ARBR { bits }
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_mask(&self) -> CAN_IF2CMSK_MASKR {
        let bits = ((self.bits >> 6) & 1) != 0;
        CAN_IF2CMSK_MASKR { bits }
    }
    #[doc = "Bit 7 - Write, Not Read"]
    #[inline(always)]
    pub fn can_if2cmsk_wrnrd(&self) -> CAN_IF2CMSK_WRNRDR {
        let bits = ((self.bits >> 7) & 1) != 0;
        CAN_IF2CMSK_WRNRDR { bits }
    }
    #[doc = "Bit 2 - Access Transmission Request"]
    #[inline(always)]
    pub fn can_if2cmsk_txrqst(&self) -> CAN_IF2CMSK_TXRQSTR {
        let bits = ((self.bits >> 2) & 1) != 0;
        CAN_IF2CMSK_TXRQSTR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Access Data Byte 4 to 7"]
    #[inline(always)]
    pub fn can_if2cmsk_datab(&mut self) -> _CAN_IF2CMSK_DATABW {
        _CAN_IF2CMSK_DATABW { w: self }
    }
    #[doc = "Bit 1 - Access Data Byte 0 to 3"]
    #[inline(always)]
    pub fn can_if2cmsk_dataa(&mut self) -> _CAN_IF2CMSK_DATAAW {
        _CAN_IF2CMSK_DATAAW { w: self }
    }
    #[doc = "Bit 2 - Access New Data"]
    #[inline(always)]
    pub fn can_if2cmsk_newdat(&mut self) -> _CAN_IF2CMSK_NEWDATW {
        _CAN_IF2CMSK_NEWDATW { w: self }
    }
    #[doc = "Bit 3 - Clear Interrupt Pending Bit"]
    #[inline(always)]
    pub fn can_if2cmsk_clrintpnd(&mut self) -> _CAN_IF2CMSK_CLRINTPNDW {
        _CAN_IF2CMSK_CLRINTPNDW { w: self }
    }
    #[doc = "Bit 4 - Access Control Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_control(&mut self) -> _CAN_IF2CMSK_CONTROLW {
        _CAN_IF2CMSK_CONTROLW { w: self }
    }
    #[doc = "Bit 5 - Access Arbitration Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_arb(&mut self) -> _CAN_IF2CMSK_ARBW {
        _CAN_IF2CMSK_ARBW { w: self }
    }
    #[doc = "Bit 6 - Access Mask Bits"]
    #[inline(always)]
    pub fn can_if2cmsk_mask(&mut self) -> _CAN_IF2CMSK_MASKW {
        _CAN_IF2CMSK_MASKW { w: self }
    }
    #[doc = "Bit 7 - Write, Not Read"]
    #[inline(always)]
    pub fn can_if2cmsk_wrnrd(&mut self) -> _CAN_IF2CMSK_WRNRDW {
        _CAN_IF2CMSK_WRNRDW { w: self }
    }
    #[doc = "Bit 2 - Access Transmission Request"]
    #[inline(always)]
    pub fn can_if2cmsk_txrqst(&mut self) -> _CAN_IF2CMSK_TXRQSTW {
        _CAN_IF2CMSK_TXRQSTW { w: self }
    }
}
