mod Worksheet;
// Basic excel

struct Basic_Excel {
    pub file_: Compound_File,                  // Compound file handler.
    pub workbook_: Workbook,                   // Raw Workbook.
    pub worksheets_: Vec<Worksheet::Worksheet>,// Raw Worksheets.
    pub yesheets_: Vec<Basic_Excel_Worksheet>, // Parsed Worksheets.
}

impl Basic_Excel {
    
    // File functions
    pub fn new(sheets: i32) {
        
    }
    
    pub fn load<T>(filename: T) -> bool {
        
    }
    
    pub fn save() -> bool {
        
    }
    
    pub fn save_as<T>(filename: T) -> bool {
        
    }
    
    pub fn close() {
        
    }
    
    // Worksheet functions
    pub fn get_total_worksheets() -> i32 {
        
    }
    
    pub fn get_worksheet<T>(sheet_index: T) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn add_worksheet<T>(sheet_index: T) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn delete_worksheet<T>(sheet_index: T) -> bool {
        
    }
    
    pub fn get_ansi_sheet_name(sheet_index: i32) {
        panic!("Rust operates completely in UTF-8 Unicode strings and as such there's no reason return an ANSI version.");
    }
    
    pub fn get_unicode_sheet_name(sheet_index: i32) -> String {
        
    }
    
    pub fn get_sheet_name(sheet_index: i32) -> (bool, String) {
        
    }
    
    pub fn rename_worksheet_from_index(sheet_index: i32, to: &str) -> bool {
        
    }

    pub fn rename_worksheet_from_str(from: &str, to: &str) -> bool {
        
    }
    
}

struct Basic_Excel_Worksheet {
    pub excel_: Basic_Excel,
    pub sheet_index: i32,
    pub max_rows_: i32,
    pub max_cols_: i32,
    pub cells_: Vec<Basic_Excel_Cell>,
    pub col_infos_: Worksheet::Col_Infos,
}

impl Basic_Excel_Worksheet {
    
}

struct Compound_File;

struct Workbook;

struct Basic_Excel_Cell;

