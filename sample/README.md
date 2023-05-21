## How to run samples
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
# Check that default toolchain is nightly
```
4.
```sh
/app$ cargo run --bin ＜sample name＞
# Takes some time to updating crates.io index and compiling
# See the result and logs from `qjack-sample-postgres` container
```
5.
```sh
$ docker compose down
# or
$ docker-compose down

# And, if you'd like to refresh DB volume：
$ docker volume rm qjack-sample-data
```

## Warning
Sample apps are so crazy in some points：

- stores passwords without hashing
- literal DB URL

or others. They are only for simplification and **must** be avoided in real app.
