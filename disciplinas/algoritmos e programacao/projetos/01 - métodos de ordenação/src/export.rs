use rust_xlsxwriter::Workbook;
use chrono::Local;
use std::collections::HashMap;

pub struct XlsxExporter {
    workbook: Workbook,

    next_row: HashMap<String, u32>,
}

impl XlsxExporter {

    pub fn new() -> Self {

        Self {
            workbook: Workbook::new(),
            next_row: HashMap::new(),
        }
    }

    fn get_sheet(&mut self, name: &str) -> &mut rust_xlsxwriter::Worksheet {
        if self.workbook.worksheet_from_name(name).is_ok() {
            self.workbook.worksheet_from_name(name).unwrap()
        } else {
            let sheet = self.workbook.add_worksheet();
            sheet.set_name(name).unwrap();
            sheet
        }
    }

    pub fn write_headers(&mut self, sheet_name: &str, headers: &[&str]) {
        let sheet = self.get_sheet(sheet_name);
        for (col, &header) in headers.iter().enumerate() {
            sheet.write(0, col as u16, header).unwrap();
        }
        // Ensure next_row starts at 1
        self.next_row.entry(sheet_name.to_string()).or_insert(1);
    }

    pub fn write_run(
        &mut self,
        sheet_name: &str,
        algorithm: &str,
        size: usize,
        run: usize,
        time: f32,
    ) {

        let row = *self.next_row.entry(sheet_name.to_string()).or_insert(1);
        let sheet = self.get_sheet(sheet_name);

        sheet.write(row, 0, algorithm).unwrap();
        sheet.write(row, 1, size as u32).unwrap();
        sheet.write(row, 2, run as u32).unwrap();
        sheet.write(row, 3, format!("{:.6}", time)).unwrap();

        self.next_row.insert(sheet_name.to_string(), row + 1);
    }

    pub fn write_summary(
        &mut self,
        sheet_name: &str,
        size: usize,
        bubble: f32,
        selection: f32,
        insertion: f32,
        merge: f32,
        quick: f32,
    ) {

        let row = *self.next_row.entry(sheet_name.to_string()).or_insert(1);
        let sheet = self.get_sheet(sheet_name);

        sheet.write(row, 0, size as u32).unwrap();
        sheet.write(row, 1, format!("{:.6}", bubble)).unwrap();
        sheet.write(row, 2, format!("{:.6}", selection)).unwrap();
        sheet.write(row, 3, format!("{:.6}", insertion)).unwrap();
        sheet.write(row, 4, format!("{:.6}", merge)).unwrap();
        sheet.write(row, 5, format!("{:.6}", quick)).unwrap();

        self.next_row.insert(sheet_name.to_string(), row + 1);
    }

    pub fn save(mut self) {

        let timestamp = Local::now().format("%Y%m%d_%H%M%S");
        let filename = format!("results_{}.xlsx", timestamp);

        self.workbook.save(filename).unwrap();
    }
}