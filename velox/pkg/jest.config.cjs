module.exports = {
  transform: {
    '^.+\.js$': 'babel-jest',
    '^.+\.wasm$': 'jest-transform-stub',
    '^.*veloxx\.js$': 'babel-jest', // Add this line to explicitly transform veloxx.js
  },
  testEnvironment: 'node',
  transformIgnorePatterns: [
    'node_modules/(?!(\.wasm$))'
  ],
};
