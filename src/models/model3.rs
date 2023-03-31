use super::*;

pub fn model3() -> HModel {
    let mut ret = HModel {
        start_addr: 32261,
        end_addr: 32306,
        model_number: 3,
        qtd: 46,
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
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "Uab", offset: 12, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ubc", offset: 13, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Uac", offset: 14, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ua", offset: 15, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ub", offset: 16, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Uc", offset: 17, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ia", offset: 18, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ib", offset: 19, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ic", offset: 20, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Frequency", offset: 21, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "Power factor", offset: 22, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Inverter efficiency", offset: 23, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "Cabinet temperature", offset: 24, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Inverter status", offset: 25, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Active power pcd", offset: 26, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Active Power", offset: 28, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Reactive Power", offset: 30, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Total input power", offset: 32, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Current electricity yct", offset: 34, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Hour", offset: 36, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Day", offset: 38, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-month", offset: 40, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Year", offset: 42, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Total", offset: 44, length: 2, write_access: false, value: 0 } ));
    ret
}
