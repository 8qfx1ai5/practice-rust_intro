# Counting Words

This is just a small coding challenge.
The goal is to load the content of a text file which is specified during the
runtime call, count all words, and give back a list of all words with the
found number.

## Criteria

The path to the text file relative to the binary should be passed to the binary during the call with a cli param:

    -f "path/to/file.txt"

Words can be seperated by one or more of the folloing characters, which have to be ignored:

- " "
- "-"
- "."
- ";"
- ","
- "/"
- "_"
- ":"
- new line

The case of all characters is not important and should be ignored, so hat "Home" and "home" are counted together as "2".

The output format is:

    word1 counter1
    word2 counter2
    word3 counter3
    ...

{counter1} ≥ {counter2} ≥ ... ≥ {counterN}

The output has to be on stdout.
