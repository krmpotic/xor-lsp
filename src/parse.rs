use std::error::Error;
use std::io::Read;
use csv;

#[derive(Debug)]
pub struct Instruction {
    syntax: String,
//    goSyntax: &'a str,
//    gnuSyntax: &'a str,
//    opcode: &'a str,
//    valid32: &'a str,
//    valid64: &'a str,
//    cpuid: &'a str,
//    tags: &'a [&str],
//    action: &'a str,
//    multisize: &'a str,
//    datasize: i32,
    desc: String 
}

pub fn parse<R: Read>(r: R) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(r);

    let mut instructions: Vec<Instruction> = Vec::new();
    for result in rdr.records() {
        let record = result?;
        eprintln!("{:?}", record);
        let syntax = record[0].to_string();
        let desc = record[11].to_string();
        instructions.push(Instruction{syntax, desc})

    }

    Ok(instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_work() {
        let input = "a string";
        parse(input.as_bytes()).unwrap();
        assert!(true);

    }

    #[test]
    fn it_fails() {
        assert!(false);
    }
}
