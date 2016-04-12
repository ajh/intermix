The output of these benches output ansi escape sequences to stderr which do horrible things to the terminal :)

Its recommended to run them while redirecting stderr to /dev/null like this:

    cargo bench 2> /dev/null
