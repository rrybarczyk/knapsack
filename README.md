# knapsack

## Problem
You have a list of transaction id's with associated weights, fees, and parent transaction id's (if any).
You are a miner, and want to select the transactions to be included in block to maximize the fee while keeping the combined weight under 4,000,000.
If a child transaction is included, the parent must be included as well and must preceed the child in the final transaction list.

## Solution
The problem of selecting transactions to maximize a fee, while keeping the combined weight under a predefined value can be solved with the O/I knapsack algorithm.
The requirement of including the parents of all included children and listing those parents first puts a different spin on what otherwise would be a O/I knapsack problem.
