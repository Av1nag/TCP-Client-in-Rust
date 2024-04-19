# TCP-Client-in-Rust
A TCP Client that calculates average for real-time BTC-USD value from Binance Websockets and sends to TCP Server

Thanks to https://github.com/binance/binance-spot-connector-rust Repository for Public API. 

## Instructions to run the project
Build the Repo by running ```cargo build``` after cloning the repository and run 

Use ```./target/debug/simple --mode=cache --times=no. of seconds``` in command line interface to get the average for BTC-USD in 10 seconds  
Use ```./target/debug/simple --mode=read``` in command line interface to get the history of aggregated data and the average value

## How It works 

When we use caching command (```./target/debug/simple --mode=cache --times = no. of seconds```), the data from the Binance websockets will be connected to the client and reads the data upto to no. of seconds we mentioned as a value for a times flag (```--times = no. of seconds```).

Then the total collected data will be aggregated a prints the average in the user interface. In the background, a text file will be created and the values which are used to aggregate will be stored in the below mentioned format.

```bash
Date: 2024-01-24, Time:16:25:48
  
[Date: 2024-01-24, Time:16:25:50] BTC Price: 39992.89000000
[Date: 2024-01-24, Time:16:25:51] BTC Price: 39992.89000000
[Date: 2024-01-24, Time:16:25:51] BTC Price: 39992.90000000
[Date: 2024-01-24, Time:16:25:52] BTC Price: 39992.89000000
[Date: 2024-01-24, Time:16:25:52] BTC Price: 39992.89000000
[Date: 2024-01-24, Time:16:25:52] BTC Price: 39992.90000000
[Date: 2024-01-24, Time:16:25:52] BTC Price: 39992.90000000
```

The saved data can be accessed in the command line interface using the read mode command(```./target/debug/simple --mode = read```) the saved data will be parsed and prints to command line interface.


## Example installation steps

```bash
git clone https://github.com/Av1nag/TCP-Client-in-Rust.git
cd TCP-Client-in-Rust
cargo build
```


## Example for using the tools 
```bash
./target/debug/simple --mode=cache --times=10  
```

```bash
./target/debug/simple --mode=read
```


## Contact Information

For queries, and personal suggestions write a mail to udayagiriavinag@gmail.com

----
PS: I'm no so proficient in Rust programming, written a application to test my capability. An I appreciate if anyone contribute to the project and always welcome everyone to drop your suggestions.
