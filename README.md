# jwc

A wc clone.

![alt text](https://github.com/fissilejonathan/jwc/blob/main/jwc.png "jwc")

Usage: jwc [OPTIONS] [FILES]...

Arguments:
  [FILES]...  Files that will be processed; Can be one or more

Options: <br>
  `-b, --bytes`            Print the byte counts <br>
  `-c, --chars`            Print the character counts<br>
  `-l, --lines`            Print the newline counts<br>
  `-L, --max-line-length`  Print the length of the longest line<br>
  `-w, --words`            Print the word counts<br>
  `--read-from <F>`    Read input from the files specified by NUL-terminated names in file F; If F is - then read names from standard input else input should be comma separated<br>
  `-h, --help`             Print help<br>
  `-V, --version`          Print version<br>
