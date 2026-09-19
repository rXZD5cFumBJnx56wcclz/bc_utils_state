use crate::prelude::*;

pub struct DIRECTION {
    pub direction: Option<f64>,
}

impl Default for DIRECTION {
    fn default() -> Self {
        Self { direction: None }
    }
}

impl DIRECTION {
    pub fn new(direction: Option<f64>) -> Self {
        Self { direction }
    }
}

impl UtilState for DIRECTION {
    fn util(
        &self,
        _state: &TradeState,
        src: &[f64],
        _signals: &[Signal],
        _s: &SETTINGS_TRADE,
    ) -> f64 {
        if let Some(direction) = self.direction {
            direction
        } else {
            if src[0] < src[1] { 1. } else { 2. }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bc_test_kit::prelude::*;
    use bc_utils_lg::structs::capital::Capital;
    use bc_utils_lg::test_state::prelude::*;

    #[test]
    fn util_res_1() {
        assert_eq_pr!(
            DIRECTION::default().util(&TradeState::new(Capital(100.)), &[0.9, 1.,], &[], &TRADE),
            1.
        );
    }
}
