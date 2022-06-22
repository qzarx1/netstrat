use std::cmp::Ordering;

use tracing::debug;

use crate::sources::binance::client::Kline;

#[derive(Default, Clone)]
pub struct Data {
    pub vals: Vec<Kline>,
    max_x: f64,
    max_y: f64,
    max_vol: f64,
}

impl Data {
    pub fn new(vals: Vec<Kline>) -> Self {
        let max_y = vals
            .iter()
            .max_by(|l, r| {
                if l.high > r.high {
                    return Ordering::Greater;
                }

                Ordering::Less
            })
            .unwrap()
            .high as f64;

        let max_vol = vals
            .iter()
            .max_by(|l, r| {
                if l.volume > r.volume {
                    return Ordering::Greater;
                }

                Ordering::Less
            })
            .unwrap()
            .volume as f64;

        let max_x = vals[vals.len() - 1].t_close as f64;

        debug!(
            "computed max_x: {},  max_y: {},  max_vol: {}",
            max_x, max_y, max_vol
        );

        Self {
            vals,
            max_x,
            max_y,
            max_vol,
        }
    }

    pub fn max_x(&self) -> f64 {
        self.max_x
    }

    pub fn max_y(&self) -> f64 {
        self.max_y
    }

    pub fn max_vol(&self) -> f64 {
        self.max_vol
    }
}
