1.
```sh
$ docker compose up
# or
$ docker-compose up
```
2.
(In another tab)
```sh
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
/app$ cargo run --example ＜example name＞
# takes some times to updating crates.io index and compiling
```
