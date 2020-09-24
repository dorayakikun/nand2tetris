// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// Put your code here.

    @24575
    D = A      // D = 24575

    @0
    M = D      // M[0] = 24575

    @SCREEN
    D = A      // D = @SCREEN Address
    @1
    M = D      // M[1] = @SCREEN Address
(LOOP)
    @KBD
    D = M
    @FILL
    D;JGT      // if out > 0 jump

    @CLEAR
    0;JMP      // jump
(FILL)
    @0
    D = M      // D = M[0]
    @1
    D = D - M  // D = M[0] - M[1]
    @LOOP
    D;JLT      // if out < 0 jump

    @1
    D = M      // D = M[1]
    A = M      // A = M[1]
    M = -1

    @1
    M = D + 1  // M = D + 1 

    @LOOP
    0;JMP      // jump
(CLEAR)
    @SCREEN
    D = A      // D = @SCREEN Address
    @1
    D = D - M  // D = @SCREEN Address - M[1]
    @LOOP
    D;JGT      // if out > 0 jump

    @1
    D = M      // D = M[1]
    A = M      // A = M[1]
    M = 0      // M[1] = 0

    @1
    M = D - 1  // M[1] = M[1] - 1

    @LOOP
    0;JMP