# memers

Uniswap meme token monitoring bot built in Rust.

## In development
Please note that this project is still in development and is not ready for production use. This is another project I'm working on to get better at rust, so it may take a while to get it to a usable state.

## Todo
- [x] Setup all of the uniswap / lp locking / token contract abis and implement them
- [x] Create a transaction pool to store transactions and check for duplicates
- [x] Setup a channel to send transactions to the pool
- [ ] Attempt to find profitable sandwich attacks and send a message to the channel
- [ ] Attach a database to store contracts, statistics, etc
    - [ ] SeaORM? Diesel? MongoDB?
- [ ] Use the pair byte code to track LP tokens
    - [ ] We already have burned and lp locks. This could be an indicator of a rug pull if the lp tokens are being moved around?
- [ ] Setup a web server to display statistics

## Ideas / Maybe
- [ ] ?? Every block check if the transactions have failed or succeeded and send a message to the channel ??