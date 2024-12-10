# pg_bedtools_rs

A `PostgreSQL` plugin for `bedtools`, implemented in `Rust`.

Please note that this version of pg_bedtools_rs only works with PostgreSQL Version 13.

## API

bed_overlap(table_name: string, padding: int, conditions: string) RETURNS chromosome, pos_start, pos_end

## Example

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
	('chr1', 10, 14),
	('chr1', 7, 15),
	('chr1', 22, 30),
	('chr1', 25, 35),
	('chr2', 5, 10),
	('chr2', 7, 15);
	
select * from bed_overlap('tube_probe_bed');
```

Returns:

| CHROMOSOME | POS_START | POS_END |
|------------|-----------|---------|
| chr1       | 5         | 15      |
| chr1       | 22        | 35      |
| chr2       | 5         | 15      |
