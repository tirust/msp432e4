#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::MCS {
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
#[doc = r"Proxy"]
pub struct _I2C_MCS_RUNW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MCS_RUNW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MCS_STARTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MCS_STARTW<'a> {
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
pub struct I2C_MCS_ADRACKR {
    bits: bool,
}
impl I2C_MCS_ADRACKR {
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
pub struct _I2C_MCS_ACKW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MCS_ACKW<'a> {
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
pub struct I2C_MCS_ARBLSTR {
    bits: bool,
}
impl I2C_MCS_ARBLSTR {
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
pub struct I2C_MCS_IDLER {
    bits: bool,
}
impl I2C_MCS_IDLER {
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
pub struct _I2C_MCS_BURSTW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MCS_BURSTW<'a> {
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
pub struct I2C_MCS_CLKTOR {
    bits: bool,
}
impl I2C_MCS_CLKTOR {
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
pub struct I2C_MCS_ACTDMATXR {
    bits: bool,
}
impl I2C_MCS_ACTDMATXR {
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
pub struct I2C_MCS_ACTDMARXR {
    bits: bool,
}
impl I2C_MCS_ACTDMARXR {
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
pub struct I2C_MCS_BUSYR {
    bits: bool,
}
impl I2C_MCS_BUSYR {
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
pub struct I2C_MCS_ERRORR {
    bits: bool,
}
impl I2C_MCS_ERRORR {
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
pub struct _I2C_MCS_STOPW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MCS_STOPW<'a> {
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
pub struct I2C_MCS_DATACKR {
    bits: bool,
}
impl I2C_MCS_DATACKR {
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
pub struct _I2C_MCS_HSW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MCS_HSW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_MCS_QCMDW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_MCS_QCMDW<'a> {
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
pub struct I2C_MCS_BUSBSYR {
    bits: bool,
}
impl I2C_MCS_BUSBSYR {
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
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 2 - Acknowledge Address"]
    #[inline(always)]
    pub fn i2c_mcs_adrack(&self) -> I2C_MCS_ADRACKR {
        let bits = ((self.bits >> 2) & 1) != 0;
        I2C_MCS_ADRACKR { bits }
    }
    #[doc = "Bit 4 - Arbitration Lost"]
    #[inline(always)]
    pub fn i2c_mcs_arblst(&self) -> I2C_MCS_ARBLSTR {
        let bits = ((self.bits >> 4) & 1) != 0;
        I2C_MCS_ARBLSTR { bits }
    }
    #[doc = "Bit 5 - I2C Idle"]
    #[inline(always)]
    pub fn i2c_mcs_idle(&self) -> I2C_MCS_IDLER {
        let bits = ((self.bits >> 5) & 1) != 0;
        I2C_MCS_IDLER { bits }
    }
    #[doc = "Bit 7 - Clock Timeout Error"]
    #[inline(always)]
    pub fn i2c_mcs_clkto(&self) -> I2C_MCS_CLKTOR {
        let bits = ((self.bits >> 7) & 1) != 0;
        I2C_MCS_CLKTOR { bits }
    }
    #[doc = "Bit 30 - DMA TX Active Status"]
    #[inline(always)]
    pub fn i2c_mcs_actdmatx(&self) -> I2C_MCS_ACTDMATXR {
        let bits = ((self.bits >> 30) & 1) != 0;
        I2C_MCS_ACTDMATXR { bits }
    }
    #[doc = "Bit 31 - DMA RX Active Status"]
    #[inline(always)]
    pub fn i2c_mcs_actdmarx(&self) -> I2C_MCS_ACTDMARXR {
        let bits = ((self.bits >> 31) & 1) != 0;
        I2C_MCS_ACTDMARXR { bits }
    }
    #[doc = "Bit 0 - I2C Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busy(&self) -> I2C_MCS_BUSYR {
        let bits = ((self.bits >> 0) & 1) != 0;
        I2C_MCS_BUSYR { bits }
    }
    #[doc = "Bit 1 - Error"]
    #[inline(always)]
    pub fn i2c_mcs_error(&self) -> I2C_MCS_ERRORR {
        let bits = ((self.bits >> 1) & 1) != 0;
        I2C_MCS_ERRORR { bits }
    }
    #[doc = "Bit 3 - Acknowledge Data"]
    #[inline(always)]
    pub fn i2c_mcs_datack(&self) -> I2C_MCS_DATACKR {
        let bits = ((self.bits >> 3) & 1) != 0;
        I2C_MCS_DATACKR { bits }
    }
    #[doc = "Bit 6 - Bus Busy"]
    #[inline(always)]
    pub fn i2c_mcs_busbsy(&self) -> I2C_MCS_BUSBSYR {
        let bits = ((self.bits >> 6) & 1) != 0;
        I2C_MCS_BUSBSYR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - I2C Master Enable"]
    #[inline(always)]
    pub fn i2c_mcs_run(&mut self) -> _I2C_MCS_RUNW {
        _I2C_MCS_RUNW { w: self }
    }
    #[doc = "Bit 1 - Generate START"]
    #[inline(always)]
    pub fn i2c_mcs_start(&mut self) -> _I2C_MCS_STARTW {
        _I2C_MCS_STARTW { w: self }
    }
    #[doc = "Bit 3 - Data Acknowledge Enable"]
    #[inline(always)]
    pub fn i2c_mcs_ack(&mut self) -> _I2C_MCS_ACKW {
        _I2C_MCS_ACKW { w: self }
    }
    #[doc = "Bit 6 - Burst Enable"]
    #[inline(always)]
    pub fn i2c_mcs_burst(&mut self) -> _I2C_MCS_BURSTW {
        _I2C_MCS_BURSTW { w: self }
    }
    #[doc = "Bit 2 - Generate STOP"]
    #[inline(always)]
    pub fn i2c_mcs_stop(&mut self) -> _I2C_MCS_STOPW {
        _I2C_MCS_STOPW { w: self }
    }
    #[doc = "Bit 4 - High-Speed Enable"]
    #[inline(always)]
    pub fn i2c_mcs_hs(&mut self) -> _I2C_MCS_HSW {
        _I2C_MCS_HSW { w: self }
    }
    #[doc = "Bit 5 - Quick Command"]
    #[inline(always)]
    pub fn i2c_mcs_qcmd(&mut self) -> _I2C_MCS_QCMDW {
        _I2C_MCS_QCMDW { w: self }
    }
}
