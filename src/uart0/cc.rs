#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CC {
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
#[doc = "Possible values of the field `UART_CC_CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_CC_CSR {
    #[doc = "System clock (based on clock source and divisor factor)"]
    UART_CC_CS_SYSCLK,
    #[doc = "PIOSC"]
    UART_CC_CS_PIOSC,
    #[doc = r"Reserved"]
    _Reserved(u8),
}
impl UART_CC_CSR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            UART_CC_CSR::UART_CC_CS_SYSCLK => 0,
            UART_CC_CSR::UART_CC_CS_PIOSC => 5,
            UART_CC_CSR::_Reserved(bits) => bits,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> UART_CC_CSR {
        match value {
            0 => UART_CC_CSR::UART_CC_CS_SYSCLK,
            5 => UART_CC_CSR::UART_CC_CS_PIOSC,
            i => UART_CC_CSR::_Reserved(i),
        }
    }
    #[doc = "Checks if the value of the field is `UART_CC_CS_SYSCLK`"]
    #[inline(always)]
    pub fn is_uart_cc_cs_sysclk(&self) -> bool {
        *self == UART_CC_CSR::UART_CC_CS_SYSCLK
    }
    #[doc = "Checks if the value of the field is `UART_CC_CS_PIOSC`"]
    #[inline(always)]
    pub fn is_uart_cc_cs_piosc(&self) -> bool {
        *self == UART_CC_CSR::UART_CC_CS_PIOSC
    }
}
#[doc = "Values that can be written to the field `UART_CC_CS`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum UART_CC_CSW {
    #[doc = "System clock (based on clock source and divisor factor)"]
    UART_CC_CS_SYSCLK,
    #[doc = "PIOSC"]
    UART_CC_CS_PIOSC,
}
impl UART_CC_CSW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            UART_CC_CSW::UART_CC_CS_SYSCLK => 0,
            UART_CC_CSW::UART_CC_CS_PIOSC => 5,
        }
    }
}
#[doc = r"Proxy"]
pub struct _UART_CC_CSW<'a> {
    w: &'a mut W,
}
impl<'a> _UART_CC_CSW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UART_CC_CSW) -> &'a mut W {
        unsafe { self.bits(variant._bits()) }
    }
    #[doc = "System clock (based on clock source and divisor factor)"]
    #[inline(always)]
    pub fn uart_cc_cs_sysclk(self) -> &'a mut W {
        self.variant(UART_CC_CSW::UART_CC_CS_SYSCLK)
    }
    #[doc = "PIOSC"]
    #[inline(always)]
    pub fn uart_cc_cs_piosc(self) -> &'a mut W {
        self.variant(UART_CC_CSW::UART_CC_CS_PIOSC)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(15 << 0);
        self.w.bits |= ((value as u32) & 15) << 0;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 0:3 - UART Baud Clock Source"]
    #[inline(always)]
    pub fn uart_cc_cs(&self) -> UART_CC_CSR {
        UART_CC_CSR::_from(((self.bits >> 0) & 15) as u8)
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:3 - UART Baud Clock Source"]
    #[inline(always)]
    pub fn uart_cc_cs(&mut self) -> _UART_CC_CSW {
        _UART_CC_CSW { w: self }
    }
}
