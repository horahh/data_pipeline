mkdir -p .data
for x in {100..900}; do cp data/foods.parquet ".data/foods$x.parquet"; done
