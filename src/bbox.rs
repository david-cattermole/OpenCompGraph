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

use std::hash::{Hash, Hasher};

use crate::cxxbridge::ffi::BBox2Df;
use crate::cxxbridge::ffi::BBox2Di;
use crate::hashutils;

impl BBox2Df {
    pub fn new(min_x: f32, min_y: f32, max_x: f32, max_y: f32) -> BBox2Df {
        BBox2Df {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn width(&self) -> f32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> f32 {
        self.max_y - self.min_y
    }
}

impl Hash for BBox2Df {
    fn hash<H: Hasher>(&self, state: &mut H) {
        hashutils::HashableF32::new(self.min_x).hash(state);
        hashutils::HashableF32::new(self.min_y).hash(state);
        hashutils::HashableF32::new(self.max_x).hash(state);
        hashutils::HashableF32::new(self.max_y).hash(state);
    }
}

impl From<BBox2Di> for BBox2Df {
    fn from(value: BBox2Di) -> Self {
        BBox2Df::new(
            value.min_x as f32,
            value.min_y as f32,
            value.max_x as f32,
            value.max_y as f32,
        )
    }
}

impl BBox2Di {
    pub fn new(min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> BBox2Di {
        BBox2Di {
            min_x,
            min_y,
            max_x,
            max_y,
        }
    }

    pub fn combine(a: BBox2Di, b: BBox2Di) -> BBox2Di {
        BBox2Di {
            min_x: i32::min(a.min_x, b.min_x),
            min_y: i32::min(a.min_y, b.min_y),
            max_x: i32::max(a.max_x, b.max_x),
            max_y: i32::max(a.max_y, b.max_y),
        }
    }

    pub fn intersection(a: BBox2Di, b: BBox2Di) -> BBox2Di {
        if ((a.min_x > b.max_x)
            || (b.min_x > a.max_x)
            || (a.min_y > b.max_y)
            || (b.min_y > a.max_y))
        {
            // No intersetion.
            BBox2Di {
                min_x: 0,
                min_y: 0,
                max_x: 0,
                max_y: 0,
            }
        } else {
            // Found intersection.
            //
            // Note: The region must be *at least* empty, that's why
            // 'i32::max' is used.
            let x1 = i32::max(a.min_x, b.min_x);
            let x2 = i32::max(x1, i32::min(a.max_x, b.max_x));
            let y1 = i32::max(a.min_y, b.min_y);
            let y2 = i32::max(y1, i32::min(a.max_y, b.max_y));
            BBox2Di {
                min_x: x1,
                min_y: y1,
                max_x: x2,
                max_y: y2,
            }
        }
    }

    // pub fn contains(&self, x: i32, y: i32) -> bool {
    //     true
    // }

    pub fn center(&self) -> (i32, i32) {
        let x = (self.min_x + self.max_x) / 2;
        let y = (self.min_y + self.max_y) / 2;
        (x, y)
    }

    pub fn area(&self) -> u32 {
        (self.width() * self.height()) as u32
    }

    pub fn width(&self) -> i32 {
        self.max_x - self.min_x
    }

    pub fn height(&self) -> i32 {
        self.max_y - self.min_y
    }
}
