# knapsack

## Problem
You have a list of transaction id's with associated weights, fees, and parent transaction id's (if any).
You are a miner, and want to select the transactions to be included in block to maximize the fee while keeping the combined weight under 4,000,000.
If a child transaction is included, the parent must be included as well and must preceed the child in the final transaction list.

The PCKP is an NP

## Solution
The problem of selecting transactions to maximize a fee, while keeping the combined weight under a predefined value can be solved with the O/I knapsack algorithm. 
This problem can be solved recursively (*O(2^n)*, or dynamically (*O(nW)*, n=#txs, W=#weights).
The dynamic programming approach is done here, and is formalized as:  

![0/1 Knapsack](https://github.com/rrybarczyk/knapsack/blob/master/docs/fig-01k-eqs.png "0/1 Knapsack Equations")
![0/1 Knapsack](https://github.com/rrybarczyk/knapsack/blob/master/docs/fig-01k-vars.png "0/1 Knapsack Variables")


This does not address the full problem, however, because it does not enforce the parent-child constraint.
This constraint turns the 0/1 knapsack problem into a *precedence constraint knapsack problem* (PCKP), also called the *partially ordered knapsack* (POK), which states that if item *i* precedes item *j* then item *j* can only be selected if item *i* is selected. 

This problem can be represented with a directed, acyclic graph (DAG) *G=(V,E)*.

![PCKP](https://github.com/rrybarczyk/knapsack/blob/master/docs/fig-pckp-eqs.png "PCKP Equations")
![PCKP](https://github.com/rrybarczyk/knapsack/blob/master/docs/fig-pckp-vars.png "PCKP Variables")

There have been several solutions posed, many choose to reduce the problem into a tree structure and then use various algorithms including the left-right approach, a branch and bound, and a depth-first-search appoach. None of these properly acocunt for the complex parent-child relationships in the problem however.

### NP-hard
The PCKP is NP-hard because it reduces to the KP which is NP-hard.

When problems are NP-hard, we will not be able to solve it efficiently (in polynomial time). Instead, we try and find alternative solutions that work efficiently for related problems, but may result in suboptimal or invalid solutions for our exact problem. To make this work for our exact parameters, we start to relax constraints.

If I were to continue with this problem I would investigate the possibility of applying a Lagrangian relaxation to reduce this problem down to a more managable one. This may not result in a perfect solution still, but it ideally run in much more efficient time than a brute force method. Or there is probably a simple algorithm that I don't know about but everyone else does that magically solves this problem in constant time and I have needlessly made this problem much more complicated that it really is.
