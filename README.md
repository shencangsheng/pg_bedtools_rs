[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE.md)

# pg_bedtools_rs

A `PostgreSQL` plugin for `bedtools`, implemented in `Rust`.

## 🌟 Features

- [x] `bed_merge`

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

Basic example:

```sql
create table tube_probe_bed (
   id SERIAL PRIMARY KEY,
   chromosome varchar NOT NULL,
   pos_start integer NOT NULL,
   pos_end integer NOT NULL
);

INSERT INTO
	TUBE_PROBE_BED (CHROMOSOME, POS_START, POS_END)
VALUES
	('chr1', 5, 10),
	('chr1', 10, 14),
	('chr1', 7, 15),
	('chr1', 22, 30),
	('chr1', 25, 35),
	('chr2', 5, 10),
	('chr2', 7, 15);
	
select * from bed_merge('tube_probe_bed');
```

Returns:

| CHROMOSOME | POS_START | POS_END |
|------------|-----------|---------|
| chr1       | 5         | 15      |
| chr1       | 22        | 35      |
| chr2       | 5         | 15      |

## 👍 Libraries Used

* [bedrs](https://github.com/noamteyssier/bedrs)

## 📝 License

A short snippet describing the license (MIT)

MIT © Cangsheng Shen