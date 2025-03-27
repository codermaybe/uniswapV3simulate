const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

module.exports = buildModule("Transaction", (m) => {
  const GLD = m.contract("GLDToken", [10000], { redeploy: true });
  return m;
});
