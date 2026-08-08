use crate::prelude::*;

pub struct QTY {
    pub amount: f64,
    pub percent_of_capital: f64,
    pub probability_mult: f64,
}

impl Default for QTY {
    fn default() -> Self {
        Self {
            amount: 0.,
            percent_of_capital: 0.,
            probability_mult: 1.,
        }
    }
}

impl QTY {
    pub fn new(amount: f64, percent_of_capital: f64, probability_mult: f64) -> Self {
        Self {
            amount,
            percent_of_capital,
            probability_mult,
        }
    }
}

impl UtilState for QTY {
    fn util(&self, state: &TradeState, _: &[f64], signals: &[Signal]) -> f64 {
        signals.first().unwrap_or(&Signal::default()).probability
            * self.probability_mult
            * (state.capital * self.percent_of_capital + self.amount)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn util_res_1() {
        assert_eq_pr!(
            QTY::new(1., 0.1, 1.).util(&TradeState::new(100.,), &[], &[Signal::new(1., 1.)]),
            11.
        );
    }
}
