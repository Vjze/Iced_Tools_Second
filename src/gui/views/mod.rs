pub mod bandin_view;
pub mod order_view;
pub mod query_view;
// pub mod other;
pub mod listview;

use std::fmt::{Display, Formatter};

// #[derive(Debug, Clone, Copy, Default)]
// pub enum TipType {
//     #[default]
//     EmptyTip,
//     LoginTip,
// }
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Selected {
    #[default]
    Sn,
    Box,
    Carton,
    Workid,
}

impl Selected {
    pub const ALL: [Selected; 4] = [
        Selected::Sn,
        Selected::Box,
        Selected::Carton,
        Selected::Workid,
    ];
}

impl Display for Selected {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Selected::Sn => "Sn查询",
                Selected::Box => "盒号查询",
                Selected::Carton => "箱号查询",
                Selected::Workid => "工号查询",
            }
        )
    }
}
#[derive(Debug, Clone, Default)]
pub struct SnDataInfo {
    pub sn: String,
    pub product_bill: String,
    pub test_type: String,
    pub result: String,
    pub ith: String,
    pub pf: String,
    pub vop: String,
    pub im: String,
    pub rs: String,
    pub sen: String,
    pub res: String,
    pub icc: String,
    pub idark: String,
    pub vbr: String,
    pub ixtalk: String,
    pub kink: String,
    pub testdate: String,
}
impl SnDataInfo {
    pub fn sn(&self) -> String {
        self.sn.clone()
    }

    pub fn productbill(&self) -> String {
        self.product_bill.clone()
    }

    pub fn testtype(&self) -> String {
        self.test_type.clone()
    }

    pub fn result(&self) -> String {
        self.result.clone()
    }

    pub fn ith(&self) -> String {
        self.ith.clone()
    }
    pub fn pf(&self) -> String {
        self.pf.clone()
    }
    pub fn vop(&self) -> String {
        self.vop.clone()
    }

    pub fn im(&self) -> String {
        self.im.clone()
    }

    pub fn rs(&self) -> String {
        self.rs.clone()
    }

    pub fn sen(&self) -> String {
        self.sen.clone()
    }

    pub fn res(&self) -> String {
        self.res.clone()
    }
    pub fn icc(&self) -> String {
        self.icc.clone()
    }
    pub fn idark(&self) -> String {
        self.idark.clone()
    }

    pub fn vbr(&self) -> String {
        self.vbr.clone()
    }

    pub fn ixtalk(&self) -> String {
        self.ixtalk.clone()
    }
    pub fn kink(&self) -> String {
        self.kink.clone()
    }
    pub fn testdate(&self) -> String {
        self.testdate.clone()
    }
}
#[derive(Debug, Clone, Default)]
pub struct BoxDataInfo {
    pub pno: String,
    pub sn: String,
    pub pn: String,
    pub workorder: String,
    pub creator: String,
    pub createtime: String,
}

impl BoxDataInfo {
    pub fn pno(&self) -> String {
        self.pno.clone()
    }

    pub fn sn(&self) -> String {
        self.sn.clone()
    }

    pub fn pn(&self) -> String {
        self.pn.clone()
    }

    pub fn workorder(&self) -> String {
        self.workorder.clone()
    }

    pub fn creator(&self) -> String {
        self.creator.clone()
    }
    pub fn createtime(&self) -> String {
        self.createtime.clone()
    }
}
