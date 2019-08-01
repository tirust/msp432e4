#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SCSR {
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
pub struct I2C_SCSR_RREQR {
    bits: bool,
}
impl I2C_SCSR_RREQR {
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
pub struct _I2C_SCSR_TXFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SCSR_TXFIFOW<'a> {
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
pub struct I2C_SCSR_FBRR {
    bits: bool,
}
impl I2C_SCSR_FBRR {
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
#[doc = r"Value of the field"]
pub struct I2C_SCSR_OAR2SELR {
    bits: bool,
}
impl I2C_SCSR_OAR2SELR {
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
#[doc = r"Value of the field"]
pub struct I2C_SCSR_QCMDSTR {
    bits: bool,
}
impl I2C_SCSR_QCMDSTR {
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
#[doc = r"Value of the field"]
pub struct I2C_SCSR_QCMDRWR {
    bits: bool,
}
impl I2C_SCSR_QCMDRWR {
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
#[doc = r"Value of the field"]
pub struct I2C_SCSR_ACTDMATXR {
    bits: bool,
}
impl I2C_SCSR_ACTDMATXR {
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
#[doc = r"Value of the field"]
pub struct I2C_SCSR_ACTDMARXR {
    bits: bool,
}
impl I2C_SCSR_ACTDMARXR {
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
pub struct _I2C_SCSR_DAW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SCSR_DAW<'a> {
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
pub struct I2C_SCSR_TREQR {
    bits: bool,
}
impl I2C_SCSR_TREQR {
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
pub struct _I2C_SCSR_RXFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SCSR_RXFIFOW<'a> {
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
    #[doc = "Bit 0 - Receive Request"]
    #[inline(always)]
    pub fn i2c_scsr_rreq(&self) -> I2C_SCSR_RREQR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_SCSR_RREQR { bits }
    }
    #[doc = "Bit 2 - First Byte Received"]
    #[inline(always)]
    pub fn i2c_scsr_fbr(&self) -> I2C_SCSR_FBRR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_SCSR_FBRR { bits }
    }
    #[doc = "Bit 3 - OAR2 Address Matched"]
    #[inline(always)]
    pub fn i2c_scsr_oar2sel(&self) -> I2C_SCSR_OAR2SELR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_SCSR_OAR2SELR { bits }
    }
    #[doc = "Bit 4 - Quick Command Status"]
    #[inline(always)]
    pub fn i2c_scsr_qcmdst(&self) -> I2C_SCSR_QCMDSTR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_SCSR_QCMDSTR { bits }
    }
    #[doc = "Bit 5 - Quick Command Read / Write"]
    #[inline(always)]
    pub fn i2c_scsr_qcmdrw(&self) -> I2C_SCSR_QCMDRWR {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_SCSR_QCMDRWR { bits }
    }
    #[doc = "Bit 30 - DMA TX Active Status"]
    #[inline(always)]
    pub fn i2c_scsr_actdmatx(&self) -> I2C_SCSR_ACTDMATXR {
        let bits = ((self.bits >> 30) & 1) != 0;
        I2C_SCSR_ACTDMATXR { bits }
    }
    #[doc = "Bit 31 - DMA RX Active Status"]
    #[inline(always)]
    pub fn i2c_scsr_actdmarx(&self) -> I2C_SCSR_ACTDMARXR {
        let bits = ((self.bits >> 31) & 1) != 0;
        I2C_SCSR_ACTDMARXR { bits }
    }
    #[doc = "Bit 1 - Transmit Request"]
    #[inline(always)]
    pub fn i2c_scsr_treq(&self) -> I2C_SCSR_TREQR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_SCSR_TREQR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 1 - TX FIFO Enable"]
    #[inline(always)]
    pub fn i2c_scsr_txfifo(&mut self) -> _I2C_SCSR_TXFIFOW {
        _I2C_SCSR_TXFIFOW { w: self }
    }
    #[doc = "Bit 0 - Device Active"]
    #[inline(always)]
    pub fn i2c_scsr_da(&mut self) -> _I2C_SCSR_DAW {
        _I2C_SCSR_DAW { w: self }
    }
    #[doc = "Bit 2 - RX FIFO Enable"]
    #[inline(always)]
    pub fn i2c_scsr_rxfifo(&mut self) -> _I2C_SCSR_RXFIFOW {
        _I2C_SCSR_RXFIFOW { w: self }
    }
}
