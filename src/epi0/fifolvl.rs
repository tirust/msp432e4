#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::FIFOLVL {
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
#[doc = "Possible values of the field `EPI_FIFOLVL_RDFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_FIFOLVL_RDFIFOR {
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_1,
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_2,
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_4,
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_6,
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_7,
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_8,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_FIFOLVL_RDFIFOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_1 => 1,
            EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_2 => 2,
            EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_4 => 3,
            EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_6 => 4,
            EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_7 => 5,
            EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_8 => 6,
            EPI_FIFOLVL_RDFIFOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_FIFOLVL_RDFIFOR {
        match value {
            1 => EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_1,
            2 => EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_2,
            3 => EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_4,
            4 => EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_6,
            5 => EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_7,
            6 => EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_8,
            i => EPI_FIFOLVL_RDFIFOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_1`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_1(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_1
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_2`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_2(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_2
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_4`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_4(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_4
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_6`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_6(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_6
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_7`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_7(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_7
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_RDFIFO_8`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_rdfifo_8(&self) -> bool {
        *self == EPI_FIFOLVL_RDFIFOR::EPI_FIFOLVL_RDFIFO_8
    }
}
#[doc = "Values that can be written to the field `EPI_FIFOLVL_RDFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_FIFOLVL_RDFIFOW {
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_1,
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_2,
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_4,
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_6,
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_7,
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    EPI_FIFOLVL_RDFIFO_8,
}
impl EPI_FIFOLVL_RDFIFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_1 => 1,
            EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_2 => 2,
            EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_4 => 3,
            EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_6 => 4,
            EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_7 => 5,
            EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_8 => 6,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_FIFOLVL_RDFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_FIFOLVL_RDFIFOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_FIFOLVL_RDFIFOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Trigger when there are 1 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_1(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_1)
    }
    #[doc = "Trigger when there are 2 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_2(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_2)
    }
    #[doc = "Trigger when there are 4 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_4(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_4)
    }
    #[doc = "Trigger when there are 6 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_6(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_6)
    }
    #[doc = "Trigger when there are 7 or more entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_7(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_7)
    }
    #[doc = "Trigger when there are 8 entries in the NBRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo_8(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_RDFIFOW::EPI_FIFOLVL_RDFIFO_8)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 0);
        self.w.bits |= ((value as u32) & 7) << 0;
        self.w
    }
}
#[doc = "Possible values of the field `EPI_FIFOLVL_WRFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_FIFOLVL_WRFIFOR {
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    EPI_FIFOLVL_WRFIFO_EMPT,
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    EPI_FIFOLVL_WRFIFO_2,
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    EPI_FIFOLVL_WRFIFO_1,
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    EPI_FIFOLVL_WRFIFO_NFULL,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EPI_FIFOLVL_WRFIFOR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_EMPT => 0,
            EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_2 => 2,
            EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_1 => 3,
            EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_NFULL => 4,
            EPI_FIFOLVL_WRFIFOR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EPI_FIFOLVL_WRFIFOR {
        match value {
            0 => EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_EMPT,
            2 => EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_2,
            3 => EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_1,
            4 => EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_NFULL,
            i => EPI_FIFOLVL_WRFIFOR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_EMPT`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_empt(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_EMPT
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_2`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_2(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_2
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_1`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_1(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_1
    }
    #[doc = "Checks if the value of the field is `EPI_FIFOLVL_WRFIFO_NFULL`"]
    #[inline(always)]
    pub fn is_epi_fifolvl_wrfifo_nfull(&self) -> bool {
        *self == EPI_FIFOLVL_WRFIFOR::EPI_FIFOLVL_WRFIFO_NFULL
    }
}
#[doc = "Values that can be written to the field `EPI_FIFOLVL_WRFIFO`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EPI_FIFOLVL_WRFIFOW {
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    EPI_FIFOLVL_WRFIFO_EMPT,
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    EPI_FIFOLVL_WRFIFO_2,
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    EPI_FIFOLVL_WRFIFO_1,
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    EPI_FIFOLVL_WRFIFO_NFULL,
}
impl EPI_FIFOLVL_WRFIFOW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_EMPT => 0,
            EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_2 => 2,
            EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_1 => 3,
            EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_NFULL => 4,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EPI_FIFOLVL_WRFIFOW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_FIFOLVL_WRFIFOW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EPI_FIFOLVL_WRFIFOW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Interrupt is triggered while WRFIFO is empty."]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_empt(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_EMPT)
    }
    #[doc = "Interrupt is triggered until there are only two slots available. Thus, trigger is deasserted when there are two WRFIFO entries present. This configuration is optimized for bursts of 2"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_2(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_2)
    }
    #[doc = "Interrupt is triggered until there is one WRFIFO entry available. This configuration expects only single writes"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_1(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_1)
    }
    #[doc = "Trigger interrupt when WRFIFO is not full, meaning trigger will continue to assert until there are four entries in the WRFIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo_nfull(self) -> &'a mut W {
        self.variant(EPI_FIFOLVL_WRFIFOW::EPI_FIFOLVL_WRFIFO_NFULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 4);
        self.w.bits |= ((value as u32) & 7) << 4;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EPI_FIFOLVL_RSERRR {
    bits: bool,
}
impl EPI_FIFOLVL_RSERRR {
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
pub struct _EPI_FIFOLVL_RSERRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_FIFOLVL_RSERRW<'a> {
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
pub struct EPI_FIFOLVL_WFERRR {
    bits: bool,
}
impl EPI_FIFOLVL_WFERRR {
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
pub struct _EPI_FIFOLVL_WFERRW<'a> {
    w: &'a mut W,
}
impl<'a> _EPI_FIFOLVL_WFERRW<'a> {
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
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo(&self) -> EPI_FIFOLVL_RDFIFOR {
        EPI_FIFOLVL_RDFIFOR::_from(((self.bits >> 0) & 7) as u8)
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo(&self) -> EPI_FIFOLVL_WRFIFOR {
        EPI_FIFOLVL_WRFIFOR::_from(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn epi_fifolvl_rserr(&self) -> EPI_FIFOLVL_RSERRR {
        let bits = ((self.bits >> 16) & 1) != 0;
        EPI_FIFOLVL_RSERRR { bits }
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn epi_fifolvl_wferr(&self) -> EPI_FIFOLVL_WFERRR {
        let bits = ((self.bits >> 17) & 1) != 0;
        EPI_FIFOLVL_WFERRR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:2 - Read FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_rdfifo(&mut self) -> _EPI_FIFOLVL_RDFIFOW {
        _EPI_FIFOLVL_RDFIFOW { w: self }
    }
    #[doc = "Bits 4:6 - Write FIFO"]
    #[inline(always)]
    pub fn epi_fifolvl_wrfifo(&mut self) -> _EPI_FIFOLVL_WRFIFOW {
        _EPI_FIFOLVL_WRFIFOW { w: self }
    }
    #[doc = "Bit 16 - Read Stall Error"]
    #[inline(always)]
    pub fn epi_fifolvl_rserr(&mut self) -> _EPI_FIFOLVL_RSERRW {
        _EPI_FIFOLVL_RSERRW { w: self }
    }
    #[doc = "Bit 17 - Write Full Error"]
    #[inline(always)]
    pub fn epi_fifolvl_wferr(&mut self) -> _EPI_FIFOLVL_WFERRW {
        _EPI_FIFOLVL_WFERRW { w: self }
    }
}
