# Rust MQTT 客户端测试



基础程序

```
use rumqtt::{MqttClient, MqttOptions, QoS, SecurityOptions};

mod js_test;

fn main() {
    let security = SecurityOptions::UsernamePassword("admin".to_string(), "admin123".to_string());

    let mqtt_options = MqttOptions::new("alkjalskfjalfj", "localhost", 1883);
    mqtt_options.clone().set_security_opts(security);


    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();

    mqtt_client.subscribe("topic/1", QoS::AtMostOnce).unwrap();

    std::thread::spawn(move || {
        for notification in notifications {
            match notification {
                rumqtt::Notification::Publish(publish) => {
                    // 将字节数组转换为字符串
                    if let Ok(payload_str) = std::str::from_utf8(&publish.payload) {
                        println!("Received: {}", payload_str);
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

                _ => {}
            }
        }
    });


    // 主线程等待
    loop {}
}
```



## 重连时间测试

MQTT服务端上剔除客户端时间消耗11秒 ` 2024-04-26 12:35:01` -`2024-04-26 12:34:50`








# Rust 性能分析




sudo dtrace -c './rust-ev large-file' -o out.stacks -n 'profile-997 /execname == "rust-ev"/ { @[ustack(100)] = count(); }'

`rust-ev` 是指编译好的二进制文件


git clone git@github.com:brendangregg/FlameGraph.git 
把这个文件放到系统环境变量

stackcollapse.pl out.stacks | flamegraph.pl > out.svg
