#!/bin/bash

for ((i = 1; i <= $1; i++))
  do
    touch "ques$i.rs" ./
done