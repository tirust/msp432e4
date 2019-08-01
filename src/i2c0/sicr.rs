#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::SICR {
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
pub struct _I2C_SICR_DATAICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_DATAICW<'a> {
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
pub struct _I2C_SICR_STARTICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_STARTICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_SICR_STOPICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_STOPICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_SICR_DMARXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_DMARXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_SICR_DMATXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_DMATXICW<'a> {
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
pub struct _I2C_SICR_TXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_TXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_SICR_RXICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_RXICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_SICR_TXFEICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_TXFEICW<'a> {
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
#[doc = r"Proxy"]
pub struct _I2C_SICR_RXFFICW<'a> {
    w: &'a mut W,
}
impl<'a> _I2C_SICR_RXFFICW<'a> {
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
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Data Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_dataic(&mut self) -> _I2C_SICR_DATAICW {
        _I2C_SICR_DATAICW { w: self }
    }
    #[doc = "Bit 1 - Start Condition Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_startic(&mut self) -> _I2C_SICR_STARTICW {
        _I2C_SICR_STARTICW { w: self }
    }
    #[doc = "Bit 2 - Stop Condition Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_stopic(&mut self) -> _I2C_SICR_STOPICW {
        _I2C_SICR_STOPICW { w: self }
    }
    #[doc = "Bit 3 - Receive DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_dmarxic(&mut self) -> _I2C_SICR_DMARXICW {
        _I2C_SICR_DMARXICW { w: self }
    }
    #[doc = "Bit 4 - Transmit DMA Interrupt Clear"]
    #[inline(always)]
    pub fn i2c_sicr_dmatxic(&mut self) -> _I2C_SICR_DMATXICW {
        _I2C_SICR_DMATXICW { w: self }
    }
    #[doc = "Bit 5 - Transmit Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_txic(&mut self) -> _I2C_SICR_TXICW {
        _I2C_SICR_TXICW { w: self }
    }
    #[doc = "Bit 6 - Receive Request Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_rxic(&mut self) -> _I2C_SICR_RXICW {
        _I2C_SICR_RXICW { w: self }
    }
    #[doc = "Bit 7 - Transmit FIFO Empty Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_txfeic(&mut self) -> _I2C_SICR_TXFEICW {
        _I2C_SICR_TXFEICW { w: self }
    }
    #[doc = "Bit 8 - Receive FIFO Full Interrupt Mask"]
    #[inline(always)]
    pub fn i2c_sicr_rxffic(&mut self) -> _I2C_SICR_RXFFICW {
        _I2C_SICR_RXFFICW { w: self }
    }
}
