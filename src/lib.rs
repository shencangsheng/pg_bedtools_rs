use pgrx::prelude::*;
use pgrx::spi::{Spi};
use bedrs::{Bed3, Coordinates, MergeIter};

pub mod bed;

// 定义扩展模块
::pgrx::pg_module_magic!();

// 定义扩展模块的入口点
#[pg_guard]
pub extern "C" fn _PG_init() {
    // 初始化代码（如果需要）
}

// 测试模块
#[cfg(any(test, feature = "pg_test"))]
#[pg_schema]
mod tests {
    use super::*;

    // 初始化测试环境
    #[pg_test]
    fn test_bed_overlap() {
        // 创建测试表并插入数据
        Spi::run("CREATE TABLE tube_probe_bed (chromosome varchar, pos_start integer, pos_end integer);").unwrap();
        Spi::run("INSERT INTO tube_probe_bed (chromosome, pos_start, pos_end) VALUES ('chr1', 5, 10), ('chr1', 10, 14), ('chr1', 10, 14), ('chr1', 7, 15), ('chr1', 22, 30), ('chr1', 25, 35), ('chr2', 5, 10), ('chr2', 7, 15);").unwrap();

        // 调用扩展函数
        let result = Spi::get_one::<i64>("SELECT count(*) FROM bed_overlap('tube_probe_bed', 0);").unwrap();
        assert_eq!(result.unwrap(), 3);
    }
}

#[cfg(test)]
pub mod pg_test {
    pub fn setup(_options: Vec<&str>) {
        // perform one-off initialization when the pg_test framework starts
    }

    #[must_use]
    pub fn postgresql_conf_options() -> Vec<&'static str> {
        // return any postgresql.conf settings that are required for your tests
        vec![]
    }
}
