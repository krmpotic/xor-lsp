use csv;
use std::collections::HashMap;
use std::error::Error;
use std::io::Read;

#[derive(Debug, serde::Deserialize)]
pub struct X86Data {
    pub syntax: String,
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
    pub desc: String,
}

pub fn parse<R: Read>(r: R) -> Result<HashMap<String, Vec<X86Data>>, Box<dyn Error>> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .comment(Some(b'#'))
        .from_reader(r);

    let mut map: HashMap<String, Vec<X86Data>> = HashMap::new();
    for result in rdr.deserialize() {
        let data: X86Data = result?;
        let inst = data.syntax.clone();
        let inst = match inst.split(' ').nth(0) {
            Some(inst) => inst.to_lowercase(),
            None => continue,
        };

        if !map.contains_key(&inst) {
            let mut v: Vec<X86Data> = Vec::new();
            v.push(data);
            map.insert(inst, v);
        } else {
            map.get_mut(&inst).unwrap().push(data);
        }
    }

    Ok(map)
}
