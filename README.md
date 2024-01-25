# TCP-Client-in-Rust
A TCP Client that calculates average for real-time BTC-USD value from Binance Websockets and sends to TCP Server

## Instructions to run the project
Build the Repo by running ```cargo build``` after cloning the repository and run 

```./target/debug/simple --mode=cache --times=10``` to get the average for BTC-USD in 10 seconds  
```./target/debug/simple --mode=read``` to get the history of aggregated data and the average value


```bash
# Example installation steps
git clone https://github.com/Av1nag/TCP-Client-in-Rust.git
cd TCP-Client-in-Rust
cargo build

