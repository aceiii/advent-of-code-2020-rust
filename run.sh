#!/bin/bash


infile=$(basename -- "$1")
outfile="${infile%.*}"
rustc $infile -o ./bin/$outfile && ./bin/$outfile <&0

