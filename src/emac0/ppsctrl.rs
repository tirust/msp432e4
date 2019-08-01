#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::PPSCTRL {
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
#[doc = "Possible values of the field `EMAC_PPSCTRL_PPSCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PPSCTRL_PPSCTRLR {
    #[doc = "When the PPSEN0 bit = 0x0, the EN0PPS signal is 1 pulse of the PTP reference clock.(of width clk_ptp_i) every second"]
    EMAC_PPSCTRL_PPSCTRL_1HZ,
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 2 Hz, and the digital rollover is 1 Hz"]
    EMAC_PPSCTRL_PPSCTRL_2HZ,
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 4 Hz, and the digital rollover is 2 Hz"]
    EMAC_PPSCTRL_PPSCTRL_4HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8 Hz, and the digital rollover is 4 Hz,"]
    EMAC_PPSCTRL_PPSCTRL_8HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16 Hz, and the digital rollover is 8 Hz"]
    EMAC_PPSCTRL_PPSCTRL_16HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32 Hz, and the digital rollover is 16 Hz"]
    EMAC_PPSCTRL_PPSCTRL_32HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 64 Hz, and the digital rollover is 32 Hz"]
    EMAC_PPSCTRL_PPSCTRL_64HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 128 Hz, and the digital rollover is 64 Hz"]
    EMAC_PPSCTRL_PPSCTRL_128HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 256 Hz, and the digital rollover is 128 Hz"]
    EMAC_PPSCTRL_PPSCTRL_256HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 512 Hz, and the digital rollover is 256 Hz"]
    EMAC_PPSCTRL_PPSCTRL_512HZ,
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 1.024 kHz, and the digital rollover is 512 Hz"]
    EMAC_PPSCTRL_PPSCTRL_1024HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 2.048 kHz, and the digital rollover is 1.024 kHz"]
    EMAC_PPSCTRL_PPSCTRL_2048HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 4.096 kHz, and the digital rollover is 2.048 kHz"]
    EMAC_PPSCTRL_PPSCTRL_4096HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8.192 kHz, and the digital rollover is 4.096 kHz"]
    EMAC_PPSCTRL_PPSCTRL_8192HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16.384 kHz, and the digital rollover is 8.092 kHz"]
    EMAC_PPSCTRL_PPSCTRL_16384HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz"]
    EMAC_PPSCTRL_PPSCTRL_32768HZ,
}
impl EMAC_PPSCTRL_PPSCTRLR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_1HZ => 0,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_2HZ => 1,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_4HZ => 2,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_8HZ => 3,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_16HZ => 4,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_32HZ => 5,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_64HZ => 6,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_128HZ => 7,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_256HZ => 8,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_512HZ => 9,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_1024HZ => 10,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_2048HZ => 11,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_4096HZ => 12,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_8192HZ => 13,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_16384HZ => 14,
            EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_32768HZ => 15,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_PPSCTRL_PPSCTRLR {
        match value {
            0 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_1HZ,
            1 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_2HZ,
            2 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_4HZ,
            3 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_8HZ,
            4 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_16HZ,
            5 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_32HZ,
            6 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_64HZ,
            7 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_128HZ,
            8 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_256HZ,
            9 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_512HZ,
            10 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_1024HZ,
            11 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_2048HZ,
            12 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_4096HZ,
            13 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_8192HZ,
            14 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_16384HZ,
            15 => EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_32768HZ,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_1HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_1hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_1HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_2HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_2hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_2HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_4HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_4hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_4HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_8HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_8hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_8HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_16HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_16hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_16HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_32HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_32hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_32HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_64HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_64hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_64HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_128HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_128hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_128HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_256HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_256hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_256HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_512HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_512hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_512HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_1024HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_1024hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_1024HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_2048HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_2048hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_2048HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_4096HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_4096hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_4096HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_8192HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_8192hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_8192HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_16384HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_16384hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_16384HZ
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_PPSCTRL_32768HZ`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_ppsctrl_32768hz(&self) -> bool {
        *self == EMAC_PPSCTRL_PPSCTRLR::EMAC_PPSCTRL_PPSCTRL_32768HZ
    }
}
#[doc = "Values that can be written to the field `EMAC_PPSCTRL_PPSCTRL`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PPSCTRL_PPSCTRLW {
    #[doc = "When the PPSEN0 bit = 0x0, the EN0PPS signal is 1 pulse of the PTP reference clock.(of width clk_ptp_i) every second"]
    EMAC_PPSCTRL_PPSCTRL_1HZ,
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 2 Hz, and the digital rollover is 1 Hz"]
    EMAC_PPSCTRL_PPSCTRL_2HZ,
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 4 Hz, and the digital rollover is 2 Hz"]
    EMAC_PPSCTRL_PPSCTRL_4HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8 Hz, and the digital rollover is 4 Hz,"]
    EMAC_PPSCTRL_PPSCTRL_8HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16 Hz, and the digital rollover is 8 Hz"]
    EMAC_PPSCTRL_PPSCTRL_16HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32 Hz, and the digital rollover is 16 Hz"]
    EMAC_PPSCTRL_PPSCTRL_32HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 64 Hz, and the digital rollover is 32 Hz"]
    EMAC_PPSCTRL_PPSCTRL_64HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 128 Hz, and the digital rollover is 64 Hz"]
    EMAC_PPSCTRL_PPSCTRL_128HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 256 Hz, and the digital rollover is 128 Hz"]
    EMAC_PPSCTRL_PPSCTRL_256HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 512 Hz, and the digital rollover is 256 Hz"]
    EMAC_PPSCTRL_PPSCTRL_512HZ,
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 1.024 kHz, and the digital rollover is 512 Hz"]
    EMAC_PPSCTRL_PPSCTRL_1024HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 2.048 kHz, and the digital rollover is 1.024 kHz"]
    EMAC_PPSCTRL_PPSCTRL_2048HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 4.096 kHz, and the digital rollover is 2.048 kHz"]
    EMAC_PPSCTRL_PPSCTRL_4096HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8.192 kHz, and the digital rollover is 4.096 kHz"]
    EMAC_PPSCTRL_PPSCTRL_8192HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16.384 kHz, and the digital rollover is 8.092 kHz"]
    EMAC_PPSCTRL_PPSCTRL_16384HZ,
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz"]
    EMAC_PPSCTRL_PPSCTRL_32768HZ,
}
impl EMAC_PPSCTRL_PPSCTRLW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_1HZ => 0,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_2HZ => 1,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_4HZ => 2,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_8HZ => 3,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_16HZ => 4,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_32HZ => 5,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_64HZ => 6,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_128HZ => 7,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_256HZ => 8,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_512HZ => 9,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_1024HZ => 10,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_2048HZ => 11,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_4096HZ => 12,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_8192HZ => 13,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_16384HZ => 14,
            EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_32768HZ => 15,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PPSCTRL_PPSCTRLW<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PPSCTRL_PPSCTRLW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_PPSCTRL_PPSCTRLW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "When the PPSEN0 bit = 0x0, the EN0PPS signal is 1 pulse of the PTP reference clock.(of width clk_ptp_i) every second"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_1hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_1HZ)
    }
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 2 Hz, and the digital rollover is 1 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_2hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_2HZ)
    }
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 4 Hz, and the digital rollover is 2 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_4hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_4HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8 Hz, and the digital rollover is 4 Hz,"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_8hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_8HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16 Hz, and the digital rollover is 8 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_16hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_16HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32 Hz, and the digital rollover is 16 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_32hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_32HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 64 Hz, and the digital rollover is 32 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_64hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_64HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 128 Hz, and the digital rollover is 64 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_128hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_128HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 256 Hz, and the digital rollover is 128 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_256hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_256HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 512 Hz, and the digital rollover is 256 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_512hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_512HZ)
    }
    #[doc = "When the PPSEN0 bit = 0x0, the binary rollover is 1.024 kHz, and the digital rollover is 512 Hz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_1024hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_1024HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 2.048 kHz, and the digital rollover is 1.024 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_2048hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_2048HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 4.096 kHz, and the digital rollover is 2.048 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_4096hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_4096HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 8.192 kHz, and the digital rollover is 4.096 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_8192hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_8192HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 16.384 kHz, and the digital rollover is 8.092 kHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_16384hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_16384HZ)
    }
    #[doc = "When thePPSEN0 bit = 0x0, the binary rollover is 32.768 KHz, and the digital rollover is 16.384 KHz"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl_32768hz(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_PPSCTRLW::EMAC_PPSCTRL_PPSCTRL_32768HZ)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct EMAC_PPSCTRL_PPSEN0R {
    bits: bool,
}
impl EMAC_PPSCTRL_PPSEN0R {
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
pub struct _EMAC_PPSCTRL_PPSEN0W<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PPSCTRL_PPSEN0W<'a> {
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
#[doc = "Possible values of the field `EMAC_PPSCTRL_TRGMODS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PPSCTRL_TRGMODS0R {
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    EMAC_PPSCTRL_TRGMODS0_INTONLY,
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    EMAC_PPSCTRL_TRGMODS0_INTPPS0,
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    EMAC_PPSCTRL_TRGMODS0_PPS0ONLY,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl EMAC_PPSCTRL_TRGMODS0R {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_INTONLY => 0,
            EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_INTPPS0 => 2,
            EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY => 3,
            EMAC_PPSCTRL_TRGMODS0R::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> EMAC_PPSCTRL_TRGMODS0R {
        match value {
            0 => EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_INTONLY,
            2 => EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_INTPPS0,
            3 => EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY,
            i => EMAC_PPSCTRL_TRGMODS0R::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_TRGMODS0_INTONLY`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_trgmods0_intonly(&self) -> bool {
        *self == EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_INTONLY
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_TRGMODS0_INTPPS0`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_trgmods0_intpps0(&self) -> bool {
        *self == EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_INTPPS0
    }
    #[doc = "Checks if the value of the field is `EMAC_PPSCTRL_TRGMODS0_PPS0ONLY`"]
    #[inline(always)]
    pub fn is_emac_ppsctrl_trgmods0_pps0only(&self) -> bool {
        *self == EMAC_PPSCTRL_TRGMODS0R::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY
    }
}
#[doc = "Values that can be written to the field `EMAC_PPSCTRL_TRGMODS0`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum EMAC_PPSCTRL_TRGMODS0W {
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    EMAC_PPSCTRL_TRGMODS0_INTONLY,
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    EMAC_PPSCTRL_TRGMODS0_INTPPS0,
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    EMAC_PPSCTRL_TRGMODS0_PPS0ONLY,
}
impl EMAC_PPSCTRL_TRGMODS0W {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            EMAC_PPSCTRL_TRGMODS0W::EMAC_PPSCTRL_TRGMODS0_INTONLY => 0,
            EMAC_PPSCTRL_TRGMODS0W::EMAC_PPSCTRL_TRGMODS0_INTPPS0 => 2,
            EMAC_PPSCTRL_TRGMODS0W::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY => 3,
        }
    }
}
#[doc = r"Proxy"]
pub struct _EMAC_PPSCTRL_TRGMODS0W<'a> {
    w: &'a mut W,
}
impl<'a> _EMAC_PPSCTRL_TRGMODS0W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EMAC_PPSCTRL_TRGMODS0W) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "Indicates that the Target Time registers are programmed only for generating the interrupt event"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0_intonly(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_TRGMODS0W::EMAC_PPSCTRL_TRGMODS0_INTONLY)
    }
    #[doc = "Indicates that the Target Time registers are programmed for generating the interrupt event and starting or stopping the generation of the EN0PPS output signal"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0_intpps0(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_TRGMODS0W::EMAC_PPSCTRL_TRGMODS0_INTPPS0)
    }
    #[doc = "Indicates that the Target Time registers are programmed only for starting or stopping the generation of the EN0PPS output signal. No interrupt is asserted"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0_pps0only(self) -> &'a mut W {
        self.variant(EMAC_PPSCTRL_TRGMODS0W::EMAC_PPSCTRL_TRGMODS0_PPS0ONLY)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(3 << 5);
        self.w.bits |= ((value as u32) & 3) << 5;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl(&self) -> EMAC_PPSCTRL_PPSCTRLR {
        EMAC_PPSCTRL_PPSCTRLR::_from(((self.bits >> 0) & 15) as u8)
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsen0(&self) -> EMAC_PPSCTRL_PPSEN0R {
        let bits = ((self.bits >> 4) & 1) != 0;
        EMAC_PPSCTRL_PPSEN0R { bits }
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0(&self) -> EMAC_PPSCTRL_TRGMODS0R {
        EMAC_PPSCTRL_TRGMODS0R::_from(((self.bits >> 5) & 3) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - EN0PPS Output Frequency Control (PPSCTRL) or Command Control (PPSCMD)"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsctrl(&mut self) -> _EMAC_PPSCTRL_PPSCTRLW {
        _EMAC_PPSCTRL_PPSCTRLW { w: self }
    }
    #[doc = "Bit 4 - Flexible PPS Output Mode Enable"]
    #[inline(always)]
    pub fn emac_ppsctrl_ppsen0(&mut self) -> _EMAC_PPSCTRL_PPSEN0W {
        _EMAC_PPSCTRL_PPSEN0W { w: self }
    }
    #[doc = "Bits 5:6 - Target Time Register Mode for PPS0 Output"]
    #[inline(always)]
    pub fn emac_ppsctrl_trgmods0(&mut self) -> _EMAC_PPSCTRL_TRGMODS0W {
        _EMAC_PPSCTRL_TRGMODS0W { w: self }
    }
}
