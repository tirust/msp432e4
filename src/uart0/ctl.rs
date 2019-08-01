#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CTL {
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
pub struct UART_CTL_UARTENR {
    bits: bool,
}
impl UART_CTL_UARTENR {
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
pub struct _UART_CTL_UARTENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_UARTENW<'a> {
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
pub struct UART_CTL_SIRENR {
    bits: bool,
}
impl UART_CTL_SIRENR {
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
pub struct _UART_CTL_SIRENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_SIRENW<'a> {
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
pub struct UART_CTL_SIRLPR {
    bits: bool,
}
impl UART_CTL_SIRLPR {
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
pub struct _UART_CTL_SIRLPW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_SIRLPW<'a> {
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
pub struct UART_CTL_SMARTR {
    bits: bool,
}
impl UART_CTL_SMARTR {
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
pub struct _UART_CTL_SMARTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_SMARTW<'a> {
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
pub struct UART_CTL_EOTR {
    bits: bool,
}
impl UART_CTL_EOTR {
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
pub struct _UART_CTL_EOTW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_EOTW<'a> {
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
pub struct UART_CTL_HSER {
    bits: bool,
}
impl UART_CTL_HSER {
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
pub struct _UART_CTL_HSEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_HSEW<'a> {
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
pub struct UART_CTL_LBER {
    bits: bool,
}
impl UART_CTL_LBER {
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
pub struct _UART_CTL_LBEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_LBEW<'a> {
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
pub struct UART_CTL_TXER {
    bits: bool,
}
impl UART_CTL_TXER {
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
pub struct _UART_CTL_TXEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_TXEW<'a> {
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
pub struct UART_CTL_RXER {
    bits: bool,
}
impl UART_CTL_RXER {
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
pub struct _UART_CTL_RXEW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_RXEW<'a> {
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
pub struct UART_CTL_DTRR {
    bits: bool,
}
impl UART_CTL_DTRR {
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
pub struct _UART_CTL_DTRW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_DTRW<'a> {
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
pub struct UART_CTL_RTSR {
    bits: bool,
}
impl UART_CTL_RTSR {
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
pub struct _UART_CTL_RTSW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_RTSW<'a> {
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
pub struct UART_CTL_RTSENR {
    bits: bool,
}
impl UART_CTL_RTSENR {
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
pub struct _UART_CTL_RTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_RTSENW<'a> {
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
pub struct UART_CTL_CTSENR {
    bits: bool,
}
impl UART_CTL_CTSENR {
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
pub struct _UART_CTL_CTSENW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CTL_CTSENW<'a> {
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
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_ctl_uarten(&self) -> UART_CTL_UARTENR {
        let bits = ((self.bits >> 0) & 1) != 0;
        UART_CTL_UARTENR { bits }
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    pub fn uart_ctl_siren(&self) -> UART_CTL_SIRENR {
        let bits = ((self.bits >> 1) & 1) != 0;
        UART_CTL_SIRENR { bits }
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    pub fn uart_ctl_sirlp(&self) -> UART_CTL_SIRLPR {
        let bits = ((self.bits >> 2) & 1) != 0;
        UART_CTL_SIRLPR { bits }
    }
    #[doc = "Bit 3 - ISO 7816 Smart Card Support"]
    #[inline(always)]
    pub fn uart_ctl_smart(&self) -> UART_CTL_SMARTR {
        let bits = ((self.bits >> 3) & 1) != 0;
        UART_CTL_SMARTR { bits }
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn uart_ctl_eot(&self) -> UART_CTL_EOTR {
        let bits = ((self.bits >> 4) & 1) != 0;
        UART_CTL_EOTR { bits }
    }
    #[doc = "Bit 5 - High-Speed Enable"]
    #[inline(always)]
    pub fn uart_ctl_hse(&self) -> UART_CTL_HSER {
        let bits = ((self.bits >> 5) & 1) != 0;
        UART_CTL_HSER { bits }
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn uart_ctl_lbe(&self) -> UART_CTL_LBER {
        let bits = ((self.bits >> 7) & 1) != 0;
        UART_CTL_LBER { bits }
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    pub fn uart_ctl_txe(&self) -> UART_CTL_TXER {
        let bits = ((self.bits >> 8) & 1) != 0;
        UART_CTL_TXER { bits }
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    pub fn uart_ctl_rxe(&self) -> UART_CTL_RXER {
        let bits = ((self.bits >> 9) & 1) != 0;
        UART_CTL_RXER { bits }
    }
    #[doc = "Bit 10 - Data Terminal Ready"]
    #[inline(always)]
    pub fn uart_ctl_dtr(&self) -> UART_CTL_DTRR {
        let bits = ((self.bits >> 10) & 1) != 0;
        UART_CTL_DTRR { bits }
    }
    #[doc = "Bit 11 - Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rts(&self) -> UART_CTL_RTSR {
        let bits = ((self.bits >> 11) & 1) != 0;
        UART_CTL_RTSR { bits }
    }
    #[doc = "Bit 14 - Enable Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rtsen(&self) -> UART_CTL_RTSENR {
        let bits = ((self.bits >> 14) & 1) != 0;
        UART_CTL_RTSENR { bits }
    }
    #[doc = "Bit 15 - Enable Clear To Send"]
    #[inline(always)]
    pub fn uart_ctl_ctsen(&self) -> UART_CTL_CTSENR {
        let bits = ((self.bits >> 15) & 1) != 0;
        UART_CTL_CTSENR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - UART Enable"]
    #[inline(always)]
    pub fn uart_ctl_uarten(&mut self) -> _UART_CTL_UARTENW {
        _UART_CTL_UARTENW { w: self }
    }
    #[doc = "Bit 1 - UART SIR Enable"]
    #[inline(always)]
    pub fn uart_ctl_siren(&mut self) -> _UART_CTL_SIRENW {
        _UART_CTL_SIRENW { w: self }
    }
    #[doc = "Bit 2 - UART SIR Low-Power Mode"]
    #[inline(always)]
    pub fn uart_ctl_sirlp(&mut self) -> _UART_CTL_SIRLPW {
        _UART_CTL_SIRLPW { w: self }
    }
    #[doc = "Bit 3 - ISO 7816 Smart Card Support"]
    #[inline(always)]
    pub fn uart_ctl_smart(&mut self) -> _UART_CTL_SMARTW {
        _UART_CTL_SMARTW { w: self }
    }
    #[doc = "Bit 4 - End of Transmission"]
    #[inline(always)]
    pub fn uart_ctl_eot(&mut self) -> _UART_CTL_EOTW {
        _UART_CTL_EOTW { w: self }
    }
    #[doc = "Bit 5 - High-Speed Enable"]
    #[inline(always)]
    pub fn uart_ctl_hse(&mut self) -> _UART_CTL_HSEW {
        _UART_CTL_HSEW { w: self }
    }
    #[doc = "Bit 7 - UART Loop Back Enable"]
    #[inline(always)]
    pub fn uart_ctl_lbe(&mut self) -> _UART_CTL_LBEW {
        _UART_CTL_LBEW { w: self }
    }
    #[doc = "Bit 8 - UART Transmit Enable"]
    #[inline(always)]
    pub fn uart_ctl_txe(&mut self) -> _UART_CTL_TXEW {
        _UART_CTL_TXEW { w: self }
    }
    #[doc = "Bit 9 - UART Receive Enable"]
    #[inline(always)]
    pub fn uart_ctl_rxe(&mut self) -> _UART_CTL_RXEW {
        _UART_CTL_RXEW { w: self }
    }
    #[doc = "Bit 10 - Data Terminal Ready"]
    #[inline(always)]
    pub fn uart_ctl_dtr(&mut self) -> _UART_CTL_DTRW {
        _UART_CTL_DTRW { w: self }
    }
    #[doc = "Bit 11 - Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rts(&mut self) -> _UART_CTL_RTSW {
        _UART_CTL_RTSW { w: self }
    }
    #[doc = "Bit 14 - Enable Request to Send"]
    #[inline(always)]
    pub fn uart_ctl_rtsen(&mut self) -> _UART_CTL_RTSENW {
        _UART_CTL_RTSENW { w: self }
    }
    #[doc = "Bit 15 - Enable Clear To Send"]
    #[inline(always)]
    pub fn uart_ctl_ctsen(&mut self) -> _UART_CTL_CTSENW {
        _UART_CTL_CTSENW { w: self }
    }
}
