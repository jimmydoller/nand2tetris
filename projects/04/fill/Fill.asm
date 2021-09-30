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

(CHECKKEY)
	@SCREEN
	D=A

	@c
	M=D

	@KBD
	D=M

	@color
	D=M-D

	@CHECKKEY
	D ; JEQ

	@KBD
	D=M

	@SET_BLACK
	D ; JNE

	@SET_WHITE
	0 ; JMP

(SET_WHITE)
	@color
	M=0

	@FILL
	0 ; JMP

(SET_BLACK)
	@color
	M=-1  // Just rolls on through no jmp needed

(FILL)
	@color
	D=M

	@c
	A=M
	M=D
	D=A

	@KBD // Off the screen then
	D=A-D

	@CHECKKEY
	D-1 ; JEQ

	@c
	M=M+1

	@FILL
	0 ; JMP
