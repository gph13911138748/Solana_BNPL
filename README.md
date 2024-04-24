# Solana_BNPL

## process

### Buy Now Pay Later
You can purchase high-priced NFTs by paying only a small portion of the upfront cost. The remaining amount can be borrowed. You have the option to repay the borrowed amount at a later time or sell your NFT and retain the profits.

### Keep the Profit When You Sell
Sell your NFT whenever you desire and retain the profit from the sale if the price of your NFT has appreciated, otherwise you will need to cover the delta between sell price and remaining amount.

### Repay to Take Full Ownership
The amount you borrowed will accumulate interest at a variable rate on a daily basis depending on the lender's conditions. You have the flexibility to choose when to repay the borrowed amount or sell the NFT, don't forget to repay or sell the NFT to avoid losing ownership of your NFT and your down payment.

## architecture

### POOL
A POOL made by many investors for users to pay for Merchant's NFTs.

### Vault
Vaults made by different users and controlled by the program. A user can buy NFTs by using money in pool but he need to stake some money in his vault when purchasing NFTs. If NFTs price goes higher and user want to sell it, it need to pay some fee for investors, so program will sell NFTs and gain some fee, and send the remaining benefits back. But if the NFTs price goes down, the money stake in vault will go down, and if the money is not enough, the NFTs will be sold and the stake money will be put in the pool.

### User, Merchant, Investor

## Problems

### NFT
We need to build a NFT platform that supports this program. I think it is not a good solution. We just achieve a debt function. So it is not involving Merchant, just for users and investors.

### Token rewards
We need to build a pool with many money. So we need to use our tokens.