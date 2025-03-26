//此处实现按照https://y1cunhui.github.io/uniswapV3-book-zh-cn/docs/milestone_1/calculating-liquidity/实现
//assume value ratio of ETH：BTC = 2200:1
//assume The lower bound of the price range is 2100 (Pl ->price lower)区间下界
//assume The upper bound of the price range is 2300 (Pu ->price upper)区间上界
//assume The price ratio is 2200 (Pc ->price current)现货区间
//The proce range is the value ratio of token1 to token0 , wen ETH:BTC change to 2200:2  , price ratio change to 1100:1
//basic format is sqrt(p) = sqrt(y/x)
//-------------------------
// 价格对应的tick  p(i).sqrt 等同于 （1.0001） ^(i/2)  ==》 i = 2 * log(1.0001) / log(p(i).sqrt) 实际计算为 i =  log sqrt(p(i))/ log sqrt(1.0001)  i为区间标号,其取值范围为[-128,128]
// L 为流动性数量 L = sqrt(x*y)
// sqrt(P) 中p为两货币的价格比，，p = y/x  。x -- token0, y -- token1
// L与 P的关系 : L =  Δy / Δ(sqrt(P))
fn round_to_two_decimals(input: f64, precision: usize) -> f64 {
    let formatted = format!("{:.1$}", input, precision);
    println!("formatted string {formatted}");
    formatted.parse::<f64>().unwrap()
}

pub fn calculate_tick() {
    //区间下界未开根
    let lower_bound: f64 = 4545.0;
    //现货价值未开根
    let value_ratio: f64 = 5000.0;
    //区间上界未开根
    let upper_bound: f64 = 5500.0;
    //对应开根取2位精度
    let pc_sqrt: f64 = round_to_two_decimals(value_ratio.sqrt(), 2);
    let pl_sqrt: f64 = round_to_two_decimals(lower_bound.sqrt(), 2);
    let pu_sqrt: f64 = round_to_two_decimals(upper_bound.sqrt(), 2);
    println!("pc_sqrt: {}", pc_sqrt);
    println!("pl_sqrt: {}", pl_sqrt);
    println!("pu_sqrt: {}", pu_sqrt);
    println!("-------------------------------");
    //基点值，1基点为万分之一
    let basepoint: f64 = 1.0001;
    //基点开跟 => 方便计算
    let basepoint_sqrt: f64 = basepoint.sqrt();
    //计算现货价格对应的tick    
    let ic: f64 = pc_sqrt.log(basepoint_sqrt);
    //计算区间下界对应的tick
    let il: f64 = pl_sqrt.log(basepoint_sqrt);
    //计算区间上界对应的tick
    let iu: f64 = pu_sqrt.log(basepoint_sqrt);
    println!("ic: {}", ic);
    println!("il: {}", il);
    println!("iu: {}", iu);
    println!("-------------------------------");
    println!("转换为q64.96格式");
    let pc_q64_96: f64 = pc_sqrt * 2_f64.powf(96.0);
    let pl_q64_96: f64 = pl_sqrt * 2_f64.powf(96.0);
    let pu_q64_96: f64 = pu_sqrt * 2_f64.powf(96.0);
    println!("pc_q64_96: {:.0}", pc_q64_96);
    println!("pl_q64_96: {:.0}", pl_q64_96);
    println!("pu_q64_96: {:.0}", pu_q64_96);
    println!("-------------------------------");
    //存入1ETH,5000BTC
    let delta_x: f64 = 1.0;
    let delta_y: f64 = 5000.0;
    //注意，此处对应的变量在文中表述有误 ！！！ 
    //[模糊部分具体定位]https://y1cunhui.github.io/uniswapV3-book-zh-cn/docs/milestone_1/calculating-liquidity/#%e6%b5%81%e5%8a%a8%e6%80%a7%e6%95%b0%e9%87%8f%e8%ae%a1%e7%ae%97
    //文中 pc ->pc , pb->pu ,pa -> pl
    //delta_x = L * (1.0/sqrt(Pc) - 1.0/sqrt(Pu))   => L = delta_x * (sqrt(pu)*sqrt(pc))/(sqrt(pu)-sqrt(pc))
    //由于上文对应变量采用了截取精度的方法，此处计算偏差较大。变为采取更加精确的计算方式
    
    //let L_from_delta_x = delta_x * (pu_sqrt*pc_sqrt)/(pu_sqrt-pc_sqrt);
    let  L_from_delta_x = delta_x * (upper_bound.sqrt()*value_ratio.sqrt())/(upper_bound.sqrt()-value_ratio.sqrt());
    println!("L_from_delta_x: {}", L_from_delta_x);
    //delta_y = L * (sqrt(PC) - sqrt(Pl)) => L = delta_y / (sqrt(pc)-sqrt(pl))
    //let L_from_delta_y = delta_y / (pc_sqrt-pl_sqrt);
    let L_from_delta_y = delta_y / (value_ratio.sqrt()-lower_bound.sqrt());
    println!("L_from_delta_y: {}", L_from_delta_y);
    
    let L_minimum = if L_from_delta_x > L_from_delta_y {
        L_from_delta_x
    } else {
        L_from_delta_y
    };
    println!("L_minimum: {}", L_minimum);
    //covert to q64.96 format
    let L_minimum_q64_96 = L_minimum * 2_f64.powf(96.0);
    //与原文差距较大，此处精确度存在问题 ？？？？没有验证python计算精度或舍入部分
    println!("L_minimum_q64_96: {:.0}", L_minimum_q64_96);
    println!("-------------------------------");
    println!("重新计算token数量");
    let token0_amount = L_minimum_q64_96 * (upper_bound.sqrt()-value_ratio.sqrt());
    let token1_amount = L_minimum_q64_96 * (value_ratio.sqrt()-lower_bound.sqrt());
    
    println!("token0_amount: {}", token0_amount);
    println!("token1_amount: {}", token1_amount);
    //精度问题搞了个寂寞，后续继续更新
}