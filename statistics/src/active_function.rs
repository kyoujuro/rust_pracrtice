pub mod active_fn{
    pub fn relu(n: i32) -> i32{
        if n < 0{
            return 0;
        }
        else{
            return n;
        }
    }
    pub fn sigmoid(n: f64) -> f64{
        let i = 1.0_f64;
        return i.exp();
    }
    pub fn logit(n: f64) -> f64{
        let i = 1.0_f64;
        return i.exp() / (1.0 + i.exp());
    }
}
