/*
 * File: lib.rs
 * Project: src
 * Created Date: 30/06/2020
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/12/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2020 Hapis Lab. All rights reserved.
 *
 */

pub mod primitives;

use crate::consts::*;
use crate::geometry::Vector3;
use crate::Float;

pub struct PointSequence {
    control_points: Vec<Vector3>,
    sample_freq_div: u16,
    sent: usize,
}

impl PointSequence {
    pub fn new() -> Self {
        Self {
            control_points: vec![],
            sample_freq_div: 1,
            sent: 0,
        }
    }

    pub fn with_control_points(control_points: Vec<Vector3>) -> Self {
        Self {
            control_points,
            sample_freq_div: 1,
            sent: 0,
        }
    }

    pub fn append_point(&mut self, point: Vector3) {
        if self.control_points.len() + 1 > POINT_SEQ_BUFFER_SIZE_MAX {
            panic!(
                "Point sequence buffer overflow. Maximum available buffer size is {}.",
                POINT_SEQ_BUFFER_SIZE_MAX
            );
        }
        self.control_points.push(point);
    }

    pub fn append_points(&mut self, points: &[Vector3]) {
        if self.control_points.len() + points.len() > POINT_SEQ_BUFFER_SIZE_MAX {
            panic!(
                "Point sequence buffer overflow. Maximum available buffer size is {}.",
                POINT_SEQ_BUFFER_SIZE_MAX
            );
        }
        self.control_points.extend_from_slice(points);
    }

    pub fn set_freq(&mut self, freq: Float) -> Float {
        let sample_freq = self.control_points.len() as Float * freq;
        let div = (POINT_SEQ_BASE_FREQ / sample_freq) as usize;
        let lm_cycle = self.control_points.len() * div;

        let actual_div = if lm_cycle > POINT_SEQ_CLK_IDX_MAX {
            (POINT_SEQ_CLK_IDX_MAX / self.control_points.len()) as u16
        } else {
            div as u16
        };
        self.sample_freq_div = actual_div;

        self.freq()
    }

    pub fn freq(&self) -> Float {
        self.sampling_freq() / self.control_points.len() as Float
    }

    pub fn sampling_freq(&self) -> Float {
        POINT_SEQ_BASE_FREQ / self.sample_freq_div as Float
    }

    pub fn sampling_freq_div(&self) -> u16 {
        self.sample_freq_div
    }

    pub fn sent(&mut self) -> &mut usize {
        &mut self.sent
    }

    pub fn control_points(&self) -> &[Vector3] {
        &self.control_points
    }
}

impl Default for PointSequence {
    fn default() -> Self {
        Self::new()
    }
}
