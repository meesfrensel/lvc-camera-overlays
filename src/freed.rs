const ANGLE_DIVISOR: f32 = 32768.0;
const POSITION_DIVISOR: f32 = 64.0;

#[derive(Debug, Default, PartialEq)]
pub struct FreeD {
    /// The Camera Pan Angle is defined as the angle between the Y-axis and the
    /// projection of the optical axis of the camera onto the horizontal (XY)
    /// plane. A zero value corresponds to the camera looking in the positive Y
    /// direction and a positive value indicates a pan to the right (ie, the
    /// camera rotates clockwise when viewed from above).
    pub pan: f32,
    /// The Camera Tilt Angle is defined as the angle between the optical axis
    /// of the camera and the horizontal (XY) plane. A positive value indicates
    /// an upwards tilt. If the pan and tilt angles are both zero, the camera is
    /// looking in the direction of the positive Y axis.
    pub tilt: f32,
    /// The Camera Roll angle is usually zero for PTZs.
    pub roll: f32,
    /// Camera position (X, Y, Z).
    pos: (f32, f32, f32),
    /// The Camera Zoom is defined as the vertical angle of view of the camera;
    /// ie, the vertical angle subtended at the camera lens by the top and
    /// bottom edges of the active picture. The value is expressed in arbitrary
    /// units related to the rotation of the 'zoom ring' on the camera lens.
    pub zoom: u32,
    /// The Camera Focus is defined as the distance between the camera lens and
    /// an object at which the object will be in sharp focus. The value is
    /// expressed in arbitrary units related to the rotation of the 'focus ring'
    /// on the camera lens.
    pub focus: u32,
}

impl FreeD {
    pub const fn zero() -> Self {
        FreeD {
            pan: 0.0,
            tilt: 0.0,
            roll: 0.0,
            pos: (0.0, 0.0, 0.0),
            zoom: 0,
            focus: 0,
        }
    }
}

impl TryFrom<&[u8]> for FreeD {
    type Error = ();

    fn try_from(data: &[u8]) -> Result<Self, Self::Error> {
        match <&[u8] as TryInto<&[u8; 29]>>::try_into(data) {
            Ok(data) => FreeD::try_from(data),
            Err(..) => Err(())
        }
    }
}

impl TryFrom<&[u8; 29]> for FreeD {
    type Error = ();

    fn try_from(data: &[u8; 29]) -> Result<Self, Self::Error> {
        if data[0] != 0xD1 {
            return Err(()) // Message type must be D1
        }
        if checksum(data) != 0 {
            return Err(()) // Invalid checksum
        }

        Ok(FreeD {
            pan: decode_float(&data[2..5]) / ANGLE_DIVISOR,
            tilt: decode_float(&data[5..8]) / ANGLE_DIVISOR,
            roll: decode_float(&data[8..11]) / ANGLE_DIVISOR,
            pos: (
                decode_float(&data[11..14]) / POSITION_DIVISOR,
                decode_float(&data[14..17]) / POSITION_DIVISOR,
                decode_float(&data[17..20]) / POSITION_DIVISOR,
                ),
            zoom: u32::from_be_bytes([0, data[20], data[21], data[22]]),
            focus: u32::from_be_bytes([0, data[23], data[24], data[25]]),
        })
    }
}

/// The checksum is calculated by subtracting (modulo 256) each byte of the
/// message, including the message type, from 40 (hex)
fn checksum(data: &[u8; 29]) -> u8 {
    data.iter().fold(0x40, |acc, el| acc.overflowing_sub(*el).0)
}

fn decode_float(bytes: &[u8]) -> f32 {
    i32::from_be_bytes([bytes[0], bytes[1], bytes[2], 0]) as f32 / 256.0
}

#[test]
fn test_example() {
    let bytes1: [u8; 29] = [0xd1, 0x00, 0xdf, 0x78, 0xaa, 0x00, 0x47, 0xef,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x40, 0x00, 0x00, 0x15, 0x01, 0xd1, 0xff, 0x12];
    let bytes2: [u8; 29] = [0xd1, 0x00, 0x21, 0x98, 0xd8, 0xff, 0xeb, 0x94,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x28, 0xc6, 0x00, 0x13, 0xb9, 0xd1, 0xff, 0xd6];

    let freed_result1 = FreeD::try_from(&bytes1);
    let freed_result2 = FreeD::try_from(&bytes2);

    assert!(freed_result1.is_ok());
    assert!(freed_result2.is_ok());

    assert_eq!(freed_result1.unwrap(), FreeD {
        pan: -65.05731,
        tilt: 0.5619812,
        roll: 0.0,
        pos: (0.0, 0.0, 0.0),
        zoom: 16384,
        focus: 5377,
    });

    assert_eq!(freed_result2.unwrap(), FreeD {
        pan: 67.19409,
        tilt: -0.1595459,
        roll: 0.0,
        pos: (0.0, 0.0, 0.0),
        zoom: 10438,
        focus: 5049,
    });
}
