use super::*;

pub fn model7() -> HModel {
    let mut ret = HModel {
        start_addr: 32114,
        end_addr: 32115,
        model_number: 7,
        qtd: 2,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Daily energy yield", offset: 0, length: 2, write_access: false, value: 0 } ));
    ret
}
