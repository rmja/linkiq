use bcd::BcdNumber;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct WMBusAddress {
    pub manufacturer_code: u16,
    pub serial_number: BcdNumber<4>,
    pub version: u8,
    pub device_type: u8,
}

impl WMBusAddress {
    pub fn new(manufacturer_code: u16, serial_number: u32, version: u8, device_type: u8) -> Self {
        Self {
            manufacturer_code,
            serial_number: BcdNumber::from_u32(serial_number),
            version,
            device_type,
        }
    }

    pub fn get_bytes(&self) -> [u8; 8] {
        let mut bytes = [0; 8];
        bytes[0..2].copy_from_slice(self.manufacturer_code.to_le_bytes().as_ref());

        let mut index = 2;
        for byte in self.serial_number.into_iter().rev() {
            bytes[index] = byte;
            index += 1;
        }

        assert_eq!(6, index);
        bytes[6] = self.version;
        bytes[7] = self.device_type;

        bytes
    }
}

impl TryFrom<&[u8]> for WMBusAddress {
    type Error = ();

    fn try_from(value: &[u8]) -> Result<Self, Self::Error> {
        if value.len() < 8 {
            return Err(());
        }

        let manufacturer_code = u16::from_le_bytes(value[0..2].try_into().map_err(|_| ())?);
        let mut serial_number = [0; 4];
        serial_number.copy_from_slice(&value[2..6]);
        serial_number.reverse();
        let serial_number = BcdNumber::try_from(serial_number).map_err(|_| ())?;
        let version = value[6];
        let device_type = value[7];
        Ok(WMBusAddress {
            manufacturer_code,
            serial_number,
            version,
            device_type,
        })
    }
}
