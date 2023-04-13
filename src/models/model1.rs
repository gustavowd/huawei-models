use super::*;

pub fn model1() -> HModel {
    let mut ret = HModel {
        start_addr: 30000,
        end_addr: 30034,
        model_number: 1,
        qtd: 35,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiString(Point { name: "Model", offset: 0, length: 15, write_access: false, value: String::new() } ));
    ret.data.push(HDataTypes::HuaweiString(Point { name: "SN", offset: 15, length: 10, write_access: false, value: String::new() } ));
    ret.data.push(HDataTypes::HuaweiString(Point { name: "PN", offset: 25, length: 10, write_access: false, value: String::new() } ));

    ret
}
