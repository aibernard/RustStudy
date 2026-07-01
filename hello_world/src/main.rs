use std::io;

use crate::processor::calculate_remaining;

mod types;
mod processor;
mod enum_list;
mod operators;// 挂在在模块树上，供main.rs使用

fn main() {
   println!("=== 星际货运物流终端启动 ===");

   let rem = calculate_remaining(500.0, 125.6);
   println!("剩余载荷: {}", rem);

   let mut input_name = String::new();
   println!("请为新入港的货运飞船命名:");
   io::stdin().read_line(&mut input_name).expect("读取飞船名称失败");
   let cleaned_name = input_name.trim().to_string();

   let my_ship = types::Spaceship {
       name: cleaned_name,
       fuel: 40,
       shield_sectors: [100.0, 100.0, 100.0, 100.0],
       core_status: 'A',
   };

   let arrival_event = types::SpaceEvent::ShipArrival(my_ship);
   processor::process_event(arrival_event);

   let full_serial = "BATCH-4092-X";
   let batch = processor::extract_batch_code(full_serial);
   println!("提取的批次代码: {}", batch.unwrap_or("无效序列号"));
}
