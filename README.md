# voicevox-wrapper

## Usage

voicevox engineの起動

```shell
docker pull voicevox/voicevox_engine:cpu-ubuntu20.04-latest
docker run -d --rm -p '127.0.0.1:50021:50021' \
  voicevox/voicevox_engine:cpu-ubuntu20.04-latest
``````

build

```shell
cargo build --release
```

run

```shell
./target/release/vv
```

## References

- VOICEVOX ENGINE: https://github.com/VOICEVOX/voicevox_engine
