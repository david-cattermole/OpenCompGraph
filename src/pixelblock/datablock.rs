/*
 * Copyright (C) 2020, 2021 David Cattermole.
 *
 * This file is part of OpenCompGraph.
 *
 * OpenCompGraph is free software: you can redistribute it and/or modify it
 * under the terms of the GNU Lesser General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * OpenCompGraph is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with OpenCompGraph.  If not, see <https://www.gnu.org/licenses/>.
 * ====================================================================
 *
 */

use half::f16;
use log::debug;
use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::DataType;
use crate::pixelblock::dataslice::DataSlice;
use crate::pixelblock::dataslice::DataSliceMut;
use crate::pixelblock::utils::convert_datablock_enum_to_f16;
use crate::pixelblock::utils::convert_datablock_enum_to_f32;
use crate::pixelblock::utils::convert_datablock_enum_to_u16;
use crate::pixelblock::utils::convert_datablock_enum_to_u8;

#[repr(u8)]
#[derive(Clone, Debug)]
pub enum DataBlockEnum {
    Float32(Vec<f32>),
    UInt8(Vec<u8>),
    UInt16(Vec<u16>),
    Half16(Vec<f16>),
}

#[derive(Clone)]
pub struct DataBlock {
    inner: DataBlockEnum,
}

impl DataBlock {
    pub fn new(size: usize, data_type: DataType) -> DataBlock {
        let fill_value_f32 = 0.0 as f32;
        let fill_value_f16 = f16::from_f32(0.0);
        let fill_value_u8 = 0 as u8;
        let fill_value_u16 = 0 as u16;
        match data_type {
            DataType::Float32 => DataBlock {
                inner: DataBlockEnum::Float32(vec![fill_value_f32; size]),
            },
            DataType::UInt8 => DataBlock {
                inner: DataBlockEnum::UInt8(vec![fill_value_u8; size]),
            },
            DataType::Half16 => DataBlock {
                inner: DataBlockEnum::Half16(vec![fill_value_f16; size]),
            },
            DataType::UInt16 => DataBlock {
                inner: DataBlockEnum::UInt16(vec![fill_value_u16; size]),
            },
            _ => panic!("Unsupported DataType"),
        }
    }

    pub fn empty(data_type: DataType) -> DataBlock {
        let size = 0;
        DataBlock::new(size, data_type)
    }

    pub fn from_slice(slice: DataSlice) -> DataBlock {
        match slice {
            DataSlice::Float32(slice_data) => {
                let mut data = Vec::<f32>::with_capacity(slice_data.len());
                data.extend_from_slice(&slice_data);
                DataBlock {
                    inner: DataBlockEnum::Float32(data),
                }
            }
            DataSlice::Half16(slice_data) => {
                let mut data = Vec::<f16>::with_capacity(slice_data.len());
                data.extend_from_slice(&slice_data);
                DataBlock {
                    inner: DataBlockEnum::Half16(data),
                }
            }
            DataSlice::UInt16(slice_data) => {
                let mut data = Vec::<u16>::with_capacity(slice_data.len());
                data.extend_from_slice(&slice_data);
                DataBlock {
                    inner: DataBlockEnum::UInt16(data),
                }
            }
            DataSlice::UInt8(slice_data) => {
                let mut data = Vec::<u8>::with_capacity(slice_data.len());
                data.extend_from_slice(&slice_data);
                DataBlock {
                    inner: DataBlockEnum::UInt8(data),
                }
            }
        }
    }

    pub fn from_slice_f32(slice: &[f32]) -> DataBlock {
        let data_slice = DataSlice::Float32(slice);
        DataBlock::from_slice(data_slice)
    }

    pub fn from_slice_u16(slice: &[u16]) -> DataBlock {
        let data_slice = DataSlice::UInt16(slice);
        DataBlock::from_slice(data_slice)
    }

    pub fn from_slice_u8(slice: &[u8]) -> DataBlock {
        let data_slice = DataSlice::UInt8(slice);
        DataBlock::from_slice(data_slice)
    }

    pub fn as_slice(&self) -> DataSlice {
        match &self.inner {
            DataBlockEnum::Float32(data) => DataSlice::Float32(&data[..]),
            DataBlockEnum::UInt8(data) => DataSlice::UInt8(&data[..]),
            DataBlockEnum::Half16(data) => DataSlice::Half16(&data[..]),
            DataBlockEnum::UInt16(data) => DataSlice::UInt16(&data[..]),
        }
    }

    pub fn as_slice_f32(&self) -> &[f32] {
        match &self.inner {
            DataBlockEnum::Float32(data) => &data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn as_slice_u16(&self) -> &[u16] {
        match &self.inner {
            DataBlockEnum::UInt16(data) => &data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn as_slice_f16(&self) -> &[f16] {
        match &self.inner {
            DataBlockEnum::Half16(data) => &data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn as_slice_u8(&self) -> &[u8] {
        match &self.inner {
            DataBlockEnum::UInt8(data) => &data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn as_mut_slice(&mut self) -> DataSliceMut {
        match &mut self.inner {
            DataBlockEnum::Float32(data) => DataSliceMut::Float32(&mut data[..]),
            DataBlockEnum::UInt8(data) => DataSliceMut::UInt8(&mut data[..]),
            DataBlockEnum::Half16(data) => DataSliceMut::Half16(&mut data[..]),
            DataBlockEnum::UInt16(data) => DataSliceMut::UInt16(&mut data[..]),
        }
    }

    pub fn as_mut_slice_f32(&mut self) -> &mut [f32] {
        match &mut self.inner {
            DataBlockEnum::Float32(data) => &mut data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn as_mut_slice_f16(&mut self) -> &mut [f16] {
        match &mut self.inner {
            DataBlockEnum::Half16(data) => &mut data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn as_mut_slice_u16(&mut self) -> &mut [u16] {
        match &mut self.inner {
            DataBlockEnum::UInt16(data) => &mut data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn as_mut_slice_u8(&mut self) -> &mut [u8] {
        match &mut self.inner {
            DataBlockEnum::UInt8(data) => &mut data[..],
            _ => panic!("Unsupported DataType."),
        }
    }

    pub fn data_type(&self) -> DataType {
        match &self.inner {
            DataBlockEnum::Float32(_) => DataType::Float32,
            DataBlockEnum::UInt8(_) => DataType::UInt8,
            DataBlockEnum::Half16(_) => DataType::Half16,
            DataBlockEnum::UInt16(_) => DataType::UInt16,
        }
    }

    pub fn len(&self) -> usize {
        match &self.inner {
            DataBlockEnum::Float32(data) => data.len(),
            DataBlockEnum::UInt8(data) => data.len(),
            DataBlockEnum::Half16(data) => data.len(),
            DataBlockEnum::UInt16(data) => data.len(),
        }
    }

    pub fn size_bytes(&self) -> usize {
        let data_type = self.data_type();
        let size_bytes = data_type.size_bytes();
        self.len() * size_bytes
    }

    pub fn data_resize(&mut self, new_length: usize, data_type: DataType) {
        debug!(
            "data_resize: new_length={:#?} data_type={:#?}",
            new_length, data_type
        );
        // Resize with all values defaulting to "1.0" in the native
        // data type.
        //
        // TODO: The fill value should be exposed to the user.
        let fill_value_f32 = 1.0 as f32;
        let fill_value_f16 = f16::from_f32(1.0);
        let fill_value_u8 = u8::MAX;
        let fill_value_u16 = u16::MAX;
        match data_type {
            DataType::Float32 => match &mut self.inner {
                DataBlockEnum::Float32(data) => {
                    data.resize(new_length, fill_value_f32);
                }
                DataBlockEnum::UInt8(_) | DataBlockEnum::Half16(_) | DataBlockEnum::UInt16(_) => {
                    let new_data = vec![fill_value_f32; new_length];
                    self.inner = DataBlockEnum::Float32(new_data);
                }
            },
            DataType::UInt8 => match &mut self.inner {
                DataBlockEnum::UInt8(data) => {
                    data.resize(new_length, fill_value_u8);
                }
                DataBlockEnum::Float32(_) | DataBlockEnum::Half16(_) | DataBlockEnum::UInt16(_) => {
                    let new_data = vec![fill_value_u8; new_length];
                    self.inner = DataBlockEnum::UInt8(new_data);
                }
            },
            DataType::UInt16 => match &mut self.inner {
                DataBlockEnum::UInt16(data) => {
                    data.resize(new_length, fill_value_u16);
                }
                DataBlockEnum::Float32(_) | DataBlockEnum::UInt8(_) | DataBlockEnum::Half16(_) => {
                    let new_data = vec![fill_value_u16; new_length];
                    self.inner = DataBlockEnum::UInt16(new_data);
                }
            },
            DataType::Half16 => match &mut self.inner {
                DataBlockEnum::Half16(data) => {
                    data.resize(new_length, fill_value_f16);
                }
                DataBlockEnum::UInt8(_) | DataBlockEnum::Float32(_) | DataBlockEnum::UInt16(_) => {
                    let new_data = vec![fill_value_f16; new_length];
                    self.inner = DataBlockEnum::Half16(new_data);
                }
            },
            _ => panic!("Unsupported DataType: {:#?}", data_type),
        };
    }

    pub fn convert_into_data_type(&mut self, to_data_type: DataType) {
        let from_data_type = self.data_type();
        debug!(
            "Convert Pixel Data Type from {:#?} to {:#?}",
            from_data_type, to_data_type
        );
        if from_data_type == to_data_type {
            return;
        }

        self.inner = match to_data_type {
            DataType::Float32 => {
                let new_data = convert_datablock_enum_to_f32(&self.inner);
                DataBlockEnum::Float32(new_data)
            }
            DataType::Half16 => {
                let new_data = convert_datablock_enum_to_f16(&self.inner);
                DataBlockEnum::Half16(new_data)
            }
            DataType::UInt8 => {
                let new_data = convert_datablock_enum_to_u8(&self.inner);
                DataBlockEnum::UInt8(new_data)
            }
            DataType::UInt16 => {
                let new_data = convert_datablock_enum_to_u16(&self.inner);
                DataBlockEnum::UInt16(new_data)
            }
            _ => panic!("Unsupported DataType: {:#?}", to_data_type),
        }
    }
}

impl Hash for DataBlock {
    fn hash<H: Hasher>(&self, _state: &mut H) {
        // Note: Skipping the actual pixels data.
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pixelblock::pixelblock::PixelBlock;

    #[test]
    fn new() {
        let size = 9;
        let data_type = DataType::Float32;
        let db = DataBlock::new(size, data_type);
        assert_eq!(db.len(), size);
    }

    #[test]
    fn from_slice() {
        let values = &[-1.0, 0.0, 1.0];
        let data_values = DataSlice::Float32(values);
        let db = DataBlock::from_slice(data_values);
        assert_eq!(db.len(), 3usize);
        assert_eq!(db.size_bytes(), values.len() * 4usize);
    }

    #[test]
    fn from_slice_f32() {
        let values = &[-1.0, 0.0, 1.0];
        let db = DataBlock::from_slice_f32(values);
        assert_eq!(db.len(), values.len());
        assert_eq!(db.size_bytes(), values.len() * 4usize);
    }

    #[test]
    fn from_slice_u16() {
        let values = &[0, 127, 255, 25252, u16::MAX];
        let db = DataBlock::from_slice_u16(values);
        assert_eq!(db.len(), values.len());
        assert_eq!(db.size_bytes(), values.len() * 2usize);
    }

    #[test]
    fn from_slice_u8() {
        let values = &[0, 42, 127, 255];
        let db = DataBlock::from_slice_u8(values);
        assert_eq!(db.len(), values.len());
        assert_eq!(db.size_bytes(), values.len());
    }
}
