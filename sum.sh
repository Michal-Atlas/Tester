#!/bin/bash

set -euo pipefail

SUM=0

while read -r LINE
do
    SUM=$((SUM + LINE))
done

echo $SUM

