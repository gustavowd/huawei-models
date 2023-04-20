use super::*;

pub fn model13() -> HModel {
    let mut ret = HModel {
        start_addr: 40122,
        end_addr: 40122,
        model_number: 13,
        qtd: 1,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Reactive power compensation (PF)", offset: 0, length: 1, write_access: false, value: 0 } ));

    ret
}
