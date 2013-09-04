use std::int::from_str;
use std::io::{Reader, ReaderUtil};

enum Result<'self, V> {
    Parsed(V, &'self str),
    NoParse
}

fn pAdditive<'input>(input: &'input str) -> Result<'input, int> {
    // additive <-- multitive
    let alt2: Result<int> = match pMultitive(input){
        Parsed(v, s) => Parsed(v, s),
        NoParse => NoParse
    };

    // additive <-- multitive '+' additive
    let alt1: Result<int> = match pMultitive(input){
        Parsed(vLeft, s2) => {
            if s2.starts_with("+") {
                match pAdditive(s2.slice_chars(1, s2.len())){
                    Parsed(vRight, s3) => Parsed(vLeft + vRight, s3),
                    _ => alt2
                }
            } else {
                alt2
            }
        },
        _ => alt2
    };

    return alt1;
}

fn pMultitive<'input>(input: &'input str) -> Result<'input, int> {
    // multitive <-- primary
    let alt2 = match pPrimary(input){
        Parsed(v, s) => Parsed(v, s),
        NoParse => NoParse
    };

    // multitive <-- primary '*' multitive
    let alt1 = match pPrimary(input){
        Parsed(vLeft, s2) => {
            if s2.starts_with("*") {
                match pMultitive(s2.slice_chars(1, s2.len())){
                    Parsed(vRight, s3) => Parsed(vLeft * vRight, s3),
                        _ => alt2
                }
            } else {
                alt2
            }
        },
        _ => alt2
    };
    return alt1;
}

fn pPrimary<'input>(input: &'input str) -> Result<'input, int> {
    // primary <-- decimal
    let alt2 = match pDecimal(input){
        Parsed(v, s2) => Parsed(v, s2),
        NoParse => NoParse
    };

    //primary <-- ('additive')
    let alt1 = if input.starts_with("("){
        match pAdditive(input.slice_chars(1, input.len())){
            Parsed(v, s2) => {
                if s2.starts_with(")") {
                    Parsed(v, s2.slice_chars(1, s2.len()))
                } else {
                    alt2
                }
            },
            _ => alt2
        }
    } else {
        alt2
    };
    return alt1;
}

fn pDecimal<'input>(input: &'input str) -> Result<'input, int> {
    if input.len() == 0 {
        return NoParse
    }
    let remainder: &'input str = if input.len() > 1 {
        input.slice(1, input.len())
    } else {
        ""
    };
    let parseResult = from_str(input.slice(0, 1));
    return match parseResult {
        Some(result) => Parsed(result, remainder),
        None => NoParse
    };
}

fn eval(input: &str) -> int {
    let result = match pAdditive(input){
        Parsed(v, s) => v,
        _ => fail!("No match for input")
    };
    return result;
}

fn main(){
    loop{
        let in = std::io::stdin().read_line();
        println(fmt!("%d", eval(in)));
    }
}


