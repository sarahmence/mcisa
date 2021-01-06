/*
 * mod.rs
 * Module header for mcisa's CPU module
 * Created by Deloi Davis
 * Created on 1/5/2021
 *
 * Copyright (C) 2021 Deloi Davis
 *
 * Licensed under the MIT License (see LICENSE file for details)
 */

// module exports
mod register;
pub use register::Register;
mod flag;
pub use flag::Flag;

// end of file
