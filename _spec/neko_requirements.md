# requirements for neko program

This markdown file descirbes requirements for "neko" program in this repository.

## multi display support

Current implementation "neko" (commit `3c23972b8bec2e650009015227fdf4c7f6610779`)
seems not to support multi display environment.

Lanuch "neko" program, and let "neko" (Sprite) chases mouse cursor, this is working.
Now suppose I have two displays, DISPLAY-No1 and DISPLAY-No2, and "neko" is launched at DISPLAY-No1.
Then move mouse cursor to DISPLAY-No2.
"neko" tries to move from DISPLAY-No1 to DISPLAY-No2, but it is blocked on the border of the DISPLAY-No1
while "running" animation keeps going.

So here is a requirement.

- [x] req-md1: "neko" can move across multi-dispay seamlessly, as much as possible. 

## Windows, Linux and MacOS support

- [] req-os1: "neko" program is supposed to support Windows 11, Linux and MacOS.

To satisfy `req-os1`, proceed `step-1/2` described below:

step-1: verify the current source code in this repository and check if any code lines which only suppose Windows or not.

step-2: Replace "Windows only" code lines with Windows, Linux and MacOS capable ones.
