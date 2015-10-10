shell mode - user typing goes to the shell. User can
command mode - commands go to intermix.
program mode - commands go to selected program. ctl-b to escape

This'll get more sophisticated once the keyboard interaction stuff is
working.

For now, use a vi inspired setup:

start in command mode.
display status on last row
hjkl keys select a pane
i enters program mode
ctrl-b escapes back to command mode
ctrl-b ctrl-b sends a literal ctrl-b to program
