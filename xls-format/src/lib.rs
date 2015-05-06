mod excel_format;
mod basic_excel;

#[test]
fn it_works() {

}

#[test]
fn test_basic_excel_creation() {
    let xls = basic_excel::Basic_Excel::new();
    
    xls.new_with_sheets(2);
}

#[test]
fn test_get_worksheet() {
    let xls = basic_excel::Basic_Excel::new();
    
    xls.new_with_sheets(1);
    
    let sheet = xls.get_worksheet(0);
}