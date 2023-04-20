use super::*;

pub fn model12() -> HModel {
    let mut ret = HModel {
        start_addr: 40120,
        end_addr: 40120,
        model_number: 12,
        qtd: 1,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Fixed active power derated", offset: 0, length: 1, write_access: true, value: 0 } ));

    ret
}
