use std::io::Write;
use crate::types::*;

pub mod model1;
pub mod model2;
pub mod model3;
pub mod model4;
pub mod model5;
pub mod model6;
pub mod model7;
pub mod model8;
pub mod model9;
pub mod model10;
pub mod model11;
pub mod model12;
pub mod model13;
pub mod model14;
pub mod model15;
pub mod model16;

#[derive(Debug, Clone)]
pub struct HModel {
    pub start_addr: u16,
    pub end_addr: u16,
    pub model_number: u16,
    pub qtd: u16,
    pub data: Vec<HDataTypes>,
}

#[derive(Debug, Clone)]
pub struct HModels {
    pub models: Vec<HModel>,
}

// Declare the struct
pub trait HuaweiModels {
    // This new function acts as a constructor
    fn new (model_number: u16) -> Self;
    fn update_data(&mut self, point: &str, value: &HDataTypes);
    fn update_data_by_index(&mut self, index: usize, value: &HDataTypes);
    fn get_data(&self, point: &str) -> HDataTypes;
    fn get_data_index(&self, point: &str) -> Option<usize>;
    fn get_string(&self, point: &str) -> Option<String>;
    fn get_string_by_index(&self, idx: usize) -> Option<String>;
    fn get_u16(&self, point: &str) -> Option<u16>;
    fn get_u16_by_index(&self, idx: usize) -> Option<u16>;
    fn get_u32(&self, point: &str) -> Option<u32>;
    fn get_u32_by_index(&self, idx: usize) -> Option<u32>;
    fn get_i16(&self, point: &str) -> Option<i16>;
    fn get_i16_by_index(&self, idx: usize) -> Option<i16>;
    fn get_i32(&self, point: &str) -> Option<i32>;
    fn get_i32_by_index(&self, idx: usize) -> Option<i32>;
    fn print(&self);
}


impl HModels {
    pub fn new () -> HModels {
        HModels { models: Vec::new() }
    }

    pub fn get_model_index(&self, model_number: u16) -> Option<usize> {
        let mut idx = 0;
        for model in self.models.iter() {
            if model_number == model.model_number {
                return Some(idx);
            }
            idx += 1;
        }
        None
    }
}

impl HuaweiModels for HModel {
    fn new (model_number: u16) -> HModel {
        match model_number {
            1 => model1::model1(),
            2 => model2::model2(),
            3 => model3::model3(),
            4 => model4::model4(),
            5 => model5::model5(),
            6 => model6::model6(),
            7 => model7::model7(),
            8 => model8::model8(),
            9 => model9::model9(),
            10 => model10::model10(),
            11 => model11::model11(),
            12 => model12::model12(),
            13 => model13::model13(),
            14 => model14::model14(),
            15 => model15::model15(),
            16 => model16::model16(),
            _ => return model1::model1(),
        }
    }

    fn update_data(&mut self, point: &str, value: &HDataTypes) {
        for data_tmp in self.data.iter_mut() {
            match data_tmp {
                HDataTypes::HuaweiString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            HDataTypes::HuaweiString(update_value) =>  data.value = update_value.value.clone(),
                            _ => {},
                        };
                    }
                },
                HDataTypes::HuaweiU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            HDataTypes::HuaweiU16(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                HDataTypes::HuaweiU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            HDataTypes::HuaweiU32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                HDataTypes::HuaweiI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            HDataTypes::HuaweiI16(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
                HDataTypes::HuaweiI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()){
                        match value {
                            HDataTypes::HuaweiI32(update_value) =>  data.value = update_value.value,
                            _ => {},
                        };
                    }
                },
            }
        }
    }

    fn update_data_by_index(&mut self, index: usize, value: &HDataTypes) {
        match &mut self.data[index] {
            HDataTypes::HuaweiString(data) => {
                match value {
                    HDataTypes::HuaweiString(update_value) =>  data.value = update_value.value.clone(),
                    _ => {},
                };
            },
            HDataTypes::HuaweiU16(data) => {
                match value {
                    HDataTypes::HuaweiU16(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            HDataTypes::HuaweiU32(data) => {
                match value {
                    HDataTypes::HuaweiU32(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            HDataTypes::HuaweiI16(data) => {
                match value {
                    HDataTypes::HuaweiI16(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
            HDataTypes::HuaweiI32(data) => {
                match value {
                    HDataTypes::HuaweiI32(update_value) =>  data.value = update_value.value,
                    _ => {},
                };
            },
        }
    }

    fn get_data(&self, point: &str) -> HDataTypes {
        for data_tmp in self.data.iter() {
            match data_tmp {
                HDataTypes::HuaweiString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                HDataTypes::HuaweiU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                HDataTypes::HuaweiU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                HDataTypes::HuaweiI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
                HDataTypes::HuaweiI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return data_tmp.clone();
                    }
                },
            };
        }
        return HDataTypes::HuaweiU16(Point { name: "", offset: 0, length: 1, write_access: false, value: 0 } )
    }

    fn get_data_index(&self, point: &str) -> Option<usize> {
        let mut idx = 0;
        for data_tmp in self.data.iter() {
            match data_tmp {
                HDataTypes::HuaweiString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                HDataTypes::HuaweiU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                HDataTypes::HuaweiU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                HDataTypes::HuaweiI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
                HDataTypes::HuaweiI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(idx);
                    }
                },
            };
            idx += 1;
        }
        return None;
    }

    fn get_string(&self, point: &str) -> Option<String> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                HDataTypes::HuaweiString(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value.clone());
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_string_by_index(&self, idx: usize) -> Option<String> {
        match &self.data[idx] {
            HDataTypes::HuaweiString(data) => {
                return Some(data.value.clone());
            },
            _ => return None,
        }
    }

    fn get_u16(&self, point: &str) -> Option<u16> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                HDataTypes::HuaweiU16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {}
            }
        }
        return None
    }

    fn get_u16_by_index(&self, idx: usize) -> Option<u16> {
        match self.data[idx] {
            HDataTypes::HuaweiU16(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_u32(&self, point: &str) -> Option<u32> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                HDataTypes::HuaweiU32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_u32_by_index(&self, idx: usize) -> Option<u32> {
        match self.data[idx] {
            HDataTypes::HuaweiU32(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_i16(&self, point: &str) -> Option<i16> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                HDataTypes::HuaweiI16(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_i16_by_index(&self, idx: usize) -> Option<i16> {
        match self.data[idx] {
            HDataTypes::HuaweiI16(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn get_i32(&self, point: &str) -> Option<i32> {
        for data_tmp in self.data.iter() {
            match data_tmp {
                HDataTypes::HuaweiI32(data) => {
                    if data.name.contains(point) && (data.name.len() == point.len()) {
                        return Some(data.value);
                    }
                },
                _ => {},
            }
        }
        return None
    }

    fn get_i32_by_index(&self, idx: usize) -> Option<i32> {
        match self.data[idx] {
            HDataTypes::HuaweiI32(data) => {
                return Some(data.value);
            },
            _ => return None,
        }
    }

    fn print(&self) {
        println!("Model {}:", self.model_number);
        for data in self.data.iter() {
            match data {
                HDataTypes::HuaweiI16(data) => println!("{}: {}", data.name, data.value),
                HDataTypes::HuaweiI32(data) => println!("{}: {}", data.name, data.value),
                HDataTypes::HuaweiU16(data) => println!("{}: {}", data.name, data.value),
                HDataTypes::HuaweiU32(data) => println!("{}: {}", data.name, data.value),
                HDataTypes::HuaweiString(data) => println!("{}: {}", data.name, data.value.clone()),
            }
        }
        println!(" ");
    }
}

pub fn srt_to_vec_u8(src: &str, mut dst: &mut [u8]){
    dst.write(src.as_bytes()).unwrap();
}

impl From<HModel> for Vec<u16> {
    fn from(from: HModel) -> Self {
        let mut registers: Vec<u16> = Vec::new();

        for data in from.data.iter() {
            match data {
                HDataTypes::HuaweiU16(data) => registers.extend(u16::encode(data.value)),
                HDataTypes::HuaweiU32(data) => registers.extend(u32::encode(data.value)),
                HDataTypes::HuaweiI16(data) => registers.extend(i16::encode(data.value)),
                HDataTypes::HuaweiI32(data) => registers.extend(i32::encode(data.value)),
                HDataTypes::HuaweiString(data) => registers.extend(Point::<String>::encode(data.clone())),
            }
        }
        registers
    }
}

impl From<(Vec<u16>, u16, u16, &HModel)> for HModel {
    fn from(from: (Vec<u16>, u16, u16, &HModel)) -> Self {
        let mut model1 = from.3.clone();
        let mut offset = from.1;
        let mut qtd = from.2;

        while qtd > 0 {
            for data in model1.data.iter_mut() {
                match data {
                    HDataTypes::HuaweiString(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = Point::<String>::decode(slice).value;
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    HDataTypes::HuaweiU16(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u16::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    HDataTypes::HuaweiU32(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = u32::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    HDataTypes::HuaweiI16(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = i16::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                    HDataTypes::HuaweiI32(data) => {
                        if offset == data.offset {
                            let slice = from.0[data.offset as usize..(data.offset + data.length) as usize].to_vec();
                            data.value = i32::decode(slice);
                            offset += data.length;
                            qtd -= data.length;
                        }
                    },
                }
            }
        }
        model1
    }
}
