use crate::string::define_string;

define_string!(CompartmentName, max = 50, validator = |c: char| !c.is_control());
