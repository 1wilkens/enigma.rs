pub mod des_const;

#[allow(non_camel_case_types)]
pub type DES_Key = u64;

#[allow(non_camel_case_types, dead_code)]
pub struct DES_KeySchedule {
    keys: [DES_Key; 16]
}
