use walkdir::{DirEntry, WalkDir};
use polars::prelude::*;
use std::collections::HashMap;
//use rayon::Scope;

use pariter::IteratorExt;

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

fn process_parquet(filename: String ) -> PolarsResult<DataFrame> {
    let col_selection=["category","fats_g","sugars_g","filename"];
    let df = LazyFrame::scan_parquet(filename.clone(), ScanArgsParquet::default())?
        //.with_columns(filename)
        .with_column(lit(filename.clone()).alias("filename"))
        .select([
            // select all columns
            //all(),
            cols(col_selection),
            // and do some aggregations
            cols(["fats_g", "sugars_g"]).sum().suffix("_summed"),
            //(cols(["category"]).replace(r"*",&filename_c).alias("category_new")),
            concat_str([col("category"),col("category")],&filename).alias("concat2"),
           // fold_exprs(filename_c,|acc,x| Ok(Some(format!("{}",filename_c))),[col("category")]).alias("example"),
        ])
        .collect()?;
    //dbg!(df.clone());
    Ok(df)
}

fn main() {
    let dir = ".data";
    let df_vec = WalkDir::new(dir)
        .into_iter()
        .filter_entry(|e| is_not_hidden(&e))
        .filter_map(|v| v.ok())
        .filter(|e| ! is_not_dir(e))
        //.for_each(|x| println!("{}", x.path().display()));
        //.for_each(|x| process_parquet(x.path().display().to_string()).unwrap() );
        .parallel_map(|x| process_parquet(x.path().display().to_string()).unwrap()).collect::<Vec<_>>();

    for df in df_vec {
        dbg!(&df);
    }
    let args: UnionArgs = UnionArgs{ parallel:true, rechunk:true, to_supertypes:true};
    //let _final =concat(&df_vec, args);

}
