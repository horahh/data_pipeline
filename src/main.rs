use walkdir::{DirEntry, WalkDir};
use polars::prelude::*;

fn is_not_hidden(entry: &DirEntry) -> bool {
    entry
         .file_name()
         .to_str()
         .map(|s| entry.depth() == 0 || (!s.starts_with(".") && s.ends_with(".parquet") ))
         .unwrap_or(false)
}

fn is_not_dir(entry: &DirEntry) -> bool {
    entry
        .file_type()
        .is_dir()
}

fn process_parquet(filename: String ) -> PolarsResult<()> {
    let df = LazyFrame::scan_parquet(filename, ScanArgsParquet::default())?
        .select([
            // select all columns
            all(),
            // and do some aggregations
            cols(["fats_g", "sugars_g"]).sum().suffix("_summed"),
        ])
        .collect()?;

    dbg!(df);
    Ok(())
}

fn main() {
    WalkDir::new("data")
        .into_iter()
        .filter_entry(|e| is_not_hidden(&e))
        .filter_map(|v| v.ok())
        .filter(|e| ! is_not_dir(e))
        //.for_each(|x| println!("{}", x.path().display()));
        .for_each(|x| process_parquet(x.path().display().to_string()).unwrap());
}
