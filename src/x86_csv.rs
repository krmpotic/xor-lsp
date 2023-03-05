use csv;
use std::error::Error;
use std::io::Read;

#[derive(Debug, serde::Deserialize)]
pub struct Instruction {
    syntax: String,
    _go_syntax: String,
    _gnu_syntax: String,
    _opcode: String,
    _valid32: String,
    _valid64: String,
    _cpuid: String,
    _tags: String,
    _action: String,
    _multisize: String,
    _datasize: Option<i32>,
    desc: String,
}

pub fn parse<R: Read>(r: R) -> Result<Vec<Instruction>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(r);

    let mut instructions: Vec<Instruction> = Vec::new();
    for result in rdr.deserialize() {
        let inst: Instruction = result?;
        instructions.push(inst)
    }

    Ok(instructions)
}
