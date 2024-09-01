// use calamine::{deserialize_as_f64_or_none, open_workbook, RangeDeserializerBuilder, Reader, Xlsx};
// use serde::Deserialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use calamine::{open_workbook, DataType, Reader, Xlsx};
    // use serde::Deserialize;

    #[test]
    fn read_ti_excel() -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("{}/tests/excel.xlsx", env!("CARGO_MANIFEST_DIR"));
        let mut workbook: Xlsx<_> = open_workbook(path)?;
        let range_raw = workbook
            .worksheet_range("Sheet1")
            .map_err(|_| calamine::Error::Msg("Cannot find Sheet1"))?;
        println!(
            "range_raw: ({:?}, {:?})",
            range_raw.start().unwrap(),
            range_raw.end().unwrap()
        );

        let range = range_raw.range(
            (1, 0),
            (
                range_raw.get_size().0 as u32 - 1,
                range_raw.get_size().1 as u32 - 1,
            ),
        );
        println!(
            "range: ({:?}, {:?})",
            range.start().unwrap(),
            range.end().unwrap()
        );
        println!(
            "range size: ({}, {})",
            range.get_size().0,
            range.get_size().1
        );

        let total_cells = range.get_size().0 * range.get_size().1;

        let non_empty_cells: usize = range.used_cells().count();
        println!(
            "Found {} cells in 'Sheet1', including {} non empty cells",
            total_cells, non_empty_cells
        );
        // alternatively, we can manually filter rows
        assert_eq!(
            non_empty_cells,
            range
                .rows()
                .flat_map(|r| r.iter().filter(|&c| !c.is_empty()))
                .count()
        );

        Ok(())
    }
}

#[cfg(test)]
mod tests2 {
    use calamine::{
        deserialize_as_f64_or_none, open_workbook, RangeDeserializerBuilder, Reader, Xlsx,
    };
    use serde::Deserialize;
    #[derive(Deserialize)]
    struct Record {
        metric: String,
        #[serde(deserialize_with = "deserialize_as_f64_or_none")]
        value: Option<f64>,
    }
    #[test]
    fn read_metric() -> Result<(), Box<dyn std::error::Error>> {
        let path = format!("{}/tests/metric.xlsx", env!("CARGO_MANIFEST_DIR"));
        let mut excel: Xlsx<_> = open_workbook(path)?;

        let range = excel
            .worksheet_range("Sheet1")
            .map_err(|_| calamine::Error::Msg("Cannot find Sheet1"))?;

        let iter_records =
            RangeDeserializerBuilder::with_headers(&["metric", "value"]).from_range(&range)?;

        for result in iter_records {
            let record: Record = result?;
            println!("metric={:?}, value={:?}", record.metric, record.value);
        }

        Ok(())
    }
}
