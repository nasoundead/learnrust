use polars::prelude::*;
use polars_excel_writer::PolarsXlsxWriter;
// use rust_xlsxwriter::{Table, TableStyle};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let df: DataFrame = df!(
        "String" => &["North", "South", "East", "West"],
        "Int" => &[1, 2, 3, 4],
        "Float" => &[1.0, 2.22, 3.333, 4.4444],
    )?;

    // 创建第二个示例 DataFrame
    let df2: DataFrame = df!(
        "col3" => &[4, 5, 6],
        "col4" => &["d", "e", "f"]
    )?;

    // 打开文件以进行写入操作
    let mut xlsx_writer = PolarsXlsxWriter::new();

    // let table = Table::new().set_style(TableStyle::Medium4);

    // xlsx_writer.set_table(&table);

    xlsx_writer.set_worksheet_name("Polars Data1")?;
    xlsx_writer.write_dataframe(&df)?;

    xlsx_writer.add_worksheet();
    xlsx_writer.set_worksheet_name("Polars Data2")?;
    xlsx_writer.write_dataframe(&df2)?;

    xlsx_writer.save("dataframe.xlsx")?;

    println!("Data saved to dataframe.xlsx");
    Ok(())
}
