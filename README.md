# memers

Uniswap meme token monitoring bot built in Rust.

## In development
Please note that this project is still in development and is not ready for production use. This is another project I'm working on to get better at rust, so it may take a while to get it to a usable state.

## Todo
- [ ] Setup all of the uniswap / lp locking / token contract abis and implement them
- [ ] Create a transaction pool to store transactions and check for duplicates
- [ ] Setup a channel to send transactions to the pool
- [ ] Every block check if the transactions have failed or succeeded and send a message to the channel