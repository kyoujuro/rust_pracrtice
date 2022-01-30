pub mod active_fn{
    pub fn relu(n: i32) -> i32{
        if n < 0{
            return 0;
        }
        else{
            return n;
        }
    }
}