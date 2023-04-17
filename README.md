# 说明
利用rust实现一个获取股票数据，展示出详细股票信息的ui界面

# 依赖库
- tui
- crossterm
- reqwest
- tokio
- serde
- serde-json

# 使用组件
- block
- list
- chart
- user_input

# 自定义kline
- 将数据结构转换为Vec<Kblock>，即一组坐标(每个坐标命名为Kblock，包含上影线、下影线、实体，是否为阳线【实体不是阳线就是阴线】)，通过这一组坐标，绘制出包含了影线和实体的单位体。