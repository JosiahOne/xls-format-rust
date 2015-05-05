mod Worksheet;
// Basic excel

struct Basic_Excel {
    pub file_: Compound_File,                  // Compound file handler.
    pub workbook_: Workbook,                   // Raw Workbook.
    pub worksheets_: Vec<Worksheet::Worksheet>,// Raw Worksheets.
    pub yesheets_: Vec<Basic_Excel_Worksheet>, // Parsed Worksheets.
}

impl Basic_Excel {
    
    pub fn new() -> Basic_Excel {
        return Basic_Excel{file_: Compound_File, workbook_: Workbook, worksheets_: vec![Worksheet::Worksheet], yesheets_: vec![Basic_Excel_Worksheet::new()]};
    }
    
    // File functions
    pub fn new_with_sheets(sheets: i32) {
        unimplemented!();
    }
    
    pub fn load<T>(filename: T) -> bool {
        let mut result = false;
        
        return result;
    }
    
    pub fn save() -> bool {
        let mut result = false;
        
        return result;
    }
    
    pub fn save_as<T>(filename: T) -> bool {
        let mut result = false;
        
        return result;
    }
    
    pub fn close() {
        unimplemented!();
    }
    
    // Worksheet functions
    pub fn get_total_worksheets() -> i32 {
        let mut result = 0;
        
        return result;
    }
    
    pub fn get_worksheet<T>(sheet_index: T) -> Basic_Excel_Worksheet {
        let mut result = Basic_Excel_Worksheet::new();
        
        return result;
    }
    
    pub fn add_worksheet<T>(sheet_index: T) -> Basic_Excel_Worksheet {
        let mut result = Basic_Excel_Worksheet::new();
        
        return result;
    }
    
    pub fn delete_worksheet<T>(sheet_index: T) -> bool {
        let mut result = false;
        
        return result;
    }
    
    pub fn get_ansi_sheet_name(sheet_index: i32) {
        panic!("Rust operates completely in UTF-8 Unicode strings and as such there's no reason return an ANSI version.");
    }
    
    pub fn get_unicode_sheet_name(sheet_index: i32) -> String {
        let mut result = String::new();
        
        return result;
    }
    
    pub fn get_sheet_name(sheet_index: i32) -> (bool, String) {
        let mut result = false;
        let mut name = String::new();
        
        return (result, name);
    }
    
    pub fn rename_worksheet_from_index(sheet_index: i32, to: &str) -> bool {
        let mut result = false;
        
        return result;
    }

    pub fn rename_worksheet_from_str(from: &str, to: &str) -> bool {
        let mut result = false;
        
        return result;
    }
    
}

struct Basic_Excel_Worksheet {
    excel_: Basic_Excel,
    sheet_index_: i32,
    max_rows_: i32,
    max_cols_: i32,
    cells_: Vec<Basic_Excel_Cell>,
    col_infos_: Worksheet::Col_Infos,
}

impl Basic_Excel_Worksheet {
    // Worksheet functions
    pub fn new() -> Basic_Excel_Worksheet {
        return Basic_Excel_Worksheet{excel_: Basic_Excel::new(), sheet_index_: 0, max_rows_: 0, max_cols_: 0, cells_: vec![Basic_Excel_Cell::new()], col_infos_: Worksheet::Col_Infos};
    }
    
    pub fn get_sheet_name() -> (bool, String) {
        unimplemented!();
    }
    
    pub fn rename(to: &str) -> bool {
        unimplemented!();
    }
    
    pub fn print() {
        unimplemented!();
    }
    
    // Cell functions
    pub fn get_total_rows() -> i32 {
        unimplemented!();
    }
    
    pub fn get_total_cols() -> i32 {
        unimplemented!();
    }
    
    pub fn cell(row: i32, col: i32) -> *mut Basic_Excel_Cell {
        unimplemented!();
    }
    
    pub fn erase_cell(row: i32, col: i32) -> bool {
        unimplemented!();
    }
    
    pub fn set_col_width(col_index: i32, col_width: u16) {
        unimplemented!();
    }
    
    pub fn get_col_width(col_index: i32) -> u16 {
        unimplemented!();
    }
    
    pub fn merge_cells(row: i32, col: i32, row_range: u16, col_range: u16) {
        unimplemented!();
    }
    
    // Private
    fn update_cells() {
        unimplemented!();
    }
}

struct Compound_File;

struct Workbook;

struct Basic_Excel_Cell {
    type_: i32,
    ival_: i32,
    fval_: f32,
    str_: String,
    
    merged_rows_: u16,
    merged_cols_: u16,
    
    _xf_idx: i32,
}

impl Basic_Excel_Cell {
    pub fn new() -> Basic_Excel_Cell {
        return Basic_Excel_Cell{type_: 0, ival_: 0, fval_: 0 as f32, str_: "".to_string(), merged_rows_: 0, merged_cols_: 0, _xf_idx: 0};
    }
    
    pub fn get_type() -> i32 {
        unimplemented!();
    }
    
    pub fn get() -> (bool, f32) { // We don't need a separate return for ints in rust, since you can easily type-cast it (E.G. let (x, z as i32) = get())
        unimplemented!();
    }
    
    pub fn get_string_length() -> usize { // Length of string contents
        unimplemented!();
    }
    
    pub fn get_integer() -> i32 {
        unimplemented!();
    }
    
    pub fn get_float() -> f32 {
        unimplemented!();
    }
    
    pub fn get_string() -> String {
        unimplemented!();
    }
    
    pub fn set<T>(val: T) {
        unimplemented!();
    }
    
    pub fn set_formula() {
        unimplemented!();
    }
    
    pub fn get_xformat_index(&self) -> i32 {
        return self._xf_idx;
    }
    
    pub fn set_xformat_index(&mut self, xf_idx: i32) {
        self._xf_idx = xf_idx;
    }
    
    pub fn set_format() {
        unimplemented!();
    }
    
    pub fn EraseContents() {
        unimplemented!();
    }
    
    pub fn get_merged_rows(&self) -> u16 {
        return self.merged_rows_;
    }
    
    pub fn get_merged_cols(&self) -> u16 {
        return self.merged_cols_;
    }
    
    pub fn set_merged_rows(&mut self, merged_rows: u16) {
        self.merged_rows_ = merged_rows;
    }
    
    pub fn set_merged_cols(&mut self, merged_cols: u16) {
        self.merged_cols_ = merged_cols;
    }
}
