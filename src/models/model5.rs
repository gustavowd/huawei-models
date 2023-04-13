use super::*;

pub fn model5() -> HModel {
    let mut ret = HModel {
        start_addr: 32064,
        end_addr: 32094,
        model_number: 5,
        qtd: 31,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Input power", offset: 0, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Uab", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ubc", offset: 3, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Uac", offset: 4, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ua", offset: 5, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Ub", offset: 6, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Uc", offset: 7, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Ia", offset: 8, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Ib", offset: 10, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Ic", offset: 12, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Active power pcd", offset: 14, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Active Power", offset: 16, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "Reactive Power", offset: 18, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "Power factor", offset: 20, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Grid Frequency", offset: 21, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Inverter efficiency", offset: 22, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI16(Point { name: "Cabinet temperature", offset: 23, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Insulation resistance", offset: 24, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Device status", offset: 25, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Fault code", offset: 26, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Startup time", offset: 27, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Shutdown time", offset: 29, length: 2, write_access: false, value: 0 } ));

    ret
}
