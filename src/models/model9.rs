use super::*;

pub fn model9() -> HModel {
    let mut ret = HModel {
        start_addr: 32000,
        end_addr: 32003,
        model_number: 9,
        qtd: 3,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiString(Point { name: "State 1", offset: 0, length: 1, write_access: false, value: String::new() } ));
    ret.data.push(HDataTypes::HuaweiString(Point { name: "State 2", offset: 1, length: 1, write_access: false, value: String::new() } ));
    ret.data.push(HDataTypes::HuaweiString(Point { name: "State 3", offset: 2, length: 2, write_access: false, value: String::new() } ));

    ret
}
