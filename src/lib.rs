pub extern crate rppal;

use rppal::i2c::I2c;
use std::{thread, time};
use serde::{Deserialize, Serialize};

//ADS1115 register map
#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115RegisterMap {
    Ads1115RegPointerConvert = 0x00,
    // Conversion register
    Ads1115RegPointerConfig = 0x01,
    // Configuration register
    Ads1115RegPointerLowthresh = 0x02,
    // Lo_thresh register
    Ads1115RegPointerHithresh = 0x03, // Hi_thresh register
}


//ADS1115 Configuration Register
#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115MuxConfig {
    Ads1115RegConfigMuxDiff01 = 0x00,
    // Differential P = AIN0, N = AIN1 (default)
    Ads1115RegConfigMuxDiff03 = 0x10,
    // Differential P = AIN0, N = AIN3
    Ads1115RegConfigMuxDiff13 = 0x20,
    // Differential P = AIN1, N = AIN3
    Ads1115RegConfigMuxDiff23 = 0x30,
    // Differential P = AIN2, N = AIN3
    Ads1115RegConfigMuxSingle0 = 0x40,
    // Single-ended P = AIN0, N = GND
    Ads1115RegConfigMuxSingle1 = 0x50,
    // Single-ended P = AIN1, N = GND
    Ads1115RegConfigMuxSingle2 = 0x60,
    // Single-ended P = AIN2, N = GND
    Ads1115RegConfigMuxSingle3 = 0x70, // Single-ended P = AIN3, N = GND
}

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115PgaConfig {
    Ads1115RegConfigPga6144v = 0x00,
    // +/-6.144V range = Gain 2/3
    Ads1115RegConfigPga4096v = 0x02,
    // +/-4.096V range = Gain 1
    Ads1115RegConfigPga2048v = 0x04,
    // +/-2.048V range = Gain 2 (default)
    Ads1115RegConfigPga1024v = 0x06,
    // +/-1.024V range = Gain 4
    Ads1115RegConfigPga0512v = 0x08,
    // +/-0.512V range = Gain 8
    Ads1115RegConfigPga0256v = 0x0A, // +/-0.256V range = Gain 16
}

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115ModeConfig {
    Ads1115RegConfigModeContin = 0x00,
    // Continuous conversion mode
    Ads1115RegConfigModeSingle = 0x01, // Power-down single-shot mode (default)
}

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115SampleRateConfig {
    Ads1115RegConfigDr8sps = 0x00,
    // 8 samples per second
    Ads1115RegConfigDr16sps = 0x20,
    // 16 samples per second
    Ads1115RegConfigDr32sps = 0x40,
    // 32 samples per second
    Ads1115RegConfigDr64sps = 0x60,
    // 64 samples per second
    Ads1115RegConfigDr128sps = 0x80,
    // 128 samples per second (default)
    Ads1115RegConfigDr250sps = 0xA0,
    // 250 samples per second
    Ads1115RegConfigDr475sps = 0xC0,
    // 475 samples per second
    Ads1115RegConfigDr860sps = 0xE0, // 860 samples per second
}

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115ComparatorModeConfig {
    Ads1115RegConfigCmodeTrad = 0x00,
    // Traditional comparator with hysteresis (default)
    Ads1115RegConfigCmodeWindow = 0x10, // Window comparator
}

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115OsConfig {
    Ads1115RegConfigOsNoeffect = 0x00,
    // No effect
    Ads1115RegConfigOsSingle = 0x80, // Begin a single conversion
}

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115CQConfig {
    Ads1115RegConfigCque1conv = 0x00,
    // Assert ALERT/RDY after one conversions
    Ads1115RegConfigCque2conv = 0x01,
    // Assert ALERT/RDY after two conversions
    Ads1115RegConfigCque4conv = 0x02,
    // Assert ALERT/RDY after four conversions
    Ads1115RegConfigCqueNone = 0x03, // Disable the comparator and put ALERT/RDY in high state (default)
}


#[repr(u16)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115Address {
    I2c48 = 0x48,
    I2c49 = 0x49,
}

#[repr(u8)]
#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum Ads1115Channel {
    Chan0 = 0,
    Chan1 = 1,
    Chan2 = 2,
    Chan3 = 3,
}

pub struct ADS1115 {
    gain: Option<Ads1115PgaConfig>,
    coefficient: Option<f64>,
}

impl ADS1115 {
    pub fn new(address: Ads1115Address, gain: Ads1115PgaConfig, i2c: &mut rppal::i2c::I2c) -> ADS1115 {
        let mut ads = ADS1115 {
            gain: None,
            coefficient: None,
        };
        ads.set_address(address, i2c);
        ads.set_gain(gain);

        ads
    }

    fn set_gain(&mut self, gain: Ads1115PgaConfig) -> Option<f64> {
        match gain {
            Ads1115PgaConfig::Ads1115RegConfigPga6144v => self.coefficient = Some(0.1875),
            Ads1115PgaConfig::Ads1115RegConfigPga4096v => self.coefficient = Some(0.125),
            Ads1115PgaConfig::Ads1115RegConfigPga2048v => self.coefficient = Some(0.0625),
            Ads1115PgaConfig::Ads1115RegConfigPga1024v => self.coefficient = Some(0.03125),
            Ads1115PgaConfig::Ads1115RegConfigPga0512v => self.coefficient = Some(0.015625),
            Ads1115PgaConfig::Ads1115RegConfigPga0256v => self.coefficient = Some(0.0078125),
        }
        self.gain = Some(gain);
        self.coefficient
    }

    fn set_address(&mut self, address: Ads1115Address, i2c: &mut rppal::i2c::I2c) -> bool {
        return !i2c.set_slave_address(address as u16).is_err();
    }


    fn configure_single(&self, channel: Ads1115Channel, i2c: &mut rppal::i2c::I2c) -> Result<(), &str> {
        match self.gain {
            Some(gain) => {
                let mut config = [(Ads1115OsConfig::Ads1115RegConfigOsSingle as u8) | gain as u8 | (Ads1115ModeConfig::Ads1115RegConfigModeContin as u8), (Ads1115SampleRateConfig::Ads1115RegConfigDr250sps as u8) | (Ads1115CQConfig::Ads1115RegConfigCqueNone as u8)];
                match channel {
                    Ads1115Channel::Chan0 => config[0] = config[0] | (Ads1115MuxConfig::Ads1115RegConfigMuxSingle0 as u8),
                    Ads1115Channel::Chan1 => config[0] = config[0] | (Ads1115MuxConfig::Ads1115RegConfigMuxSingle1 as u8),
                    Ads1115Channel::Chan2 => config[0] = config[0] | (Ads1115MuxConfig::Ads1115RegConfigMuxSingle2 as u8),
                    Ads1115Channel::Chan3 => config[0] = config[0] | (Ads1115MuxConfig::Ads1115RegConfigMuxSingle3 as u8),
                }
                if i2c.block_write(Ads1115RegisterMap::Ads1115RegPointerConfig as u8, &config).is_err() {
                    return Err("failed to write data to i2c bus. Check that you're using the correct address");
                }
            }
            None => return Err("gain is none. Set the gain")
        };

        Ok(())
    }

    fn read_value(&self, i2c: &mut rppal::i2c::I2c) -> Result<u16, &str> {
        return match self.coefficient {
            Some(coef) => {
                let mut data = [0u8; 2];
                if i2c.block_read(Ads1115RegisterMap::Ads1115RegPointerConvert as u8, &mut data).is_err() {
                    return Err("couldn't read data");
                }

                let mut raw_adc: u16 = data[0] as u16 * 256 + data[1] as u16;
                if raw_adc > 32767 {
                    raw_adc -= 65535;
                }

                let scaled: u16 = (raw_adc as f64 * coef) as u16;
                Ok(scaled)
            }
            None => Err("couldn't read data. Gain has not been set")
        };
    }

    pub fn read_voltage(&mut self, channel: Ads1115Channel, i2c: &mut rppal::i2c::I2c) -> Result<u16, &str> {
        if let Err(error) = self.configure_single(channel, i2c) {
            return Err(error);
        }
        let hundred_millies = time::Duration::from_millis(75); //allow config to take effect
        thread::sleep(hundred_millies);
        self.read_value(i2c)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut i2c = I2c::new().unwrap();
        let mut ads = ADS1115::new(Ads1115Address::I2c49, Ads1115PgaConfig::Ads1115RegConfigPga6144v, &mut i2c);
        let ads0 = ads.read_voltage(Ads1115Channel::Chan0, &mut i2c);
        assert!(ads0.is_ok());
        let ads1 = ads.read_voltage(Ads1115Channel::Chan1, &mut i2c);
        assert!(ads1.is_ok());
        let ads2 = ads.read_voltage(Ads1115Channel::Chan2, &mut i2c);
        assert!(ads2.is_ok());
        let ads3 = ads.read_voltage(Ads1115Channel::Chan3, &mut i2c);
        assert!(ads3.is_ok());
    }
}
