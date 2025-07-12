use core::panic;
use std::collections::HashMap;
use super::enums::FrameSymbol;
use super::types::FrameCounter;

pub struct Parser {
    json: String,
    frame_counter: FrameCounter,
}

impl Parser {
    pub fn new(json: String) -> Self {
        Parser { json, frame_counter: HashMap::new() }
    }

    pub fn execute(&mut self) {
        for (_, symbol) in self.json.clone().chars().enumerate() {
            self.symbol_validator(&symbol);
        }
    }

    fn symbol_validator(&mut self, symbol: &char) {
        if self.frame_counter.len() == 0 {
            self.equal_or_panic(symbol, &'{');
            self.frame_counter.insert(FrameSymbol::Brace, 1);
        }
    }


    fn equal_or_panic(&self, symbol1: &char, symbol2: &char) {
        if symbol1 != symbol2 {
            panic!("Incorrect JSON");
        }
    }
}
