use crate::hass_mqtt::{DiscoverSensor};
use anyhow::{Result};
use rumqttc::{AsyncClient, Client, MqttOptions, QoS};
use std::env;
use std::sync::Arc;
use std::time::Duration;
use tokio::{task, time};

mod fimer;
mod hass_mqtt;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
  println!("Hello, world!");

  let host = env::var("MQTT_HOST").expect("MQTT_HOST is not set");
  let port:u16 = env::var("MQTT_PORT").unwrap_or(String::from("1883")).parse()?;
  let auth: (String, String) = (env::var("MQTT_USERNAME").expect("MQTT_USER should be set"), env::var("MQTT_PASSWORD").expect("MQTT_PASSWORD is not set").to_string());

  let mut mqttoptions = MqttOptions::new("fimer-mqtt", host, port);
  mqttoptions.set_keep_alive(Duration::from_secs(5));
  mqttoptions.set_credentials(auth.0, auth.1);

  let (client, mut connection) = AsyncClient::new(mqttoptions, 10);
  let clnt = Arc::new(client);

  loop {
    if let Ok(devices) = fimer::list_live_data().await {
      let first = devices.first().unwrap().clone();
      let key_device = first.get_id().to_string();


      let client = Arc::clone(&clnt);
      task::spawn(async move {
        loop {
          let data = first.points.iter().map(|point| DiscoverSensor::new(point.into()));
          for i in data {
            let json = serde_json::to_string(&i).unwrap();
            let _test = client.publish(format!("homeassistant/sensor/{name}/config", name = i.name), QoS::AtLeastOnce, false, json.as_bytes()).await;
            time::sleep(Duration::from_millis(10)).await;
          }

          time::sleep(Duration::from_secs(36000)).await;
        }
      });

      let client = Arc::clone(&clnt);
      task::spawn(async move {
        loop {
          if let Ok(dt) = fimer::get_live_data().await {
            let enrich = dt.get(&key_device).unwrap().points.clone();

            for i in enrich.iter() {
              let json = serde_json::to_string(&i).unwrap();
              let _send = client.publish(format!("fimer/{name}/state", name = i.name), QoS::AtLeastOnce, false, json.as_bytes()).await;
            }
          }
          time::sleep(Duration::from_secs(30)).await;
        }
      });

      loop {
        let notification = connection.poll().await?;
        println!("Received = {:?}", notification);
      }
    }

    time::sleep(Duration::from_secs(3600)).await;
  }
}
