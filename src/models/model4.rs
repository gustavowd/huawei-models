use super::*;

pub fn model4() -> HModel {
    let mut ret = HModel {
        start_addr: 32016,
        end_addr: 32027,
        model_number: 4,
        qtd: 12,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV1 Voltage", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV1 Current", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV2 Voltage", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV2 Current", offset: 3, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV3 Voltage", offset: 4, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV3 Current", offset: 5, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV4 Voltage", offset: 6, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV4 Current", offset: 7, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV5 Voltage", offset: 8, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV5 Current", offset: 9, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV6 Voltage", offset: 10, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV6 Current", offset: 11, length: 1, write_access: false, value: 0 } ));

    ret
}
