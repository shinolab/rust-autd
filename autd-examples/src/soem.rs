/*
 * File: soem.rs
 * Project: examples
 * Created Date: 16/12/2019
 * Author: Shun Suzuki
 * -----
 * Last Modified: 31/12/2020
 * Modified By: Shun Suzuki (suzuki@hapis.k.u-tokyo.ac.jp)
 * -----
 * Copyright (c) 2019 Hapis Lab. All rights reserved.
 *
 */

extern crate colored;

mod tests;

use autd::prelude::*;
use autd_soem_link::{EthernetAdapters, SoemLink};
use colored::*;
use std::io;
use std::io::Write;
use tests::*;

fn get_adapter() -> String {
    let adapters: EthernetAdapters = Default::default();
    for (index, adapter) in adapters.into_iter().enumerate() {
        println!("[{}]: {}", index, adapter);
    }

    let i: usize;
    loop {
        print!("{}", "Choose number: ".green().bold());
        io::stdout().flush().unwrap();

        let mut s = String::new();
        io::stdin().read_line(&mut s).unwrap();
        if let Ok(num) = s.trim().parse() {
            if num < adapters.len() {
                i = num;
                break;
            }
        }
    }
    let adapter = &adapters[i];
    adapter.name.to_string()
}

fn main() {
    let mut geometry = Geometry::new();
    geometry.add_device(Vector3::zeros(), Vector3::zeros());

    let ifname = get_adapter();
    let link = SoemLink::new(&ifname, geometry.num_devices() as u16);

    let autd = AUTD::open(geometry, link).expect("Failed to open");

    run(autd).expect("Some error occurred.");
}
