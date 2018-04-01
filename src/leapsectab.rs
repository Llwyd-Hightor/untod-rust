extern crate chrono;
use self::chrono::NaiveDate;

use super::todinfo::*;

#[derive(Clone, Debug)]
pub struct LeapSec {
    day:   NaiveDate,
    tod:   u64,
    count: i64,
}

#[derive(Debug, Default)]
pub struct LeapSecTable(Vec<LeapSec,>,);
impl LeapSecTable {
    pub fn new() -> LeapSecTable {
        LeapSecTable(vec![
            LeapSec {
                day:   NaiveDate::from_ymd(2017, 1, 1,),
                tod:   0x000D_1E0D_6817_3CC0,
                count: 27,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(2015, 7, 1,),
                tod:   0x000C_F2D5_4B4F_BA80,
                count: 26,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(2012, 7, 1,),
                tod:   0x000C_9CC9_A704_D840,
                count: 25,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(2009, 1, 1,),
                tod:   0x000C_3870_CB9B_B600,
                count: 24,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(2006, 1, 1,),
                tod:   0x000B_E251_0979_73C0,
                count: 23,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1999, 1, 1,),
                tod:   0x000B_1962_F930_5180,
                count: 22,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1997, 7, 1,),
                tod:   0x000A_EE3E_FA40_2F40,
                count: 21,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1996, 1, 1,),
                tod:   0x000A_C343_36FE_CD00,
                count: 20,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1994, 7, 1,),
                tod:   0x000A_981F_380E_AAC0,
                count: 19,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1993, 7, 1,),
                tod:   0x000A_7B70_ABEB_8880,
                count: 18,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1992, 7, 1,),
                tod:   0x000A_5EC2_1FC8_6640,
                count: 17,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1991, 1, 1,),
                tod:   0x000A_33C6_5C87_0400,
                count: 16,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1990, 1, 1,),
                tod:   0x000A_1717_D063_E1C0,
                count: 15,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1988, 1, 1,),
                tod:   0x0009_DDA6_9A55_7F80,
                count: 14,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1985, 7, 1,),
                tod:   0x0009_95D4_0F51_7D40,
                count: 13,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1983, 7, 1,),
                tod:   0x0009_5C62_D943_1B00,
                count: 12,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1982, 7, 1,),
                tod:   0x0009_3FB4_4D1F_F8C0,
                count: 11,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1981, 7, 1,),
                tod:   0x0009_2305_C0FC_D680,
                count: 10,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1980, 1, 1,),
                tod:   0x0008_F809_FDBB_7440,
                count: 9,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1979, 1, 1,),
                tod:   0x0008_DB5B_7198_5200,
                count: 8,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1978, 1, 1,),
                tod:   0x0008_BEAC_E575_2FC0,
                count: 7,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1977, 1, 1,),
                tod:   0x0008_A1FE_5952_0D80,
                count: 6,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1976, 1, 1,),
                tod:   0x0008_853B_AF57_8B40,
                count: 5,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1975, 1, 1,),
                tod:   0x0008_688D_2334_6900,
                count: 4,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1974, 1, 1,),
                tod:   0x0008_4BDE_9711_46C0,
                count: 3,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1973, 1, 1,),
                tod:   0x0008_2F30_0AEE_2480,
                count: 2,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(1972, 7, 1,),
                tod:   0x0008_20BA_9811_E240,
                count: 1,
            },
            LeapSec {
                day:   NaiveDate::from_ymd(0000, 1, 1,),
                tod:   0x0000_0000_0000_0000,
                count: 0,
            },
        ],)
    }

    pub fn ls_search_day(&self, todwork: &TodInfo,) -> i64 {
        let thedate = todwork.date.date();
        if todwork.utc {
            match self.0.iter().find(|x| x.day <= thedate,) {
                Some(x,) => x.count,
                None => self.0[self.0.len() - 1].count,
            }
        } else {
            0
        }
    }

    pub fn ls_search_tod(&self, todwork: &TodInfo,) -> i64 {
        if todwork.utc {
            match self.0.iter().find(|x| x.tod <= todwork.tod.0,) {
                Some(x,) => x.count,
                None => self.0[0].count,
            }
        } else {
            0
        }
    }
}
