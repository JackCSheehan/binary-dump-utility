# binary-dump-utility
A command line utility that can read a binary file and dump its contents as hex values or as ASCII values

## Usage
**To dump a file to the console**
```
$ ./binary-dump-utility.exe -c -{h|b|d|o|a} <file>
```
For dumping to the console, it is only necessary to provide a file to read data from.

**To dump a file to to a file**
```
$ ./binary-dump-utility.exe -f -{h|b|d|o|a} <read file> <write file>
```
For dumping to a file, it is necessary to supply a file to read from and the name of a file to write to. If the write file does not exist, it will be created.

## Flag Reference
|Flag       |Description    |
|-----------|---------------|
|h          |Dump to hex    |   
|b          |Dump to binary |
|d          |Dump to decimal|
|o          |Dump to octal  |
|a          |Dump to ASCII  |
