module.exports = {
  networks: {
    development: {
      host: "127.0.0.1",     // Ganache default host
      port: 7545,            // Ganache default port
      network_id: "*",       // Match any network id
    },
  },

  // Set default mocha options here, use special reporters, etc.
  mocha: {
    timeout: 100000,         // Ensure tests don't time out quickly
  },

  // Configure your compilers
  compilers: {
    solc: {
      version: "0.8.0",       // Match Solidity version in your contract
      settings: {             // Optional compiler settings
        optimizer: {
          enabled: true,      // Enable optimization for gas efficiency
          runs: 200,          // Optimize for how many times you intend to run the code
        },
        evmVersion: "istanbul", // Optional: specify the EVM version (use "istanbul" or "london")
      },
    },
  },

  // Truffle DB configuration (disabled by default)
  db: {
    enabled: false,
  },
};
