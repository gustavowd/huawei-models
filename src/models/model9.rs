use super::*;

pub fn model9() -> HModel {
    let mut ret = HModel {
        start_addr: 32000,
        end_addr: 32004,
        model_number: 9,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "State 1", offset: 0, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PAD", offset: 1, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "State 2", offset: 2, length: 1, write_access: false, value: 0xFFFF } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "State 3", offset: 3, length: 2, write_access: false, value: 0xFFFFFFFF } ));

    ret
}
