use rand::Rng;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Guid {
    pub data1: u32,
    pub data2: u16,
    pub data3: u16,
    pub data4: [u8; 8],
}

impl Guid {
    /// 생성: 시간 + 랜덤 기반 GUID
    pub fn new() -> Self {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        Guid {
            data1: rng.r#gen(),
            data2: rng.r#gen(),
            data3: rng.r#gen(),
            data4: rng.r#gen(),
        }
    }

    /// 문자열 변환: "XXXXXXXX-XXXX-XXXX-XXXX-XXXXXXXXXXXX"
    pub fn to_string(&self) -> String {
        format!(
            "{:08X}-{:04X}-{:04X}-{:02X}{:02X}-{:02X}{:02X}{:02X}{:02X}{:02X}{:02X}",
            self.data1, self.data2, self.data3,
            self.data4[0], self.data4[1],
            self.data4[2], self.data4[3], self.data4[4], self.data4[5], self.data4[6], self.data4[7]
        )
    }

    /// 문자열 → GUID 변환
    pub fn from_string(s: &str) -> Option<Self> {
        let clean: String = s.chars().filter(|c| c.is_ascii_hexdigit()).collect();
        if clean.len() != 32 { return None; }

        let parse = |i| u8::from_str_radix(&clean[i..i+2], 16).ok();
        Some(Guid {
            data1: u32::from_str_radix(&clean[0..8], 16).ok()?,
            data2: u16::from_str_radix(&clean[8..12], 16).ok()?,
            data3: u16::from_str_radix(&clean[12..16], 16).ok()?,
            data4: [
                parse(16)?, parse(18)?, parse(20)?, parse(22)?,
                parse(24)?, parse(26)?, parse(28)?, parse(30)?
            ],
        })
    }

    /// Null GUID
    pub fn null() -> Self {
        Guid {
            data1: 0,
            data2: 0,
            data3: 0,
            data4: [0; 8],
        }
    }

    /// Null 여부 확인
    pub fn is_null(&self) -> bool {
        *self == Guid::null()
    }
}


