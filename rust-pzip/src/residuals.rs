const ONE_ZERO_U32: u32 = 2863311530;
const ZERO_ONE_U32: u32 = 1431655765;
// const ZERO_ONES: (u32,u32) = (2863311530, 1431655765);
// const GRAY_ONE_ZERO_U32: u32 = (4294967295 >> 11) + (3 << 30);
// const GRAY_ZERO_ONE_U32: u32 = (2147483647 >> 11) + (3 << 30);
// const GRAY_ONE_ZERO_U32: u32 = (3 << 30);
// const GRAY_ZERO_ONE_U32: u32 = (3 << 30);
const GRAY_ONE_ZERO_U32: u32 = 3435973836;
const GRAY_ZERO_ONE_U32: u32 = 858993459;
// TODO: Current implmentation only supports u32 values. Add to this u64.
// const ONE_ZERO_U64: u64 = 12297829382473034410;
// const ZERO_ONE_U64: u64 = 6148914691236517205;
// use log::{debug, warn};

// const BINARY_GOAL: GoalBase = GoalBase { one_zero: 2863311530, zero_one: 1431655765 };
// const GRAY_GOAL: GoalBase = GoalBase { one_zero: 3 << 30, zero_one: 3 << 30 };

// #[derive(Debug)]
// struct GoalBase {
//     one_zero: u32,
//     zero_one: u32
// }

#[derive(Debug)]
pub struct RContext {
    cut: u32,
    truth: u32,
    prediction: u32,
    prediction_too_low: bool,
}

impl RContext {
    pub fn new(cut: u32) -> Self {
        RContext { cut: cut , truth: 0, prediction: 0, prediction_too_low: false}
    }
}

pub trait ResidualTrait {
    fn residual(&self, truth: u32, prediction: u32, rctx: &mut RContext) -> u32;
    fn truth(&self, residual: u32, prediction: u32, rctx: &mut RContext) -> u32;
    fn update(&self, truth: u32, prediction: u32, rctx: &mut RContext);
}

#[derive(Debug)]
pub enum ResidualCalculation {
    ExclusiveOR,
    Shifted,
    ShiftedLZC,
    ShiftedGray,
    Diff,
    // TODO: Simply Difference
    // TODO: Choose a goal for shifted value solely based on the prediction
    // TODO: Choose residual based on experience (past values) instead of LZC or given cut
    // TODO: Choose cut based on length of ones/zeros and(!) decimal point of these positions in floating point representation
    // TODO: Choose shift in such a way that the former LZC position will be a 1
    // TODO: Choose shift in such a way that the former LZC position will be a 0
}

impl ResidualTrait for ResidualCalculation {
    fn residual(&self, truth: u32, prediction: u32, rctx: &mut RContext) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => {
                let result = truth ^ prediction;
                //debug!("Residual (Normal XOR) Truth: {:032b} Prediction: {:032b} XOR: {:032b}", truth, prediction, result);
                result
            },
            ResidualCalculation::Shifted => {
                let (add, shift) = shift_calculation(prediction, rctx);
                let shifted_prediction = apply_shift(prediction, add, shift);
                let shifted_truth = apply_shift(truth, add, shift);
                let result = shifted_prediction ^ shifted_truth;
                //debug!("Panic?\n T{:032b}\n P{:032b}\nST{:032b}\nSP{:032b}\n X{:032b}\nSX{:032b}\n", truth, prediction, shifted_truth, shifted_prediction, (truth ^ prediction), result);
                //debug!("New: {} Old: {}", result.leading_zeros(), (truth ^ prediction).leading_zeros());
                if result.leading_zeros() < (truth ^ prediction).leading_zeros() - 1 {
                    //warn!("LZC worse using shift by {}", (truth ^ prediction).leading_zeros() - result.leading_zeros());
                }
                result
            },
            ResidualCalculation::ShiftedGray => {
                let (add, shift) = shift_calculation_gray(prediction, rctx);
                let shifted_prediction = apply_shift(prediction, add, shift);
                let shifted_truth = apply_shift(truth, add, shift);
                let result = shifted_prediction ^ shifted_truth;
                //debug!("Panic?\n T{:032b}\n P{:032b}\nST{:032b}\nSP{:032b}\n X{:032b}\nSX{:032b}\n", truth, prediction, shifted_truth, shifted_prediction, (truth ^ prediction), result);
                //debug!("New: {} Old: {}", result.leading_zeros(), (truth ^ prediction).leading_zeros());
                if result.leading_zeros() < (truth ^ prediction).leading_zeros() - 1 {
                    //warn!("LZC worse using shift by {}", (truth ^ prediction).leading_zeros() - result.leading_zeros());
                }
                result
            },
            ResidualCalculation::ShiftedLZC => {
                let (add, shift) = shift_calculation(prediction, rctx);
                let shifted_prediction = apply_shift(prediction, add, shift);
                let shifted_truth = apply_shift(truth, add, shift);
                let result = shifted_prediction ^ shifted_truth;
                //debug!("Panic?\n T{:032b}\n P{:032b}\nST{:032b}\nSP{:032b}\n X{:032b}\nSX{:032b}\n", truth, prediction, shifted_truth, shifted_prediction, (truth ^ prediction), result);
                //debug!("Panic?\n T{}\n P{}\nST{}\nSP{}\n X{}\nSX{}\n", truth, prediction, shifted_truth, shifted_prediction, (truth ^ prediction), result);
                //debug!("New: {} Old: {}", result.leading_zeros(), (truth ^ prediction).leading_zeros());
                if result.leading_zeros() < (truth ^ prediction).leading_zeros() - 1 {
                    //warn!("LZC worse using shift by {}", (truth ^ prediction).leading_zeros() - result.leading_zeros());
                }
                result
            },
            ResidualCalculation::Diff => {
                (prediction).max(truth) - (prediction).min(truth)
            }
        }
    }
    fn truth(&self, residual: u32, prediction: u32, rctx: &mut RContext) -> u32 {
        match self {
            ResidualCalculation::ExclusiveOR => residual ^ prediction,
            ResidualCalculation::Shifted => {
                let (add, shift) = shift_calculation(prediction, rctx);
                let shifted_prediction = apply_shift(prediction, add, shift);
                let shifted_truth = residual ^ shifted_prediction;
                let truth = apply_shift(shifted_truth, !add, shift);
                truth
            },
            ResidualCalculation::ShiftedGray => {
                let (add, shift) = shift_calculation_gray(prediction, rctx);
                let shifted_prediction = apply_shift(prediction, add, shift);
                let shifted_truth = residual ^ shifted_prediction;
                let truth = apply_shift(shifted_truth, !add, shift);
                truth
            },
            ResidualCalculation::ShiftedLZC => {
                let (add, shift) = shift_calculation(prediction, rctx);
                let shifted_prediction = apply_shift(prediction, add, shift);
                let shifted_truth = residual ^ shifted_prediction;
                let truth = apply_shift(shifted_truth, !add, shift);
                truth
            },
            ResidualCalculation::Diff => {
                if rctx.prediction_too_low {
                    prediction + residual
                } else {
                    prediction - residual
                }
            }
        }
    }
    fn update(&self, truth: u32, prediction: u32, rctx: &mut RContext) {
        rctx.prediction = prediction;
        rctx.truth = truth;
        match self {
            ResidualCalculation::ExclusiveOR => {},
            ResidualCalculation::Shifted => {},
            ResidualCalculation::ShiftedGray => {},
            ResidualCalculation::ShiftedLZC => {
                let new_cut = 32 - (truth ^ prediction).leading_zeros();
                rctx.cut = new_cut.max(4); //TODO: Test influence of minimal cut
            },
            ResidualCalculation::Diff => {
                rctx.prediction_too_low = prediction < truth;
            }
        }
    }
}

fn shift_calculation(num: u32, rctx: &mut RContext) -> (bool, u32) {
    let bits = 32;
    let base = (num >> rctx.cut) << rctx.cut;
    let last_value = (num >> rctx.cut) & 1;
    if last_value == 1 {
        let delta = ZERO_ONE_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        //debug!("Shift {0:032b} @ {1} by {2:032b} to {3:032b} f", num, rctx.cut, shift, goal);
        return (num <= goal, shift);
    } else {
        let delta = ONE_ZERO_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        //debug!("Shift {0:032b} @ {1} by {2:032b} to {3:032b} t", num, rctx.cut, shift, goal);
        return (num < goal, shift);
    }
}

fn shift_calculation_gray(num: u32, rctx: &mut RContext) -> (bool, u32) {
    // let bits = 32;
    let base = (num >> rctx.cut) << rctx.cut;
    let last_value = (num >> rctx.cut) & 1;
    if last_value == 1 {
        let delta = GRAY_ZERO_ONE_U32 ;//>> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        //debug!("Shift {0:032b} @ {1} by {2:032b} to {3:032b} f", num, rctx.cut, shift, goal);
        return (num <= goal, shift);
    } else {
        let delta = GRAY_ONE_ZERO_U32;// >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        //debug!("Shift {0:032b} @ {1} by {2:032b} to {3:032b} t", num, rctx.cut, shift, goal);
        return (num < goal, shift);
    }
}

fn apply_shift(num: u32, sign: bool, delta: u32) -> u32 {
    if sign {
        let result = num + delta;
        //debug!("Apply Shift {0:032b} + {2:032b} = {1:032b}", *delta, result, num);
        return result
    } else {
        let result = num - delta;
        // debug!("Apply Shift {0:032b} + {2:032b} = {1:032b}", delta, result, num);
        return result
    }
}
