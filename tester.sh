#!/bin/bash

while [ ! $# -eq 0 ]
do
    case "$1" in
        --test-only-success)
            TEST_ONLY_SUCCESS=1;
            ;;
        --compare-stdout-on-error)
            COMPARE_ON_ERROR=1;
            ;;
    esac
    shift;
done

#set -euo pipefail
RUNC=$1;
TDIR="tests";

for test_dir in $(find $TDIR -type d -not -name tests); do
    echo "[Running Job: $test_dir]";

    # EXECUTION
    OG="$($RUNC < "$test_dir/in")";
    ECG=$?; # Exit Code Got + Expected
    ECE=$(cat "$test_dir/exit");
    OE=$(cat "$test_dir/out"); # Output Got + Expected

    # REPORT GENERATION
    if [[ "$OG" = "$OE" ]&&[ "$ECG" -eq "$ECE" ]] || [ TEST_ONLY_SUCCESS && $ECG==0 ]
    then
        echo "OK";
    else
        if [ "$ECG" -eq "$ECE" ]
        then
            echo "  EXIT CODE: SUCCESS";
        else
            echo "  EXIT CODE: FAILURE";
            echo "      EXPECTED: $ECE";
            echo "      GOT: $ECG";
        fi;
        if [ ! COMPARE_ON_ERROR && $ECG == 0 ] then
            if [ "$OG" = "$OE" ]
            then
                echo "  OUTPUT: SUCCESS";
            else
                echo "  OUTPUT: FAILURE";
                echo "      EXPECTED: $OE";
                echo "      GOT: $OG";
            fi;
        fi;       
    fi;
done;