const ONE_ZERO_U32: u32 = 2863311530;
const ZERO_ONE_U32: u32 = 1431655765;


pub trait ResidualTrait {
    fn residual(&self, truth: u32, prediction: u32, rctx: &mut RContext) -> u32;
    fn truth(&self, residual: u32, prediction: u32, rctx: &mut RContext) -> u32;
}

pub struct RShifted {}

impl ResidualTrait for RShifted {
    fn residual(&self, truth: u32, prediction: u32, rctx: &mut RContext) -> u32 {
        let (add, shift) = shift_calculation(prediction, rctx);
        let shifted_prediction = apply_shift(prediction, &add, &shift);
        let shifted_truth = apply_shift(truth, &add, &shift);
        let result = shifted_prediction ^ shifted_truth;
        result
    }
    fn truth(&self, residual: u32, prediction: u32, rctx: &mut RContext) -> u32 {
        let (add, shift) = shift_calculation(prediction, rctx);
        let shifted_prediction = apply_shift(prediction, &add, &shift);
        let shifted_truth = residual ^ shifted_prediction;
        let truth = apply_shift(shifted_truth, &!add, &shift);
        truth
    }
}

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


fn shift_calculation(num: u32, rctx: &mut RContext) -> (bool, u32) {
    let bits = 32;
    let base = (num >> rctx.cut) << rctx.cut;
    let last_value = (num >> rctx.cut) & 1;
    if last_value == 1 {
        let delta = ZERO_ONE_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        return (num <= goal, shift);
    } else {
        let delta = ONE_ZERO_U32 >> (bits - rctx.cut);
        let goal = base + delta;
        let shift = num.max(goal) - num.min(goal);
        return (num < goal, shift);
    }
}

fn apply_shift(num: u32, sign: &bool, delta: &u32) -> u32 {
    if *sign {
        let result = num + *delta;
        return result
    } else {
        let result = num - *delta;
        return result
    }
}
