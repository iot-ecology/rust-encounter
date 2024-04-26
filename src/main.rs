use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use js_sandbox::{AnyError, Script};
use rumqtt::{MqttClient, MqttOptions, QoS, SecurityOptions};

mod js_test;

fn main() {
    run_mqtt(1);


    // 主线程等待
    loop {}
}

fn run_mqtt(i: i32) {
    let security = SecurityOptions::UsernamePassword("admin".to_string(), "admin123".to_string());

    let mqtt_client_id = format!("{}{}", "rust", i);
    let mqtt_options = MqttOptions::new(mqtt_client_id, "localhost", 1883);
    mqtt_options.clone().set_security_opts(security);


    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();

    mqtt_client.subscribe(format!("{}{}", "topic/", i), QoS::AtMostOnce).unwrap();

    let src = r#"
function main(data) {
    var datac = JSON.parse(data).Timestamp;
    return datac;
}
    "#;

    let src2 = r#"
function main(data) {
    var datac = JSON.parse(data).Data;
    var parse = JSON.parse(datac);
    var keyValuePairs = {};


    return [parse]
}
"#;
    std::thread::spawn(move || {
        for notification in notifications {
            match notification {
                rumqtt::Notification::Publish(publish) => {
                    // 将字节数组转换为字符串
                    if let Ok(payload_str) = std::str::from_utf8(&publish.payload) {
                        let time = get_timestamp(src, payload_str).unwrap();
                        handler_data(src2, payload_str);
                        time_sub(time, i);
                    } else {
                        println!("Received message is not valid UTF-8");
                    };
                }
                rumqtt::Notification::Disconnection => {
                    // 处理断开连接通知
                    println!("Disconnected from MQTT broker");
                    mqtt_client.subscribe("topic/1", QoS::AtMostOnce).unwrap();
                }
                rumqtt::Notification::Reconnection => {
                    // 处理重新连接通知
                    println!("Reconnected to MQTT broker");
                }
                // 2024-04-26 12:35:01
                // 2024-04-26 12:34:50
                _ => {}
            }
        }
    });
}

pub fn get_timestamp(src: &str, data: &str) -> Result<u64, AnyError> {
    let mut script = Script::from_string(src)?;
    let res_str: f64 = script.call("main", (data, ))?;
    let res = res_str as u64;
    Ok(res)
}


pub fn handler_data(src: &str, data: &str) -> Result<(), AnyError> {
    let mut script = Script::from_string(src)?;

    let res: Vec<HashMap<String, u64>> = script.call("main", (data, ))?;

    for item in res.iter() {
        for (key, value) in item.iter() {
            // println!("Key: {}, Value: {}", key, value);
        }
    }

    Ok(())
}


pub fn get_now() -> u64 {
    let current_time = SystemTime::now();
    let since_epoch = current_time.duration_since(UNIX_EPOCH).expect("Time went backwards");
    let milliseconds = since_epoch.as_secs() * 1000 + u64::from(since_epoch.subsec_nanos()) / 1_000_000;
    return milliseconds;
}

pub fn time_sub(up_time: u64, topic: i32) {
    let i = get_now() - up_time;
    println!("topic/{topic} ,  时间差  = {i} 毫秒")
}