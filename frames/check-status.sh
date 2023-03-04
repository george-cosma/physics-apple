#!/bin/bash

DONE=$(find ./ -name "*.field" | wc -l)
TOTAL=$(find ./ -name "*.png" | wc -l)
PERCENTAGE=$((10000 * DONE / TOTAL))

printf "%s%% | %s/%s" ${PERCENTAGE:0:-2}.${PERCENTAGE: -2} $DONE $TOTAL