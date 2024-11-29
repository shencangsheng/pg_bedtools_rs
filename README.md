[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)

# pg_bedtools_rs

A `PostgreSQL` plugin for `bedtools`, implemented in `Rust`.

## 🌟 Features

- [x] `bed_overlap`

## 🛠️ Installer

```bash
cargo install cargo-pgrx --version 0.11.4 --locked
cargo pgrx install --pg-config [PATH]
```

```sql
CREATE EXTENSION pg_bedtools_rs
```

## 📦 Supports

- PostgreSQL 13

## 💡 Trying

```sql
select * from bed_overlap('tube_probe_bed', 20)
```

## 👍 Libraries Used

* [bedrs](https://github.com/noamteyssier/bedrs)

## 📝 License

A short snippet describing the license (MIT)

MIT © Cangsheng Shen