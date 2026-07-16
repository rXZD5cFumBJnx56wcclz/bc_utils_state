use std::any::Any;

use bc_utils_lg::structs::{signals::Signal, trade::TradeState};

pub trait UtilState: Any {
    fn util(
        &self,
        state: &TradeState,
        src: &[f64],
        signals: &[Signal],
    ) -> f64;
}
