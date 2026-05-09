use airlang::VERSION_MAJOR;
use airlang::VERSION_MINOR;
use airlang::VERSION_PATCH;
use airlang::bug;
use airlang::cfg::CfgMod;
use airlang::cfg::extend_func;
use airlang::semantics::cfg::Cfg;
use airlang::semantics::core::PREFIX_CELL;
use airlang::semantics::func::CtxFreeFunc;
use airlang::semantics::val::PrimFuncVal;
use airlang::semantics::val::Val;
use airlang::type_::Bit;
use airlang::type_::Int;
use const_format::concatcp;

#[derive(Copy, Clone)]
pub struct LangLib {
    pub check_version: PrimFuncVal,
}

const DEV_LANGUAGE: &str = "dev.language";

pub const CHECK_VERSION: &str = concatcp!(PREFIX_CELL, DEV_LANGUAGE, ".check_version");

impl Default for LangLib {
    fn default() -> Self {
        Self { check_version: CtxFreeFunc { fn_: check_version }.build() }
    }
}

impl CfgMod for LangLib {
    fn extend(self, cfg: &mut Cfg) {
        extend_func(cfg, CHECK_VERSION, self.check_version);
    }
}

fn check_version(cfg: &mut Cfg, input: Val) -> Val {
    let Val::Pair(version) = input else {
        return bug!(cfg, "{CHECK_VERSION}: expect input to be a pair, but got {input}");
    };
    let Val::Pair(minor_patch) = &version.right else {
        return bug!(cfg, "{CHECK_VERSION}: expect input.right to be a pair, but got {}", &version.right);
    };
    let major = &version.left;
    let minor = &minor_patch.left;
    let patch = &minor_patch.right;
    let Val::Int(major) = major else {
        return bug!(cfg, "{CHECK_VERSION}: expect input.left to be an integer, but got {major}");
    };
    let Val::Int(minor) = minor else {
        return bug!(cfg, "{CHECK_VERSION}: expect input.right.left to be an integer, but got {minor}");
    };
    let Val::Int(patch) = patch else {
        return bug!(cfg, "{CHECK_VERSION}: expect input.right.right to be an integer, but got {patch}");
    };

    let crate_major = Int::from_str_radix(VERSION_MAJOR, 10).unwrap();
    let crate_minor = Int::from_str_radix(VERSION_MINOR, 10).unwrap();
    let crate_patch = Int::from_str_radix(VERSION_PATCH, 10).unwrap();

    let major_match = **major == crate_major;
    let minor_match = **minor == crate_minor;
    let patch_match = **patch == crate_patch;

    Val::Bit(Bit::from(major_match && minor_match && patch_match))
}
