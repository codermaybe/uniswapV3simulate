# 最初设计
>[DEXsimulate](./history/contract/DEXsimulate.sol) 比较久远的个人设想草稿，固定转换比率，无兑换限制。仅实现代币流转
# UniswapV3
以[uniswapV3-book-zh-cn](https://y1cunhui.github.io/uniswapV3-book-zh-cn/)为蓝本逐步实现的完整项目。将在项目进展的各个阶段更新对应代码
## 1.设定第一笔流动性交易
ETH：USDT
<s>结合某日夜间现价 1:2179.68   初步设定比率为1:2200。</s>不过随时可调整参数

按原文同比例实现 ETH：USDT = 1：5000

tools文件夹存储费率计算工具(rust小脚本)

## 2.