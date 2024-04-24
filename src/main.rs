use crossbeam_channel::select;
use rumqtt::{MqttClient, MqttOptions, QoS, SecurityOptions};

use std::{thread, time::Duration};

fn main() {
    let security = SecurityOptions::UsernamePassword("admin".to_string(), "admin123".to_string());

    let mqtt_options = MqttOptions::new("alkjalskfjalfj", "localhost", 1883);
    mqtt_options.clone().set_security_opts(security);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    let (done_tx, done_rx) = crossbeam_channel::bounded(1);

    mqtt_client.subscribe("topic/1", QoS::AtLeastOnce).unwrap();
    let sleep_time = Duration::from_secs(1);
    thread::spawn(move || {
        for i in 0..100 {
            let payload = format!("publish {}", i);
            thread::sleep(sleep_time);
            mqtt_client.publish("hello/world", QoS::AtLeastOnce, false, payload).unwrap();
        }

        thread::sleep(sleep_time * 10);
        done_tx.send(true).unwrap();
    });

    // select between mqtt notifications and other channel rx
    loop {
        select! {
            recv(notifications) -> notification => {
                println!("{:?}", notification)
            }
            recv(done_rx) -> _done => break
        }
    }
}