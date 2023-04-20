use super::*;

pub fn model11() -> HModel {
    let mut ret = HModel {
        start_addr: 40038,
        end_addr: 40038,
        model_number: 11,
        qtd: 1,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U dispatch trigger power (%)", offset: 0, length: 1, write_access: true, value: 0 } ));
    ret
}
