// Basic excel

struct Basic_Excel() {
    pub file_: Compound_File,                  // Compound file handler.
    pub workbook_: Workbook,                   // Raw Workbook.
    pub worksheets_: vec<Worksheet>,           // Raw Worksheets.
    pub yesheets_: Vec<Basic_Excel_Worksheet>, // Parsed Worksheets.
}

impl Basic_Excel() {
    // File functions
    pub fn new(sheets: i32) {
        
    }
    
    pub fn load(filename: &str) -> bool {
        
    }
    
    pub fn load(filename: String) -> bool {
        
    }
    
    pub fn save() -> bool {
        
    }
    
    pub fn save_as(filename: &str) -> bool {
        
    }
    
    pub fn save_as(filename: String) -> bool {
        
    }
    
    pub fn close() {
        
    }
    
    // Worksheet functions
    pub fn get_total_worksheets() -> i32 {
        
    }
    
    pub fn get_worksheet(sheet_index: i32) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn get_worksheet(name: &str) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn get_worksheet(name: String) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn add_worksheet(sheet_index: i32) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn add_worksheet(name: &str) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn add_worksheet(name: String) -> Basic_Excel_Worksheet {
        
    }
    
    pub fn delete_worksheet(sheet_index: i32) -> bool {
        
    }
    
    pub fn delete_worksheet(name: &str) -> bool {
        
    }
    
    pub fn delete_worksheet(name: String) -> bool {
        
    }
    
    pub fn get_ansi_sheet_name(sheet_index: i32) {
        panic!("Rust operates completely in UTF-8 Unicode strings and as such there's no reason return an ANSI version.");
    }
    
    pub fn get_unicode_sheet_name(sheet_index: i32) -> String {
        
    }
    
    pub fn get_sheet_name(sheet_index: i32) -> (bool, String) {
        
    }
    
    pub fn rename_worksheet(sheet_index: i32, to: &str) -> bool {
        
    }

    pub fn rename_worksheet(from: &str, to: &str) -> bool {
        
    }
    
}