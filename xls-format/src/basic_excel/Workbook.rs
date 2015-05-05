// Workbook.
struct File_Protection;
struct Code_Page;
struct DSF;
struct Tab_ID;
struct FnGroupCount;
struct WorkbookProtection;

struct Window {
    horizontal_pos: u16,
    vertical_pos: u16,
    width_: u16,
    height_: u16,
    options_: u16,
    active_worksheet_index_: u16,
    first_viisble_tab_index_: u16,
    selected_worksheet_no_: u16,
    worksheet_tabbar_width_: u16,
}

impl Window {
    pub fn new() -> Window {
        unimplemented!();
    }
    
    pub fn Read() {
        unimplemented!();
    }
    
    pub fn Write() {
        unimplemented!();
    }
}

struct Backup;
struct Hide_Obj;
struct Date_Mode;
struct Precision;
struct Refresh_All;
struct Book_Bool;

struct Font {
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

impl Font {
    pub fn new() -> Font {
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

struct Format {
    index_: u16,
    fmtstring_: String,
}

impl Format {
    
    pub fn new() -> Format {
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

struct XF {
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

impl XF {
    pub fn new() -> XF {
        unimplemented!();
    }
    
    pub fn Read() {
        unimplemented!();
    }
    
    pub fn Write() {
        unimplemented!();
    }
}

struct Style {
    xf_record_index_: u16,
    identifier_: u8,
    level_: u8,
    name_: String,
}

impl Style {
    pub fn new() -> Style {
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

struct Palette;
struct Use_Selfs;

struct Bound_Sheet {
    bof_pos_: u32,
    visibility_: u8,
    type_: u8,
    name_: String,
}

impl Bound_Sheet {
    pub fn new() -> Bound_Sheet {
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

struct Country;
struct Link_Table;

struct Shared_String_Table {
    strings_total_: u32,
    unique_strings_Total_: u32,
    strings_: Vec<String>,
}

impl Shared_String_Table {
    pub fn new() -> Shared_String_Table {
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

struct Ext_SST {
    strings_total_: u16,
    stream_pos_: Vec<u32>,
    first_string_pos_: Vec<u16>,
    unused_: Vec<u16>,
}

impl Ext_SST {
    pub fn new() -> Ext_SST {
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

pub struct Workbook {
    bof_: BOF,
    window_: Window,
    fonts_: Vec<Font>,
    xfs_: Vec<XF>,
    styles_: Vec<Style>,
    formats_: Vec<Format>,
    bound_sheets_: Vec<Bound_Sheet>,
    sst_: Shared_String_Table,
    eof_: YEOF,
}

pub impl Workbook {
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