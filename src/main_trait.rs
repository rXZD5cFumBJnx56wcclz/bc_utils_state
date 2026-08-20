use crate::prelude::*;

pub trait UtilState: Any {
    fn util(&self, state: &TradeState, src: &[f64], signals: &[Signal], s: &SETTINGS_TRADE) -> f64;
}
