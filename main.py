import pandas as pd
import pathlib

def process_parquet(filename):
    df = pd.read_parquet(filename);
    col_selection=["category","fats_g","sugars_g","filename","fats_g_summed","sugars_g_summed","concat_2"];
    df["filename"]=filename
    df["fats_g_summed"]=df["fats_g"].sum()
    df["sugars_g_summed"]=df["sugars_g"].sum()
    df["concat_2"]=df[["category","category"]].apply(str(filename).join,axis=1)
    return df[col_selection]

def get_files(path):
    files = [ f for f in pathlib.Path(path).rglob("*.parquet") if f.is_file()  ]
    return files

def main():
    path=".data"
    files = get_files(path)
    dfs = [ process_parquet(f) for f in files ]
    final = pd.concat(dfs)
    print(final)

main()
