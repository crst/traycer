#!/bin/bash

for filename in ./*.json; do
    ../target/$1/traycer --scene-file $filename --out-file ${filename%.json}.png
done
