use super::*;

pub fn model15() -> HModel {
    let mut ret = HModel {
        start_addr: 40125,
        end_addr: 40125,
        model_number: 15,
        qtd: 1,
        data: Vec::new(),
    };

    ret.data.push(HDataTypes::HuaweiU16(Point { name: "PGS - Active power percentage derating (0.1%)", offset: 0, length: 1, write_access: false, value: 0 } ));

    ret
}
