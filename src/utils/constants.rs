#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Month {
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December,
}

impl Month {
    pub fn from_zero_based(month: u32) -> Self {
        match month {
            0 => Self::January,
            1 => Self::February,
            2 => Self::March,
            3 => Self::April,
            4 => Self::May,
            5 => Self::June,
            6 => Self::July,
            7 => Self::August,
            8 => Self::September,
            9 => Self::October,
            10 => Self::November,
            11 => Self::December,
            _ => panic!("The month must be between 0 and 11"),
        }
    }

    pub const fn full_name(self) -> &'static str {
        match self {
            Self::January => "gennaio",
            Self::February => "febbraio",
            Self::March => "marzo",
            Self::April => "aprile",
            Self::May => "maggio",
            Self::June => "giugno",
            Self::July => "luglio",
            Self::August => "agosto",
            Self::September => "settembre",
            Self::October => "ottobre",
            Self::November => "novembre",
            Self::December => "dicembre",
        }
    }

    pub const fn short_name(self) -> &'static str {
        match self {
            Self::January => "gen",
            Self::February => "feb",
            Self::March => "mar",
            Self::April => "apr",
            Self::May => "mag",
            Self::June => "giu",
            Self::July => "lug",
            Self::August => "ago",
            Self::September => "set",
            Self::October => "ott",
            Self::November => "nov",
            Self::December => "dic",
        }
    }
}
