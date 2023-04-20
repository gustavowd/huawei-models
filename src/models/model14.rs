use super::*;

pub fn model14() -> HModel {
    let mut ret = HModel {
        start_addr: 40123,
        end_addr: 40123,
        model_number: 14,
        qtd: 1,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiI16(Point { name: "PGS - Reactive power compensation (Q/S)", offset: 0, length: 1, write_access: true, value: 0 } ));

    ret
}
