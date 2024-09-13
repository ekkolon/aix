// Copyright 2024 Nelson Dominguez
// SPDX-License-Identifier: MIT OR Apache-2.0

pub mod ci;
pub mod docker;

/// Available types of extras available to setup when generating projects.
#[derive(Clone, PartialEq, Eq)]
pub enum ProjectExtra {
    Docker,
    CI,
}
