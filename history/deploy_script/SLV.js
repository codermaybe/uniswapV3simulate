const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

module.exports = buildModule("Transaction", (m) => {
  const SLV = m.contract("SlvToken", [20000], { redeploy: true });
  return m;
});
