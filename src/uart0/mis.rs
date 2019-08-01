#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MIS {
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
pub struct UART_MIS_RIMISR {
    bits: bool,
}
impl UART_MIS_RIMISR {
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
pub struct _UART_MIS_RIMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_RIMISW<'a> {
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
pub struct UART_MIS_CTSMISR {
    bits: bool,
}
impl UART_MIS_CTSMISR {
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
pub struct _UART_MIS_CTSMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_CTSMISW<'a> {
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
pub struct UART_MIS_DCDMISR {
    bits: bool,
}
impl UART_MIS_DCDMISR {
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
pub struct _UART_MIS_DCDMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_DCDMISW<'a> {
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
pub struct UART_MIS_DSRMISR {
    bits: bool,
}
impl UART_MIS_DSRMISR {
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
pub struct _UART_MIS_DSRMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_DSRMISW<'a> {
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
pub struct UART_MIS_RXMISR {
    bits: bool,
}
impl UART_MIS_RXMISR {
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
pub struct _UART_MIS_RXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_RXMISW<'a> {
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
pub struct UART_MIS_TXMISR {
    bits: bool,
}
impl UART_MIS_TXMISR {
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
pub struct _UART_MIS_TXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_TXMISW<'a> {
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
pub struct UART_MIS_RTMISR {
    bits: bool,
}
impl UART_MIS_RTMISR {
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
pub struct _UART_MIS_RTMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_RTMISW<'a> {
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
pub struct UART_MIS_FEMISR {
    bits: bool,
}
impl UART_MIS_FEMISR {
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
pub struct _UART_MIS_FEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_FEMISW<'a> {
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
pub struct UART_MIS_PEMISR {
    bits: bool,
}
impl UART_MIS_PEMISR {
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
pub struct _UART_MIS_PEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_PEMISW<'a> {
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
pub struct UART_MIS_BEMISR {
    bits: bool,
}
impl UART_MIS_BEMISR {
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
pub struct _UART_MIS_BEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_BEMISW<'a> {
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
pub struct UART_MIS_OEMISR {
    bits: bool,
}
impl UART_MIS_OEMISR {
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
pub struct _UART_MIS_OEMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_OEMISW<'a> {
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
pub struct UART_MIS_EOTMISR {
    bits: bool,
}
impl UART_MIS_EOTMISR {
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
pub struct _UART_MIS_EOTMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_EOTMISW<'a> {
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
pub struct UART_MIS_9BITMISR {
    bits: bool,
}
impl UART_MIS_9BITMISR {
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
pub struct _UART_MIS_9BITMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_9BITMISW<'a> {
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
pub struct UART_MIS_DMARXMISR {
    bits: bool,
}
impl UART_MIS_DMARXMISR {
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
pub struct _UART_MIS_DMARXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_DMARXMISW<'a> {
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
pub struct UART_MIS_DMATXMISR {
    bits: bool,
}
impl UART_MIS_DMATXMISR {
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
pub struct _UART_MIS_DMATXMISW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_MIS_DMATXMISW<'a> {
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
    #[doc = "Bit 0 - UART Ring Indicator Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rimis(&self) -> UART_MIS_RIMISR {
        let bits = ((self.bits >> 0) & 1) != 0;
        UART_MIS_RIMISR { bits }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_ctsmis(&self) -> UART_MIS_CTSMISR {
        let bits = ((self.bits >> 1) & 1) != 0;
        UART_MIS_CTSMISR { bits }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dcdmis(&self) -> UART_MIS_DCDMISR {
        let bits = ((self.bits >> 2) & 1) != 0;
        UART_MIS_DCDMISR { bits }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dsrmis(&self) -> UART_MIS_DSRMISR {
        let bits = ((self.bits >> 3) & 1) != 0;
        UART_MIS_DSRMISR { bits }
    }
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rxmis(&self) -> UART_MIS_RXMISR {
        let bits = ((self.bits >> 4) & 1) != 0;
        UART_MIS_RXMISR { bits }
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_txmis(&self) -> UART_MIS_TXMISR {
        let bits = ((self.bits >> 5) & 1) != 0;
        UART_MIS_TXMISR { bits }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rtmis(&self) -> UART_MIS_RTMISR {
        let bits = ((self.bits >> 6) & 1) != 0;
        UART_MIS_RTMISR { bits }
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_femis(&self) -> UART_MIS_FEMISR {
        let bits = ((self.bits >> 7) & 1) != 0;
        UART_MIS_FEMISR { bits }
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_pemis(&self) -> UART_MIS_PEMISR {
        let bits = ((self.bits >> 8) & 1) != 0;
        UART_MIS_PEMISR { bits }
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_bemis(&self) -> UART_MIS_BEMISR {
        let bits = ((self.bits >> 9) & 1) != 0;
        UART_MIS_BEMISR { bits }
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_oemis(&self) -> UART_MIS_OEMISR {
        let bits = ((self.bits >> 10) & 1) != 0;
        UART_MIS_OEMISR { bits }
    }
    #[doc = "Bit 11 - End of Transmission Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_eotmis(&self) -> UART_MIS_EOTMISR {
        let bits = ((self.bits >> 11) & 1) != 0;
        UART_MIS_EOTMISR { bits }
    }
    #[doc = "Bit 12 - 9-Bit Mode Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_9bitmis(&self) -> UART_MIS_9BITMISR {
        let bits = ((self.bits >> 12) & 1) != 0;
        UART_MIS_9BITMISR { bits }
    }
    #[doc = "Bit 16 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmarxmis(&self) -> UART_MIS_DMARXMISR {
        let bits = ((self.bits >> 16) & 1) != 0;
        UART_MIS_DMARXMISR { bits }
    }
    #[doc = "Bit 17 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmatxmis(&self) -> UART_MIS_DMATXMISR {
        let bits = ((self.bits >> 17) & 1) != 0;
        UART_MIS_DMATXMISR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - UART Ring Indicator Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rimis(&mut self) -> _UART_MIS_RIMISW {
        _UART_MIS_RIMISW { w: self }
    }
    #[doc = "Bit 1 - UART Clear to Send Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_ctsmis(&mut self) -> _UART_MIS_CTSMISW {
        _UART_MIS_CTSMISW { w: self }
    }
    #[doc = "Bit 2 - UART Data Carrier Detect Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dcdmis(&mut self) -> _UART_MIS_DCDMISW {
        _UART_MIS_DCDMISW { w: self }
    }
    #[doc = "Bit 3 - UART Data Set Ready Modem Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dsrmis(&mut self) -> _UART_MIS_DSRMISW {
        _UART_MIS_DSRMISW { w: self }
    }
    #[doc = "Bit 4 - UART Receive Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rxmis(&mut self) -> _UART_MIS_RXMISW {
        _UART_MIS_RXMISW { w: self }
    }
    #[doc = "Bit 5 - UART Transmit Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_txmis(&mut self) -> _UART_MIS_TXMISW {
        _UART_MIS_TXMISW { w: self }
    }
    #[doc = "Bit 6 - UART Receive Time-Out Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_rtmis(&mut self) -> _UART_MIS_RTMISW {
        _UART_MIS_RTMISW { w: self }
    }
    #[doc = "Bit 7 - UART Framing Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_femis(&mut self) -> _UART_MIS_FEMISW {
        _UART_MIS_FEMISW { w: self }
    }
    #[doc = "Bit 8 - UART Parity Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_pemis(&mut self) -> _UART_MIS_PEMISW {
        _UART_MIS_PEMISW { w: self }
    }
    #[doc = "Bit 9 - UART Break Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_bemis(&mut self) -> _UART_MIS_BEMISW {
        _UART_MIS_BEMISW { w: self }
    }
    #[doc = "Bit 10 - UART Overrun Error Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_oemis(&mut self) -> _UART_MIS_OEMISW {
        _UART_MIS_OEMISW { w: self }
    }
    #[doc = "Bit 11 - End of Transmission Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_eotmis(&mut self) -> _UART_MIS_EOTMISW {
        _UART_MIS_EOTMISW { w: self }
    }
    #[doc = "Bit 12 - 9-Bit Mode Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_9bitmis(&mut self) -> _UART_MIS_9BITMISW {
        _UART_MIS_9BITMISW { w: self }
    }
    #[doc = "Bit 16 - Receive DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmarxmis(&mut self) -> _UART_MIS_DMARXMISW {
        _UART_MIS_DMARXMISW { w: self }
    }
    #[doc = "Bit 17 - Transmit DMA Masked Interrupt Status"]
    #[inline(always)]
    pub fn uart_mis_dmatxmis(&mut self) -> _UART_MIS_DMATXMISW {
        _UART_MIS_DMATXMISW { w: self }
    }
}
