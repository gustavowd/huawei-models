use super::*;

pub fn model8() -> HModel {
    let mut ret = HModel {
        start_addr: 35300,
        end_addr: 35307,
        model_number: 8,
        qtd: 8,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Active adjustment mode", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Active adjustment value", offset: 1, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Active adjustment command", offset: 3, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Reactive adjustment mode", offset: 4, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Reactive adjustment value", offset: 5, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Reactive adjustment command", offset: 7, length: 1, write_access: false, value: 0 } ));
    ret
}
