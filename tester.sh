#!/bin/bash

set -euo pipefail
TDIR="tests";
TEST_ONLY_SUCCESS=0;
#COMPARE_ON_ERROR=0;
ALL_TEST_PASS=1;

FLAG=1;
while [ $FLAG -eq 1 ]
do
    case "$1" in
        --test-only-success)
            TEST_ONLY_SUCCESS=1;
            shift;
            ;;
#        --compare-stdout-on-error)
#            COMPARE_ON_ERROR=1;
#            shift;
#            ;;
        *)
            FLAG=0;
            ;;
    esac
done

for test_dir in "$TDIR"/*; do
    echo "[Running Job: $test_dir]";

    # EXECUTION
    OG="$("$@" < "$test_dir/in")" && true; # Exit Code Got + Expected
    ECG="$?";
    ECE=$(cat "$test_dir/exit");
    OE=$(cat "$test_dir/out"); # Output Got + Expected

    # REPORT GENERATION
    if [ "$OG" = "$OE" ]&&[ "$ECG" -eq "$ECE" ]
    then
        echo "OK";
    else
        if [ $TEST_ONLY_SUCCESS -eq 1 ] && { { [ "$ECG" -eq 0 ] && [ "$ECE" -eq 0 ]; } || { [ ! "$ECG" -eq 0 ] && [ ! "$ECE" -eq 0 ]; }; } || [ "$ECG" -eq "$ECE" ]
        then
            echo "  EXIT CODE: SUCCESS";
        else
            echo "  EXIT CODE: FAILURE";
            echo "      EXPECTED: $ECE";
            echo "      GOT: $ECG";
            ALL_TEST_PASS=0;
        fi;

        if [ ! $TEST_ONLY_SUCCESS ] && [ "$OG" = "$OE" ] || [ ! "$ECE" -eq 0 ]
        then
            echo "  OUTPUT: SUCCESS";
        else
            echo "  OUTPUT: FAILURE";
            echo "      EXPECTED: $OE";
            echo "      GOT: $OG";
            ALL_TEST_PASS=0;
        fi;
    fi;
done;

if [ $ALL_TEST_PASS -eq 1 ]; then
    printf "\n--- ALL TESTS SUCCESSFULL ---\n";
fi;