#[doc = r"Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r"Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::BOOTCFG {
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
pub struct FLASH_BOOTCFG_DBG0R {
    bits: bool,
}
impl FLASH_BOOTCFG_DBG0R {
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
pub struct _FLASH_BOOTCFG_DBG0W<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_DBG0W<'a> {
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
pub struct FLASH_BOOTCFG_DBG1R {
    bits: bool,
}
impl FLASH_BOOTCFG_DBG1R {
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
pub struct _FLASH_BOOTCFG_DBG1W<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_DBG1W<'a> {
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
pub struct FLASH_BOOTCFG_KEYR {
    bits: bool,
}
impl FLASH_BOOTCFG_KEYR {
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
pub struct _FLASH_BOOTCFG_KEYW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_KEYW<'a> {
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
pub struct FLASH_BOOTCFG_ENR {
    bits: bool,
}
impl FLASH_BOOTCFG_ENR {
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
pub struct _FLASH_BOOTCFG_ENW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_ENW<'a> {
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
pub struct FLASH_BOOTCFG_POLR {
    bits: bool,
}
impl FLASH_BOOTCFG_POLR {
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
pub struct _FLASH_BOOTCFG_POLW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_POLW<'a> {
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
#[doc = "Possible values of the field `FLASH_BOOTCFG_PIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_BOOTCFG_PINR {
    #[doc = "Pin 0"]
    FLASH_BOOTCFG_PIN_0,
    #[doc = "Pin 1"]
    FLASH_BOOTCFG_PIN_1,
    #[doc = "Pin 2"]
    FLASH_BOOTCFG_PIN_2,
    #[doc = "Pin 3"]
    FLASH_BOOTCFG_PIN_3,
    #[doc = "Pin 4"]
    FLASH_BOOTCFG_PIN_4,
    #[doc = "Pin 5"]
    FLASH_BOOTCFG_PIN_5,
    #[doc = "Pin 6"]
    FLASH_BOOTCFG_PIN_6,
    #[doc = "Pin 7"]
    FLASH_BOOTCFG_PIN_7,
}
impl FLASH_BOOTCFG_PINR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_0 => 0,
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_1 => 1,
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_2 => 2,
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_3 => 3,
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_4 => 4,
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_5 => 5,
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_6 => 6,
            FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_7 => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_BOOTCFG_PINR {
        match value {
            0 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_0,
            1 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_1,
            2 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_2,
            3 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_3,
            4 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_4,
            5 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_5,
            6 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_6,
            7 => FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_0`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_0(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_0
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_1`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_1(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_1
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_2`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_2(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_2
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_3`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_3(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_3
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_4`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_4(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_4
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_5`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_5(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_5
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_6`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_6(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_6
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PIN_7`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_pin_7(&self) -> bool {
        *self == FLASH_BOOTCFG_PINR::FLASH_BOOTCFG_PIN_7
    }
}
#[doc = "Values that can be written to the field `FLASH_BOOTCFG_PIN`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_BOOTCFG_PINW {
    #[doc = "Pin 0"]
    FLASH_BOOTCFG_PIN_0,
    #[doc = "Pin 1"]
    FLASH_BOOTCFG_PIN_1,
    #[doc = "Pin 2"]
    FLASH_BOOTCFG_PIN_2,
    #[doc = "Pin 3"]
    FLASH_BOOTCFG_PIN_3,
    #[doc = "Pin 4"]
    FLASH_BOOTCFG_PIN_4,
    #[doc = "Pin 5"]
    FLASH_BOOTCFG_PIN_5,
    #[doc = "Pin 6"]
    FLASH_BOOTCFG_PIN_6,
    #[doc = "Pin 7"]
    FLASH_BOOTCFG_PIN_7,
}
impl FLASH_BOOTCFG_PINW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_0 => 0,
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_1 => 1,
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_2 => 2,
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_3 => 3,
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_4 => 4,
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_5 => 5,
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_6 => 6,
            FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_7 => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_BOOTCFG_PINW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_PINW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_BOOTCFG_PINW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Pin 0"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_0(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_0)
    }
    #[doc = "Pin 1"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_1(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_1)
    }
    #[doc = "Pin 2"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_2(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_2)
    }
    #[doc = "Pin 3"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_3(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_3)
    }
    #[doc = "Pin 4"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_4(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_4)
    }
    #[doc = "Pin 5"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_5(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_5)
    }
    #[doc = "Pin 6"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_6(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_6)
    }
    #[doc = "Pin 7"]
    #[inline(always)]
    pub fn flash_bootcfg_pin_7(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PINW::FLASH_BOOTCFG_PIN_7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 10);
        self.w.bits |= ((value as u32) & 7) << 10;
        self.w
    }
}
#[doc = "Possible values of the field `FLASH_BOOTCFG_PORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_BOOTCFG_PORTR {
    #[doc = "Port A"]
    FLASH_BOOTCFG_PORT_A,
    #[doc = "Port B"]
    FLASH_BOOTCFG_PORT_B,
    #[doc = "Port C"]
    FLASH_BOOTCFG_PORT_C,
    #[doc = "Port D"]
    FLASH_BOOTCFG_PORT_D,
    #[doc = "Port E"]
    FLASH_BOOTCFG_PORT_E,
    #[doc = "Port F"]
    FLASH_BOOTCFG_PORT_F,
    #[doc = "Port G"]
    FLASH_BOOTCFG_PORT_G,
    #[doc = "Port H"]
    FLASH_BOOTCFG_PORT_H,
}
impl FLASH_BOOTCFG_PORTR {
    #[doc = r"Value of the field as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u8 {
        match *self {
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_A => 0,
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_B => 1,
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_C => 2,
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_D => 3,
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_E => 4,
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_F => 5,
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_G => 6,
            FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_H => 7,
        }
    }
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _from(value: u8) -> FLASH_BOOTCFG_PORTR {
        match value {
            0 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_A,
            1 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_B,
            2 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_C,
            3 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_D,
            4 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_E,
            5 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_F,
            6 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_G,
            7 => FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_H,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_A`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_a(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_A
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_B`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_b(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_B
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_C`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_c(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_C
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_D`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_d(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_D
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_E`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_e(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_E
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_F`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_f(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_F
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_G`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_g(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_G
    }
    #[doc = "Checks if the value of the field is `FLASH_BOOTCFG_PORT_H`"]
    #[inline(always)]
    pub fn is_flash_bootcfg_port_h(&self) -> bool {
        *self == FLASH_BOOTCFG_PORTR::FLASH_BOOTCFG_PORT_H
    }
}
#[doc = "Values that can be written to the field `FLASH_BOOTCFG_PORT`"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FLASH_BOOTCFG_PORTW {
    #[doc = "Port A"]
    FLASH_BOOTCFG_PORT_A,
    #[doc = "Port B"]
    FLASH_BOOTCFG_PORT_B,
    #[doc = "Port C"]
    FLASH_BOOTCFG_PORT_C,
    #[doc = "Port D"]
    FLASH_BOOTCFG_PORT_D,
    #[doc = "Port E"]
    FLASH_BOOTCFG_PORT_E,
    #[doc = "Port F"]
    FLASH_BOOTCFG_PORT_F,
    #[doc = "Port G"]
    FLASH_BOOTCFG_PORT_G,
    #[doc = "Port H"]
    FLASH_BOOTCFG_PORT_H,
}
impl FLASH_BOOTCFG_PORTW {
    #[allow(missing_docs)]
    #[doc(hidden)]
    #[inline(always)]
    pub fn _bits(&self) -> u8 {
        match *self {
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_A => 0,
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_B => 1,
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_C => 2,
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_D => 3,
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_E => 4,
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_F => 5,
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_G => 6,
            FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_H => 7,
        }
    }
}
#[doc = r"Proxy"]
pub struct _FLASH_BOOTCFG_PORTW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_PORTW<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: FLASH_BOOTCFG_PORTW) -> &'a mut W {
        {
            self.bits(variant._bits())
        }
    }
    #[doc = "Port A"]
    #[inline(always)]
    pub fn flash_bootcfg_port_a(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_A)
    }
    #[doc = "Port B"]
    #[inline(always)]
    pub fn flash_bootcfg_port_b(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_B)
    }
    #[doc = "Port C"]
    #[inline(always)]
    pub fn flash_bootcfg_port_c(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_C)
    }
    #[doc = "Port D"]
    #[inline(always)]
    pub fn flash_bootcfg_port_d(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_D)
    }
    #[doc = "Port E"]
    #[inline(always)]
    pub fn flash_bootcfg_port_e(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_E)
    }
    #[doc = "Port F"]
    #[inline(always)]
    pub fn flash_bootcfg_port_f(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_F)
    }
    #[doc = "Port G"]
    #[inline(always)]
    pub fn flash_bootcfg_port_g(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_G)
    }
    #[doc = "Port H"]
    #[inline(always)]
    pub fn flash_bootcfg_port_h(self) -> &'a mut W {
        self.variant(FLASH_BOOTCFG_PORTW::FLASH_BOOTCFG_PORT_H)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits &= !(7 << 13);
        self.w.bits |= ((value as u32) & 7) << 13;
        self.w
    }
}
#[doc = r"Value of the field"]
pub struct FLASH_BOOTCFG_NWR {
    bits: bool,
}
impl FLASH_BOOTCFG_NWR {
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
pub struct _FLASH_BOOTCFG_NWW<'a> {
    w: &'a mut W,
}
impl<'a> _FLASH_BOOTCFG_NWW<'a> {
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
        self.w.bits &= !(1 << 31);
        self.w.bits |= ((value as u32) & 1) << 31;
        self.w
    }
}
impl R {
    #[doc = r"Value of the register as raw bits"]
    #[inline(always)]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg0(&self) -> FLASH_BOOTCFG_DBG0R {
        let bits = ((self.bits >> 0) & 1) != 0;
        FLASH_BOOTCFG_DBG0R { bits }
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg1(&self) -> FLASH_BOOTCFG_DBG1R {
        let bits = ((self.bits >> 1) & 1) != 0;
        FLASH_BOOTCFG_DBG1R { bits }
    }
    #[doc = "Bit 4 - KEY Select"]
    #[inline(always)]
    pub fn flash_bootcfg_key(&self) -> FLASH_BOOTCFG_KEYR {
        let bits = ((self.bits >> 4) & 1) != 0;
        FLASH_BOOTCFG_KEYR { bits }
    }
    #[doc = "Bit 8 - Boot GPIO Enable"]
    #[inline(always)]
    pub fn flash_bootcfg_en(&self) -> FLASH_BOOTCFG_ENR {
        let bits = ((self.bits >> 8) & 1) != 0;
        FLASH_BOOTCFG_ENR { bits }
    }
    #[doc = "Bit 9 - Boot GPIO Polarity"]
    #[inline(always)]
    pub fn flash_bootcfg_pol(&self) -> FLASH_BOOTCFG_POLR {
        let bits = ((self.bits >> 9) & 1) != 0;
        FLASH_BOOTCFG_POLR { bits }
    }
    #[doc = "Bits 10:12 - Boot GPIO Pin"]
    #[inline(always)]
    pub fn flash_bootcfg_pin(&self) -> FLASH_BOOTCFG_PINR {
        FLASH_BOOTCFG_PINR::_from(((self.bits >> 10) & 7) as u8)
    }
    #[doc = "Bits 13:15 - Boot GPIO Port"]
    #[inline(always)]
    pub fn flash_bootcfg_port(&self) -> FLASH_BOOTCFG_PORTR {
        FLASH_BOOTCFG_PORTR::_from(((self.bits >> 13) & 7) as u8)
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline(always)]
    pub fn flash_bootcfg_nw(&self) -> FLASH_BOOTCFG_NWR {
        let bits = ((self.bits >> 31) & 1) != 0;
        FLASH_BOOTCFG_NWR { bits }
    }
}
impl W {
    #[doc = r"Writes raw bits to the register"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bit 0 - Debug Control 0"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg0(&mut self) -> _FLASH_BOOTCFG_DBG0W {
        _FLASH_BOOTCFG_DBG0W { w: self }
    }
    #[doc = "Bit 1 - Debug Control 1"]
    #[inline(always)]
    pub fn flash_bootcfg_dbg1(&mut self) -> _FLASH_BOOTCFG_DBG1W {
        _FLASH_BOOTCFG_DBG1W { w: self }
    }
    #[doc = "Bit 4 - KEY Select"]
    #[inline(always)]
    pub fn flash_bootcfg_key(&mut self) -> _FLASH_BOOTCFG_KEYW {
        _FLASH_BOOTCFG_KEYW { w: self }
    }
    #[doc = "Bit 8 - Boot GPIO Enable"]
    #[inline(always)]
    pub fn flash_bootcfg_en(&mut self) -> _FLASH_BOOTCFG_ENW {
        _FLASH_BOOTCFG_ENW { w: self }
    }
    #[doc = "Bit 9 - Boot GPIO Polarity"]
    #[inline(always)]
    pub fn flash_bootcfg_pol(&mut self) -> _FLASH_BOOTCFG_POLW {
        _FLASH_BOOTCFG_POLW { w: self }
    }
    #[doc = "Bits 10:12 - Boot GPIO Pin"]
    #[inline(always)]
    pub fn flash_bootcfg_pin(&mut self) -> _FLASH_BOOTCFG_PINW {
        _FLASH_BOOTCFG_PINW { w: self }
    }
    #[doc = "Bits 13:15 - Boot GPIO Port"]
    #[inline(always)]
    pub fn flash_bootcfg_port(&mut self) -> _FLASH_BOOTCFG_PORTW {
        _FLASH_BOOTCFG_PORTW { w: self }
    }
    #[doc = "Bit 31 - Not Written"]
    #[inline(always)]
    pub fn flash_bootcfg_nw(&mut self) -> _FLASH_BOOTCFG_NWW {
        _FLASH_BOOTCFG_NWW { w: self }
    }
}
