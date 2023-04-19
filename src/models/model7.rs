use super::*;

pub fn model7() -> HModel {
    let mut ret = HModel {
        start_addr: 32114,
        end_addr: 32119,
        model_number: 7,
        qtd: 6,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Daily energy yield", offset: 0, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Month energy yield", offset: 2, length: 2, write_access: false, value: 0 } ));
    ret.data.push(HDataTypes::HuaweiU32(Point { name: "Year energy yield", offset: 4, length: 2, write_access: false, value: 0 } ));
    ret
}
