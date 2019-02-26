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