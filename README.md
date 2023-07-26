# Covid Tracker

Collecting information from [worldometers](https://www.worldometers.info/coronavirus)

## How to use


- install with `cargo install covid_tracker`

help:  
`covid_tracker -h` 

```Usage: covid_tracker [OPTIONS]

Options:
  -n, --count <COUNT>  Number of rows to show [default: 1000]
  -w, --world-stats    Should the world stats be excluded
  -f, --show_full      Show full or only totals [default: false]
  -h, --help           Print help information
  -V, --version        Print version information
  ```

This will output a CLI table like this:

![Output Example](./__image/5%20line%20output.png)