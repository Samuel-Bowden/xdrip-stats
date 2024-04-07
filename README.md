# xdrip-stats

Custom module to display live blood glucose readings from [xDrip+](https://github.com/NightscoutFoundation/xDrip) in various Linux utils (currently only supports [Waybar](https://github.com/NightscoutFoundation/xDrip)).

Values must be written by xDrip+ to a personal InfluxDB instance ([instructions here](https://xdrip.readthedocs.io/en/latest/use/cloud/#influxdb)). This module will then continuously poll this data and print structured data to stdout, which is then read by the utility.

In my setup, I am using the [Libre 2 Direct](https://xdrip.readthedocs.io/en/latest/install/libre2/) method in xDrip+ on my phone, and then uploading the data to InfluxDB2 running on my homelab over a tailscale mesh.

## Installing

```bash
git clone https://github.com/Samuel-Bowden/xdrip-stats
cd xdrip-stats
cargo install --path .
```

## Running

### Waybar:

```json
{
  "modules-right": [
    "custom/xdrip"
  ],
  "custom/xdrip": {
    "return-type": "json",
    "exec": "INFLUX_TOKEN=<TOKEN> xdrip-stats --influx-url <INFLUX_URL> --influx-database <INFLUX_DB> waybar"
  }
}
```
