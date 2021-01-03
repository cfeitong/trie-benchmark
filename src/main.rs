use ac_benchmark::naive_ac::NaiveAcBuilder;
use anyhow::bail;
use anyhow::Result;
use argh::FromArgs;
use std::{fs::File, io::Read, path::PathBuf};
use std::{
    io::{BufRead, BufReader, Write},
    path::Path,
};

#[derive(FromArgs)]
/// main args
struct Args {
    #[argh(positional)]
    dict: PathBuf,
    #[argh(positional)]
    input: PathBuf,
    /// optional output file path, default to [input].out
    #[argh(option)]
    output: Option<PathBuf>,
}

impl Args {
    fn output(&self) -> PathBuf {
        self.output.clone().unwrap_or_else(|| {
            let filename = self
                .input
                .file_name()
                .map(|n| format!("{}.out", n.to_string_lossy()))
                .unwrap_or_else(|| "output.out".to_owned());
            self.input
                .parent()
                .unwrap_or_else(|| Path::new("."))
                .join(filename)
        })
    }
}

fn main() -> Result<()> {
    let args: Args = argh::from_env();
    let dict = File::open(&args.dict)?;
    let r = BufReader::new(dict);
    let mut ac = NaiveAcBuilder::<Vec<u8>>::new();
    for line in r.lines() {
        let line = line?;
        let mut line: Vec<_> = line.split_whitespace().collect();
        if line.len() != 2 {
            bail!("Invalid dict file");
        }
        let value = line.pop().expect("bug!");
        let key = line.pop().expect("bug!");
        ac.insert(key.as_bytes(), value.as_bytes().to_vec());
    }
    let ac = ac.build();
    let mut input = File::open(&args.input)?;
    let mut text = String::new();
    input.read_to_string(&mut text)?;
    let result = ac.replace(text.as_bytes());
    File::create(args.output())?.write_all(&result)?;
    Ok(())
}
