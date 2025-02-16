use polars::prelude::*;
use polars_excel_writer::ExcelWriter;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 创建第一个示例 DataFrame
    let df1 = df!(
        "col1" => &[1, 2, 3],
        "col2" => &["a", "b", "c"]
    )?;

    // 创建第二个示例 DataFrame
    let df2 = df!(
        "col3" => &[4, 5, 6],
        "col4" => &["d", "e", "f"]
    )?;

    // 创建一个 ExcelWriter 实例
    let mut writer = ExcelWriter::new("output.xlsx");

    // 将第一个 DataFrame 写入名为 "Sheet1" 的工作表
    writer = writer.with_dataframe(&df1, "Sheet1")?;

    // 将第二个 DataFrame 写入名为 "Sheet2" 的工作表
    writer = writer.with_dataframe(&df2, "Sheet2")?;

    // 完成写入操作
    writer.finish()?;

    println!("Data saved to output.xlsx");
    Ok(())
}
