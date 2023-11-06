# Appendix B, Camera positioning parameters
The following sections describe the parameters used to convey the position,
orientation etc. of the studio camera.

## B.1 Definition of axes
A set of orthogonal right-handed axes (X, Y and Z) is used, fixed with respect
to the reference frame of the studio. The X and Y axes lie in the horizontal
plane, and the Z axis is vertical. The positive direction of the Z-axis is
upwards.

## B.2 Camera pan angle
The Camera Pan Angle is defined as the angle between the Y-axis and the
projection of the optical axis of the camera onto the horizontal (XY) plane. A
zero value corresponds to the camera looking in the positive Y direction and a
positive value indicates a pan to the right (ie, the camera rotates clockwise
when viewed from above).

The value is expressed in degrees as a 24-bit twos-complements signed number,
where the most-significant bit (bit 23) is the sign bit, the next 8 bits (bits
22 to 15) are the integer part and the remaining bits (bits 14 to 0) are the
fractional part; alternatively, this may be thought of as a signed integer value
in units of 1/32768 degree. The range of values is from -180.0 degrees
(0xA60000) to +180.0 degrees (0x5A0000).

## B.3 Camera tilt angle
The Camera Tilt Angle is defined as the angle between the optical axis of the
camera and the horizontal (XY) plane. A positive value indicates an upwards
tilt. If the pan and tilt angles are both zero, the camera is looking in the
direction of the positive Y axis.

The value is expressed in degrees as a 24-bit twos-complements signed number,
where the most-significant bit (bit 23) is the sign bit, the next 8 bits (bits
22 to 15) are the integer part and the remaining bits (bits 14 to 0) are the
fractional part; alternatively, this may be thought of as a signed integer value
in units of 1/32768 degree. The range of values is from -90.0 degrees (0xD30000)
to +90.0 degrees (0x2D0000).

## B.4 Camera roll angle
The Camera Roll Angle is defined as the angle of rotation of the camera about
its optical axis. A roll angle of zero corresponds to a 'scan line' of the
camera sensor (ie, a horizontal in the image) being parallel to the horizontal
(XY) plane. A positive value indicates a clockwise roll, when viewd from behind
the camera.

The value is expressed in degrees as a 24-bit twos-complements signed number,
where the most-significant bit (bit 23) is the sign bit, the next 8 bits (bits
22 to 15) are the integer part and the remaining bits (bits 14 to 0) are the
fractional part; alternatively, this may be thought of as a signed integer value
in units of 1/32768 degree. The range of values is from -180.0 degrees
(0xA60000) to +180.0 degrees (0x5A0000).

---

(B.5 to B.7 - Camera position)

---

## B.8 Camera zoom
The Camera Zoom is defined as the vertical angle of view of the camera; ie, the
vertical angle subtended at the camera lens by the top and bottom edges of the
active picture.

The value is expressed as a 24-bit positive unsigned number in arbitrary units
related to the rotation of the 'zoom ring' on the camera lens. It will be
necessary for the host system to convert this to a true zoom value based on the
type and particular sample of lens and camera in use.

## B.9 Camera focus
The Camera Focus is defined as the distance between the camera lens and an
object at which the object will be in sharp focus. The value is expressed as a
24-bit positive unsigned number in arbitrary units related to the rotation of
the 'focus ring' on the camera lens. It will be necessary for the host system to
convert this to a true focus value based on the type and particular sample of
lens and camera in use.
