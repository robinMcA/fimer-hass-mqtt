use anyhow::{Ok, Result};
use serde::{Deserialize, Serialize};
use serde_json::from_slice;
use std::collections::HashMap;
use std::env;
use std::ops::Deref;
use std::sync::Arc;

#[derive(Deserialize, Serialize)]
pub enum Family {
  #[serde(alias = "VEGA_B")]
  VegaB,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum TypeIo {
  #[serde(alias = "in")]
  In,
  #[serde(alias = "out")]
  Out,
  #[serde(alias = "other")]
  Other,
  #[serde(alias = "integer")]
  Integer,
  #[serde(alias = "statistics")]
  Statistics,
}

#[derive(Deserialize, Serialize)]
#[repr(i8)]
pub enum Phase {
  One = 1,
  Three = 3,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Unit {
  model_id: String,
  model_id_descr: String,
  family: Family,
  meter_compatibility: bool,
  device_id: String,
  wiring_box_pn: String,
  input_channel_number: i8,
  output_phase_number: i8,
  ethernet_presence: bool,
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Units {
  Wh,
  W,
  #[serde(alias = "VAR")]
  Var,
  A,
  uA,
  MOhm,
  V,
  Hz,
  #[serde(alias = "degC")]
  DegC,
  #[serde(alias = "")]
  None,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Point {
  pub(crate) name: String,
  pub(crate) unit: Units,
  pub(crate) description: String,
  #[serde(alias = "type")]
  pub(crate) type_io: TypeIo,
  pub(crate) kind: String,
  pub(crate) decimal_precision: i8,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct Device {
  pub device_id: String,
  #[serde(alias = "type")]
  type_device: String,
  device_type: Option<String>,
  device_model: Option<String>,
  pub points: Vec<Point>,
}

impl Device {
  pub fn get_id(&self) -> &str {
    self.device_id.deref()
  }
}

#[derive(Deserialize, Serialize)]
pub struct DeviceResponse {
  #[serde(alias = "Devices")]
  devices: Vec<Device>,
}

#[derive(Deserialize, Clone, Serialize)]
pub struct LivePoint {
  pub name: String,
  value: f32,
}

#[derive(Deserialize, Serialize)]
pub struct LiveDevice {
  device_id: String,
  device_type: Option<String>,
  timestamp: String,
  device_model: Option<String>,
  pub points: Vec<LivePoint>,
}

pub type LiveData = HashMap<String, LiveDevice>;

pub async fn list_live_data() -> Result<Vec<Device>> {
  let client = reqwest::Client::new();
  let api = env::var("FIMER_HOST").expect("");
  let user= env::var("FIMER_USER").expect("");
  let pass:Option<String>= Some(env::var("FIMER_PASSWORD").expect(""));
  let response = client
    .get(format!("{api}/livedata/list", api=api ).as_str())
    .basic_auth(user,pass) 
    .send()
    .await?
    .bytes()
    .await?;
  Ok(from_slice::<DeviceResponse>(&response)?.devices)
}

pub async fn get_live_data() -> Result<LiveData> {
  let client = reqwest::Client::new();
  let api = env::var("FIMER_HOST").expect("");
  let user= env::var("FIMER_USER").expect("");
  let pass:Option<String>= Some(env::var("FIMER_PASSWORD").expect(""));
  let response = client
    .get(format!("{api}/livedata", api = api))
    .basic_auth(user,pass) 
    .send()
    .await?
    .bytes()
    .await?;
  Ok(from_slice(&response)?)
}
