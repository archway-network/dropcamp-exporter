use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

pub const RANKING_FILE: &str = "ranking.toml";

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Empty {}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ActivitiesGroup<T> {
    pub weight: f32,
    pub activities: T,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Activity {
    pub weight: f32,
    pub goal: u32,
    pub curve: Curve,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Curve {
    pub numerator: f32,
    pub denominator: f32,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Archway {
    pub stake: Activity,
    pub ibc: Activity,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Ecosystem {
    pub archid: Activity,
    pub astrovault: Activity,
    pub liquid_finance: Activity,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Ranking {
    pub social: ActivitiesGroup<Option<Empty>>,
    pub archway: ActivitiesGroup<Archway>,
    pub ecosystem: ActivitiesGroup<Ecosystem>,
}

impl Ranking {
    pub fn load(path: PathBuf) -> Result<Self> {
        super::load(path)
    }
}
