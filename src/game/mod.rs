/// (中间层)动作处理逻辑(所有会改变游戏世界的逻辑都要经过这个中间层)
/// 输入系统发送 action_logic 的 actions 消息个 action_logic 的 systems
/// action_logic 的 systems 收到 actions 后转换为对应世界状态插件的 actions
/// 世界状态插件的 systems 收到 actions 后改变对应的世界状态
/// input/其他逻辑系统 (发送)-> action_logic 的 actions ->(拿到) actions转换systems
/// (转换)-> 世界状态的actions (发送)-> 世界状态systems (改变)-> 世界状态
mod action_logic;
/// 玩家/角色
mod player;
/// 插件
pub mod plugin;
/// 世界
mod world;
