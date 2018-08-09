extern crate csv;

use std::env;
use std::error::Error;
// use std::fs;
use std::fs::File;
use std::path::Path;
use std::process;

fn process_csv(file_path: &str) -> Result<(String,String), Box<Error>> {
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut predicates = vec!();
    let mut semmed_types = vec!();

    for result in rdr.records() {
        let record = result?;
        let predicate = record[3].to_string();
        let subject_semtype = record[6].to_string();
        let object_semtype = record[10].to_string();

        //vs hashsets, vectors are just easier to work with
        if !predicates.contains(&predicate){
            predicates.push(predicate);
        }

        if !semmed_types.contains(&subject_semtype){
            semmed_types.push(subject_semtype);
        }

        if !semmed_types.contains(&object_semtype){
            semmed_types.push(object_semtype);
        }
    }

    fn quote_and_line (mut v: Vec<String>) -> String {
        v.sort();
        v.iter()
         .fold(String::new(),
               |mut s, pred|
               {
                   s.push('"');
                   for ch in pred.chars(){
                       s.push(ch);
                   }
                   s.push('"');
                   s.push('\n');
                   s
               })
    }

    let semtypes_out: String = quote_and_line(semmed_types);
    let preds_out: String = quote_and_line(predicates);

    Ok((preds_out,semtypes_out))
}

fn run() -> Result<(), Box<Error>>{
    let args: Vec<String> = env::args().collect();

    if let Some(input_csv_path) = args.get(1) {
        //assert exists////////////////////////////////////////////////////////////////////////////
        let (preds_string,semtypes_string) = process_csv(input_csv_path)?;

        //check for out files else dump to stdout//////////////////////////////////////////////////
        print!("{}", preds_string);
        print!("{}", semtypes_string);
        Ok(())
    } else {
        Err(From::from("need input csv path as first arg"))
    }
}

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}
