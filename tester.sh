#!/bin/bash

#set -euo pipefail
RUNC=$1;
TDIR="tests";

for test_dir in $(find $TDIR -type d -not -name tests); do
    echo "[Running Job: $test_dir]";

    # EXECUTION
    OG="$(echo "$test_dir/in" | $RUNC)";
    ECG=$?; # Exit Code Got + Expected
    ECE=$(cat "$test_dir/exit");
    OE=$(cat "$test_dir/out"); # Output Got + Expected

    # REPORT GENERATION
    if [ "$OG" = "$OE" ]
    then
        echo "  OUTPUT: SUCCESS";
    else
        echo "  OUTPUT: FAILURE";
        echo "      EXPECTED: $OE";
        echo "      GOT: $OG";
    fi;

    if [ $ECG -eq "$ECE" ]
    then
        echo "  EXIT CODE: SUCCESS";
    else
        echo "  EXIT CODE: FAILURE";
        echo "      EXPECTED: $ECE";
        echo "      GOT: $ECG";
    fi;
done;