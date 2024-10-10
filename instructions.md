# Instructions

##  The CAT challenge


The `ls` command takes all the files and folders in your working directory and displays them on the screen.
`ls` stands for list stuff.

The `cat` command displays the contents of a file in your terminal.

#### Demo:

```
/home/ace > ls
file_0.txt      | file
file_1.txt      | file
folder          | directory
/home/ace > cat file_0.txt
This file is top secret! Do not read if unauthorised!
I ate the biscuit...
/home/ace >
```



1. type `ls`
2. run `cat [file]` for each file displayed.
3. copy the output of `cat flag.txt` by triple clicking it and pressing `Ctrl + Shift + c`
4. type `exit`
5. press `Ctrl + Shift + v` to paste the flag

## The MAZE challenge

The `cd` command moves your working directory.
`cd ../` (there's a white space between `cd` and `../`) will move your working directory up.
`cd [dir]` (where `[file]` is replaced with the name of a folder in the working directory) will move the working directory into that folder.


1. type `ls` and you will see 3 directories: `desktop`, `documents` and `downloads`
2. pick a directory and type `cd [folder]`
3. then type `ls`
4. run `cat [file]` for each file displayed
5. if the flag isn't there type `cd ../` and go back to step 2 and pick a different directory
6. once you have found the flag, copy it by triple clicking it and pressing `Ctrl + Shift + c`
7. type `exit`
8. press `Ctrl + Shift + v` to paste

## The TREE challenge

1. type `tree`
2. locate the `flag.txt` file
3. run `cd [folder]` replacing `[folder]` with the folder holding the flag
4. run `cat flag.txt`
5. copy the output of `cat flag.txt` by triple clicking it and pressing `Ctrl + Shift + c`
6. type `exit`
7. press `Ctrl + Shift + v` to paste


## Ending

Congratulations! You finished the game! You can go back and try to find the 3 secrets for a biscuit (if there are any left).



