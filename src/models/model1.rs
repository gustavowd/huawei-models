use super::*;

pub fn model1() -> HModel {
    let mut ret = HModel {
        start_addr: 32000,
        end_addr: 32012,
        model_number: 1,
        qtd: 12,
        data: Vec::new(),
    };
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Rated inverter Power", offset: 0, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU16(Point { name: "Output Mode", offset: 1, length: 1, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiString(Point { name: "ESN", offset: 2, length: 10, write_access: false, value: String::new() } ));

    ret
}
