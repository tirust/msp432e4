#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::IF1MCTL {
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
pub struct CAN_IF1MCTL_DLCR {
    bits: u8,
}
impl CAN_IF1MCTL_DLCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _CAN_IF1MCTL_DLCW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_DLCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct CAN_IF1MCTL_EOBR {
    bits: bool,
}
impl CAN_IF1MCTL_EOBR {
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
pub struct _CAN_IF1MCTL_EOBW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_EOBW<'a> {
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
pub struct CAN_IF1MCTL_TXRQSTR {
    bits: bool,
}
impl CAN_IF1MCTL_TXRQSTR {
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
pub struct _CAN_IF1MCTL_TXRQSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_TXRQSTW<'a> {
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
pub struct CAN_IF1MCTL_RMTENR {
    bits: bool,
}
impl CAN_IF1MCTL_RMTENR {
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
pub struct _CAN_IF1MCTL_RMTENW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_RMTENW<'a> {
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
pub struct CAN_IF1MCTL_RXIER {
    bits: bool,
}
impl CAN_IF1MCTL_RXIER {
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
pub struct _CAN_IF1MCTL_RXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_RXIEW<'a> {
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
pub struct CAN_IF1MCTL_TXIER {
    bits: bool,
}
impl CAN_IF1MCTL_TXIER {
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
pub struct _CAN_IF1MCTL_TXIEW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_TXIEW<'a> {
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
pub struct CAN_IF1MCTL_UMASKR {
    bits: bool,
}
impl CAN_IF1MCTL_UMASKR {
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
pub struct _CAN_IF1MCTL_UMASKW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_UMASKW<'a> {
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
pub struct CAN_IF1MCTL_INTPNDR {
    bits: bool,
}
impl CAN_IF1MCTL_INTPNDR {
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
pub struct _CAN_IF1MCTL_INTPNDW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_INTPNDW<'a> {
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
pub struct CAN_IF1MCTL_MSGLSTR {
    bits: bool,
}
impl CAN_IF1MCTL_MSGLSTR {
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
pub struct _CAN_IF1MCTL_MSGLSTW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_MSGLSTW<'a> {
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
pub struct CAN_IF1MCTL_NEWDATR {
    bits: bool,
}
impl CAN_IF1MCTL_NEWDATR {
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
pub struct _CAN_IF1MCTL_NEWDATW<'a> {
    w: &'a mut W,
}
impl<'a> _CAN_IF1MCTL_NEWDATW<'a> {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn can_if1mctl_dlc(&self) -> CAN_IF1MCTL_DLCR {
        let bits = ((self.bits >> 0) & 15) as u8;
        CAN_IF1MCTL_DLCR { bits }
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn can_if1mctl_eob(&self) -> CAN_IF1MCTL_EOBR {
        let bits = ((self.bits >> 7) & 1) != 0;
        CAN_IF1MCTL_EOBR { bits }
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn can_if1mctl_txrqst(&self) -> CAN_IF1MCTL_TXRQSTR {
        let bits = ((self.bits >> 8) & 1) != 0;
        CAN_IF1MCTL_TXRQSTR { bits }
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn can_if1mctl_rmten(&self) -> CAN_IF1MCTL_RMTENR {
        let bits = ((self.bits >> 9) & 1) != 0;
        CAN_IF1MCTL_RMTENR { bits }
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn can_if1mctl_rxie(&self) -> CAN_IF1MCTL_RXIER {
        let bits = ((self.bits >> 10) & 1) != 0;
        CAN_IF1MCTL_RXIER { bits }
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn can_if1mctl_txie(&self) -> CAN_IF1MCTL_TXIER {
        let bits = ((self.bits >> 11) & 1) != 0;
        CAN_IF1MCTL_TXIER { bits }
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn can_if1mctl_umask(&self) -> CAN_IF1MCTL_UMASKR {
        let bits = ((self.bits >> 12) & 1) != 0;
        CAN_IF1MCTL_UMASKR { bits }
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn can_if1mctl_intpnd(&self) -> CAN_IF1MCTL_INTPNDR {
        let bits = ((self.bits >> 13) & 1) != 0;
        CAN_IF1MCTL_INTPNDR { bits }
    }
    #[doc = "Bit 14 - Message Lost"]
    #[inline(always)]
    pub fn can_if1mctl_msglst(&self) -> CAN_IF1MCTL_MSGLSTR {
        let bits = ((self.bits >> 14) & 1) != 0;
        CAN_IF1MCTL_MSGLSTR { bits }
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn can_if1mctl_newdat(&self) -> CAN_IF1MCTL_NEWDATR {
        let bits = ((self.bits >> 15) & 1) != 0;
        CAN_IF1MCTL_NEWDATR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - Data Length Code"]
    #[inline(always)]
    pub fn can_if1mctl_dlc(&mut self) -> _CAN_IF1MCTL_DLCW {
        _CAN_IF1MCTL_DLCW { w: self }
    }
    #[doc = "Bit 7 - End of Buffer"]
    #[inline(always)]
    pub fn can_if1mctl_eob(&mut self) -> _CAN_IF1MCTL_EOBW {
        _CAN_IF1MCTL_EOBW { w: self }
    }
    #[doc = "Bit 8 - Transmit Request"]
    #[inline(always)]
    pub fn can_if1mctl_txrqst(&mut self) -> _CAN_IF1MCTL_TXRQSTW {
        _CAN_IF1MCTL_TXRQSTW { w: self }
    }
    #[doc = "Bit 9 - Remote Enable"]
    #[inline(always)]
    pub fn can_if1mctl_rmten(&mut self) -> _CAN_IF1MCTL_RMTENW {
        _CAN_IF1MCTL_RMTENW { w: self }
    }
    #[doc = "Bit 10 - Receive Interrupt Enable"]
    #[inline(always)]
    pub fn can_if1mctl_rxie(&mut self) -> _CAN_IF1MCTL_RXIEW {
        _CAN_IF1MCTL_RXIEW { w: self }
    }
    #[doc = "Bit 11 - Transmit Interrupt Enable"]
    #[inline(always)]
    pub fn can_if1mctl_txie(&mut self) -> _CAN_IF1MCTL_TXIEW {
        _CAN_IF1MCTL_TXIEW { w: self }
    }
    #[doc = "Bit 12 - Use Acceptance Mask"]
    #[inline(always)]
    pub fn can_if1mctl_umask(&mut self) -> _CAN_IF1MCTL_UMASKW {
        _CAN_IF1MCTL_UMASKW { w: self }
    }
    #[doc = "Bit 13 - Interrupt Pending"]
    #[inline(always)]
    pub fn can_if1mctl_intpnd(&mut self) -> _CAN_IF1MCTL_INTPNDW {
        _CAN_IF1MCTL_INTPNDW { w: self }
    }
    #[doc = "Bit 14 - Message Lost"]
    #[inline(always)]
    pub fn can_if1mctl_msglst(&mut self) -> _CAN_IF1MCTL_MSGLSTW {
        _CAN_IF1MCTL_MSGLSTW { w: self }
    }
    #[doc = "Bit 15 - New Data"]
    #[inline(always)]
    pub fn can_if1mctl_newdat(&mut self) -> _CAN_IF1MCTL_NEWDATW {
        _CAN_IF1MCTL_NEWDATW { w: self }
    }
}
