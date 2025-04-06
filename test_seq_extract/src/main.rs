use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use calamine::{open_workbook, DataType, Range, Reader, Xlsx};
use clap::Parser;
use rayon::prelude::*;
use serde_json::{json, Value};

#[derive(Parser, Debug)]
#[clap(version = "1.0", author = "Your Name")]
struct Args {
    /// Excel文件路径列表
    #[clap(required = true)]
    files: Vec<PathBuf>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    args.files.par_iter().for_each(|file| {
        if let Err(e) = process_file(file) {
            eprintln!("处理文件失败: {:?}, 错误: {}", file, e);
        }
    });

    Ok(())
}

fn process_file(file: &Path) -> Result<(), Box<dyn std::error::Error>> {
    let mut workbook: Xlsx<_> = open_workbook(file)?;
    let mut result = HashMap::new();

    for sheet_name in workbook.sheet_names() {
        if let Some(Ok(range)) = workbook.worksheet_range(&sheet_name) {
            // let merged_cells = get_merged_cells(&range);
            // let sheet_data = process_sheet(&range, &merged_cells);
            let sheet_data = process_sheet(&range, &[]);
            result.insert(sheet_name, sheet_data);
        }
    }

    save_json(file, &result)?;
    Ok(())
}

// fn get_merged_cells(range: &Range<DataType>) -> Vec<((u32, u32), (u32, u32))> {
//     let mut merged_cells = Vec::new();
//     for merged in range.merged_cells() {
//         let start = (merged.start.0, merged.start.1);
//         let end = (merged.end.0, merged.end.1);
//         merged_cells.push((start, end));
//     }
//     merged_cells
// }

fn process_sheet(range: &Range<DataType>, merged_cells: &[((u32, u32), (u32, u32))]) -> Value {
    let mut map = HashMap::new();

    // 跳过首行（列标题）
    for (row_idx, row) in range.rows().skip(1).enumerate() {
        if row.len() < 2 {
            continue;
        }

        let (Some(key), Some(value)) = (
            get_cell_value(row, 0, merged_cells, range, row_idx as u32),
            get_cell_value(row, 1, merged_cells, range, row_idx as u32),
        ) else {
            continue;
        };

        if key.is_empty() || value.is_empty() {
            continue;
        }

        insert_to_map(&mut map, key, value);
    }

    json!(map)
}

fn get_cell_value(
    row: &[DataType],
    col: u32,
    merged: &[((u32, u32), (u32, u32))],
    range: &Range<DataType>,
    row_idx: u32,
) -> Option<String> {
    let cell_pos = (row_idx + 1, col); // +1 因为跳过了标题行

    // 查找合并单元格
    let merged_value = merged.iter().find_map(|&(start, end)| {
        if start.0 <= cell_pos.0
            && cell_pos.0 <= end.0
            && start.1 <= cell_pos.1
            && cell_pos.1 <= end.1
        {
            range.get_value(start).map(|v| v.to_string())
        } else {
            None
        }
    });

    merged_value.or_else(|| {
        row.get(col as usize)
            .map(|v| v.to_string())
            .filter(|s| !s.is_empty())
    })
}

fn insert_to_map(map: &mut HashMap<String, Value>, key: String, value: String) {
    match map.get_mut(&key) {
        Some(existing) => match existing {
            Value::String(s) => {
                *existing = json!([s, value]);
            }
            Value::Array(arr) => {
                arr.push(json!(value));
            }
            _ => {}
        },
        None => {
            map.insert(key, json!(value));
        }
    }
}

fn save_json(
    excel_path: &Path,
    data: &HashMap<String, Value>,
) -> Result<(), Box<dyn std::error::Error>> {
    let json_path = excel_path.with_extension("json");
    let json = serde_json::to_string_pretty(&json!(data))?;
    std::fs::write(json_path, json)?;
    Ok(())
}
