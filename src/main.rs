#![allow(unused)]
use std::error::Error;
use std::process;
use serde::{Serialize, Deserialize};
use std::path::PathBuf;

static DENOMINATOR: u64 = 1000;

fn main() {
    // this is peak rust

    //let m5_paths: Vec<PathBuf> = glob::glob("raw_data/5m_data/*.csv").unwrap().into_iter().map(|x| x.unwrap()).collect();
    //let m1_paths: Vec<PathBuf> = glob::glob("raw_data/1m_data/*.csv").unwrap().into_iter().map(|x| x.unwrap()).collect();

    //let mut reader = std::fs::read("raw_data/1m_data/ETHUSDT-1m-2019-01.csv").unwrap();
    //let data = parse_records(reader.as_mut_slice(), 4000).unwrap();

    let mut reader = std::fs::read("raw_data/1m_data/ETHUSDT-1m-2019-01.csv").unwrap();
    let data = parse_records(reader.as_mut_slice(), 4000).unwrap();

    {
        use std::io::prelude::Write;
        let mut file = std::fs::File::create("data.ron").unwrap();
        let mut ser = ron::Serializer::new(file, None, false).unwrap();
        
        data.serialize(&mut ser);
    }
    {
        let d = ronstd::fs::read("data.ron").unwrap();
    }

}

#[derive(Debug, Clone, Default, Deserialize)]
struct Record {
    open_time: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
    close_time: u128,
    quote_asset_volume: f64,
    number_of_trades: u128,
    taker_buy_base_asset_volume: f64,
    taker_buy_quote_asset_volume: f64,
    ignore: u8,
}


#[derive(Debug, Clone, Default, Serialize, Deserialize)]
struct Data {
    open_time: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

fn parse_records(reader: &mut [u8], size: usize) -> Result<Vec<Data>, Box<dyn Error>> {
    let mut output: Vec<Data> = Vec::with_capacity(size);

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b'|')
        .from_reader(&*reader);

    for rec in rdr.deserialize::<Record>() {
        match rec {
            Err(e) => panic!("{}", e),
            Ok(val) => {
                output.push(Data {
                    open_time: val.open_time / DENOMINATOR,
                    open: val.open,
                    high: val.high,
                    low: val.low,
                    close: val.close,
                });
            },
        }
    }

    return Ok(output);
}

