# 整体架构

## 数据流
### 输入/逻辑产生消息/消息改变世界
input/其他逻辑系统 -> [发送] -> actions_logic 的 actions 给 action_logic 的 actions_listener -> [内部判断逻辑] -> [转换] -> 世界状态(world_light/farm/...) 的 actions -> [发送] -> 世界状态 -> [执行] -> 对应 action 的逻辑 -> [修改] -> 世界状态

## 目录结构
### 按功能插件划分
全局功能放在 src/
各全局功能可以有自己的子功能, 即:多个子功能合成一个大功能,允许无限嵌套
各功能插件的文件夹/模块内有: commponents(组件) plugin(插件) systems(系统) actions(动作) 及其他功能性模块组成

## 各模块可见性
### 你我不相见(只能见部分)
每个 模块(功能) 除了 plugin actions 可以 pub 公开(并且 plugin 需要严格遵守父子模块继承{父plugin add 子 plugin})外, 其他全部不得 pub 公开
如果遇到需要公开的 结构体/枚举... 可以使用 pub use 统一导出, 并且使用 as 明确是哪个模块导出的, 避免重名;
