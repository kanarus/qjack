1.
```sh
$ docker compose up
# or
$ docker-compose up
```
2.
```sh
# (in another tab)

$ docker compose exec app bash
# or
$ docker-compose exec app bash
```
3.
```sh
/app$ rustc --version
# check that default toolchain is nightly
```
4.
```sh
/app$ cargo run
# takes some time to updating crates.io index and compiling
# (`--release` will takes crazily long time)
```
