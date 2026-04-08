use chrono::{TimeZone, FixedOffset};
use blf_asc::{BlfReader};
use std::path::{Path, PathBuf};
use std::{fs, env};
use anyhow::Context;
use std::io::{BufWriter, Write};
use rayon::prelude::*;

fn format_timestamp(ts_float: f64, time_format: &str) -> String {

    let seconds = ts_float as i64;
    let nanoseconds = ((ts_float - seconds as f64) * 1_000_000_000.0) as u32;
    let bj_time_zone = FixedOffset::east_opt(8 * 3600).unwrap();

    if let Some(dt) = bj_time_zone.timestamp_opt(seconds, nanoseconds).single() {
        dt.format(time_format).to_string()
    } else {
        "Invalid Timestamp".to_string()
    }
}

fn parse_blf<P: AsRef<Path>>(file_path: P) -> anyhow::Result<()> {
    let res_dir = Path::new("res");
    if !res_dir.exists() {
        fs::create_dir_all(res_dir)?;
    }

    let mut tmp_filepath = PathBuf::from(file_path.as_ref());
    tmp_filepath.set_extension("tmp");

    let mut first_msg = None;
    let mut last_msg = None;

    {
        let reader_res = BlfReader::open(file_path.as_ref());
        if let Ok(mut reader) = reader_res {
            let out_file = fs::File::create(&tmp_filepath)?;
            let mut writer = BufWriter::new(out_file);
    
            for msg in reader.by_ref() {
                // println!("[{}] chn={} id={} dlc={} data={:?}", format_timestamp(msg.timestamp, "%Y-%m-%d %H:%M:%S%.3f"), msg.channel, msg.arbitration_id, msg.dlc, msg.data);
                if first_msg.is_none() {
                    first_msg = Some(format_timestamp(msg.timestamp, "%Y%m%d-%H%M%S"));
                }
    
                last_msg = Some(format_timestamp(msg.timestamp, "%Y%m%d-%H%M%S"));
                
                writeln!(writer,
                        "[{}]\tchn={}\t {} \t dlc={}\t{:?}",
                        format_timestamp(msg.timestamp, "%Y-%m-%d %H:%M:%S%.3f"),
                        msg.channel,
                        msg.arbitration_id,
                        msg.dlc,
                        msg.data)?;
    
            }
        } else {
            eprintln!("Failed to open BLF: {:?}", reader_res.err());
        }


    }
    let final_name = if let (Some(first), Some(last)) = (first_msg, last_msg) {
        let res = format!("{}---{}.txt", first, last);
        
        res_dir.join(res)
    } else {
        return Err(anyhow::anyhow!("Failed to decided the last filename"));
    };

    fs::rename(&tmp_filepath, final_name)?;

    Ok(())
}


fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} ./<dir contains BLF files>", args[0]);
        return Ok(());
    }

    let target_dir = &args[1];
    let path = Path::new(target_dir);
    if !path.is_dir() {
        anyhow::bail!("Invalid Path {}", target_dir);
    }

    let entries: Vec<_> = fs::read_dir(path).context("Failed to read dir")?.collect();
    entries.par_iter().for_each(|entry| {
        if let Ok(entry) = entry {
            let file_path = entry.path();

            if file_path.is_file() {
                println!("Processing {:?}", file_path.file_name().unwrap());
                if let Err(e) = parse_blf(&file_path) {
                    eprintln!("Parse {:?} Err {}", file_path, e);
                }
            }
        }
    });

    Ok(())
}