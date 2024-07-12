#!/bin/bash
assert() {
   expected="$1"
   input="$2"

   mkdir -p tmp
   ./target/debug/rust-9cc "$input" >tmp/tmp.s
   cc -o tmp/tmp tmp/tmp.s
   ./tmp/tmp
   actual="$?"

   if [ "$actual" = "$expected" ]; then
      echo "$input => $actual"
   else
      echo "$input => $expected expected, but got $actual"
      exit 1
   fi
}

cargo build

assert 0 0
assert 42 42
assert 21 "5+20-4"
assert 41 " 12 + 34  - 5 "

echo OK
