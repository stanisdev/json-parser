mod system;
use system::parser::Parser;

fn main() {
    let json = "{\"name\":\"Derek\",\"age\":35,\"isDeveloper\":false,\"tags\":[\"land\",\"off\"]}";

    let mut parser = Parser::new(json.to_string());
    parser.execute();
}
