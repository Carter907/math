# Math
rust utility binary for basic math operations that I use for productivity purposes. I made this as a simple way to check my work from the command line while in class.

# Installation
clone the repo and enter the following in the command line.
```
$ cargo install -f --path .
```
>this will install the binary in your .cargo/bin directory

this requires cargo to be installed. make sure your cli has access to the cargo 
executable via your environment variables.

# Features

The first feature I added was the ability to convert between units as I find myself hitting a wall trying to make a google search to get
the correct conversion.
e.g.
```
$ math convert -a 50.23 feet-to-meters

$ 50.23 ft -> 15.31 m
```
To quickly list all the conversion factors you can do the following:
```
$ math convert -l
```


# Geometry

Another issue I always run into is remembering the key geometric formulas.
I would be nice if I could just quickly via every formula from the command line, wouldn't it?

```
$ math geomtry -l
```