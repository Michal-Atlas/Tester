#!/bin/bash

set -uo pipefail

testdir=$1;
if [[ $testdir = "" ]]; then echo "Empty Testdir"; exit 1; fi;
shift;
runcmd=$1;
if [[ $runcmd = "" ]]; then echo "Empty Runcommand"; exit 1; fi;
shift;

for f in ./"$testdir"/*; do
    printf "%s: " "$(echo "$f" | sed "s/\.\/tests\///")";
    out=$($runcmd < "$f"/in) && true;
    excode=$?;
    if [[ "$excode" -eq 0 ]]; then
        if [[ "$out" = "$(cat "$f"/out)" ]]; then
            printf "OK\n";
        else
            printf "\n\n___ DIFF ___\n%s\n___ END ___\n\n" "$(diff <(echo "$out") <(cat "$f"/out))";
        fi;
    else
        printf "[ Exit Code: %s ]\n" "$excode";
    fi;
done;
