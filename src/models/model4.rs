use super::*;

pub fn model4() -> HModel {
    let mut ret = HModel {
        start_addr: 32016,
        end_addr: 32039,
        model_number: 4,
        qtd: 24,
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
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV7 Voltage", offset: 12, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV7 Current", offset: 13, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV8 Voltage", offset: 14, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV8 Current", offset: 15, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV9 Voltage", offset: 16, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV9 Current", offset: 17, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV10 Voltage", offset: 18, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV10 Current", offset: 19, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV11 Voltage", offset: 20, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV11 Current", offset: 21, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV12 Voltage", offset: 22, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PV12 Current", offset: 23, length: 1, write_access: false, value: 0 } ));

    ret
}
