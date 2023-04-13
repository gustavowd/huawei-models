use super::*;

pub fn model2() -> HModel {
    let mut ret = HModel {
        start_addr: 30070,
        end_addr: 30082,
        model_number: 2,
        qtd: 13,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Model ID", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Number of PV strings", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Number of MPPT trackers", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Rated Power", offset: 3, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Maximum active Power", offset: 5, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Maximum apparent Power", offset: 7, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "QMax Fed", offset: 9, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiI32(Point { name: "QMax Absorved", offset: 11, length: 2, write_access: false, value: 0 } ));

    ret
}
