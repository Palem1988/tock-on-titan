// Copyright 2018 Google LLC
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     https://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use core::fmt::*;
use crate::pinmux;

use crate::uart;

pub struct Writer;

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> ::core::fmt::Result {
        unsafe {
            let uart = &uart::UART0;

            static mut INITIALIZED: bool = false;
            if !INITIALIZED {
                INITIALIZED = true;

                let pinmux = &mut *pinmux::PINMUX;
                // Drive DIOA0 from TX
                pinmux.dioa0.select.set(pinmux::Function::Uart0Tx);

                uart.config(115200);
            }

            uart.send_bytes_sync(s.as_bytes());
 
            Ok(())
        }
    }
}

#[macro_export]
macro_rules! print {
        ($($arg:tt)*) => (
            {
                use core::fmt::write;
                let mut writer = $crate::io::Writer;
                let _ = write(&mut writer, format_args!($($arg)*));
            }
        );
}

#[macro_export]
macro_rules! println {
        ($fmt:expr) => (print!(concat!($fmt, "\n")));
            ($fmt:expr, $($arg:tt)*) => (print!(concat!($fmt, "\n"), $($arg)*));
}
