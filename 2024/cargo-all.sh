#!/bin/bash

for d in quest-[0-2][0-9]; do (echo ">>> $d"; cd $d; time cargo $*); done
