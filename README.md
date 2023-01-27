# Solana Program (Smart Contract)

- This repo created for findit game. You can access it from [here](https://github.com/antinucleus/findit)
- If you want to deploy or test this smart-contract i explained below step by step

## Prerequisites

- You have to install **solana-cli** and **rust** on your machine

## Introduction

- This is the first version of the solana program (smart-contract) for findit game
- In this version of the program, it reads data from transaction instruction, logs and store on solana network


## How to use

- set Solana CLI configuration for localhost or if you want you can set devnet.

  `solana config set --url localhost`

- Build rust program

  `cargo build-bpf`

- Run a local test validator

  `solana-test-validator`

- Deploy rust program. This requires PATH. It will be found under project_location>target>deploy>findtit-solana-smart-contract.so

  `solana program deploy <PATH>`
  
- For example 

  `solana program deploy /Users/%USER%/Documents/projects/findit-solana-smart-contract/target/deploy/findit_solana_smart_contract.so`

- After deployment cli returns PROGRAM_ID. You can view program log using this PROGRAM_ID.

  `solana logs <PROGRAM_ID>`
