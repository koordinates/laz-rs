/*
===============================================================================

  PROGRAMMERS:

    martin.isenburg@rapidlasso.com  -  http://rapidlasso.com
    uday.karan@gmail.com - Hobu, Inc.

  COPYRIGHT:

    (c) 2007-2014, martin isenburg, rapidlasso - tools to catch reality
    (c) 2014, Uday Verma, Hobu, Inc.
    (c) 2019, Thomas Montaigu

    This is free software; you can redistribute and/or modify it under the
    terms of the GNU Lesser General Licence as published by the Free Software
    Foundation. See the COPYING file for more information.

    This software is distributed WITHOUT ANY WARRANTY and without even the
    implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.

  CHANGE HISTORY:
    6 June 2019: Translated to Rust
===============================================================================
*/

use std::mem::size_of;

use crate::packers::Packable;
use byteorder::{LittleEndian, ReadBytesExt, WriteBytesExt};
use std::io::{Read, Write};

pub trait LasPoint0 {
    // Non mutable accessors
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn z(&self) -> i32;
    fn intensity(&self) -> u16;

    fn bit_fields(&self) -> u8;
    fn number_of_returns_of_given_pulse(&self) -> u8;
    fn scan_direction_flag(&self) -> bool;
    fn edge_of_flight_line(&self) -> bool;
    fn return_number(&self) -> u8;

    fn classification(&self) -> u8;
    fn scan_angle_rank(&self) -> i8;
    fn user_data(&self) -> u8;
    fn point_source_id(&self) -> u16;

    // Mutable accessors

    fn set_x(&mut self, new_val: i32);
    fn set_y(&mut self, new_val: i32);
    fn set_z(&mut self, new_val: i32);
    fn set_intensity(&mut self, new_val: u16);

    fn set_bit_fields(&mut self, new_val: u8);

    fn set_classification(&mut self, new_val: u8);
    fn set_scan_angle_rank(&mut self, new_val: i8);
    fn set_user_data(&mut self, new_val: u8);
    fn set_point_source_id(&mut self, new_val: u16);

    fn read_from<R: Read>(&mut self, src: &mut R) -> std::io::Result<()> {
        self.set_x(src.read_i32::<LittleEndian>()?);
        self.set_y(src.read_i32::<LittleEndian>()?);
        self.set_z(src.read_i32::<LittleEndian>()?);
        self.set_intensity(src.read_u16::<LittleEndian>()?);

        self.set_bit_fields(src.read_u8()?);
        self.set_classification(src.read_u8()?);
        self.set_scan_angle_rank(src.read_i8()?);
        self.set_user_data(src.read_u8()?);
        self.set_point_source_id(src.read_u16::<LittleEndian>()?);
        Ok(())
    }

    fn write_to<W: Write>(&self, dst: &mut W) -> std::io::Result<()> {
        dst.write_i32::<LittleEndian>(self.x())?;
        dst.write_i32::<LittleEndian>(self.y())?;
        dst.write_i32::<LittleEndian>(self.z())?;

        dst.write_u16::<LittleEndian>(self.intensity())?;

        dst.write_u8(self.bit_fields())?;
        dst.write_u8(self.classification())?;
        dst.write_i8(self.scan_angle_rank())?;
        dst.write_u8(self.user_data())?;
        dst.write_u16::<LittleEndian>(self.point_source_id())?;
        Ok(())
    }

    fn set_fields_from<P: LasPoint0>(&mut self, other: &P) {
        self.set_x(other.x());
        self.set_y(other.y());
        self.set_z(other.z());

        self.set_intensity(other.intensity());
        self.set_bit_fields(other.bit_fields());
        self.set_classification(other.classification());
        self.set_scan_angle_rank(other.scan_angle_rank());
        self.set_user_data(other.user_data());
        self.set_point_source_id(other.point_source_id());
    }
}

#[derive(Default, Copy, Clone, PartialEq, Debug)]
pub struct Point0 {
    pub x: i32,
    pub y: i32,
    pub z: i32,
    pub intensity: u16,

    // 3 bits
    pub number_of_returns_of_given_pulse: u8,
    // 3 bits
    pub scan_direction_flag: bool,
    // 1 bit
    pub edge_of_flight_line: bool,
    // 1 bit
    pub return_number: u8,

    // 5 bits for classification the rest are bit flags
    pub classification: u8,

    pub scan_angle_rank: i8,
    pub user_data: u8,
    pub point_source_id: u16,
}

impl Point0 {
    pub fn populate_bit_fields_from(&mut self, byte: u8) {
        self.return_number = byte & 0x7;
        self.number_of_returns_of_given_pulse = (byte >> 3) & 0x7;
        self.scan_direction_flag = ((byte >> 6) & 0x1) != 0;
        self.edge_of_flight_line = ((byte >> 7) & 0x1) != 0;
    }

    pub fn bit_fields_to_byte(&self) -> u8 {
        let a = self.return_number;
        let b = self.number_of_returns_of_given_pulse;
        let c = self.scan_direction_flag as u8;
        let d = self.edge_of_flight_line as u8;

        ((d & 0x1) << 7) | (c & 0x1) << 6 | (b & 0x7) << 3 | (a & 0x7)
    }
}

impl LasPoint0 for Point0 {
    fn x(&self) -> i32 {
        self.x
    }

    fn y(&self) -> i32 {
        self.y
    }

    fn z(&self) -> i32 {
        self.z
    }

    fn intensity(&self) -> u16 {
        self.intensity
    }

    fn bit_fields(&self) -> u8 {
        self.bit_fields_to_byte()
    }

    fn number_of_returns_of_given_pulse(&self) -> u8 {
        self.number_of_returns_of_given_pulse
    }

    fn scan_direction_flag(&self) -> bool {
        self.scan_direction_flag
    }

    fn edge_of_flight_line(&self) -> bool {
        self.edge_of_flight_line
    }

    fn return_number(&self) -> u8 {
        self.return_number
    }

    fn classification(&self) -> u8 {
        self.classification
    }

    fn scan_angle_rank(&self) -> i8 {
        self.scan_angle_rank
    }

    fn user_data(&self) -> u8 {
        self.user_data
    }

    fn point_source_id(&self) -> u16 {
        self.point_source_id
    }

    fn set_x(&mut self, new_val: i32) {
        self.x = new_val;
    }

    fn set_y(&mut self, new_val: i32) {
        self.y = new_val;
    }

    fn set_z(&mut self, new_val: i32) {
        self.z = new_val;
    }

    fn set_intensity(&mut self, new_val: u16) {
        self.intensity = new_val
    }

    fn set_bit_fields(&mut self, new_val: u8) {
        self.populate_bit_fields_from(new_val);
    }

    fn set_classification(&mut self, new_val: u8) {
        self.classification = new_val;
    }

    fn set_scan_angle_rank(&mut self, new_val: i8) {
        self.scan_angle_rank = new_val;
    }

    fn set_user_data(&mut self, new_val: u8) {
        self.user_data = new_val;
    }

    fn set_point_source_id(&mut self, new_val: u16) {
        self.point_source_id = new_val;
    }
}

pub(crate) struct Point0Wrapper<'a> {
    slc: &'a mut [u8],
}

impl<'a> Point0Wrapper<'a> {
    fn new(slc: &'a mut [u8]) -> Self {
        if slc.len() < 20 {
            panic!("Point0Wrapper expected a buffer of 20 bytes");
        } else {
            Self { slc }
        }
    }
}

impl<'a> LasPoint0 for Point0Wrapper<'a> {
    fn x(&self) -> i32 {
        unsafe {
            i32::from_le_bytes([
                *self.slc.get_unchecked(0),
                *self.slc.get_unchecked(1),
                *self.slc.get_unchecked(2),
                *self.slc.get_unchecked(3),
            ])
        }
    }

    fn y(&self) -> i32 {
        unsafe {
            i32::from_le_bytes([
                *self.slc.get_unchecked(4),
                *self.slc.get_unchecked(5),
                *self.slc.get_unchecked(6),
                *self.slc.get_unchecked(7),
            ])
        }
    }

    fn z(&self) -> i32 {
        unsafe {
            i32::from_le_bytes([
                *self.slc.get_unchecked(8),
                *self.slc.get_unchecked(9),
                *self.slc.get_unchecked(10),
                *self.slc.get_unchecked(11),
            ])
        }
    }

    fn intensity(&self) -> u16 {
        unsafe { u16::from_le_bytes([*self.slc.get_unchecked(12), *self.slc.get_unchecked(13)]) }
    }

    fn bit_fields(&self) -> u8 {
        unsafe { u8::from_le_bytes([*self.slc.get_unchecked(14)]) }
    }

    fn number_of_returns_of_given_pulse(&self) -> u8 {
        (self.bit_fields() >> 3) & 0x7
    }

    fn scan_direction_flag(&self) -> bool {
        ((self.bit_fields() >> 6) & 0x1) != 0
    }

    fn edge_of_flight_line(&self) -> bool {
        ((self.bit_fields() >> 7) & 0x1) != 0
    }

    fn return_number(&self) -> u8 {
        self.bit_fields() & 0x7
    }

    fn classification(&self) -> u8 {
        unsafe { u8::from_le_bytes([*self.slc.get_unchecked(15)]) }
    }

    fn scan_angle_rank(&self) -> i8 {
        unsafe { i8::from_le_bytes([*self.slc.get_unchecked(16)]) }
    }

    fn user_data(&self) -> u8 {
        unsafe { u8::from_le_bytes([*self.slc.get_unchecked(17)]) }
    }

    fn point_source_id(&self) -> u16 {
        unsafe { u16::from_le_bytes([*self.slc.get_unchecked(18), *self.slc.get_unchecked(19)]) }
    }

    fn set_x(&mut self, new_val: i32) {
        unsafe {
            self.slc
                .get_unchecked_mut(0..4)
                .copy_from_slice(&new_val.to_le_bytes());
        }
    }

    fn set_y(&mut self, new_val: i32) {
        unsafe {
            self.slc
                .get_unchecked_mut(4..8)
                .copy_from_slice(&new_val.to_le_bytes());
        }
    }

    fn set_z(&mut self, new_val: i32) {
        unsafe {
            self.slc
                .get_unchecked_mut(8..12)
                .copy_from_slice(&new_val.to_le_bytes());
        }
    }

    fn set_intensity(&mut self, new_val: u16) {
        unsafe {
            self.slc
                .get_unchecked_mut(12..14)
                .copy_from_slice(&new_val.to_le_bytes());
        }
    }

    fn set_bit_fields(&mut self, new_val: u8) {
        unsafe {
            *self.slc.get_unchecked_mut(14) = new_val;
        }
    }

    fn set_classification(&mut self, new_val: u8) {
        unsafe {
            *self.slc.get_unchecked_mut(15) = new_val;
        }
    }

    fn set_scan_angle_rank(&mut self, new_val: i8) {
        unsafe {
            *self.slc.get_unchecked_mut(16) = new_val as u8;
        }
    }

    fn set_user_data(&mut self, new_val: u8) {
        unsafe {
            *self.slc.get_unchecked_mut(17) = new_val;
        }
    }

    fn set_point_source_id(&mut self, new_val: u16) {
        unsafe {
            self.slc
                .get_unchecked_mut(18..20)
                .copy_from_slice(&new_val.to_le_bytes());
        }
    }
}

impl Packable for Point0 {
    type Type = Point0;

    fn unpack_from(input: &[u8]) -> Self::Type {
        if input.len() < 20 {
            panic!("Point10::unpack_from expected buffer of 20 bytes");
        }

        unsafe {
            let mut point = Point0::default();

            let mut start = 0;
            let mut end = size_of::<i32>();
            point.x = i32::unpack_from(&input.get_unchecked(start..end));
            start += size_of::<i32>();
            end += size_of::<i32>();
            point.y = i32::unpack_from(&input.get_unchecked(start..end));
            start += size_of::<i32>();
            end += size_of::<i32>();
            point.z = i32::unpack_from(&input.get_unchecked(start..end));

            start = end;
            end += size_of::<u16>();
            point.intensity = u16::unpack_from(&input.get_unchecked(start..end));

            start = end;
            end += size_of::<u8>();
            let bitfields = u8::unpack_from(&input.get_unchecked(start..end));
            point.populate_bit_fields_from(bitfields);

            start = end;
            end += size_of::<u8>();
            point.classification = u8::unpack_from(&input.get_unchecked(start..end));

            start = end;
            end += size_of::<i8>();
            point.scan_angle_rank = i8::unpack_from(&input.get_unchecked(start..end));

            start = end;
            end += size_of::<i8>();
            point.user_data = u8::unpack_from(&input.get_unchecked(start..end));

            start = end;
            end += size_of::<u16>();
            point.point_source_id = u16::unpack_from(&input.get_unchecked(start..end));
            debug_assert_eq!(end, 20);
            point
        }
    }

    fn pack_into(&self, output: &mut [u8]) {
        if output.len() < 20 {
            panic!("Point10::unpack_from expected buffer of 20 bytes");
        }
        unsafe {
            let mut start = 0;
            let mut end = size_of::<i32>();

            i32::pack_into(&self.x, &mut output.get_unchecked_mut(start..end));
            start += size_of::<i32>();
            end += size_of::<i32>();
            i32::pack_into(&self.y, &mut output.get_unchecked_mut(start..end));
            start += size_of::<i32>();
            end += size_of::<i32>();
            i32::pack_into(&self.z, &mut output.get_unchecked_mut(start..end));

            start = end;
            end += size_of::<u16>();
            u16::pack_into(&self.intensity, &mut output.get_unchecked_mut(start..end));

            start = end;
            end += size_of::<u8>();
            u8::pack_into(
                &self.bit_fields_to_byte(),
                &mut output.get_unchecked_mut(start..end),
            );

            start = end;
            end += size_of::<u8>();
            u8::pack_into(
                &self.classification,
                &mut output.get_unchecked_mut(start..end),
            );

            start = end;
            end += size_of::<i8>();
            i8::pack_into(
                &self.scan_angle_rank,
                &mut output.get_unchecked_mut(start..end),
            );

            start = end;
            end += size_of::<i8>();
            u8::pack_into(&self.user_data, &mut output.get_unchecked_mut(start..end));

            start = end;
            end += size_of::<u16>();
            u16::pack_into(
                &self.point_source_id,
                &mut output.get_unchecked_mut(start..end),
            );
            debug_assert_eq!(end, 20);
        }
    }
}

pub mod v1 {
    use std::io::{Read, Write};

    use crate::compressors::{
        IntegerCompressor, IntegerCompressorBuilder, DEFAULT_COMPRESS_CONTEXTS,
    };
    use crate::decoders::ArithmeticDecoder;
    use crate::decompressors::{
        IntegerDecompressor, IntegerDecompressorBuilder, DEFAULT_DECOMPRESS_CONTEXTS,
    };
    use crate::encoders::ArithmeticEncoder;
    use crate::models::{ArithmeticModel, ArithmeticModelBuilder};
    use crate::packers::Packable;
    use crate::record::{
        BufferFieldCompressor, BufferFieldDecompressor, PointFieldCompressor,
        PointFieldDecompressor,
    };

    use super::Point0;
    use crate::las::point10::{LasPoint0, Point0Wrapper};

    /// find median difference from 3 preceding differences
    fn median_diff(diff_array: &[i32; 3]) -> i32 {
        if diff_array[0] < diff_array[1] {
            if diff_array[1] < diff_array[2] {
                diff_array[1]
            } else if diff_array[0] < diff_array[2] {
                diff_array[2]
            } else {
                diff_array[0]
            }
        } else {
            if diff_array[0] < diff_array[2] {
                diff_array[0]
            } else if diff_array[1] < diff_array[2] {
                diff_array[2]
            } else {
                diff_array[1]
            }
        }
    }

    pub struct LasPoint0Decompressor {
        last_point: Point0,
        last_x_diffs: [i32; 3],
        last_y_diffs: [i32; 3],
        last_incr: usize,

        ic_dx: IntegerDecompressor,
        ic_dy: IntegerDecompressor,
        ic_dz: IntegerDecompressor,
        ic_intensity: IntegerDecompressor,
        ic_scan_angle_rank: IntegerDecompressor,
        ic_point_source_id: IntegerDecompressor,

        changed_values_model: ArithmeticModel,
        // All theses vec have 256 elements
        // all the associated dimensions have 256 elements: [0..255]
        bit_byte_models: Vec<Option<ArithmeticModel>>,
        classification_models: Vec<Option<ArithmeticModel>>,
        user_data_models: Vec<Option<ArithmeticModel>>,
    }

    impl LasPoint0Decompressor {
        pub fn new() -> Self {
            Self {
                last_point: Default::default(),
                last_x_diffs: [0i32; 3],
                last_y_diffs: [0i32; 3],
                last_incr: 0,
                ic_dx: IntegerDecompressorBuilder::new()
                    .bits(32)
                    .build_initialized(),
                ic_dy: IntegerDecompressorBuilder::new()
                    .bits(32)
                    .contexts(20)
                    .build_initialized(),
                ic_dz: IntegerDecompressorBuilder::new()
                    .bits(32)
                    .contexts(20)
                    .build_initialized(),
                ic_intensity: IntegerDecompressorBuilder::new()
                    .bits(16)
                    .build_initialized(),
                ic_scan_angle_rank: IntegerDecompressorBuilder::new()
                    .bits(8)
                    .contexts(2)
                    .build_initialized(),
                ic_point_source_id: IntegerDecompressorBuilder::new()
                    .bits(16)
                    .build_initialized(),
                changed_values_model: ArithmeticModelBuilder::new(64).build(),
                bit_byte_models: (0..256).into_iter().map(|_| None).collect(),
                classification_models: (0..256).into_iter().map(|_| None).collect(),
                user_data_models: (0..256).into_iter().map(|_| None).collect(),
            }
        }

        fn median_x_diff(&self) -> i32 {
            median_diff(&self.last_x_diffs)
        }

        fn median_y_diff(&self) -> i32 {
            median_diff(&self.last_y_diffs)
        }
    }

    impl<R: Read, P: LasPoint0> PointFieldDecompressor<R, P> for LasPoint0Decompressor {
        fn init_first_point(
            &mut self,
            mut src: &mut R,
            first_point: &mut P,
        ) -> std::io::Result<()> {
            first_point.read_from(&mut src)?;
            self.last_point.set_fields_from(first_point);
            Ok(())
        }

        fn decompress_field_with(
            &mut self,
            mut decoder: &mut ArithmeticDecoder<R>,
            current_point: &mut P,
        ) -> std::io::Result<()> {
            // Decompress x, y, z
            let median_x = self.median_x_diff();
            let median_y = self.median_y_diff();

            let x_diff =
                self.ic_dx
                    .decompress(&mut decoder, median_x, DEFAULT_DECOMPRESS_CONTEXTS)?;
            self.last_point.x += x_diff;
            // we use the number k of bits corrector bits to switch contexts
            let k_bits = self.ic_dx.k();
            let y_diff = self.ic_dy.decompress(
                &mut decoder,
                median_y,
                if k_bits < 19 { k_bits } else { 19 },
            )?;
            self.last_point.y += y_diff;
            let k_bits = (k_bits + self.ic_dy.k()) / 2;
            self.last_point.z = self.ic_dz.decompress(
                &mut decoder,
                self.last_point.z,
                if k_bits < 19 { k_bits } else { 19 },
            )?;

            let changed_value = decoder.decode_symbol(&mut self.changed_values_model)? as i32;
            //TODO use get or insert
            if changed_value != 0 {
                if (changed_value & 32) != 0 {
                    self.last_point.intensity = self.ic_intensity.decompress(
                        &mut decoder,
                        self.last_point.intensity as i32,
                        DEFAULT_DECOMPRESS_CONTEXTS,
                    )? as u16;
                }

                if (changed_value & 16) != 0 {
                    let model = &mut self.bit_byte_models[self.last_point.bit_fields() as usize];
                    if (*model).is_none() {
                        *model = Some(ArithmeticModelBuilder::new(256).build());
                    }
                    self.last_point
                        .set_bit_fields(decoder.decode_symbol((*model).as_mut().unwrap())? as u8);
                }

                if (changed_value & 8) != 0 {
                    let model =
                        &mut self.classification_models[self.last_point.classification as usize];
                    if (*model).is_none() {
                        *model = Some(ArithmeticModelBuilder::new(256).build());
                    }
                    self.last_point.set_classification(
                        decoder.decode_symbol((*model).as_mut().unwrap())? as u8,
                    );
                }

                if (changed_value & 4) != 0 {
                    self.last_point
                        .set_scan_angle_rank(self.ic_scan_angle_rank.decompress(
                            &mut decoder,
                            self.last_point.scan_angle_rank() as i32,
                            (k_bits < 3) as u32,
                        )? as i8);
                }

                if (changed_value & 2) != 0 {
                    let model = &mut self.user_data_models[self.last_point.user_data() as usize];
                    if (*model).is_none() {
                        *model = Some(ArithmeticModelBuilder::new(256).build());
                    }
                    self.last_point
                        .set_user_data(decoder.decode_symbol((*model).as_mut().unwrap())? as u8);
                }

                if (changed_value & 1) != 0 {
                    self.last_point
                        .set_point_source_id(self.ic_point_source_id.decompress(
                            &mut decoder,
                            self.last_point.point_source_id() as i32,
                            DEFAULT_DECOMPRESS_CONTEXTS,
                        )? as u16);
                }
            }

            // record the differences
            self.last_x_diffs[self.last_incr] = x_diff;
            self.last_y_diffs[self.last_incr] = y_diff;
            self.last_incr += 1;
            if self.last_incr > 2 {
                self.last_incr = 0;
            }
            current_point.set_fields_from(&self.last_point);
            Ok(())
        }
    }

    pub struct LasPoint0Compressor {
        last_point: Point0,
        last_x_diffs: [i32; 3],
        last_y_diffs: [i32; 3],
        last_incr: usize,

        ic_dx: IntegerCompressor,
        ic_dy: IntegerCompressor,
        ic_dz: IntegerCompressor,
        ic_intensity: IntegerCompressor,
        ic_scan_angle_rank: IntegerCompressor,
        ic_point_source_id: IntegerCompressor,

        changed_values_model: ArithmeticModel,
        // All theses vec have 256 elements
        // all the associated dimensions have 256 elements: [0..255]
        bit_byte_models: Vec<Option<ArithmeticModel>>,
        classification_models: Vec<Option<ArithmeticModel>>,
        user_data_models: Vec<Option<ArithmeticModel>>,
    }

    impl LasPoint0Compressor {
        pub fn new() -> Self {
            Self {
                last_point: Default::default(),
                last_x_diffs: [0i32; 3],
                last_y_diffs: [0i32; 3],
                last_incr: 0,
                ic_dx: IntegerCompressorBuilder::new().bits(32).build_initialized(),
                ic_dy: IntegerCompressorBuilder::new()
                    .bits(32)
                    .contexts(20)
                    .build_initialized(),
                ic_dz: IntegerCompressorBuilder::new()
                    .bits(32)
                    .contexts(20)
                    .build_initialized(),
                ic_intensity: IntegerCompressorBuilder::new().bits(16).build_initialized(),
                ic_scan_angle_rank: IntegerCompressorBuilder::new()
                    .bits(8)
                    .contexts(2)
                    .build_initialized(),
                ic_point_source_id: IntegerCompressorBuilder::new().bits(16).build_initialized(),
                changed_values_model: ArithmeticModelBuilder::new(64).build(),
                bit_byte_models: (0..256).into_iter().map(|_| None).collect(),
                classification_models: (0..256).into_iter().map(|_| None).collect(),
                user_data_models: (0..256).into_iter().map(|_| None).collect(),
            }
        }
    }

    impl<W: Write, P: LasPoint0> PointFieldCompressor<W, P> for LasPoint0Compressor {
        fn init_first_point(&mut self, mut dst: &mut W, first_point: &P) -> std::io::Result<()> {
            first_point.write_to(&mut dst)?;
            self.last_point.set_fields_from(first_point);
            Ok(())
        }

        fn compress_field_with(
            &mut self,
            mut encoder: &mut ArithmeticEncoder<W>,
            current_point: &P,
        ) -> std::io::Result<()> {
            let median_x = median_diff(&self.last_x_diffs);
            let median_y = median_diff(&self.last_y_diffs);

            let x_diff = current_point.x() - self.last_point.x();
            let y_diff = current_point.y() - self.last_point.y();

            self.ic_dx
                .compress(&mut encoder, median_x, x_diff, DEFAULT_COMPRESS_CONTEXTS)?;
            let k_bits = self.ic_dx.k();
            self.ic_dy.compress(
                &mut encoder,
                median_y,
                y_diff,
                if k_bits < 19 { k_bits } else { 19 },
            )?;

            let k_bits = (k_bits + self.ic_dy.k()) / 2;
            self.ic_dz.compress(
                &mut encoder,
                self.last_point.z(),
                current_point.z(),
                if k_bits < 19 { k_bits } else { 19 },
            )?;

            let changed_values: u8 = ((self.last_point.intensity() != current_point.intensity())
                as u8)
                << 5
                | ((self.last_point.bit_fields() != current_point.bit_fields()) as u8) << 4
                | ((self.last_point.classification() != current_point.classification()) as u8) << 3
                | ((self.last_point.scan_angle_rank() != current_point.scan_angle_rank()) as u8)
                    << 2
                | ((self.last_point.user_data() != current_point.user_data()) as u8) << 1
                | (self.last_point.point_source_id() != current_point.point_source_id()) as u8;

            encoder.encode_symbol(&mut self.changed_values_model, changed_values as u32)?;

            if changed_values != 0 {
                if (changed_values & 32) != 0 {
                    self.ic_intensity.compress(
                        &mut encoder,
                        self.last_point.intensity() as i32,
                        current_point.intensity() as i32,
                        DEFAULT_COMPRESS_CONTEXTS,
                    )?;
                }

                if (changed_values & 16) != 0 {
                    let model = &mut self.bit_byte_models[self.last_point.bit_fields() as usize]
                        .get_or_insert(ArithmeticModelBuilder::new(256).build());
                    encoder.encode_symbol(model, current_point.bit_fields() as u32)?;
                }

                if (changed_values & 8) != 0 {
                    let model = &mut self.classification_models
                        [self.last_point.classification() as usize]
                        .get_or_insert(ArithmeticModelBuilder::new(256).build());
                    encoder.encode_symbol(model, current_point.classification() as u32)?;
                }

                if (changed_values & 4) != 0 {
                    self.ic_scan_angle_rank.compress(
                        &mut encoder,
                        self.last_point.scan_angle_rank() as i32,
                        current_point.scan_angle_rank() as i32,
                        (k_bits < 3) as u32,
                    )?;
                }

                if (changed_values & 2) != 0 {
                    let model = self.user_data_models[self.last_point.user_data() as usize]
                        .get_or_insert(ArithmeticModelBuilder::new(256).build());
                    encoder.encode_symbol(model, current_point.user_data() as u32)?;
                }

                if (changed_values & 1) != 0 {
                    self.ic_point_source_id.compress(
                        &mut encoder,
                        self.last_point.point_source_id() as i32,
                        current_point.point_source_id() as i32,
                        DEFAULT_COMPRESS_CONTEXTS,
                    )?;
                }
            }
            self.last_x_diffs[self.last_incr] = x_diff;
            self.last_y_diffs[self.last_incr] = y_diff;
            self.last_incr += 1;
            if self.last_incr > 2 {
                self.last_incr = 0;
            }
            self.last_point.set_fields_from(current_point);
            Ok(())
        }
    }

    impl<R: Read> BufferFieldDecompressor<R> for LasPoint0Decompressor {
        fn size_of_field(&self) -> usize {
            20
        }

        fn decompress_first(
            &mut self,
            src: &mut R,
            mut first_point: &mut [u8],
        ) -> std::io::Result<()> {
            let mut current = Point0Wrapper {
                slc: &mut first_point,
            };
            self.init_first_point(src, &mut current)?;
            Ok(())
        }

        fn decompress_with(
            &mut self,
            mut decoder: &mut ArithmeticDecoder<R>,
            buf: &mut [u8],
        ) -> std::io::Result<()> {
            let mut current = Point0Wrapper { slc: buf };
            self.decompress_field_with(&mut decoder, &mut current)?;
            Ok(())
        }
    }

    impl<W: Write> BufferFieldCompressor<W> for LasPoint0Compressor {
        fn size_of_field(&self) -> usize {
            20
        }

        fn compress_first(&mut self, mut dst: &mut W, buf: &[u8]) -> std::io::Result<()> {
            let current = Point0::unpack_from(buf);
            self.init_first_point(&mut dst, &current)
        }

        fn compress_with(
            &mut self,
            mut encoder: &mut ArithmeticEncoder<W>,
            buf: &[u8],
        ) -> std::io::Result<()> {
            let current = Point0::unpack_from(buf);
            self.compress_field_with(&mut encoder, &current)
        }
    }
}

pub mod v2 {
    use std::io::{Read, Write};

    use crate::compressors::{IntegerCompressor, IntegerCompressorBuilder};
    use crate::decoders::ArithmeticDecoder;
    use crate::decompressors::{IntegerDecompressor, IntegerDecompressorBuilder};
    use crate::encoders::ArithmeticEncoder;
    use crate::las::utils;
    use crate::models::{ArithmeticModel, ArithmeticModelBuilder};
    use crate::packers::Packable;
    use crate::record::{
        BufferFieldCompressor, BufferFieldDecompressor, PointFieldCompressor,
        PointFieldDecompressor,
    };

    use super::Point0;
    use crate::las::point10::{LasPoint0, Point0Wrapper};

    struct Point10ChangedValues {
        value: i32,
    }

    /// Only valid for version 2 of the compression / decompression
    /// Compared to version 1, the flag bit used for the intensity & bit_fields
    /// have been swapped
    impl Point10ChangedValues {
        pub fn from_points<P: LasPoint0, OP: LasPoint0>(
            current: &P,
            last: &OP,
            last_intensity: u16,
        ) -> Self {
            // This logic here constructs a 5-bit changed value which is basically a bit map of what has changed
            // since the last point, not considering the x, y and z values

            let bit_fields_changed = ((last.return_number() ^ current.return_number()) != 0)
                | ((last.number_of_returns_of_given_pulse()
                    ^ current.number_of_returns_of_given_pulse())
                    != 0)
                | (last.scan_direction_flag() ^ current.scan_direction_flag())
                | (last.edge_of_flight_line() ^ current.edge_of_flight_line());

            let intensity_changed = (last_intensity ^ current.intensity()) != 0;
            let classification_changed = (last.classification() ^ current.classification()) != 0;
            let scan_angle_rank_changed = (last.scan_angle_rank() ^ current.scan_angle_rank()) != 0;
            let user_data_changed = (last.user_data() ^ current.user_data()) != 0;
            let point_source_id_changed = (last.point_source_id() ^ current.point_source_id()) != 0;
            Point10ChangedValues {
                value: (bit_fields_changed as i32) << 5
                    | (intensity_changed as i32) << 4
                    | (classification_changed as i32) << 3
                    | (scan_angle_rank_changed as i32) << 2
                    | (user_data_changed as i32) << 1
                    | (point_source_id_changed as i32),
            }
        }

        pub fn bit_fields_changed(&self) -> bool {
            (self.value & (1 << 5)) != 0
        }

        pub fn intensity_changed(&self) -> bool {
            (self.value & (1 << 4)) != 0
        }

        pub fn classification_changed(&self) -> bool {
            (self.value & (1 << 3)) != 0
        }

        pub fn scan_angle_rank_changed(&self) -> bool {
            (self.value & (1 << 2)) != 0
        }

        pub fn user_data_changed(&self) -> bool {
            (self.value & (1 << 1)) != 0
        }

        pub fn point_source_id_changed(&self) -> bool {
            (self.value & 1) != 0
        }
    }

    // All the things we need to compress a point, group them into structs
    // so we don't have too many names flying around
    struct Common {
        last_intensity: [u16; 16],

        // can't have arrays as StreamingMedian is not a copy type
        // 16 elements both
        last_x_diff_median: Vec<utils::StreamingMedian<i32>>,
        last_y_diff_median: Vec<utils::StreamingMedian<i32>>,

        last_height: [i32; 8],

        changed_values: ArithmeticModel,

        // can't have arrays as ArithmeticModel is not a copy type
        scan_angle_rank: Vec<ArithmeticModel>,
        // 2
        bit_byte: Vec<ArithmeticModel>,
        // 256
        classification: Vec<ArithmeticModel>,
        //256
        user_data: Vec<ArithmeticModel>, //256
    }

    impl Common {
        pub fn new() -> Self {
            Self {
                last_intensity: [0u16; 16],
                last_x_diff_median: (0..16)
                    .into_iter()
                    .map(|_i| utils::StreamingMedian::<i32>::new())
                    .collect(),
                last_y_diff_median: (0..16)
                    .into_iter()
                    .map(|_i| utils::StreamingMedian::<i32>::new())
                    .collect(),
                last_height: [0i32; 8],
                changed_values: ArithmeticModelBuilder::new(64).build(),
                scan_angle_rank: (0..2)
                    .into_iter()
                    .map(|_i| ArithmeticModelBuilder::new(256).build())
                    .collect(),
                bit_byte: (0..256)
                    .into_iter()
                    .map(|_i| ArithmeticModelBuilder::new(256).build())
                    .collect(),
                classification: (0..256)
                    .into_iter()
                    .map(|_i| ArithmeticModelBuilder::new(256).build())
                    .collect(),
                user_data: (0..256)
                    .into_iter()
                    .map(|_i| ArithmeticModelBuilder::new(256).build())
                    .collect(),
            }
        }
    }

    pub struct LasPoint0Compressor {
        last_point: Point0,
        ic_intensity: IntegerCompressor,
        ic_point_source_id: IntegerCompressor,
        ic_dx: IntegerCompressor,
        ic_dy: IntegerCompressor,
        ic_z: IntegerCompressor,
        common: Common,
    }

    impl LasPoint0Compressor {
        pub fn new() -> Self {
            Self {
                last_point: Default::default(),
                ic_intensity: IntegerCompressorBuilder::new()
                    .bits(16)
                    .contexts(4)
                    .build_initialized(),
                ic_point_source_id: IntegerCompressorBuilder::new().bits(16).build_initialized(),
                ic_dx: IntegerCompressorBuilder::new()
                    .bits(32)
                    .contexts(2)
                    .build_initialized(),
                ic_dy: IntegerCompressorBuilder::new()
                    .bits(32)
                    .contexts(22)
                    .build_initialized(),
                ic_z: IntegerCompressorBuilder::new()
                    .bits(32)
                    .contexts(20)
                    .build_initialized(),
                common: Common::new(),
            }
        }
    }

    impl<W: Write, P: LasPoint0> PointFieldCompressor<W, P> for LasPoint0Compressor {
        fn init_first_point(&mut self, mut dst: &mut W, first_point: &P) -> std::io::Result<()> {
            first_point.write_to(&mut dst)?;
            self.last_point.set_fields_from(first_point);
            Ok(())
        }

        fn compress_field_with(
            &mut self,
            mut encoder: &mut ArithmeticEncoder<W>,
            current_point: &P,
        ) -> std::io::Result<()> {
            let r = current_point.return_number();
            let n = current_point.number_of_returns_of_given_pulse();
            // According to table  m is in range 0..16
            let m = utils::NUMBER_RETURN_MAP[n as usize][r as usize];
            // According to table l is in range 0..8
            let l = utils::NUMBER_RETURN_LEVEL[n as usize][r as usize];

            let changed_values =
                Point10ChangedValues::from_points(current_point, &self.last_point, *unsafe {
                    self.common.last_intensity.get_unchecked(m as usize)
                });

            // compress which other values have changed

            encoder.encode_symbol(&mut self.common.changed_values, changed_values.value as u32)?;

            if changed_values.bit_fields_changed() {
                let b = current_point.bit_fields();
                let last_b = self.last_point.bit_fields();
                encoder.encode_symbol(
                    unsafe { self.common.bit_byte.get_unchecked_mut(last_b as usize) },
                    b as u32,
                )?;
            }

            if changed_values.intensity_changed() {
                self.ic_intensity.compress(
                    &mut encoder,
                    self.common.last_intensity[m as usize] as i32,
                    current_point.intensity() as i32,
                    if m < 3 { m as u32 } else { 3 },
                )?;
                self.common.last_intensity[m as usize] = current_point.intensity();
            }

            if changed_values.classification_changed() {
                encoder.encode_symbol(
                    unsafe {
                        self.common
                            .classification
                            .get_unchecked_mut(self.last_point.classification as usize)
                    },
                    current_point.classification() as u32,
                )?;
            }

            if changed_values.scan_angle_rank_changed() {
                // the "as u8" before "as u32" is vital
                encoder.encode_symbol(
                    unsafe {
                        self.common
                            .scan_angle_rank
                            .get_unchecked_mut(current_point.scan_direction_flag() as usize)
                    },
                    (current_point.scan_angle_rank() - self.last_point.scan_angle_rank) as u8
                        as u32,
                )?;
            }

            if changed_values.user_data_changed() {
                encoder.encode_symbol(
                    unsafe {
                        self.common
                            .user_data
                            .get_unchecked_mut(self.last_point.user_data as usize)
                    },
                    current_point.user_data() as u32,
                )?;
            }

            if changed_values.point_source_id_changed() {
                self.ic_point_source_id.compress(
                    &mut encoder,
                    self.last_point.point_source_id as i32,
                    current_point.point_source_id() as i32,
                    0,
                )?;
            }

            //compress x coordinates
            let median = unsafe { self.common.last_x_diff_median.get_unchecked(m as usize) }.get();
            let diff = current_point.x() - self.last_point.x;
            self.ic_dx
                .compress(&mut encoder, median, diff, (n == 1) as u32)?;
            unsafe { self.common.last_x_diff_median.get_unchecked_mut(m as usize) }.add(diff);

            //compress y coordinates
            let k_bits = self.ic_dx.k();
            let median = unsafe {
                self.common
                    .last_y_diff_median
                    .get_unchecked(m as usize)
                    .get()
            };
            let diff = current_point.y() - self.last_point.y;
            let context = (n == 1) as u32
                + if k_bits < 20 {
                    utils::u32_zero_bit(k_bits)
                } else {
                    20
                };
            self.ic_dy.compress(&mut encoder, median, diff, context)?;
            unsafe {
                self.common
                    .last_y_diff_median
                    .get_unchecked_mut(m as usize)
                    .add(diff);
            }

            //compress z coordinates
            let k_bits = (self.ic_dx.k() + self.ic_dy.k()) / 2;
            let context = (n == 1) as u32
                + if k_bits < 18 {
                    utils::u32_zero_bit(k_bits)
                } else {
                    18
                };
            self.ic_z.compress(
                &mut encoder,
                *unsafe { self.common.last_height.get_unchecked(l as usize) },
                current_point.z(),
                context,
            )?;
            unsafe { *self.common.last_height.get_unchecked_mut(l as usize) = current_point.z() };
            self.last_point.set_fields_from(current_point);
            Ok(())
        }
    }

    impl<W: Write> BufferFieldCompressor<W> for LasPoint0Compressor {
        fn size_of_field(&self) -> usize {
            20
        }

        fn compress_first(&mut self, mut dst: &mut W, buf: &[u8]) -> std::io::Result<()> {
            let current_point = Point0::unpack_from(&buf);
            self.init_first_point(&mut dst, &current_point)
        }

        fn compress_with(
            &mut self,
            mut encoder: &mut ArithmeticEncoder<W>,
            buf: &[u8],
        ) -> std::io::Result<()> {
            let current_point = Point0::unpack_from(&buf);
            self.compress_field_with(&mut encoder, &current_point)
        }
    }

    pub struct LasPoint0Decompressor {
        last_point: Point0,
        ic_intensity: IntegerDecompressor,
        ic_point_source_id: IntegerDecompressor,
        ic_dx: IntegerDecompressor,
        ic_dy: IntegerDecompressor,
        ic_z: IntegerDecompressor,

        common: Common,
    }

    impl LasPoint0Decompressor {
        pub fn new() -> Self {
            Self {
                last_point: Default::default(),
                ic_intensity: IntegerDecompressorBuilder::new()
                    .bits(16)
                    .contexts(4)
                    .build_initialized(),
                ic_point_source_id: IntegerDecompressorBuilder::new()
                    .bits(16)
                    .build_initialized(),
                ic_dx: IntegerDecompressorBuilder::new()
                    .bits(32)
                    .contexts(2)
                    .build_initialized(),
                ic_dy: IntegerDecompressorBuilder::new()
                    .bits(32)
                    .contexts(22)
                    .build_initialized(),
                ic_z: IntegerDecompressorBuilder::new()
                    .bits(32)
                    .contexts(20)
                    .build_initialized(),
                common: Common::new(),
            }
        }
    }

    impl<R: Read, P: LasPoint0> PointFieldDecompressor<R, P> for LasPoint0Decompressor {
        fn init_first_point(
            &mut self,
            mut src: &mut R,
            first_point: &mut P,
        ) -> std::io::Result<()> {
            first_point.read_from(&mut src)?;
            self.last_point.set_fields_from(first_point);
            // But set intensity to 0
            self.last_point.intensity = 0;
            Ok(())
        }

        fn decompress_field_with(
            &mut self,
            mut decoder: &mut ArithmeticDecoder<R>,
            current_point: &mut P,
        ) -> std::io::Result<()> {
            let changed_value = Point10ChangedValues {
                value: decoder.decode_symbol(&mut self.common.changed_values)? as i32,
            };

            let r;
            let n;
            let m;
            let l;

            unsafe {
                if changed_value.value != 0 {
                    // there was some change in one of the fields (other than x, y and z)

                    if changed_value.bit_fields_changed() {
                        let mut b = self.last_point.bit_fields();
                        b = decoder
                            .decode_symbol(self.common.bit_byte.get_unchecked_mut(b as usize))?
                            as u8;
                        self.last_point.set_bit_fields(b);
                    }

                    r = self.last_point.return_number();
                    n = self.last_point.number_of_returns_of_given_pulse();
                    // According to table m is in range 0..16
                    m = utils::NUMBER_RETURN_MAP[n as usize][r as usize];
                    // According to table l is in range 0..8
                    l = utils::NUMBER_RETURN_LEVEL[n as usize][r as usize];

                    if changed_value.intensity_changed() {
                        self.last_point.intensity = self.ic_intensity.decompress(
                            &mut decoder,
                            *self.common.last_intensity.get_unchecked(m as usize) as i32,
                            if m < 3 { m as u32 } else { 3 },
                        )? as u16;
                        *self.common.last_intensity.get_unchecked_mut(m as usize) =
                            self.last_point.intensity;
                    } else {
                        self.last_point.intensity =
                            *self.common.last_intensity.get_unchecked(m as usize);
                    }

                    if changed_value.classification_changed() {
                        self.last_point.set_classification(
                            decoder.decode_symbol(
                                self.common
                                    .classification
                                    .get_unchecked_mut(self.last_point.classification as usize),
                            )? as u8,
                        );
                    }

                    if changed_value.scan_angle_rank_changed() {
                        let val = decoder.decode_symbol(
                            self.common
                                .scan_angle_rank
                                .get_unchecked_mut(self.last_point.scan_direction_flag as usize),
                        )? as i8;
                        self.last_point
                            .set_scan_angle_rank(val + self.last_point.scan_angle_rank);
                    }

                    if changed_value.user_data_changed() {
                        self.last_point.set_user_data(
                            decoder.decode_symbol(
                                self.common
                                    .user_data
                                    .get_unchecked_mut(self.last_point.user_data as usize),
                            )? as u8,
                        );
                    }

                    if changed_value.point_source_id_changed() {
                        self.last_point
                            .set_point_source_id(self.ic_point_source_id.decompress(
                                &mut decoder,
                                self.last_point.point_source_id as i32,
                                0,
                            )? as u16);
                    }
                } else {
                    r = self.last_point.return_number();
                    n = self.last_point.number_of_returns_of_given_pulse();
                    m = utils::NUMBER_RETURN_MAP[n as usize][r as usize];
                    l = utils::NUMBER_RETURN_LEVEL[n as usize][r as usize];
                }

                // decompress x
                let median = self
                    .common
                    .last_x_diff_median
                    .get_unchecked(m as usize)
                    .get();
                let diff = self
                    .ic_dx
                    .decompress(&mut decoder, median, (n == 1) as u32)?;
                self.last_point.x += diff;
                self.common
                    .last_x_diff_median
                    .get_unchecked_mut(m as usize)
                    .add(diff);

                // decompress y
                let median = self
                    .common
                    .last_y_diff_median
                    .get_unchecked(m as usize)
                    .get();
                let k_bits = self.ic_dx.k();
                let context = (n == 1) as u32
                    + if k_bits < 20 {
                        utils::u32_zero_bit(k_bits)
                    } else {
                        20
                    };
                let diff = self.ic_dy.decompress(&mut decoder, median, context)?;
                self.last_point.y += diff;
                self.common
                    .last_y_diff_median
                    .get_unchecked_mut(m as usize)
                    .add(diff);

                // decompress z coordinate
                let k_bits = (self.ic_dx.k() + self.ic_dy.k()) / 2;
                let context = (n == 1) as u32
                    + if k_bits < 18 {
                        utils::u32_zero_bit(k_bits)
                    } else {
                        18
                    };
                self.last_point.z = self.ic_z.decompress(
                    &mut decoder,
                    *self.common.last_height.get_unchecked(l as usize),
                    context,
                )?;
                *self.common.last_height.get_unchecked_mut(l as usize) = self.last_point.z();
                current_point.set_fields_from(&self.last_point);
                Ok(())
            }
        }
    }

    impl<R: Read> BufferFieldDecompressor<R> for LasPoint0Decompressor {
        fn size_of_field(&self) -> usize {
            20
        }

        fn decompress_first(&mut self, src: &mut R, first_point: &mut [u8]) -> std::io::Result<()> {
            let mut current = Point0Wrapper::new(first_point);
            self.init_first_point(src, &mut current)?;
            Ok(())
        }

        fn decompress_with(
            &mut self,
            mut decoder: &mut ArithmeticDecoder<R>,
            buf: &mut [u8],
        ) -> std::io::Result<()> {
            let mut current = Point0Wrapper::new(buf);
            self.decompress_field_with(&mut decoder, &mut current)?;
            Ok(())
        }
    }
}
