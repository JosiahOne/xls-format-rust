use super::Record::*;

// Workbook.

pub struct Workbook {
    pub bof_: BOF,
    pub window_: Window,
    pub fonts_: Vec<Font>,
    pub xfs_: Vec<XF>,
    pub styles_: Vec<Style>,
    pub formats_: Vec<Format>,
    pub bound_sheets_: Vec<Bound_Sheet>,
    pub sst_: Shared_String_Table,
    pub eof_: YEOF,
}

impl Workbook {
    pub fn new() -> Workbook {
        unimplemented!();
    }
    
    pub fn Read() {
        unimplemented!();
    }
    
    pub fn Write() {
        unimplemented!();
    }
    
    pub fn Data_Size() -> u32 {
        unimplemented!();
    }
    
    pub fn Record_Size() -> u32 {
        unimplemented!();
    }
}

struct File_Protection;
struct Code_Page;
struct DSF;
struct Tab_ID;
struct FnGroupCount;
struct WorkbookProtection;

pub struct Window {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32
    continue_indices_: Vec<u32>,
    
    horizontal_pos: u16,
    vertical_pos: u16,
    width_: u16,
    height_: u16,
    options_: u16,
    active_worksheet_index_: u16,
    first_visble_tab_index_: u16,
    selected_worksheet_no_: u16,
    worksheet_tabbar_width_: u16,
}

impl Record_Capabilities for Window {
    fn new() -> Window {
        return Window{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], horizontal_pos: 0, vertical_pos: 0, width_: 0, height_: 0, options_: 0, active_worksheet_index_: 0, first_visble_tab_index_: 0, selected_worksheet_no_: 0, worksheet_tabbar_width_: 0};
    }
}

struct Backup;
struct Hide_Obj;
struct Date_Mode;
struct Precision;
struct Refresh_All;
struct Book_Bool;

pub struct Font {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    height_: u16,
    options_: u16,
    color_index: u16, // Colo*U*r people just need to deal. :]
    weight_: u16,
    escapement_type: u16,
    underline_type: u8,
    family_: u8,
    character_set_: u8,
    unused_: u8,
    name_: String,
}

impl Record_Capabilities for Font {
    fn new() -> Font {
        return Font{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], height_: 0, options_: 0, color_index: 0, weight_: 0, escapement_type: 0, underline_type: 0, family_: 0, character_set_: 0, unused_: 0, name_: "".to_string()};
    }
}

pub struct Format {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    index_: u16,
    fmtstring_: String,
}

impl Record_Capabilities for Format {
    fn new() -> Format {
        return Format{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], index_: 0, fmtstring_: "".to_string()};
    }
}

pub struct XF {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    font_record_index_: u16,
    format_record_index_: u16,
    protection_type_: u16,
    alignment_: u8,
    rotation_: u8,
    text_properties_: u8,
    used_attributes_: u8,
    border_lines_: u32,
    color1_: u32,
    color2_: u16,
}

impl Record_Capabilities for XF {
    fn new() -> XF {
        return XF{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], font_record_index_: 0, format_record_index_: 0, protection_type_: 0, alignment_: 0, rotation_: 0, text_properties_: 0, used_attributes_: 0, border_lines_: 0, color1_: 0, color2_: 0};
    }
}

pub struct Style {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    xf_record_index_: u16,
    identifier_: u8,
    level_: u8,
    name_: String,
}

impl Record_Capabilities for Style {
    fn new() -> Style {
        return Style{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], xf_record_index_: 0, identifier_: 0, level_: 0, name_: "".to_string()};
    }
}

struct Palette;
struct Use_Selfs;

pub struct Bound_Sheet {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    bof_pos_: u32,
    visibility_: u8,
    type_: u8,
    name_: String,
}

impl Record_Capabilities for Bound_Sheet {
    fn new() -> Bound_Sheet {
        return Bound_Sheet{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], bof_pos_: 0, visibility_: 0, type_: 0, name_: "".to_string()};
    }
}

struct Country;
struct Link_Table;

pub struct Shared_String_Table {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    strings_total_: u32,
    unique_strings_total_: u32,
    strings_: Vec<String>,
}

impl Record_Capabilities for Shared_String_Table {
    fn new() -> Shared_String_Table {
        return Shared_String_Table{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], strings_total_: 0, unique_strings_total_: 0, strings_: vec!["".to_string()]};
    }
}

struct Ext_SST {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    strings_total_: u16,
    stream_pos_: Vec<u32>,
    first_string_pos_: Vec<u16>,
    unused_: Vec<u16>,
}

impl Record_Capabilities for Ext_SST {
    fn new() -> Ext_SST {
        return Ext_SST{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], strings_total_: 0, stream_pos_: vec![0], first_string_pos_: vec![0], unused_: vec![0]};
    }
}

