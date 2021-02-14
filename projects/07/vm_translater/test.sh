#!/bin/sh
cargo build

echo 'Starts testing BasicTest'
./target/debug/vm_translater -- ../MemoryAccess/BasicTest
sh ../../../tools/CPUEmulator.sh ../MemoryAccess/BasicTest/BasicTest.tst

echo 'Starts testing PointerTest'
./target/debug/vm_translater -- ../MemoryAccess/PointerTest
sh ../../../tools/CPUEmulator.sh ../MemoryAccess/PointerTest/PointerTest.tst

echo 'Starts testing StaticTest'
./target/debug/vm_translater -- ../MemoryAccess/StaticTest
sh ../../../tools/CPUEmulator.sh ../MemoryAccess/StaticTest/StaticTest.tst

echo 'Starts testing SimpleAdd'
./target/debug/vm_translater -- ../StackArithmetic/SimpleAdd
sh ../../../tools/CPUEmulator.sh ../StackArithmetic/SimpleAdd/SimpleAdd.tst

echo 'Starts testing StackTest'
./target/debug/vm_translater -- ../StackArithmetic/StackTest
sh ../../../tools/CPUEmulator.sh ../StackArithmetic/StackTest/StackTest.tst