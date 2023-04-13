use super::*;

pub fn model3() -> HModel {
    let mut ret = HModel {
        start_addr: 32008,
        end_addr: 32010,
        model_number: 3,
        qtd: 3,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Alarm 1", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Alarm 2", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Alarm 3", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret
}
