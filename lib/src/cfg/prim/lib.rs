use self::bit::BitLib;
use self::byte::ByteLib;
use self::call::CallLib;
use self::cell::CellLib;
use self::cfg::CfgLib;
use self::ctrl::CtrlLib;
use self::ctx::CtxLib;
use self::decimal::DecimalLib;
use self::error::ErrorLib;
use self::fact::FactLib;
use self::func::FuncLib;
use self::int::IntLib;
use self::key::KeyLib;
use self::lang::LangLib;
use self::link::LinkLib;
use self::list::ListLib;
use self::map::MapLib;
use self::pair::PairLib;
use self::quote::QuoteLib;
use self::solve::SolveLib;
use self::text::TextLib;
use self::unit::UnitLib;
use self::value::ValueLib;
use crate::cfg::CfgMod;
use crate::semantics::val::Val;
use crate::type_::Key;
use crate::type_::Map;

#[derive(Default, Copy, Clone)]
pub struct BasePrimLib {
    pub unit: UnitLib,
    pub bit: BitLib,
    pub key: KeyLib,
    pub text: TextLib,
    pub int: IntLib,
    pub decimal: DecimalLib,
    pub byte: ByteLib,
    pub cell: CellLib,
    pub pair: PairLib,
    pub list: ListLib,
    pub map: MapLib,
    pub quote: QuoteLib,
    pub call: CallLib,
    pub solve: SolveLib,
    pub fact: FactLib,
    pub link: LinkLib,
    pub cfg: CfgLib,
    pub func: FuncLib,
    pub ctx: CtxLib,
    pub ctrl: CtrlLib,
    pub value: ValueLib,
    pub error: ErrorLib,
    pub lang: LangLib,
}

impl CfgMod for BasePrimLib {
    fn export(self, cfg: &mut Map<Key, Val>) {
        self.unit.export(cfg);
        self.bit.export(cfg);
        self.key.export(cfg);
        self.text.export(cfg);
        self.int.export(cfg);
        self.decimal.export(cfg);
        self.byte.export(cfg);
        self.cell.export(cfg);
        self.pair.export(cfg);
        self.list.export(cfg);
        self.map.export(cfg);
        self.quote.export(cfg);
        self.call.export(cfg);
        self.solve.export(cfg);
        self.fact.export(cfg);
        self.link.export(cfg);
        self.cfg.export(cfg);
        self.func.export(cfg);
        self.ctx.export(cfg);
        self.ctrl.export(cfg);
        self.value.export(cfg);
        self.error.export(cfg);
        self.lang.export(cfg);
    }
}

pub mod unit;

pub mod bit;

pub mod key;

pub mod text;

pub mod int;

pub mod decimal;

pub mod byte;

pub mod cell;

pub mod pair;

pub mod list;

pub mod map;

pub mod quote;

pub mod call;

pub mod solve;

pub mod fact;

pub mod link;

pub mod cfg;

pub mod func;

// -----

pub mod ctx;

pub mod ctrl;

pub mod value;

pub mod error;

pub mod lang;
