use super::*;

pub fn model6() -> HModel {
    let mut ret = HModel {
        start_addr: 32106,
        end_addr: 32107,
        model_number: 6,
        qtd: 2,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Accumulated energy yield", offset: 0, length: 2, write_access: false, value: 0 } ));
    ret
}
