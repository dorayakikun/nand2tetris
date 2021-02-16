#!/bin/sh

cargo build

./target/debug/vm_translater ../FunctionCalls/FibonacciElement/
sh ../../../tools/CPUEmulator.sh ../FunctionCalls/FibonacciElement/FibonacciElement.tst

./target/debug/vm_translater ../FunctionCalls/NestedCall/
sh ../../../tools/CPUEmulator.sh ../FunctionCalls/NestedCall/NestedCall.tst

./target/debug/vm_translater ../FunctionCalls/SimpleFunction
sh ../../../tools/CPUEmulator.sh ../FunctionCalls/SimpleFunction/SimpleFunction.tst

./target/debug/vm_translater ../FunctionCalls/StaticsTest
sh ../../../tools/CPUEmulator.sh ../FunctionCalls/StaticsTest/StaticsTest.tst

./target/debug/vm_translater ../ProgramFlow/BasicLoop
sh ../../../tools/CPUEmulator.sh ../ProgramFlow/BasicLoop/BasicLoop.tst

./target/debug/vm_translater ../ProgramFlow/FibonacciSeries
sh ../../../tools/CPUEmulator.sh ../ProgramFlow/FibonacciSeries/FibonacciSeries.tst
