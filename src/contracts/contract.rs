#![allow(dead_code)]
trait Contract {}

struct UniswapV2Pair {}

impl UniswapV2Pair {
    fn new() -> Self {
        UniswapV2Pair {}
    }
}

impl Contract for UniswapV2Pair {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_new_contract() {
        let contrtact = UniswapV2Pair::new();
    }
}
