#[derive(Copy, Clone)]
pub struct Lighting {
    value: u8,
}

impl Default for Lighting {
    fn default() -> Self {
        Lighting {
            value: 0,
        }
    }
}

impl Lighting {
    pub fn byte(&self) -> u8 {
        self.value
    }

    pub fn set_current(&mut self, value: LightingCurrent) -> &mut Self {
        self.set_bits(0b1111, value as u8);
        self
    }

    pub fn set_audio_gain(&mut self, value: LightingAudioGain) -> &mut Self {
        self.set_bits(0b111_0000, value as u8);
        self
    }

    fn set_bits(&mut self, mask: u8, value: u8) {
        self.value = (self.value & !mask) | value;
    }
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum LightingCurrent {
    Current5mA  = 0b1000,
    Current10mA = 0b1001,
    Current15mA = 0b1010,
    Current20mA = 0b1011,
    Current25mA = 0b1100,
    Current30mA = 0b1101,
    Current35mA = 0b1110,
    Current40mA = 0b0000,
    Current45mA = 0b0001,
    Current50mA = 0b0010,
    Current55mA = 0b0011,
    Current60mA = 0b0100,
    Current65mA = 0b0101,
    Current70mA = 0b0110,
    Current75mA = 0b0111,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum LightingAudioGain {
    /// 0dB
    Gain0dB = 0b000_0000,
    /// +3dB
    Gain3dB = 0b001_0000,
    /// +6dB
    Gain6dB = 0b010_0000,
    /// +9dB
    Gain9dB = 0b011_0000,
    /// +12dB
    Gain12dB = 0b100_0000,
    /// +15dB
    Gain15dB = 0b101_0000,
    /// +18dB
    Gain18dB = 0b110_0000,
    /// -6dB
    GainMinus6dB = 0b111_0000,
}