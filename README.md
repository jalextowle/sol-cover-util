# WIP
This analyzer is a work in progress and as such should not be used in production. This disclaimer will be removed when the tool has been more thoroughly tested and works correctly. 

# Solidity Code Coverage Analyzer

This analyzer will use the modified `ganache-core` codebase at: https://github.com/jalextowle/ganache-core. 

To use this analyzer, you will need to clone and build `ganache-cli` from source. The source code for this project can be
found at https://github.com/trufflesuite/ganache-cli. After building ganache-cli using `npm install`, remove the npm package ganache-core from the node_modules folder and replace it with the modified `ganache-core` codebase. 
