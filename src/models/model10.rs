use super::*;

pub fn model10() -> HModel {
    let mut ret = HModel {
        start_addr: 40037,
        end_addr: 40037,
        model_number: 10,
        qtd: 1,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Q-U characteristic curve mode", offset: 0, length: 1, write_access: true, value: 0 } ));
    ret
}
