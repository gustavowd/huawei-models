use super::*;

pub fn model5() -> HModel {
    let mut ret = HModel {
        start_addr: 32319,
        end_addr: 32322,
        model_number: 5,
        qtd: 4,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Locking", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "InvStatus", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Inverter on-grid", offset: 2, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Insulation resistance", offset: 3, length: 1, write_access: false, value: 0 } ));

    ret
}
