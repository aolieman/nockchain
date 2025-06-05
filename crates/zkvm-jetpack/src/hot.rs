use either::Either::*;
use nockvm::jets::Jet;
use nockvm::jets::hot::{HotEntry, K_138};
use nockvm::jets::names::JET_NAME_MAP;

use crate::jets::base_jets::*;
use crate::jets::bp_jets::*;
use crate::jets::cheetah_jets::*;
use crate::jets::crypto_jets::*;
use crate::jets::fext_jets::*;
use crate::jets::mary_jets::*;
use crate::jets::tip5_jets::*;
use crate::jets::verifier_jets::*;
use crate::jets::mega_jets::*;

pub fn produce_prover_hot_state() -> Vec<HotEntry> {
    index_zkvm_rsjet_names();
    let mut jets: Vec<HotEntry> = Vec::new();
    jets.extend(BASE_FIELD_JETS);
    jets.extend(BASE_POLY_JETS);
    jets.extend(CURVE_JETS);
    jets.extend(ZTD_JETS);
    jets.extend(KEYGEN_JETS);
    jets.extend(XTRA_JETS);
    jets.extend(EXTENSION_FIELD_JETS);

    jets
}

pub const XTRA_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"weld"),
        ],
        1,
        mary_weld_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"swag"),
        ],
        1,
        mary_swag_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"misc-lib"),
            Left(b"proof-lib"),
            Left(b"utils"),
            Left(b"fri"),
            Left(b"table-lib"),
            Left(b"stark-core"),
            Left(b"fock-core"),
            Left(b"pow"),
            Left(b"stark-engine"),
            Left(b"stark-verifier"),
            Left(b"evaluate-deep"),
        ],
        1,
        evaluate_deep_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ave"),
            Left(b"transpose"),
        ],
        1,
        mary_transpose_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"mp-to-mega"),
            Left(b"mpeval"),
        ],
        1,
        mpeval_jet,
    ),
];

pub const EXTENSION_FIELD_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-shift"),
        ],
        1,
        bp_shift_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-coseword"),
        ],
        1,
        bp_coseword_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fadd"),
        ],
        1,
        fadd_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fsub"),
        ],
        1,
        fsub_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fneg"),
        ],
        1,
        fneg_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fmul"),
        ],
        1,
        fmul_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"finv"),
        ],
        1,
        finv_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fdiv"),
        ],
        1,
        fdiv_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"fpow"),
        ],
        1,
        fpow_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"mp-substitute-mega"),
        ],
        1,
        mp_substitute_mega_jet,
    ),
];

pub const BASE_FIELD_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"badd"),
        ],
        1,
        badd_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bsub"),
        ],
        1,
        bsub_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bneg"),
        ],
        1,
        bneg_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bmul"),
        ],
        1,
        bmul_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ordered-root"),
        ],
        1,
        ordered_root_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpow"),
        ],
        1,
        bpow_jet,
    ),
];

pub const BASE_POLY_JETS: &[HotEntry] = &[
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpoly-to-list"),
        ],
        1,
        bpoly_to_list_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpadd"),
        ],
        1,
        bpadd_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpneg"),
        ],
        1,
        bpneg_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpsub"),
        ],
        1,
        bpsub_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpscal"),
        ],
        1,
        bpscal_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bpmul"),
        ],
        1,
        bpmul_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"bp-hadamard"),
        ],
        1,
        bp_hadamard_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-ntt"),
        ],
        1,
        bp_ntt_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-fft"),
        ],
        1,
        bp_fft_jet,
    ),
    (
        &[
            K_138,
            Left(b"one"),
            Left(b"two"),
            Left(b"tri"),
            Left(b"qua"),
            Left(b"pen"),
            Left(b"zeke"),
            Left(b"ext-field"),
            Left(b"bp-ifft"),
        ],
        1,
        bp_ifft_jet,
    ),
    // (
    //     &[
    //         K_138,
    //         Left(b"one"),
    //         Left(b"two"),
    //         Left(b"tri"),
    //         Left(b"qua"),
    //         Left(b"pen"),
    //         Left(b"zeke"),
    //         Left(b"init-bpoly"),
    //     ],
    //     1,
    //     init_bpoly_jet,
    // ),
];

pub const ZTD_JETS: &[HotEntry] = &[(
    &[
        K_138,
        Left(b"one"),
        Left(b"two"),
        Left(b"tri"),
        Left(b"qua"),
        Left(b"pen"),
        Left(b"zeke"),
        Left(b"ext-field"),
        Left(b"misc-lib"),
        Left(b"tip5-lib"),
        Left(b"permutation"),
    ],
    1,
    permutation_jet,
)];

pub const KEYGEN_JETS: &[HotEntry] = &[(
    &[
        K_138,
        Left(b"one"),
        Left(b"two"),
        Left(b"tri"),
        Left(b"qua"),
        Left(b"pen"),
        Left(b"zeke"),
        Left(b"ext-field"),
        Left(b"misc-lib"),
        Left(b"proof-lib"),
        Left(b"utils"),
        Left(b"fri"),
        Left(b"table-lib"),
        Left(b"stark-core"),
        Left(b"fock-core"),
        Left(b"pow"),
        Left(b"stark-engine"),
        Left(b"zose"),
        Left(b"argon"),
        Left(b"argon2"),
    ],
    1,
    argon2_jet,
)];

pub const CURVE_JETS: &[HotEntry] = &[(
    &[
        K_138,
        Left(b"one"),
        Left(b"two"),
        Left(b"tri"),
        Left(b"qua"),
        Left(b"pen"),
        Left(b"zeke"),
        Left(b"ext-field"),
        Left(b"misc-lib"),
        Left(b"cheetah"),
        Left(b"curve"),
        Left(b"affine"),
        Left(b"ch-scal"),
    ],
    1,
    ch_scal_jet,
)];


pub fn index_zkvm_rsjet_names() {
    let mut m = JET_NAME_MAP.lock().expect("Failed to lock JET_NAME_MAP");
    // XTRA_JETS
    m.insert(mary_weld_jet as Jet, "mary_weld_jet");
    m.insert(mary_swag_jet as Jet, "mary_swag_jet");
    m.insert(evaluate_deep_jet as Jet, "evaluate_deep_jet");
    m.insert(mary_transpose_jet as Jet, "mary_transpose_jet");
    m.insert(mpeval_jet as Jet, "mpeval_jet");
    // EXTENSION_FIELD_JETS
    m.insert(bp_shift_jet as Jet, "bp_shift_jet");
    m.insert(bp_coseword_jet as Jet, "bp_coseword_jet");
    m.insert(fadd_jet as Jet, "fadd_jet");
    m.insert(fsub_jet as Jet, "fsub_jet");
    m.insert(fneg_jet as Jet, "fneg_jet");
    m.insert(fmul_jet as Jet, "fmul_jet");
    m.insert(finv_jet as Jet, "finv_jet");
    m.insert(fdiv_jet as Jet, "fdiv_jet");
    m.insert(fpow_jet as Jet, "fpow_jet");
    m.insert(mp_substitute_mega_jet as Jet, "mp_substitute_mega_jet");
    // BASE_FIELD_JETS
    m.insert(badd_jet as Jet, "badd_jet");
    m.insert(bsub_jet as Jet, "bsub_jet");
    m.insert(bneg_jet as Jet, "bneg_jet");
    m.insert(bmul_jet as Jet, "bmul_jet");
    m.insert(ordered_root_jet as Jet, "ordered_root_jet");
    m.insert(bpow_jet as Jet, "bpow_jet");
    // BASE_POLY_JETS
    m.insert(bpoly_to_list_jet as Jet, "bpoly_to_list_jet");
    m.insert(bpadd_jet as Jet, "bpadd_jet");
    m.insert(bpneg_jet as Jet, "bpneg_jet");
    m.insert(bpsub_jet as Jet, "bpsub_jet");
    m.insert(bpscal_jet as Jet, "bpscal_jet");
    m.insert(bpmul_jet as Jet, "bpmul_jet");
    m.insert(bp_hadamard_jet as Jet, "bp_hadamard_jet");
    m.insert(bp_ntt_jet as Jet, "bp_ntt_jet");
    m.insert(bp_fft_jet as Jet, "bp_fft_jet");
    m.insert(bp_ifft_jet as Jet, "bp_ifft_jet");
    m.insert(init_bpoly_jet as Jet, "init_bpoly_jet");
    // ZTD_JETS
    m.insert(permutation_jet as Jet, "permutation_jet");
    // KEYGEN_JETS
    m.insert(argon2_jet as Jet, "argon2_jet");
    // CURVE_JETS
    m.insert(ch_scal_jet as Jet, "ch_scal_jet");
    // MISSING JETS
}
