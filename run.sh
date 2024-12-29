# Run with either 
# - `./run.sh` to run all days
# - `./run.sh <day>` to run a specific day
if [ -z "$1" ]
then
    cargo build --release && ./target/release/AoC2024
else
    cargo build --release && ./target/release/AoC2024 $1
fi
