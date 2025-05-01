# Json Prettify

A simple CLI tool to format JSON data. There are three main ways to use this program. 
To build simply run cargo build --release and the executable will be found under target/release/json_prettify.
I made this to get the hang of rust for another project I was working on.

# Format a JSON file and print to stdout
json_prettify --input input.json

# Read from stdin
cat input.json | json_prettify

# Write to an output file
json_prettify --input input.json --output pretty.json

[![Screenshot-2025-04-30-232138.png](https://i.postimg.cc/tRwK5yYc/Screenshot-2025-04-30-232138.png)](https://postimg.cc/z3w2zsww)