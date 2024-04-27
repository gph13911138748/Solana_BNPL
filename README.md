# Solana_BNPL

##   preface
The first chapter briefly describes our design of BNPL, but due to limited time, we can only complete a simple demo. The main function is investors' pledge of the liquidity pool and borrowers borrowing from the pool.

##   1.process

### Buy Now Pay Later
You can purchase high-priced NFTs by paying only a small portion of the upfront cost. The remaining amount can be borrowed. You have the option to repay the borrowed amount at a later time or sell your NFT and retain the profits.

### Keep the Profit When You Sell
Sell your NFT whenever you desire and retain the profit from the sale if the price of your NFT has appreciated, otherwise you will need to cover the delta between sell price and remaining amount.

### Repay to Take Full Ownership
The amount you borrowed will accumulate interest at a variable rate on a daily basis depending on the lender's conditions. You have the flexibility to choose when to repay the borrowed amount or sell the NFT, don't forget to repay or sell the NFT to avoid losing ownership of your NFT and your down payment.

##   2.architecture

### POOL
A POOL made by many investors for users to pay for Merchant's NFTs.

### Vault
Vaults made by different users and controlled by the program. A user can buy NFTs by using money in pool but he need to stake some money in his vault when purchasing NFTs. If NFTs price goes higher and user want to sell it, it need to pay some fee for investors, so program will sell NFTs and gain some fee, and send the remaining benefits back. But if the NFTs price goes down, the money stake in vault will go down, and if the money is not enough, the NFTs will be sold and the stake money will be put in the pool.

### User, Merchant, Investor

##   3.Problems

### NFT
We need to build a NFT platform that supports this program. I think it is not a good solution. We just achieve a debt function. So it is not involving Merchant, just for users and investors.

### Token rewards
We need to build a pool with many money. So we need to use our tokens.

##   4.design

### logic
initlize_mint -> airdrop -> initialize_vault(pool) -> initialize_auth(investor's record) -> stake -> withdraw

## experiment
program_id : CCiZVVLRhS48AHYcym6LxFhNGV4WrH1gp7dFChZwbG1B
mint : 8LEAvBfaSLAVPjz42oq5uLjXq9Q11Uh3igRvRyqm8EQS
pool's token_account: DyfDontwskdrdojzrS5wXW8MSe75gG7HrvQT3BxtZSLU
vault: 91rwLSZvNvckREDJniD1v1upV2dLpV9t8MgwJadNSoJv

test investor:
wallet: 2nm2FGvfcyYPPrP9EGYpdZbnemJAxFHR5b7BbdFLEqiN
investor's token_account: spl-token create-account 8LEAvBfaSLAVPjz42oq5uLjXq9Q11Uh3igRvRyqm8EQS
                       = ANvNSca2qYL883yXBv6iN8idQHDWjSmzSYYAQ84qUa3Z
authority: GjqAdgTXiH8KRvfaoh43vU9GarJLzJVmLnA4P5NMouZ1
record: 9fm5Xobj3ofC1CBr1hayJW7j5hEfZs7RXopZK29hZk11

