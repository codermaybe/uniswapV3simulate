const { buildModule } = require("@nomicfoundation/hardhat-ignition/modules");

module.exports = buildModule("Transaction", (m) => {
  const DEX = m.contract(
    "DEXsimulate",
    [process.env.gldaddress, process.env.slvaddress],
    { redeploy: true }
  );
  return m;
});
