use crate::fimer;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum Platform {
    Sensor,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DeviceClass {
    Voltage,
    Temperature,
    ReactivePower,
    Power,
    PowerFactor,
    Energy,
    Duration,
    ApparentPower,
    Current,
    Frequency,
    #[serde(rename = "")]
    None,
}

#[derive(Deserialize, Debug, Serialize)]
enum Units {
    Wh,
    W,
    #[serde(alias = "VAR")]
    Var,
    A,
    V,
    Hz,
    #[serde(alias = "°C")]
    DegC,
    #[serde(rename = "")]
    None,
}

impl From<fimer::Units> for Units {
    fn from(value: fimer::Units) -> Self {
        match value {
            fimer::Units::Wh => Self::Wh,
            fimer::Units::W => Self::W,
            fimer::Units::Var => Self::Var,
            fimer::Units::A => Self::A,
            fimer::Units::Hz => Self::Hz,
            fimer::Units::DegC => Self::DegC,
            fimer::Units::V => Self::V,
            _ => Self::None,
        }
    }
}

impl From<fimer::Units> for DeviceClass {
    fn from(value: fimer::Units) -> Self {
        match value {
            fimer::Units::Wh => Self::Energy,
            fimer::Units::W => Self::Power,
            fimer::Units::Var => Self::ReactivePower,
            fimer::Units::uA => Self::Current,
            fimer::Units::A => Self::Current,
            fimer::Units::Hz => Self::Frequency,
            fimer::Units::DegC => Self::Temperature,
            fimer::Units::None => Self::None,
            fimer::Units::V => Self::Voltage,
            _ => Self::None,
        }
    }
}

impl fimer::Units {
    fn to_comp(&self) -> (DeviceClass, Units, Option<String>) {
        match self {
            fimer::Units::Wh => (DeviceClass::Energy, Units::Wh, Some("total".to_string())),
            fimer::Units::W => (DeviceClass::Power, Units::W, None),
            fimer::Units::Var => (DeviceClass::ReactivePower, Units::Var, None),
            fimer::Units::A => (DeviceClass::Current, Units::A, None),
            fimer::Units::Hz => (DeviceClass::Frequency, Units::Hz, None),
            fimer::Units::DegC => (DeviceClass::Temperature, Units::DegC, None),
            fimer::Units::None => (DeviceClass::None, Units::None, None),
            fimer::Units::V => (DeviceClass::Voltage, Units::V, None),
            _ => (DeviceClass::None, Units::None, None),
        }
    }
}

#[derive(Serialize)]
struct Device {
    ids: String,
    name: String,
    mf: String,
    mdl: String,
    sw: String,
    hw: String,
    // device_class: String,
}

impl Default for Device {
    fn default() -> Self {
        Self {
            ids: "some_id_4_lam".to_string(),
            name: "fimer at 4 ".to_string(),
            mf: "firm".to_string(),
            mdl: "could be better".to_string(),
            sw: "0.1".to_string(),
            hw: "0.1".to_string(),
            // device_class: "energy".to_string(),
        }
    }
}

#[derive(Serialize)]
struct Origin {
    name: String,
    sw: String,
    url: String,
}

impl Default for Origin {
    fn default() -> Self {
        Self {
            name: "rnlm_fimer".to_string(),
            sw: "0.1".to_string(),
            url: "https://github.com".to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct SensorComponent {
    device_class: DeviceClass,
    unit_of_measurement: Units,
    value_template: String,
    unique_id: String,
    state_topic: String,
    qos: i8,
    state_class: Option<String>,
}

impl From<fimer::Point> for SensorComponent {
    fn from(value: fimer::Point) -> Self {
        let (device_class, unit_of_measurement, state_class) = value.unit.to_comp();
        Self {
            device_class,
            unit_of_measurement,
            value_template: "{{ value_json.sensor }}".to_string(),
            unique_id: value.name.clone(),
            state_topic: format!("fimer/{name}/state", name = value.name),
            qos: 0,
            state_class,
        }
    }
}

impl From<&fimer::Point> for SensorComponent {
    fn from(value: &fimer::Point) -> Self {
        let (device_class, unit_of_measurement, state_class) = value.unit.to_comp();
        Self {
            device_class,
            unit_of_measurement,
            value_template: "{{ value_json.value }}".to_string(),
            unique_id: value.name.clone(),
            state_topic: format!("fimer/{name}/state", name = value.name),
            qos: 0,
            state_class,
        }
    }
}

#[derive(Serialize)]
pub struct DiscoverSensor {
    #[serde(skip_serializing)]
    pub(crate) name: String,
    #[serde(alias = "dev")]
    device: Device,
    #[serde(alias = "o")]
    origin: Origin,
    #[serde(flatten)]
    sensor_component: SensorComponent,
}

impl DiscoverSensor {
    pub fn new(sensor_component: SensorComponent) -> Self {
        Self {
            name: sensor_component.unique_id.clone(),
            sensor_component,
            device: Device::default(),
            origin: Origin::default(),
        }
    }
}

#[derive(Serialize)]
pub struct DiscoverDevice {
    #[serde(alias = "dev")]
    device: Device,
    #[serde(alias = "o")]
    origin: Origin,
    #[serde(alias = "cmp")]
    components: Vec<SensorComponent>,
}

impl DiscoverDevice {
    pub fn new(points: Vec<fimer::Point>) -> Self {
        Self {
            device: Device::default(),
            origin: Origin::default(),
            components: points.iter().map(|p| p.into()).collect(),
        }
    }
}
