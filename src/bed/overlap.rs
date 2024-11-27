// 定义扩展模块

use bedrs::{Bed3, Coordinates, MergeIter};
use pgrx::{default, name, pg_extern, Spi};
use pgrx::iter::TableIterator;

#[pg_extern]
fn bed_overlap(table_name: &str, padding: default!(i32, 0)) -> TableIterator<'static, (name!(chromosome, String), name!(pos_start, i32), name!(pos_end, i32))> {
    let query = format!("SELECT chromosome, pos_start, pos_end FROM {} order by chromosome, pos_start asc", table_name);

    Spi::connect(|client| -> Result<TableIterator<'static, (name!(chromosome, String), name!(pos_start, i32), name!(pos_end, i32))>, Box<dyn std::error::Error>> {
        let mut intervals = vec![];

        let mut result = client.select(&query, None, None)?;
        while let Some(row) = result.next() {
            let chromosome = row.get::<String>(1).unwrap().unwrap();
            let pos_start = row.get::<i32>(2).unwrap().unwrap() - padding;
            let pos_end = row.get::<i32>(3).unwrap().unwrap() + padding;
            intervals.push(Bed3::new(chromosome, pos_start, pos_end));
        }

        Ok(TableIterator::new(MergeIter::new(intervals.into_iter()).map(|p| {
            (p.chr().clone(), p.start(), p.end())
        }
        )))
    }).unwrap()
}
