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

mod row;
mod row_iter;
mod rows;
mod rows_iter;

use crate::bbox::bbox2di::row::BBox2DRow;
use crate::bbox::bbox2di::rows::BBox2DRows;
use crate::bbox::bbox2di::rows_iter::BBox2DRowsIterator;
use crate::cxxbridge::ffi::BBox2Di;

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
        if (a.min_x > b.max_x) || (b.min_x > a.max_x) || (a.min_y > b.max_y) || (b.min_y > a.max_y)
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

impl<'a> BBox2DRows<'a> for BBox2Di {
    fn row(&self, row: usize) -> BBox2DRow {
        BBox2DRow::new(self, row)
    }

    fn rows(&'a self) -> Box<dyn Iterator<Item = BBox2DRow> + 'a> {
        Box::new(BBox2DRowsIterator::new(self))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new() {
        let bbox = BBox2Di::new(-20, -20, 20, 20);
        assert_eq!(bbox.width(), 40);
        assert_eq!(bbox.height(), 40);
    }

    #[test]
    fn iter1() {
        let bbox = BBox2Di::new(-20, -20, 20, 20);
        let row = bbox.row(0);

        let mut iter = BBox2DRowIterator::new(&row);
        let x = iter.next();
        let y = iter.next();
        let z = iter.last();
        assert_eq!(x, Some(-20));
        assert_eq!(y, Some(-19));
        assert_eq!(z, Some(19));
    }

    #[test]
    fn iter2() {
        let min = -20;
        let max = 20;
        let bbox = BBox2Di::new(min, min, max, max);
        let row = bbox.row(1);

        let iter = BBox2DRowIterator::new(&row);
        for (i, j) in iter.zip(min..max) {
            assert_eq!(i as i32, j as i32);
        }
    }
}
