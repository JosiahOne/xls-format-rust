// Record

pub struct Record {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
}

pub trait Record_Capabilities {
    fn new() -> Self;
    
    fn read() -> u32 {
        unimplemented!();
    }
    
    fn write() -> u32 {
        unimplemented!();
    }
    
    fn data_size(&self) -> u32 {
        unimplemented!();
    }
    
    fn record_size(&self) -> u32 {
        unimplemented!();
    }
}

impl Record_Capabilities for Record {
    fn new() -> Record {
        return Record{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0]};
    }
}

pub struct BOF {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
    
    version_: u16,
    type_: u16,
    build_identifier_: u16,
    build_year_: u16,
    file_history_flags_: u32,
    lowest_excel_version_: u32,
}

impl Record_Capabilities for BOF {
    fn new() -> BOF {
        return BOF{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0], version_: 0, type_: 0, build_identifier_: 0, build_year_: 0, file_history_flags_: 0, lowest_excel_version_: 0};
    }
}

pub struct YEOF {
    code_: u16,
    data_: String,
    data_size_: u32,
    record_size_: u32,
    continue_indices_: Vec<u32>,
}

impl Record_Capabilities for YEOF {
    fn new() -> YEOF {
        return YEOF{code_: 0, data_: "".to_string(), data_size_: 0, record_size_: 0, continue_indices_: vec![0]};
    }
}