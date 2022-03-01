# Simple library for smart house

## smart_house_lib
```toml
[dependencies]
smart_house_lib = "0.3"
```

## House

### Create House
```rust,no_run
use smart_house_lib::{House};
let house = House::new("test_id".to_string(), "name".to_string());
```
### Add apartment to house
```rust,no_run
use smart_house_lib::{House, Apartment};
let mut house = House::new("test_id".to_string(), "name".to_string());
house.add_apartment(Apartment::new("name".to_string()));
```
### Remove apartment from house
```rust,no_run
use smart_house_lib::{House, Apartment};
let mut house = House::new("test_id".to_string(), "name".to_string());
house.add_apartment(Apartment::new("name".to_string()));
house.remove_apartment("name".to_string());
```

## Apartment
### Create Apartment
```rust,no_run
use smart_house_lib::Apartment;
let apartment = Apartment::new("name".to_string());
```
### Add devices to apartment
```rust,no_run
use smart_house_lib::{Apartment, Device, Rosette};
let mut apartment = Apartment::new("name".to_string());
let device = Device::Rosette(Rosette::new("test".to_string(), "127.0.0.1:8080".to_string()));
apartment.add_device(device);
```
### Remove device from apartment
```rust,no_run
use smart_house_lib::{Apartment, Device, Rosette};
let mut apartment = Apartment::new("name".to_string());
let device = Device::Rosette(Rosette::new("test".to_string(), "127.0.0.1:8080".to_string()));
apartment.add_device(device);
apartment.remove_device("test".to_string());
```

## Device
### Create device
```rust,no_run
use smart_house_lib::{Device, Rosette, Thermometer};
let thermometer = Device::Thermometer(Thermometer::new("name".to_string(), 23.0, "127.0.0.1:9091".to_string()));
let rosette = Device::Rosette(Rosette::new("name".to_string(), "127.0.0.1:8081".to_string()));
```

### Create rosette
```rust,no_run
use smart_house_lib::Rosette;
let rosette = Rosette::new("name".to_string(), "127.0.0.1:9091".to_string());
```

### Create thermometer
```rust,no_run
use smart_house_lib::Thermometer;
let thermometer = Thermometer::new("name".to_string(), 23.0, "127.0.0.1:9091".to_string());
```

## License

This project is licensed under the [MIT license].

[MIT license]: https://github.com/zhuravlevma/smart-house/blob/main/LICENSE
