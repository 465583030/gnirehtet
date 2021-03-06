/*
 * Copyright (C) 2017 Genymobile
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

extern crate byteorder;
extern crate chrono;
#[macro_use]
extern crate log;
extern crate mio;
extern crate rand;
extern crate slab;

mod relay;

use std::io;
use relay::Relay;

pub fn relay() -> io::Result<()> {
    const PORT: u16 = 31416;
    Relay::new(PORT).run()
}
