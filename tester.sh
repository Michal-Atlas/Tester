#!/bin/env bash
set -uo pipefail

testdir=$1;
if [[ $testdir = "" ]]; then echo "Empty Testdir"; exit 1; fi;
shift;
runcmd=$1;
if [[ $runcmd = "" ]]; then echo "Empty Runcommand"; exit 1; fi;
shift;

successfull="";
failed="";

for f in ./"$testdir"/*; do
    printf "%s: " "$(echo "$f" | sed "s/\.\/tests\///")";
    out=$($runcmd < "$f"/in) && true;
    excode=$?;
    if [[ "$excode" -eq 0 ]]; then
        if [[ "$out" = "$(cat "$f"/out)" ]]; then
            printf "OK\n";
            successfull="$successfull $(echo "$f" | sed "s/\.\/tests\///")";
        else
            printf "\n\n___ DIFF ___\n%s\n___ END ___\n\n" "$(diff <(echo "$out") <(cat "$f"/out))";
            failed="$failed $(echo "$f" | sed "s/\.\/tests\///")";
        fi;
        
    else
        printf "[ Exit Code: %s ]\n" "$excode";
        failed="$failed $(echo "$f" | sed "s/\.\/tests\///")";
    fi;
done;

printf "\n\nSuccessfull: %s\nFailed: %s\n" "$successfull" "$failed";
