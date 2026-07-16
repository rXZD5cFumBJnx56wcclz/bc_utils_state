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
        state: &TradeState,
        src: &[f64],
        _: &[Signal],
    ) -> f64 {
        if let Some(direction) = self.direction {
            direction
        } else {
            if state.src[4] < src[0] {
                1.
            } else {
                2.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude_tests::prelude::*;

    #[test]
    fn util_res_1() {
        assert_eq_pr!(
            DIRECTION::default().util(
                &TradeState::new(100., vec![0.9; 5], Default::default()),
                &[1.],
                &[]
            ),
            1.
        );
    }
}
