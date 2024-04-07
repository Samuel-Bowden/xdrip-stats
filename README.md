# xdrip-stats

![screenshot](https://github.com/Samuel-Bowden/xdrip-stats/assets/91887909/8ad6b882-d679-4b3e-9624-e62e5c75aa90)


Custom module to display live blood glucose readings from [xDrip+](https://github.com/NightscoutFoundation/xDrip) in various Linux utils (currently only supports [Waybar](https://github.com/NightscoutFoundation/xDrip)).

Values must be written by xDrip+ to a personal InfluxDB instance ([instructions here](https://xdrip.readthedocs.io/en/latest/use/cloud/#influxdb)). This module will then continuously poll these values and print them as structured data to stdout in a format the supported utility understands (such as JSON).

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

## Utils Supported

- [x] Waybar
- [ ] Mangohud

## Disclaimer

Please note that this repository is provided "as is", and should be used at your own risk. The author(s) are not responsible for any potential damages that may be incurred from its use.
