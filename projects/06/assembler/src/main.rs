use std::env;
use std::io::{BufReader, BufWriter};
use std::io::prelude::*;
use std::fs::File;
use std::collections::HashMap;

enum Inst {
    C(CInst),
    A(AInst),
    Symbol(String),
    Blank,
}

#[derive(Debug)]
struct CInst {
    dest: String,
    comp: String,
    jump: String,
}

#[derive(Debug)]
struct AInst {
    value: String
}


fn parse(line: String) -> Inst {
    let mut line = line.clone();

    let comment_offset = line.find("//");
    if let Some(x) = comment_offset {
        line.truncate(x);
    };

    let l = line.trim();

    if !l.is_ascii() {
        panic!("not ascii");
    }

    if l.is_empty() {
        return Inst::Blank;
    }



    if l.contains('@') {

        Inst::A(AInst{value: l.to_string()})

    } else if l.contains('(') {

        Inst::Symbol(l.trim_matches(&['(', ')'][..]).to_string())

    } else {

        let mut s = l.split(&['=', ';'][..]);

        let (dest, comp) = if l.contains('=') {
            (s.next().unwrap().to_string(),
             s.next().unwrap().to_string())
        } else {
            (String::new(),
             s.next().unwrap().to_string())
        };

        let jump = s.next().unwrap_or("").to_string();

        Inst::C(CInst{dest: dest, comp: comp, jump: jump})
    }


}


fn lex_cinst(inst: CInst) -> String {
    let a = if inst.comp.contains('M') {"1"} else {"0"};

    let comp_str = inst.comp.replace(&['A', 'M'][..], "?");

    let c_bits = match comp_str.as_str() {
        "0"   => "101010",
        "1"   => "111111",
        "-1"  => "111010",
        "D"   => "001100",
        "?"   => "110000",
        "!D"  => "001101",
        "!?"  => "110001",
        "-D"  => "001111",
        "-?"  => "110011",
        "D+1" => "011111",
        "?+1" => "110111",
        "D-1" => "001110",
        "?-1" => "110010",
        "D+?" => "000010",
        "D-?" => "010011",
        "?-D" => "000111",
        "D&?" => "000000",
        "D|?" => "010101",
        _     => panic!("invalid computation"),
    };

    let d_bits = match inst.dest.as_str() {
        ""    => "000",
        "M"   => "001",
        "D"   => "010",
        "MD"  => "011",
        "A"   => "100",
        "AM"  => "101",
        "AD"  => "110",
        "AMD" => "111",
        _     => panic!("invalid dest"),
    };

    let j_bits = match inst.jump.as_str() {
        ""    => "000",
        "JGT" => "001",
        "JEQ" => "010",
        "JGE" => "011",
        "JLT" => "100",
        "JNE" => "101",
        "JLE" => "110",
        "JMP" => "111",
        _     => panic!("invalid jump"),
    };

    format!("111{}{}{}{}", a, c_bits, d_bits, j_bits)

}

fn lex_ainst(inst: AInst, count: &mut usize, symbols: &mut HashMap<String, usize>) -> String {
    let a_value = inst.value[1..].parse::<usize>();

    if let Ok(a) = a_value {
        format!("{:016b}", a)
    } else {
        let i_value = &inst.value[1..];
        if let Some(s) = symbols.get(i_value) {
            format!("{:016b}", s)
        } else {
            println!("{:?}", i_value);
            symbols.insert(
                i_value.to_string(),
                *count + 16
                );
            *count += 1;
            format!("{:016b}", symbols.get(i_value).unwrap())
        }
    }
}

fn init_symbols() -> HashMap<String, usize> {
    let mut s = HashMap::new();

    s.insert("R0".to_string(),  0);
    s.insert("R1".to_string(),  1);
    s.insert("R2".to_string(),  2);
    s.insert("R3".to_string(),  3);
    s.insert("R4".to_string(),  4);
    s.insert("R5".to_string(),  5);
    s.insert("R6".to_string(),  6);
    s.insert("R7".to_string(),  7);
    s.insert("R8".to_string(),  8);
    s.insert("R9".to_string(),  9);
    s.insert("R10".to_string(), 10);
    s.insert("R11".to_string(), 11);
    s.insert("R12".to_string(), 12);
    s.insert("R13".to_string(), 13);
    s.insert("R14".to_string(), 14);
    s.insert("R15".to_string(), 15);

    s.insert("SCREEN".to_string(), 16384);
    s.insert("KBD".to_string(), 24576);
    s.insert("SP".to_string(), 0);
    s.insert("LCL".to_string(), 1);
    s.insert("ARG".to_string(), 2);
    s.insert("THIS".to_string(), 3);
    s.insert("THAT".to_string(), 4);

    s
}


fn main() -> std::io::Result<()> {
    let filename = env::args().nth(1).expect("Please provide filename");

    let r = File::open(&filename)?;
    let r = BufReader::new(r);

    let new_filename = format!("{}.hack", &filename
                               .strip_suffix(".asm")
                               .expect("Please provide .asm"));

    let w = File::create(new_filename)?;
    // let mut w = OptDelayWriter::new(w);
    let mut w = BufWriter::new(w);

    let mut symbols = init_symbols();

    let mut v = vec![];

    let mut blank_line_count = 0;
    for (line_num, line) in r.lines().enumerate() {
        let p = parse(line.unwrap());
        match p {
            Inst::C(_) => v.push(p),
            Inst::A(_) => v.push(p),
            Inst::Symbol(i) => {
                let inserted = symbols.insert(
                    i.trim_matches(&['(', ')'][..]).to_string(),
                    line_num - blank_line_count
                    );
                if inserted.is_none() {
                    ()
                } else {
                    panic!("symbol used twice");
                }
                blank_line_count += 1;
            }
            Inst::Blank => blank_line_count += 1,
        };

    }

    let mut count = 0;
    for inst in v {
        match inst {
            Inst::A(i) => w.write(lex_ainst(i, &mut count, &mut symbols).as_bytes()),
            Inst::C(i) => w.write(lex_cinst(i).as_bytes()),
            _        => panic!("dunno how this happened")
        };
        w.write("\n".as_bytes());
    }

    w.flush();


    Ok(())

}
