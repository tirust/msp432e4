#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::STATUS {
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
pub struct EMAC_STATUS_RPER {
    bits: bool,
}
impl EMAC_STATUS_RPER {
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
pub struct _EMAC_STATUS_RPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_RPEW<'a> {
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
pub struct EMAC_STATUS_RFCFCR {
    bits: u8,
}
impl EMAC_STATUS_RFCFCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_STATUS_RFCFCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_RFCFCW<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 1);
        self.w.bits |= ((value as u32) & 3) << 1;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_STATUS_RWCR {
    bits: bool,
}
impl EMAC_STATUS_RWCR {
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
pub struct _EMAC_STATUS_RWCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_RWCW<'a> {
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
#[doc = "Possible values of the field `EMAC_STATUS_RRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_RRCR {
    #[doc = "IDLE state"]
    EMAC_STATUS_RRC_IDLE,
    #[doc = "Reading frame data"]
    EMAC_STATUS_RRC_STATUS,
    #[doc = "Reading frame status (or timestamp)"]
    EMAC_STATUS_RRC_DATA,
    #[doc = "Flushing the frame data and status"]
    EMAC_STATUS_RRC_FLUSH,
}
impl EMAC_STATUS_RRCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_RRCR::EMAC_STATUS_RRC_IDLE => 0,
            EMAC_STATUS_RRCR::EMAC_STATUS_RRC_STATUS => 1,
            EMAC_STATUS_RRCR::EMAC_STATUS_RRC_DATA => 2,
            EMAC_STATUS_RRCR::EMAC_STATUS_RRC_FLUSH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_STATUS_RRCR {
        match value {
            0 => EMAC_STATUS_RRCR::EMAC_STATUS_RRC_IDLE,
            1 => EMAC_STATUS_RRCR::EMAC_STATUS_RRC_STATUS,
            2 => EMAC_STATUS_RRCR::EMAC_STATUS_RRC_DATA,
            3 => EMAC_STATUS_RRCR::EMAC_STATUS_RRC_FLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_IDLE`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_idle(&self) -> bool {
        *self == EMAC_STATUS_RRCR::EMAC_STATUS_RRC_IDLE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_STATUS`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_status(&self) -> bool {
        *self == EMAC_STATUS_RRCR::EMAC_STATUS_RRC_STATUS
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_DATA`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_data(&self) -> bool {
        *self == EMAC_STATUS_RRCR::EMAC_STATUS_RRC_DATA
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RRC_FLUSH`"]
    #[inline(always)]
    pub fn is_emac_status_rrc_flush(&self) -> bool {
        *self == EMAC_STATUS_RRCR::EMAC_STATUS_RRC_FLUSH
    }
}
#[doc = "Values that can be written to the field `EMAC_STATUS_RRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_RRCW {
    #[doc = "IDLE state"]
    EMAC_STATUS_RRC_IDLE,
    #[doc = "Reading frame data"]
    EMAC_STATUS_RRC_STATUS,
    #[doc = "Reading frame status (or timestamp)"]
    EMAC_STATUS_RRC_DATA,
    #[doc = "Flushing the frame data and status"]
    EMAC_STATUS_RRC_FLUSH,
}
impl EMAC_STATUS_RRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_RRCW::EMAC_STATUS_RRC_IDLE => 0,
            EMAC_STATUS_RRCW::EMAC_STATUS_RRC_STATUS => 1,
            EMAC_STATUS_RRCW::EMAC_STATUS_RRC_DATA => 2,
            EMAC_STATUS_RRCW::EMAC_STATUS_RRC_FLUSH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_STATUS_RRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_RRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_STATUS_RRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn emac_status_rrc_idle(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRCW::EMAC_STATUS_RRC_IDLE)
    }
    #[doc = "Reading frame data"]
    #[inline(always)]
    pub fn emac_status_rrc_status(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRCW::EMAC_STATUS_RRC_STATUS)
    }
    #[doc = "Reading frame status (or timestamp)"]
    #[inline(always)]
    pub fn emac_status_rrc_data(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRCW::EMAC_STATUS_RRC_DATA)
    }
    #[doc = "Flushing the frame data and status"]
    #[inline(always)]
    pub fn emac_status_rrc_flush(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RRCW::EMAC_STATUS_RRC_FLUSH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 5);
        self.w.bits |= ((value as u32) & 3) << 5;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_STATUS_RXF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_RXFR {
    #[doc = "RX FIFO Empty"]
    EMAC_STATUS_RXF_EMPTY,
    #[doc = "RX FIFO fill level is below the flow-control deactivate threshold"]
    EMAC_STATUS_RXF_BELOW,
    #[doc = "RX FIFO fill level is above the flow-control activate threshold"]
    EMAC_STATUS_RXF_ABOVE,
    #[doc = "RX FIFO Full"]
    EMAC_STATUS_RXF_FULL,
}
impl EMAC_STATUS_RXFR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_RXFR::EMAC_STATUS_RXF_EMPTY => 0,
            EMAC_STATUS_RXFR::EMAC_STATUS_RXF_BELOW => 1,
            EMAC_STATUS_RXFR::EMAC_STATUS_RXF_ABOVE => 2,
            EMAC_STATUS_RXFR::EMAC_STATUS_RXF_FULL => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_STATUS_RXFR {
        match value {
            0 => EMAC_STATUS_RXFR::EMAC_STATUS_RXF_EMPTY,
            1 => EMAC_STATUS_RXFR::EMAC_STATUS_RXF_BELOW,
            2 => EMAC_STATUS_RXFR::EMAC_STATUS_RXF_ABOVE,
            3 => EMAC_STATUS_RXFR::EMAC_STATUS_RXF_FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_EMPTY`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_empty(&self) -> bool {
        *self == EMAC_STATUS_RXFR::EMAC_STATUS_RXF_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_BELOW`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_below(&self) -> bool {
        *self == EMAC_STATUS_RXFR::EMAC_STATUS_RXF_BELOW
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_ABOVE`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_above(&self) -> bool {
        *self == EMAC_STATUS_RXFR::EMAC_STATUS_RXF_ABOVE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_RXF_FULL`"]
    #[inline(always)]
    pub fn is_emac_status_rxf_full(&self) -> bool {
        *self == EMAC_STATUS_RXFR::EMAC_STATUS_RXF_FULL
    }
}
#[doc = "Values that can be written to the field `EMAC_STATUS_RXF`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_RXFW {
    #[doc = "RX FIFO Empty"]
    EMAC_STATUS_RXF_EMPTY,
    #[doc = "RX FIFO fill level is below the flow-control deactivate threshold"]
    EMAC_STATUS_RXF_BELOW,
    #[doc = "RX FIFO fill level is above the flow-control activate threshold"]
    EMAC_STATUS_RXF_ABOVE,
    #[doc = "RX FIFO Full"]
    EMAC_STATUS_RXF_FULL,
}
impl EMAC_STATUS_RXFW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_RXFW::EMAC_STATUS_RXF_EMPTY => 0,
            EMAC_STATUS_RXFW::EMAC_STATUS_RXF_BELOW => 1,
            EMAC_STATUS_RXFW::EMAC_STATUS_RXF_ABOVE => 2,
            EMAC_STATUS_RXFW::EMAC_STATUS_RXF_FULL => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_STATUS_RXFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_RXFW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_STATUS_RXFW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "RX FIFO Empty"]
    #[inline(always)]
    pub fn emac_status_rxf_empty(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXFW::EMAC_STATUS_RXF_EMPTY)
    }
    #[doc = "RX FIFO fill level is below the flow-control deactivate threshold"]
    #[inline(always)]
    pub fn emac_status_rxf_below(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXFW::EMAC_STATUS_RXF_BELOW)
    }
    #[doc = "RX FIFO fill level is above the flow-control activate threshold"]
    #[inline(always)]
    pub fn emac_status_rxf_above(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXFW::EMAC_STATUS_RXF_ABOVE)
    }
    #[doc = "RX FIFO Full"]
    #[inline(always)]
    pub fn emac_status_rxf_full(self) -> &'a mut W {
        self.variant(EMAC_STATUS_RXFW::EMAC_STATUS_RXF_FULL)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 8);
        self.w.bits |= ((value as u32) & 3) << 8;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_STATUS_TPER {
    bits: bool,
}
impl EMAC_STATUS_TPER {
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
pub struct _EMAC_STATUS_TPEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_TPEW<'a> {
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
#[doc = "Possible values of the field `EMAC_STATUS_TFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_TFCR {
    #[doc = "IDLE state"]
    EMAC_STATUS_TFC_IDLE,
    #[doc = "Waiting for status of previous frame or IFG or backoff period to be over"]
    EMAC_STATUS_TFC_STATUS,
    #[doc = "Generating and transmitting a PAUSE control frame (in the full-duplex mode)"]
    EMAC_STATUS_TFC_PAUSE,
    #[doc = "Transferring input frame for transmission"]
    EMAC_STATUS_TFC_INPUT,
}
impl EMAC_STATUS_TFCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_TFCR::EMAC_STATUS_TFC_IDLE => 0,
            EMAC_STATUS_TFCR::EMAC_STATUS_TFC_STATUS => 1,
            EMAC_STATUS_TFCR::EMAC_STATUS_TFC_PAUSE => 2,
            EMAC_STATUS_TFCR::EMAC_STATUS_TFC_INPUT => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_STATUS_TFCR {
        match value {
            0 => EMAC_STATUS_TFCR::EMAC_STATUS_TFC_IDLE,
            1 => EMAC_STATUS_TFCR::EMAC_STATUS_TFC_STATUS,
            2 => EMAC_STATUS_TFCR::EMAC_STATUS_TFC_PAUSE,
            3 => EMAC_STATUS_TFCR::EMAC_STATUS_TFC_INPUT,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_IDLE`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_idle(&self) -> bool {
        *self == EMAC_STATUS_TFCR::EMAC_STATUS_TFC_IDLE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_STATUS`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_status(&self) -> bool {
        *self == EMAC_STATUS_TFCR::EMAC_STATUS_TFC_STATUS
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_PAUSE`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_pause(&self) -> bool {
        *self == EMAC_STATUS_TFCR::EMAC_STATUS_TFC_PAUSE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TFC_INPUT`"]
    #[inline(always)]
    pub fn is_emac_status_tfc_input(&self) -> bool {
        *self == EMAC_STATUS_TFCR::EMAC_STATUS_TFC_INPUT
    }
}
#[doc = "Values that can be written to the field `EMAC_STATUS_TFC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_TFCW {
    #[doc = "IDLE state"]
    EMAC_STATUS_TFC_IDLE,
    #[doc = "Waiting for status of previous frame or IFG or backoff period to be over"]
    EMAC_STATUS_TFC_STATUS,
    #[doc = "Generating and transmitting a PAUSE control frame (in the full-duplex mode)"]
    EMAC_STATUS_TFC_PAUSE,
    #[doc = "Transferring input frame for transmission"]
    EMAC_STATUS_TFC_INPUT,
}
impl EMAC_STATUS_TFCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_TFCW::EMAC_STATUS_TFC_IDLE => 0,
            EMAC_STATUS_TFCW::EMAC_STATUS_TFC_STATUS => 1,
            EMAC_STATUS_TFCW::EMAC_STATUS_TFC_PAUSE => 2,
            EMAC_STATUS_TFCW::EMAC_STATUS_TFC_INPUT => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_STATUS_TFCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_TFCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_STATUS_TFCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn emac_status_tfc_idle(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFCW::EMAC_STATUS_TFC_IDLE)
    }
    #[doc = "Waiting for status of previous frame or IFG or backoff period to be over"]
    #[inline(always)]
    pub fn emac_status_tfc_status(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFCW::EMAC_STATUS_TFC_STATUS)
    }
    #[doc = "Generating and transmitting a PAUSE control frame (in the full-duplex mode)"]
    #[inline(always)]
    pub fn emac_status_tfc_pause(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFCW::EMAC_STATUS_TFC_PAUSE)
    }
    #[doc = "Transferring input frame for transmission"]
    #[inline(always)]
    pub fn emac_status_tfc_input(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TFCW::EMAC_STATUS_TFC_INPUT)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 17);
        self.w.bits |= ((value as u32) & 3) << 17;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_STATUS_TXPAUSEDR {
    bits: bool,
}
impl EMAC_STATUS_TXPAUSEDR {
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
pub struct _EMAC_STATUS_TXPAUSEDW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_TXPAUSEDW<'a> {
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
        self.w.bits &= !(1 << 19);
        self.w.bits |= ((value as u32) & 1) << 19;
        self.w
    }
}
#[doc = "Possible values of the field `EMAC_STATUS_TRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_TRCR {
    #[doc = "IDLE state"]
    EMAC_STATUS_TRC_IDLE,
    #[doc = "READ state (transferring data to MAC transmitter)"]
    EMAC_STATUS_TRC_READ,
    #[doc = "Waiting for TX Status from MAC transmitter"]
    EMAC_STATUS_TRC_WAIT,
    #[doc = "Writing the received TX Status or flushing the TX FIFO"]
    EMAC_STATUS_TRC_WRFLUSH,
}
impl EMAC_STATUS_TRCR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_TRCR::EMAC_STATUS_TRC_IDLE => 0,
            EMAC_STATUS_TRCR::EMAC_STATUS_TRC_READ => 1,
            EMAC_STATUS_TRCR::EMAC_STATUS_TRC_WAIT => 2,
            EMAC_STATUS_TRCR::EMAC_STATUS_TRC_WRFLUSH => 3,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_STATUS_TRCR {
        match value {
            0 => EMAC_STATUS_TRCR::EMAC_STATUS_TRC_IDLE,
            1 => EMAC_STATUS_TRCR::EMAC_STATUS_TRC_READ,
            2 => EMAC_STATUS_TRCR::EMAC_STATUS_TRC_WAIT,
            3 => EMAC_STATUS_TRCR::EMAC_STATUS_TRC_WRFLUSH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_IDLE`"]
    #[inline(always)]
    pub fn is_emac_status_trc_idle(&self) -> bool {
        *self == EMAC_STATUS_TRCR::EMAC_STATUS_TRC_IDLE
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_READ`"]
    #[inline(always)]
    pub fn is_emac_status_trc_read(&self) -> bool {
        *self == EMAC_STATUS_TRCR::EMAC_STATUS_TRC_READ
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_WAIT`"]
    #[inline(always)]
    pub fn is_emac_status_trc_wait(&self) -> bool {
        *self == EMAC_STATUS_TRCR::EMAC_STATUS_TRC_WAIT
    }
    #[doc = "Checks if the value of the field is `EMAC_STATUS_TRC_WRFLUSH`"]
    #[inline(always)]
    pub fn is_emac_status_trc_wrflush(&self) -> bool {
        *self == EMAC_STATUS_TRCR::EMAC_STATUS_TRC_WRFLUSH
    }
}
#[doc = "Values that can be written to the field `EMAC_STATUS_TRC`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_STATUS_TRCW {
    #[doc = "IDLE state"]
    EMAC_STATUS_TRC_IDLE,
    #[doc = "READ state (transferring data to MAC transmitter)"]
    EMAC_STATUS_TRC_READ,
    #[doc = "Waiting for TX Status from MAC transmitter"]
    EMAC_STATUS_TRC_WAIT,
    #[doc = "Writing the received TX Status or flushing the TX FIFO"]
    EMAC_STATUS_TRC_WRFLUSH,
}
impl EMAC_STATUS_TRCW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_STATUS_TRCW::EMAC_STATUS_TRC_IDLE => 0,
            EMAC_STATUS_TRCW::EMAC_STATUS_TRC_READ => 1,
            EMAC_STATUS_TRCW::EMAC_STATUS_TRC_WAIT => 2,
            EMAC_STATUS_TRCW::EMAC_STATUS_TRC_WRFLUSH => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_STATUS_TRCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_TRCW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_STATUS_TRCW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "IDLE state"]
    #[inline(always)]
    pub fn emac_status_trc_idle(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRCW::EMAC_STATUS_TRC_IDLE)
    }
    #[doc = "READ state (transferring data to MAC transmitter)"]
    #[inline(always)]
    pub fn emac_status_trc_read(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRCW::EMAC_STATUS_TRC_READ)
    }
    #[doc = "Waiting for TX Status from MAC transmitter"]
    #[inline(always)]
    pub fn emac_status_trc_wait(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRCW::EMAC_STATUS_TRC_WAIT)
    }
    #[doc = "Writing the received TX Status or flushing the TX FIFO"]
    #[inline(always)]
    pub fn emac_status_trc_wrflush(self) -> &'a mut W {
        self.variant(EMAC_STATUS_TRCW::EMAC_STATUS_TRC_WRFLUSH)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 20);
        self.w.bits |= ((value as u32) & 3) << 20;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_STATUS_TWCR {
    bits: bool,
}
impl EMAC_STATUS_TWCR {
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
pub struct _EMAC_STATUS_TWCW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_TWCW<'a> {
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
        self.w.bits &= !(1 << 22);
        self.w.bits |= ((value as u32) & 1) << 22;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_STATUS_TXFER {
    bits: bool,
}
impl EMAC_STATUS_TXFER {
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
pub struct _EMAC_STATUS_TXFEW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_TXFEW<'a> {
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
        self.w.bits &= !(1 << 24);
        self.w.bits |= ((value as u32) & 1) << 24;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_STATUS_TXFFR {
    bits: bool,
}
impl EMAC_STATUS_TXFFR {
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
pub struct _EMAC_STATUS_TXFFW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_STATUS_TXFFW<'a> {
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
        self.w.bits &= !(1 << 25);
        self.w.bits |= ((value as u32) & 1) << 25;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_rpe(&self) -> EMAC_STATUS_RPER {
        let bits = ((self.bits >> 0) & 1) != 0;
        EMAC_STATUS_RPER { bits }
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn emac_status_rfcfc(&self) -> EMAC_STATUS_RFCFCR {
        let bits = ((self.bits >> 1) & 3) as u8;
        EMAC_STATUS_RFCFCR { bits }
    }
    #[doc = "Bit 4 - TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_rwc(&self) -> EMAC_STATUS_RWCR {
        let bits = ((self.bits >> 4) & 1) != 0;
        EMAC_STATUS_RWCR { bits }
    }
    #[doc = "Bits 5:6 - TX/RX Controller Read Controller State"]
    #[inline(always)]
    pub fn emac_status_rrc(&self) -> EMAC_STATUS_RRCR {
        EMAC_STATUS_RRCR::_from(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bits 8:9 - TX/RX Controller RX FIFO Fill-level Status"]
    #[inline(always)]
    pub fn emac_status_rxf(&self) -> EMAC_STATUS_RXFR {
        EMAC_STATUS_RXFR::_from(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_tpe(&self) -> EMAC_STATUS_TPER {
        let bits = ((self.bits >> 16) & 1) != 0;
        EMAC_STATUS_TPER { bits }
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn emac_status_tfc(&self) -> EMAC_STATUS_TFCR {
        EMAC_STATUS_TFCR::_from(((self.bits >> 17) & 3) as u8)
    }
    #[doc = "Bit 19 - MAC Transmitter PAUSE"]
    #[inline(always)]
    pub fn emac_status_txpaused(&self) -> EMAC_STATUS_TXPAUSEDR {
        let bits = ((self.bits >> 19) & 1) != 0;
        EMAC_STATUS_TXPAUSEDR { bits }
    }
    #[doc = "Bits 20:21 - TX/RX Controller's TX FIFO Read Controller Status"]
    #[inline(always)]
    pub fn emac_status_trc(&self) -> EMAC_STATUS_TRCR {
        EMAC_STATUS_TRCR::_from(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_twc(&self) -> EMAC_STATUS_TWCR {
        let bits = ((self.bits >> 22) & 1) != 0;
        EMAC_STATUS_TWCR { bits }
    }
    #[doc = "Bit 24 - TX/RX Controller TX FIFO Not Empty Status"]
    #[inline(always)]
    pub fn emac_status_txfe(&self) -> EMAC_STATUS_TXFER {
        let bits = ((self.bits >> 24) & 1) != 0;
        EMAC_STATUS_TXFER { bits }
    }
    #[doc = "Bit 25 - TX/RX Controller TX FIFO Full Status"]
    #[inline(always)]
    pub fn emac_status_txff(&self) -> EMAC_STATUS_TXFFR {
        let bits = ((self.bits >> 25) & 1) != 0;
        EMAC_STATUS_TXFFR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - MAC MII Receive Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_rpe(&mut self) -> _EMAC_STATUS_RPEW {
        _EMAC_STATUS_RPEW { w: self }
    }
    #[doc = "Bits 1:2 - MAC Receive Frame Controller FIFO Status"]
    #[inline(always)]
    pub fn emac_status_rfcfc(&mut self) -> _EMAC_STATUS_RFCFCW {
        _EMAC_STATUS_RFCFCW { w: self }
    }
    #[doc = "Bit 4 - TX/RX Controller RX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_rwc(&mut self) -> _EMAC_STATUS_RWCW {
        _EMAC_STATUS_RWCW { w: self }
    }
    #[doc = "Bits 5:6 - TX/RX Controller Read Controller State"]
    #[inline(always)]
    pub fn emac_status_rrc(&mut self) -> _EMAC_STATUS_RRCW {
        _EMAC_STATUS_RRCW { w: self }
    }
    #[doc = "Bits 8:9 - TX/RX Controller RX FIFO Fill-level Status"]
    #[inline(always)]
    pub fn emac_status_rxf(&mut self) -> _EMAC_STATUS_RXFW {
        _EMAC_STATUS_RXFW { w: self }
    }
    #[doc = "Bit 16 - MAC MII Transmit Protocol Engine Status"]
    #[inline(always)]
    pub fn emac_status_tpe(&mut self) -> _EMAC_STATUS_TPEW {
        _EMAC_STATUS_TPEW { w: self }
    }
    #[doc = "Bits 17:18 - MAC Transmit Frame Controller Status"]
    #[inline(always)]
    pub fn emac_status_tfc(&mut self) -> _EMAC_STATUS_TFCW {
        _EMAC_STATUS_TFCW { w: self }
    }
    #[doc = "Bit 19 - MAC Transmitter PAUSE"]
    #[inline(always)]
    pub fn emac_status_txpaused(&mut self) -> _EMAC_STATUS_TXPAUSEDW {
        _EMAC_STATUS_TXPAUSEDW { w: self }
    }
    #[doc = "Bits 20:21 - TX/RX Controller's TX FIFO Read Controller Status"]
    #[inline(always)]
    pub fn emac_status_trc(&mut self) -> _EMAC_STATUS_TRCW {
        _EMAC_STATUS_TRCW { w: self }
    }
    #[doc = "Bit 22 - TX/RX Controller TX FIFO Write Controller Active Status"]
    #[inline(always)]
    pub fn emac_status_twc(&mut self) -> _EMAC_STATUS_TWCW {
        _EMAC_STATUS_TWCW { w: self }
    }
    #[doc = "Bit 24 - TX/RX Controller TX FIFO Not Empty Status"]
    #[inline(always)]
    pub fn emac_status_txfe(&mut self) -> _EMAC_STATUS_TXFEW {
        _EMAC_STATUS_TXFEW { w: self }
    }
    #[doc = "Bit 25 - TX/RX Controller TX FIFO Full Status"]
    #[inline(always)]
    pub fn emac_status_txff(&mut self) -> _EMAC_STATUS_TXFFW {
        _EMAC_STATUS_TXFFW { w: self }
    }
}
