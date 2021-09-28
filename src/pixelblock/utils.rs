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
use log::error;

use crate::cxxbridge::ffi::DataType;
use crate::pixelblock::datablock::DataBlockEnum;

#[inline]
pub fn convert_datablock_enum_to_f32(data_block_enum: &DataBlockEnum) -> Vec<f32> {
    let mut new_pixels = Vec::<f32>::new();
    match &data_block_enum {
        DataBlockEnum::Half16(data) => {
            let f = |x: f16| x.to_f32();
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::UInt8(data) => {
            let f = |x: u8| (x as f32) / (u8::max_value() as f32);
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::UInt16(data) => {
            let f = |x: u16| (x as f32) / (u16::max_value() as f32);
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        _ => panic!("Unsupported DataType: {:#?}", data_block_enum),
    }
    new_pixels
}

#[inline]
pub fn convert_datablock_enum_to_f16(data_block_enum: &DataBlockEnum) -> Vec<f16> {
    let mut new_pixels = Vec::<f16>::new();
    match &data_block_enum {
        DataBlockEnum::Float32(data) => {
            let f = |x: f32| f16::from_f32(x);
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::UInt8(data) => {
            let f = |x: u8| f16::from_f32((x as f32) / (u8::max_value() as f32));
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::UInt16(data) => {
            let f = |x: u16| f16::from_f32((x as f32) / (u16::max_value() as f32));
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        _ => panic!("Unsupported DataType: {:#?}", data_block_enum),
    }
    new_pixels
}

#[inline]
pub fn convert_datablock_enum_to_u16(data_block_enum: &DataBlockEnum) -> Vec<u16> {
    let mut new_pixels = Vec::<u16>::new();
    match &data_block_enum {
        DataBlockEnum::Float32(data) => {
            let f = |x: f32| ((x as f32) * (u16::max_value() as f32)) as u16;
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::Half16(data) => {
            let f = |x: f16| {
                let v = f32::from(x);
                (v * (u16::max_value() as f32)) as u16
            };
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::UInt8(data) => {
            let f = |x: u8| {
                let v = (x as f32) / (u8::max_value() as f32);
                (v * (u16::max_value() as f32)) as u16
            };
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        _ => panic!("Unsupported DataType: {:#?}", data_block_enum),
    }
    new_pixels
}

#[inline]
pub fn convert_datablock_enum_to_u8(data_block_enum: &DataBlockEnum) -> Vec<u8> {
    let mut new_pixels = Vec::<u8>::new();
    match &data_block_enum {
        DataBlockEnum::Float32(data) => {
            let f = |x: f32| ((x as f32) * (u8::max_value() as f32)) as u8;
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::Half16(data) => {
            let f = |x: f16| (f32::from(x) * (u8::max_value() as f32)) as u8;
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        DataBlockEnum::UInt16(data) => {
            let f = |x: u16| {
                let v = (x as f32) / (u16::max_value() as f32);
                (v * (u8::max_value() as f32)) as u8
            };
            convert_pixel_data_value_types(&data[..], &mut new_pixels, f);
        }
        _ => panic!("Unsupported DataType: {:#?}", data_block_enum),
    }
    new_pixels
}

/// Get the number of channels that will be stored in-memory, aligned
/// to memory in such a way that OpenGL inside Maya will work easily.
#[inline]
pub fn stride_num_channels(num_channels: i32, pixel_data_type: DataType) -> usize {
    match pixel_data_type {
        // Force all 8-bit image pixels to be 4-byte aligned. Add
        // an extra channel for RGB images.
        //
        // 8 + 8 + 8 + 8 = 32 bytes per-pixel (aligns to 4-bytes)
        DataType::UInt8 => 4,

        // Force all 16-bit image pixels to be 4-byte aligned. Add
        // an extra channel for RGB images.
        //
        // 16 + 16 + 16 + 16 = 64 bytes per-pixel (aligns to 4-bytes)
        DataType::Half16 => 4,
        DataType::UInt16 => 4,

        // A 32-bit-per-channel image pixel is always 4-byte aligned,
        // so we can handle either 3 or 4 channels without any
        // padding.
        //
        // 32 + 32 + 32 + 32 = 128 bytes per-pixel (aligns to 4-bytes)
        // 32 + 32 + 32      = 96 bytes per-pixel (aligns to 4-bytes)
        DataType::Float32 => num_channels as usize,
        _ => panic!("Invalid DataType: {:?}", pixel_data_type),
    }
}

/// Get the number of bytes for each data type.
#[inline]
pub fn channel_size_bytes(pixel_data_type: DataType) -> usize {
    let num_bytes = match pixel_data_type {
        DataType::UInt8 => std::mem::size_of::<u8>(),
        DataType::Half16 => std::mem::size_of::<f16>(),
        DataType::Float32 => std::mem::size_of::<f32>(),
        DataType::UInt16 => std::mem::size_of::<u16>(),
        _ => {
            error!("Invalid DataType: {:?}", pixel_data_type);
            0
        }
    };
    num_bytes
}

/// Get the pixel index into a packed data structure of RGB(A) pixel
/// data.
///
/// Returns the index if the x,y coordinates are with-in bounds,
/// otherwise -1 is returned.
#[inline]
pub fn get_pixel_index(
    width: i32,
    height: i32,
    x_stride: i32,
    y_stride: i32,
    x: i32,
    y: i32,
) -> isize {
    let index_x = x * x_stride;
    let index_y = y * y_stride;
    let max_x = (width - 1) * x_stride;
    let max_y = (height - 1) * y_stride;
    let mut index = index_x + index_y;
    if (index_x < 0) || (index_x > max_x) || (index_y < 0) || (index_y > max_y) {
        index = -1;
    }
    return index as isize;
}

/// SAFETY: It is safe to convert the pixel data from 'f32' to 'u8'
/// because there can be no alignment issues. f32 is 4-byte
/// aligned. u8 is 1-byte aligned which fits into 4-bytes evenly.
#[inline]
pub fn transmute_slice_f32_to_u8(pixel_slice: &[f32]) -> &[u8] {
    let pixels_u8 = unsafe { std::mem::transmute::<&[f32], &[u8]>(pixel_slice) };
    pixels_u8
}

/// SAFETY: It is safe to convert the pixel data from 'f16' to 'u8'
/// because there can be no alignment issues. f16 is 2-byte
/// aligned. u8 is 1-byte aligned which fits into 4-bytes evenly.
#[inline]
pub fn transmute_slice_f16_to_u8(pixel_slice: &[f16]) -> &[u8] {
    let pixels_u8 = unsafe { std::mem::transmute::<&[f16], &[u8]>(pixel_slice) };
    pixels_u8
}

/// SAFETY: It is safe to convert the pixel data from 'u16' to 'u8'
/// because there can be no alignment issues. u16 is 2-byte
/// aligned. u8 is 1-byte aligned which fits into 4-bytes evenly.
#[inline]
pub fn transmute_slice_u16_to_u8(pixel_slice: &[u16]) -> &[u8] {
    let pixels_u8 = unsafe { std::mem::transmute::<&[u16], &[u8]>(pixel_slice) };
    pixels_u8
}

/// SAFETY: It is safe to convert the pixel data from 'f32' to 'u16'
/// because there can be no alignment issues. f32 is 4-byte
/// aligned. u16 is 2-byte aligned, which fits into 4-bytes evenly.
#[inline]
pub fn transmute_slice_f32_to_u16(pixel_slice: &[f32]) -> &[u16] {
    let pixels_u16 = unsafe { std::mem::transmute::<&[f32], &[u16]>(pixel_slice) };
    pixels_u16
}

/// SAFETY: It is safe to convert the pixel data from 'f32' to 'f16'
/// because there can be no alignment issues. f32 is 4-byte
/// aligned. f16 is 2-byte aligned, which fits into 4-bytes evenly.
#[inline]
pub fn transmute_slice_f32_to_f16(pixel_slice: &[f32]) -> &[f16] {
    let pixels_f16 = unsafe { std::mem::transmute::<&[f32], &[f16]>(pixel_slice) };
    pixels_f16
}

/// SAFETY: It is safe to convert the pixel data from 'f16' to 'u16'
/// because both data types are aligned the same and take the same
/// amount of memory.
#[inline]
pub fn transmute_slice_f16_to_u16(pixel_slice: &[f16]) -> &[u16] {
    let pixels_u16 = unsafe { std::mem::transmute::<&[f16], &[u16]>(pixel_slice) };
    pixels_u16
}

/// SAFETY: It is safe to convert the pixel data from 'f16' to 'u16'
/// because both data types are aligned the same and take the same
/// amount of memory.
pub fn transmute_slice_mut_f16_to_u16(pixel_slice: &mut [f16]) -> &mut [u16] {
    let pixels_u16 = unsafe { std::mem::transmute::<&mut [f16], &mut [u16]>(pixel_slice) };
    pixels_u16
}

/// Convert a slice of pixels from T to f32 type.
///
/// The values from the pixel slice T are converted and stored into a
/// Vec<U>.
#[inline]
pub fn convert_pixel_data_value_types<T, U, F>(
    pixel_slice: &[T],
    new_pixels: &mut Vec<U>,
    convert_func: F,
) where
    T: Copy,
    U: Copy,
    F: Fn(T) -> U,
{
    assert_eq!(new_pixels.len(), 0);
    let pixels_count = pixel_slice.len();
    new_pixels.reserve_exact(pixels_count);
    for from_value in pixel_slice {
        let to_value = convert_func(*from_value);
        new_pixels.push(to_value);
    }
}
