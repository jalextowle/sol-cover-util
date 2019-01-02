# Solidity Code Coverage Analyzer

This analyzer will use the modified `ganache-core` codebase at: https://github.com/jalextowle/ganache-core. 

To use this analyzer, you will need to clone and build `ganache-cli` from source. The source code for this project can be
found at https://github.com/trufflesuite/ganache-cli. After building ganache-cli using `npm install`, remove the npm package ganache-core from the node_modules folder and replace it with the modified `ganache-core` codebase. 

# WIP

This project is a work in progress. It is not currently tested, and the way that the tool is designed only allows the tool to work on tests for contracts that do not make any external calls (I think). 
