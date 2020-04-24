// Copyright (c) 2020 rose garden cult All rights reserved.
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
//===----------------------------------------------------------------------===//
//
// Three by three matrix representation
//
//===----------------------------------------------------------------------===//

use crate::matrix::Base;
use crate::Component;

use std::fmt;
use std::ops::*;

#[doc = "A 3x3 matrix"]
#[doc = "derives traits: Default, Debug, PartialEq, Eq, Copy, Clone, Hash"]
#[derive(Default, Debug, PartialEq, Eq, Copy, Clone, Hash)]
pub struct Matrix3<T: Component> {
  pub x: T,
}
