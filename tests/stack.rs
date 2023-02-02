use self::examples::*;
use assert_hex::assert_eq_hex;
use bitvec::prelude::*;
use heapless::Vec;
use linkiq::stack::{
    mbal::{self, MbalFunctionCode},
    phl, Packet, Stack,
};
use rand::prelude::*;

#[test]
fn can_read_examples() {
    can_read_example_case(&EXAMPLE41, 0, 0, 0, 0.00);
    can_read_example_case(&EXAMPLE41, 0, 1, 2, 0.01);
    can_read_example_case(&EXAMPLE41, 1, 1, 8, 0.02);
    can_read_example_case(&EXAMPLE41, 2, 1, 11, 0.03);
    can_read_example_case(&EXAMPLE41, 3, 1, 19, 0.04);
    can_read_example_case(&EXAMPLE41, 4, 2, 24, 0.05);
    can_read_example_case(&EXAMPLE41, 4, 2, 25, 0.06);
    can_read_example_case(&EXAMPLE41, 5, 4, 32, 0.07);
    can_read_example_case(&EXAMPLE42, 0, 0, 0, 0.00);
    can_read_example_case(&EXAMPLE42, 0, 1, 5, 0.01);
    can_read_example_case(&EXAMPLE42, 1, 1, 15, 0.02);
    can_read_example_case(&EXAMPLE42, 2, 1, 24, 0.03);
    can_read_example_case(&EXAMPLE42, 3, 1, 41, 0.04);
    can_read_example_case(&EXAMPLE42, 4, 2, 48, 0.05);
    can_read_example_case(&EXAMPLE42, 4, 2, 53, 0.06);
    can_read_example_case(&EXAMPLE42, 5, 4, 63, 0.07);
    can_read_example_case(&EXAMPLE42, 7, 5, 71, 0.08);
    can_read_example_case(&EXAMPLE43, 0, 0, 0, 0.00);
    can_read_example_case(&EXAMPLE43, 0, 1, 3, 0.01);
    can_read_example_case(&EXAMPLE43, 1, 1, 11, 0.02);
    can_read_example_case(&EXAMPLE43, 2, 1, 14, 0.03);
    can_read_example_case(&EXAMPLE43, 3, 1, 23, 0.04);
    can_read_example_case(&EXAMPLE43, 4, 1, 28, 0.05);
    can_read_example_case(&EXAMPLE43, 4, 1, 31, 0.06);
    can_read_example_case(&EXAMPLE43, 5, 2, 38, 0.07);
    can_read_example_case(&EXAMPLE43, 7, 2, 44, 0.08);
    can_read_example_case(&EXAMPLE43, 7, 1, 51, 0.09);
    can_read_example_case(&EXAMPLE43, 8, 2, 62, 0.10);
    can_read_example_case(&EXAMPLE43, 9, 2, 66, 0.11);
    can_read_example_case(&EXAMPLE43, 9, 2, 71, 0.12);
    can_read_example_case(&EXAMPLE43, 10, 3, 76, 0.13);
    can_read_example_case(&EXAMPLE43, 11, 8, 85, 0.14);
    can_read_example_case(&EXAMPLE44, 0, 0, 0, 0.00);
    can_read_example_case(&EXAMPLE44, 0, 1, 2, 0.01);
    can_read_example_case(&EXAMPLE44, 1, 1, 8, 0.02);
    can_read_example_case(&EXAMPLE44, 2, 1, 11, 0.03);
    can_read_example_case(&EXAMPLE44, 3, 1, 19, 0.04);
    can_read_example_case(&EXAMPLE44, 4, 1, 24, 0.05);
    can_read_example_case(&EXAMPLE44, 4, 1, 25, 0.06);
    can_read_example_case(&EXAMPLE44, 5, 1, 32, 0.07);
    can_read_example_case(&EXAMPLE44, 7, 1, 37, 0.08);
    can_read_example_case(&EXAMPLE44, 7, 1, 43, 0.09);
    can_read_example_case(&EXAMPLE44, 8, 2, 51, 0.10);
    can_read_example_case(&EXAMPLE44, 9, 2, 54, 0.11);
    can_read_example_case(&EXAMPLE44, 9, 2, 58, 0.12);
    can_read_example_case(&EXAMPLE44, 10, 2, 62, 0.13);
    can_read_example_case(&EXAMPLE44, 11, 5, 69, 0.14);
}

fn can_read_example_case(
    vector: &ExampleVector,
    header_distance: usize,
    decode_iterations: usize,
    decode_distance: usize,
    ber: f64,
) {
    // Given
    let stack = Stack::new();
    let mut frame = Vec::<u8, 400>::from_slice(vector.frame).unwrap();

    if ber > 0.0 {
        let mut rng = rand_chacha::ChaCha8Rng::seed_from_u64(0x1337);
        let slice = BitSlice::<u8, Lsb0>::from_slice_mut(&mut frame);

        for mut bit in slice.iter_mut() {
            if rng.gen::<f64>() < ber {
                *bit = !*bit;
            }
        }
    }

    // When
    let packet: Packet<300> = stack.read(&frame).unwrap();

    // Then
    assert_eq!(vector.frame.len(), phl::get_frame_length(&frame).unwrap());

    let phl = packet.phl.unwrap();
    assert_eq!(vector.code_rate, phl.code_rate);
    assert_eq!(
        (header_distance, decode_iterations, decode_distance),
        (
            phl.header_distance,
            phl.decode_iterations,
            phl.decode_distance
        )
    );

    let mbal = packet.mbal.unwrap();
    assert_eq!(vector.is_prioritized, mbal.control.is_prioritized);
    assert_eq!(vector.address, mbal.address);
    assert_eq!(
        vector.is_installation,
        mbal.command.function_code == MbalFunctionCode::SendInstallationRequest
    );

    assert_eq!(vector.mbus_data, packet.mbus_data);
}

#[test]
fn can_write_examples() {
    can_write_example_case(&EXAMPLE41);
    can_write_example_case(&EXAMPLE42);
    can_write_example_case(&EXAMPLE43);
    can_write_example_case(&EXAMPLE44);
}

fn can_write_example_case(vector: &ExampleVector) {
    // Given
    let stack = Stack::new();
    let mut writer = Vec::<u8, 400>::new();
    let packet: Packet<300> = Packet {
        rssi: None,
        phl: Some(phl::PhlFields {
            code_rate: vector.code_rate,
            header_distance: 0,
            decode_iterations: 0,
            decode_distance: 0,
        }),
        mbal: Some(mbal::MbalFields {
            control: mbal::MbalControl {
                is_prioritized: vector.is_prioritized,
            },
            address: vector.address,
            command: match vector.is_installation {
                true => mbal::MbalCommand {
                    function_code: mbal::MbalFunctionCode::SendInstallationRequest,
                },
                false => mbal::MbalCommand {
                    function_code: mbal::MbalFunctionCode::SendUnsolicitedApplicationData,
                },
            },
        }),
        mbus_data: Vec::from_slice(vector.mbus_data).unwrap(),
    };

    // When
    stack.write(&mut writer, &packet).unwrap();

    // Then
    assert_eq_hex!(vector.frame, &writer);
}

#[rustfmt::skip]
mod examples {
    use once_cell::sync::Lazy;
    use linkiq::{fec::CodeRate, wmbus::WMBusAddress};

    pub struct ExampleVector {
        pub name: &'static str,
        pub code_rate: CodeRate,
        pub is_prioritized: bool,
        pub is_installation: bool,
        pub address: WMBusAddress,
        pub mbus_data: &'static [u8],
        pub frame: &'static [u8],
    }

    /// <summary>
    /// Water meter installation
    /// </summary>
    pub static EXAMPLE41: Lazy<ExampleVector> = Lazy::new(|| ExampleVector
    {
        name: "Example 4.1",
        code_rate: CodeRate::OneHalf,
        is_prioritized: true,
        is_installation: true,
        address: WMBusAddress::new(0x2c37, 12341234, 27, 0x16),
        mbus_data: 
        &[
            0x7A, 0x01, 0x00, 0x20, 0x05, 0x19, 0x32,
            0x29, 0xBC, 0xE6, 0x4D, 0x65, 0x1F, 0x1D, 0xED, 0x42, 0x68, 0x73, 0x03, 0xB2, 0x9A, 0xF6, 0xA6, 0x80, 0x53, 0x36, 0x08, 0x4A, 0x0C, 0xC4, 0xB4, 0xB9, 0x23, 0x71, 0xA3, 0xCA, 0xB9,
        ],
        frame:
        &[
            0xCC,
            0x48, 0xDE, 0x49, 0x5C, 0xD1, 0x75, 0x12, 0x40, 0x2F, 0x09, 0x32,
            0x01, 0x37, 0x2C, 0x34, 0x12, 0x34, 0x12, 0x1B, 0x16,
            0x60,
            0x16, 0x61,
            0x7A, 0x01, 0x00, 0x20, 0x05,
            0x19, 0x32, 0x29, 0xBC, 0xE6, 0x4D, 0x65, 0x1F, 0x1D, 0xED, 0x42, 0x68, 0x73, 0x03, 0xB2, 0x9A, 0xF6, 0xA6, 0x80, 0x53, 0x36, 0x08, 0x4A, 0x0C, 0xC4, 0xB4, 0xB9, 0x23, 0x71, 0xA3, 0xCA, 0xB9,
            0xFC, 0x9B, 0x4F, 0xFE,
            0x09, 0xD3, 0x5F, 0xE3, 0xFB, 0x1E, 0x3B, 0x5A, 0x49, 0xA7, 0x1A, 0x34, 0x24, 0x39, 0x87, 0x30, 0x07, 0xBD, 0x8E, 0x41, 0x78, 0x77, 0x7A, 0x82, 0x7C, 0x72, 0x3B, 0x81, 0x49, 0xBE, 0x18, 0x74, 0x50, 0x08, 0xDB, 0x6E, 0x1F, 0x01, 0x33, 0x14, 0x96, 0x79, 0xAC, 0x67, 0xA4, 0xE3, 0xFA, 0x08, 0x38, 0x42, 0x99, 0x18, 0x31,
        ]
    });

    /// Heat meter
    pub static EXAMPLE42: Lazy<ExampleVector> = Lazy::new(|| ExampleVector
    {
        name: "Example 4.2",
        code_rate: CodeRate::OneHalf,
        is_prioritized: false,
        is_installation: false,
        address: WMBusAddress::new(0x2c2d, 71006389, 0x34, 0x04),
        mbus_data:
        &[
            0x90, 0x0F, 0x00, 0x2C, 0x25, 0x45, 0x42, 0x01, 0x00, 0xC9, 0xFE, 0x78, 0x01, 0x18, 0xB7, 0xE8, 0x31,
            0x7A, 0x12, 0x18, 0x40, 0x07, 0x10,
            0x35, 0xCD, 0x99, 0x1D, 0xE9, 0xC5, 0x3C, 0x5D, 0xCC, 0x31, 0x05, 0x01, 0x87, 0x82, 0xD7, 0x2D, 0x1C, 0xDB, 0x39, 0xC5, 0xDB, 0x1B, 0x7C, 0x21, 0x82, 0x05, 0x7E, 0x19, 0x35, 0xD7, 0x73, 0xAF, 0xDA, 0xAA, 0x24, 0xF4, 0xFA, 0x17, 0x38, 0xE2, 0xBD, 0x8B, 0x13, 0xF3, 0xFC, 0x77, 0xA3, 0x2B, 0x68, 0xF1, 0xD1, 0x2E, 0x73, 0x66, 0xFE, 0xC6, 0x1D, 0x69, 0xD7, 0xE7, 0x81, 0xC2, 0x88, 0x65,
        ],
        frame:
        &[
            0xD8,
            0xD1, 0xE7, 0x09, 0xAF, 0x91, 0x9E, 0x11, 0x67, 0x79, 0x0E, 0x64,
            0x00, 0x2D, 0x2C, 0x89, 0x63, 0x00, 0x71, 0x34, 0x04,
            0x40,
            0x90, 0x01,
            0x90, 0x0F, 0x00, 0x2C, 0x25, 0x45, 0x42, 0x01, 0x00, 0xC9, 0xFE, 0x78, 0x01, 0x18, 0xB7, 0xE8, 0x31,
            0x7A, 0x12, 0x18, 0x40, 0x07, 0x10,
            0x35, 0xCD, 0x99, 0x1D, 0xE9, 0xC5, 0x3C, 0x5D, 0xCC, 0x31, 0x05, 0x01, 0x87, 0x82, 0xD7, 0x2D, 0x1C, 0xDB, 0x39, 0xC5, 0xDB, 0x1B, 0x7C, 0x21, 0x82, 0x05, 0x7E, 0x19, 0x35, 0xD7, 0x73, 0xAF, 0xDA, 0xAA, 0x24, 0xF4, 0xFA, 0x17, 0x38, 0xE2, 0xBD, 0x8B, 0x13, 0xF3, 0xFC, 0x77, 0xA3, 0x2B, 0x68, 0xF1, 0xD1, 0x2E, 0x73, 0x66, 0xFE, 0xC6, 0x1D, 0x69, 0xD7, 0xE7, 0x81, 0xC2, 0x88, 0x65,
            0x53, 0x88, 0x24, 0xCE,
            0x04, 0x85, 0xF9, 0x43, 0x6D, 0x25, 0xED, 0xCF, 0xEB, 0xC7, 0xED, 0x35, 0x22, 0x39, 0x7F, 0x29, 0x4B, 0x44, 0x01, 0xFB, 0xAB, 0xEB, 0xC6, 0x73, 0xBD, 0xDA, 0xF1, 0xFA, 0xAA, 0x8B, 0x49, 0xB4, 0x10, 0x81, 0x7C, 0x15, 0xDF, 0x0D, 0xFE, 0x1B, 0xAA, 0x11, 0xCF, 0x05, 0xCF, 0x5F, 0x64, 0x31, 0x15, 0x34, 0xEC, 0x65, 0x5E, 0x0C, 0x96, 0x6C, 0x93, 0xC8, 0x54, 0xBE, 0x53, 0x76, 0xDB, 0xD1, 0x85, 0x6D, 0x15, 0x9D, 0xC7, 0x7A, 0x94, 0xCC, 0xDA, 0xE1, 0xE6, 0x9C, 0x74, 0x47, 0x63, 0xB2, 0xBD, 0xAA, 0xFF, 0xF5, 0xBC, 0xD5, 0xEC, 0xDA, 0x2D, 0xBA, 0xD8, 0xDB, 0x75, 0x60, 0x1C, 0x39, 0xF0, 0x4D, 0x6F, 0xD4, 0x33, 0x88, 0xFB,
        ]
    });

    /// Combined humidity and temperature sensor
    pub static EXAMPLE43: Lazy<ExampleVector> = Lazy::new(|| ExampleVector
    {
        name: "Example 4.3",
        code_rate: CodeRate::OneThird,
        is_prioritized: false,
        is_installation: false,
        address: WMBusAddress::new(0x2c2d, 05040302, 6, 0x00),
        mbus_data:
        &[
            0x7A, 0x22, 0xAB, 0xFF, 0x2A, 0x10, 0x01, 0xFF, 0xEE, 0xDD, 0xCC,
            0xE6, 0x0D, 0x1F, 0x01, 0xDA, 0xB0, 0xE2, 0x83, 0x2A, 0x65, 0x18, 0x00, 0x3E, 0xE7, 0x42, 0x4E, 0xE8, 0x65, 0xDF, 0xEE, 0x22, 0x53, 0xC0, 0xD6, 0x35, 0xEE, 0xE6, 0x69, 0x77, 0xF4, 0x20, 0x4B, 0xA9, 0x3F, 0xD3, 0x44, 0x1C,
        ],
        frame:
        &[
            0xCF,
            0x0A, 0x89, 0x13, 0x5B, 0x52, 0xC6, 0x52, 0xF2, 0xF2, 0x1B, 0xD6,
            0x00, 0x2D, 0x2C, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00,
            0x40,
            0xC0, 0xBE,
            0x7A, 0x22, 0xAB, 0xFF, 0x2A, 0x10, 0x01, 0xFF, 0xEE, 0xDD, 0xCC,
            0xE6, 0x0D, 0x1F, 0x01, 0xDA, 0xB0, 0xE2, 0x83, 0x2A, 0x65, 0x18, 0x00, 0x3E, 0xE7, 0x42, 0x4E, 0xE8, 0x65, 0xDF, 0xEE, 0x22, 0x53, 0xC0, 0xD6, 0x35, 0xEE, 0xE6, 0x69, 0x77, 0xF4, 0x20, 0x4B, 0xA9, 0x3F, 0xD3, 0x44, 0x1C,
            0xE5, 0x74, 0xFB, 0x6D,
            0x00, 0x35, 0xD1, 0xC8, 0x5E, 0x90, 0xBF, 0x04, 0x5C, 0xC0, 0x8B, 0x4C, 0x0B, 0xF4, 0x26, 0x34, 0xFB, 0xD5, 0xCA, 0xD1, 0xBC, 0xC0, 0xAD, 0xC1, 0x9E, 0x66, 0x2F, 0x20, 0x8C, 0x0D, 0x67, 0xD5, 0xD4, 0x86, 0x5C, 0x90, 0x57, 0x26, 0x72, 0xB8, 0x43, 0x26, 0x92, 0x3F, 0x17, 0x6E, 0xCD, 0x0A, 0x77, 0x78, 0xAE, 0x95, 0x17, 0xAB, 0xE3, 0x9C, 0x06, 0xB9, 0xC7, 0x81, 0x7A, 0x97, 0x33, 0x10, 0x22, 0x04, 0xA5, 0xE6, 0x78, 0x32, 0x50, 0xA0, 0xA6, 0x81, 0x19, 0x84, 0x92, 0x01, 0xC8, 0x02, 0xF2, 0xD9, 0x48, 0x6B, 0x44, 0xBB, 0xA8, 0xCE, 0x91, 0xC6, 0x78, 0xA7, 0x6A, 0x7A, 0xBC, 0xE1, 0xF9, 0xF3, 0xDD, 0x29, 0xCF, 0xC9, 0xB1, 0x07, 0x5D, 0x88, 0x5B, 0x3D, 0x98, 0x83, 0x26, 0x5F, 0x8A, 0x70, 0xBD, 0xC7, 0x18, 0xC4, 0xBB, 0x22, 0x00, 0x90, 0xED, 0x2D, 0xA6, 0x3F, 0xAD, 0x02,
        ]
    });

    /// Unencrypted sensor device
    pub static EXAMPLE44: Lazy<ExampleVector> = Lazy::new(|| ExampleVector
    {
        name: "Example 4.4",
        code_rate: CodeRate::OneThird,
        is_prioritized: false,
        is_installation: false,
        address: WMBusAddress::new(0x2c2d, 05040302, 6, 0x00),
        mbus_data:
        &[
            0x7A, 0x2A, 0x00, 0x00, 0x00,
            0x0D, 0xFD, 0x09, 0xE3, 0x0A, 0x03, 0x01, 0x41, 0x7C, 0x03, 0x34, 0x53, 0x44, 0x0D, 0x42, 0x66, 0x1B, 0x01, 0x42, 0xFB, 0x1A, 0x42, 0x02, 0x44, 0x6D, 0x1E, 0x29, 0xAB, 0x23,
        ],
        frame:
        &[
            0xCB,
            0x8D, 0xEC, 0xD3, 0xA9, 0xD2, 0x33, 0x10, 0x0B, 0xC0, 0x1E, 0x56,
            0x00, 0x2D, 0x2C, 0x02, 0x03, 0x04, 0x05, 0x06, 0x00,
            0x40,
            0xC0, 0xBE,
            0x7A, 0x2A, 0x00, 0x00, 0x00,
            0x0D, 0xFD, 0x09, 0xE3, 0x0A, 0x03, 0x01, 0x41, 0x7C, 0x03, 0x34, 0x53, 0x44, 0x0D, 0x42, 0x66, 0x1B, 0x01, 0x42, 0xFB, 0x1A, 0x42, 0x02, 0x44, 0x6D, 0x1E, 0x29, 0xAB, 0x23,
            0x7A, 0x0E, 0x72, 0xF2,
            0x00, 0x35, 0xD1, 0xC8, 0x5E, 0x90, 0xBF, 0x04, 0x5C, 0xC0, 0x8B, 0x4C, 0x0B, 0xFB, 0xCB, 0x97, 0x2E, 0x55, 0x45, 0xEB, 0xB5, 0x0C, 0xE7, 0xE4, 0x56, 0x98, 0xBB, 0x79, 0xAE, 0xE9, 0xC2, 0xB1, 0x6F, 0xD8, 0xCA, 0x08, 0xF8, 0xD9, 0x54, 0xE6, 0x7E, 0xA9, 0x49, 0xA5, 0x9F, 0xDB, 0x0B, 0xC0, 0xBD, 0x3D, 0x2D, 0xC5, 0x9D, 0xDB, 0x6F, 0x84, 0x71, 0x7A, 0x28, 0x77, 0xEB, 0x11, 0x3E, 0xF4, 0x71, 0xC7, 0x6C, 0xF7, 0x19, 0x0B, 0xD0, 0xF2, 0x89, 0x37, 0x1F, 0xA0, 0x3C, 0xC9, 0x50, 0x4D, 0x17, 0x09, 0xF9, 0xB4, 0x96, 0x31, 0xD6, 0x2C, 0xD5, 0x34, 0xF5, 0x66, 0x63, 0xD3, 0x33, 0xD2, 0xFB, 0xEE, 0x5F, 0x76,
        ],
    });
}
