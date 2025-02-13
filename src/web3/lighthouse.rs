use lighthouse::beacon_node::BeaconNode;
use lighthouse::eth_spec::MainnetEthSpec;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 初始化信标节点
    let beacon_node = BeaconNode::<MainnetEthSpec>::new(/* 参数 */).await?;

    // 获取链头信息
    let head_info = beacon_node.get_head_info().await?;
    println!("Chain head: {:?}", head_info);

    Ok(())
}
