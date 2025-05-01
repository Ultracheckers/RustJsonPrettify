# Json Prettify

A simple CLI tool to format JSON data.

# Format a JSON file and print to stdout
json_prettify --input input.json

# Read from stdin
cat input.json | json_prettify

# Write to an output file
json_prettify --input input.json --output pretty.json