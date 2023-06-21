use super::*;

pub fn model16() -> HModel {
    let mut ret = HModel {
        start_addr: 40154,
        end_addr: 40174,
        model_number: 16,
        qtd: 21,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U characteristic curve number of points:", offset: 0, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 1", offset: 1, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 1", offset: 2, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 2", offset: 3, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 2", offset: 4, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 3", offset: 5, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 3", offset: 6, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 4", offset: 7, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 4", offset: 8, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 5", offset: 9, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 5", offset: 10, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 6", offset: 11, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 6", offset: 12, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 7", offset: 13, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 7", offset: 14, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 8", offset: 15, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 8", offset: 16, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 9", offset: 17, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 9", offset: 18, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U curve U/Un value at point 10", offset: 19, length: 1, write_access: true, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Q-U curve Q/S value at point 10", offset: 20, length: 1, write_access: true, value: 0 } ));

    ret
}
