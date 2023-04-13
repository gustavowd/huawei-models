use super::*;

pub fn model6() -> HModel {
    let mut ret = HModel {
        start_addr: 32064,
        end_addr: 32094,
        model_number: 6,
        qtd: 31,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Current electricity yct", offset: 34, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Hour", offset: 36, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Day", offset: 38, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-month", offset: 40, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Year", offset: 42, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "E-Total", offset: 44, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Locking", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "InvStatus", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Inverter on-grid", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Insulation resistance", offset: 3, length: 1, write_access: false, value: 0 } ));

    ret
}
