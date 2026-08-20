use crate::prelude::*;

pub struct QTY {
    pub amount_usd: f64,
    pub percent_of_position: f64,
    pub position_idx: usize,
    pub percent_of_capital: f64,
    pub probability_mult: f64,
}

impl Default for QTY {
    fn default() -> Self {
        Self {
            amount_usd: 0.,
            percent_of_capital: 0.,
            probability_mult: 1.,
            position_idx: 1,
            percent_of_position: 0.,
        }
    }
}

impl QTY {
    pub fn new(
        amount_usd: f64,
        percent_of_position: f64,
        position_idx: usize,
        percent_of_capital: f64,
        probability_mult: f64,
    ) -> Self {
        Self {
            amount_usd,
            percent_of_position,
            position_idx,
            percent_of_capital,
            probability_mult,
        }
    }
}

impl UtilState for QTY {
    fn util(&self, state: &TradeState, src: &[f64], signals: &[Signal], s: &SETTINGS_TRADE) -> f64 {
        signals
            .first()
            .copied()
            .unwrap_or(Signal::default())
            .probability
            * self.probability_mult
            * (state.capital * self.percent_of_capital
                + self.amount_usd
                + self.percent_of_position
                    * state
                        .positions
                        .borrow()
                        .get(&self.position_idx)
                        .cloned()
                        .unwrap_or_default()
                        .qty
                    * self.percent_of_position)
            * s.leverage
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_test_kit::prelude::*;

    #[test]
    fn util_res_1() {
        assert_eq_pr!(
            QTY::new(1., 0., 0, 0.1, 1.,).util(
                &TradeState::new(100.,),
                &[],
                &[Signal::new(1., 1.)],
                &TRADE
            ),
            110.
        );
    }
}
