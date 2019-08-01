#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::RIS {
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
pub struct UART_RIS_RIRISR {
    bits: bool,
}
impl UART_RIS_RIRISR {
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
pub struct _UART_RIS_RIRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_RIRISW<'a> {
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
pub struct UART_RIS_CTSRISR {
    bits: bool,
}
impl UART_RIS_CTSRISR {
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
pub struct _UART_RIS_CTSRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_CTSRISW<'a> {
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
pub struct UART_RIS_DCDRISR {
    bits: bool,
}
impl UART_RIS_DCDRISR {
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
pub struct _UART_RIS_DCDRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_DCDRISW<'a> {
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
pub struct UART_RIS_DSRRISR {
    bits: bool,
}
impl UART_RIS_DSRRISR {
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
pub struct _UART_RIS_DSRRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_DSRRISW<'a> {
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
pub struct UART_RIS_RXRISR {
    bits: bool,
}
impl UART_RIS_RXRISR {
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
pub struct _UART_RIS_RXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_RXRISW<'a> {
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
pub struct UART_RIS_TXRISR {
    bits: bool,
}
impl UART_RIS_TXRISR {
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
pub struct _UART_RIS_TXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_TXRISW<'a> {
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
pub struct UART_RIS_RTRISR {
    bits: bool,
}
impl UART_RIS_RTRISR {
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
pub struct _UART_RIS_RTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_RTRISW<'a> {
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
pub struct UART_RIS_FERISR {
    bits: bool,
}
impl UART_RIS_FERISR {
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
pub struct _UART_RIS_FERISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_FERISW<'a> {
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
pub struct UART_RIS_PERISR {
    bits: bool,
}
impl UART_RIS_PERISR {
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
pub struct _UART_RIS_PERISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_PERISW<'a> {
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
pub struct UART_RIS_BERISR {
    bits: bool,
}
impl UART_RIS_BERISR {
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
pub struct _UART_RIS_BERISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_BERISW<'a> {
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
pub struct UART_RIS_OERISR {
    bits: bool,
}
impl UART_RIS_OERISR {
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
pub struct _UART_RIS_OERISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_OERISW<'a> {
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
pub struct UART_RIS_EOTRISR {
    bits: bool,
}
impl UART_RIS_EOTRISR {
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
pub struct _UART_RIS_EOTRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_EOTRISW<'a> {
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
pub struct UART_RIS_9BITRISR {
    bits: bool,
}
impl UART_RIS_9BITRISR {
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
pub struct _UART_RIS_9BITRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_9BITRISW<'a> {
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
pub struct UART_RIS_DMARXRISR {
    bits: bool,
}
impl UART_RIS_DMARXRISR {
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
pub struct _UART_RIS_DMARXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_DMARXRISW<'a> {
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
#[doc = r"Value of the field"]
pub struct UART_RIS_DMATXRISR {
    bits: bool,
}
impl UART_RIS_DMATXRISR {
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
pub struct _UART_RIS_DMATXRISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_RIS_DMATXRISW<'a> {
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
        self.w.bits &= !(1 << 17);
        self.w.bits |= ((value as u32) & 1) << 17;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - UART Ring Indicator Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_riris(&self) -> UART_RIS_RIRISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        UART_RIS_RIRISR { bits }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_ctsris(&self) -> UART_RIS_CTSRISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        UART_RIS_CTSRISR { bits }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dcdris(&self) -> UART_RIS_DCDRISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        UART_RIS_DCDRISR { bits }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dsrris(&self) -> UART_RIS_DSRRISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        UART_RIS_DSRRISR { bits }
    }
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rxris(&self) -> UART_RIS_RXRISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        UART_RIS_RXRISR { bits }
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_txris(&self) -> UART_RIS_TXRISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        UART_RIS_TXRISR { bits }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rtris(&self) -> UART_RIS_RTRISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        UART_RIS_RTRISR { bits }
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_feris(&self) -> UART_RIS_FERISR {
        let bits = ((self.bits >> 7) & 1) != 0;
        UART_RIS_FERISR { bits }
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_peris(&self) -> UART_RIS_PERISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        UART_RIS_PERISR { bits }
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_beris(&self) -> UART_RIS_BERISR {
        let bits = ((self.bits >> 9) & 1) != 0;
        UART_RIS_BERISR { bits }
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_oeris(&self) -> UART_RIS_OERISR {
        let bits = ((self.bits >> 10) & 1) != 0;
        UART_RIS_OERISR { bits }
    }
    #[doc = "Bit 11 - End of Transmission Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_eotris(&self) -> UART_RIS_EOTRISR {
        let bits = ((self.bits >> 11) & 1) != 0;
        UART_RIS_EOTRISR { bits }
    }
    #[doc = "Bit 12 - 9-Bit Mode Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_9bitris(&self) -> UART_RIS_9BITRISR {
        let bits = ((self.bits >> 12) & 1) != 0;
        UART_RIS_9BITRISR { bits }
    }
    #[doc = "Bit 16 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmarxris(&self) -> UART_RIS_DMARXRISR {
        let bits = ((self.bits >> 16) & 1) != 0;
        UART_RIS_DMARXRISR { bits }
    }
    #[doc = "Bit 17 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmatxris(&self) -> UART_RIS_DMATXRISR {
        let bits = ((self.bits >> 17) & 1) != 0;
        UART_RIS_DMATXRISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - UART Ring Indicator Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_riris(&mut self) -> _UART_RIS_RIRISW {
        _UART_RIS_RIRISW { w: self }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_ctsris(&mut self) -> _UART_RIS_CTSRISW {
        _UART_RIS_CTSRISW { w: self }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dcdris(&mut self) -> _UART_RIS_DCDRISW {
        _UART_RIS_DCDRISW { w: self }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dsrris(&mut self) -> _UART_RIS_DSRRISW {
        _UART_RIS_DSRRISW { w: self }
    }
    #[doc = "Bit 4 - UART Receive Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rxris(&mut self) -> _UART_RIS_RXRISW {
        _UART_RIS_RXRISW { w: self }
    }
    #[doc = "Bit 5 - UART Transmit Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_txris(&mut self) -> _UART_RIS_TXRISW {
        _UART_RIS_TXRISW { w: self }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_rtris(&mut self) -> _UART_RIS_RTRISW {
        _UART_RIS_RTRISW { w: self }
    }
    #[doc = "Bit 7 - UART Framing Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_feris(&mut self) -> _UART_RIS_FERISW {
        _UART_RIS_FERISW { w: self }
    }
    #[doc = "Bit 8 - UART Parity Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_peris(&mut self) -> _UART_RIS_PERISW {
        _UART_RIS_PERISW { w: self }
    }
    #[doc = "Bit 9 - UART Break Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_beris(&mut self) -> _UART_RIS_BERISW {
        _UART_RIS_BERISW { w: self }
    }
    #[doc = "Bit 10 - UART Overrun Error Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_oeris(&mut self) -> _UART_RIS_OERISW {
        _UART_RIS_OERISW { w: self }
    }
    #[doc = "Bit 11 - End of Transmission Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_eotris(&mut self) -> _UART_RIS_EOTRISW {
        _UART_RIS_EOTRISW { w: self }
    }
    #[doc = "Bit 12 - 9-Bit Mode Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_9bitris(&mut self) -> _UART_RIS_9BITRISW {
        _UART_RIS_9BITRISW { w: self }
    }
    #[doc = "Bit 16 - Receive DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmarxris(&mut self) -> _UART_RIS_DMARXRISW {
        _UART_RIS_DMARXRISW { w: self }
    }
    #[doc = "Bit 17 - Transmit DMA Raw Interrupt Status"]
    #[inline(always)]
    pub fn uart_ris_dmatxris(&mut self) -> _UART_RIS_DMATXRISW {
        _UART_RIS_DMATXRISW { w: self }
    }
}
