use std::path::PathBuf;

use anyhow::Result;
use serde::Deserialize;

pub const RANKING_FILE: &str = "ranking.toml";
const MIN_RANKING_PERCENTAGE: f64 = 0.1;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct ActivitiesGroup<T> {
    pub weight: f64,
    pub activities: T,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Activity {
    pub weight: f64,
    pub goal: u32,
    pub curve: Curve,
}

impl Activity {
    pub fn ranking(&self, score: f64) -> f64 {
        let numerator = self.curve.numerator;
        let denominator = self.curve.denominator;

        // Michaelis-Menten hyperbolic curve
        // y = (numerator * x) / (denominator + x)
        let ranking = (numerator * score) / (denominator + score);
        tracing::debug!(score, numerator, denominator, ranking, "ranking calculated");

        if ranking == 0.0 {
            0.0
        } else {
            ranking.clamp(MIN_RANKING_PERCENTAGE, 100.0)
        }
    }
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Curve {
    pub numerator: f64,
    pub denominator: f64,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Social {}

impl ActivitiesGroup<Option<Social>> {
    pub fn weighted_ranking(&self, social_score: u16) -> f64 {
        social_score as f64 * self.weight
    }
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
    pub social: ActivitiesGroup<Option<Social>>,
    pub archway: ActivitiesGroup<Archway>,
    pub ecosystem: ActivitiesGroup<Ecosystem>,
}

impl Ranking {
    pub fn load(path: PathBuf) -> Result<Self> {
        super::load(path)
    }
}
