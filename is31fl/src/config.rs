#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Register {
    /// Configuration Register, default 0000 0000
    Config = 0x00,
    /// Lighting Effect Register, Store the intensity control settings, default 0000 0000
    LightingEffect = 0x0d,
    /// Matrix 1 Data Register, Store the on or off state of each LED, default 0000 0000
    Matrix1Begin = 0x01,
    Matrix1End = 0x0b,
    /// Matrix 2 Data Register, Store the on or off state of each LED, default 0000 0000
    Matrix2Begin = 0x0e,
    Matrix2End = 0x18,
    /// Update Column Register
    UpdateColumn = 0x0c,
    /// PWM Register, Modulate LED light with 128 different items, default 1000 0000
    Pwm = 0x19,
    /// Reset Register, Reset all registers to default value
    Reset = 0xff,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ConfigMask {
    /// Software Shutdown Enable
    SoftwareShutdown = 0b10000000,
    /// Display Mode
    DisplayMode = 0b00011000,
    /// Audio Input Enable
    Audio = 0b00000100,
    /// Matrix Mode Selection
    MatrixMode = 0b00000011,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ConfigAudio {
    /// Matrix intensity is controlled  by  the  current setting in the Lighting Effect Register
    LightingEffect = 0b00000000,
    /// Enable audio signal to modulate the intensity of the matrix
    Signal = 0b00000100,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ConfigDisplayMode {
    /// Matrix 1 only
    Matrix1Only = 0b00000000,
    /// Matrix 2 only
    Matrix2Only = 0b00001000,
    /// Matrix 1 and Matrix 2
    Matrix1and2 = 0b00011000,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum ConfigMatrixMode {
    Size8x8 = 0b00000000,
    Size7x9 = 0b00000001,
    Size6x10 = 0b00000010,
    Size5x11 = 0b00000011,
}

#[derive(Copy, Clone)]
#[repr(u8)]
pub enum Current {
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
pub struct Configuration {
    configuration: u8,
}

impl Configuration {
    pub fn byte(&self) -> u8 {
        self.configuration
    }

    pub fn set_matrix_mode(&mut self, matrix_mode: ConfigMatrixMode) -> &mut Self {
        self.set_bits(ConfigMask::MatrixMode as u8, matrix_mode as u8);
        self
    }

    pub fn set_audio_input_enable(&mut self, value: bool) -> &mut Self {
        self.set_bits(ConfigMask::Audio as u8, value as u8);
        self
    }

    pub fn set_display_mode(&mut self, display_mode: ConfigDisplayMode) -> &mut Self {
        self.set_bits(ConfigMask::DisplayMode as u8, display_mode as u8);
        self
    }

    pub fn set_software_shutdown(&mut self, value: bool) -> &mut Self {
        self.set_bits(ConfigMask::SoftwareShutdown as u8, value as u8);
        self
    }

    fn set_bits(&mut self, mask: u8, value: u8) {
        self.configuration = (self.configuration & !mask) | value;
    }
}

impl Default for Configuration {
    fn default() -> Self {
        Configuration {
            configuration: 0b00000000,
        }
    }
}