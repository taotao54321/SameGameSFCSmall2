//! スコア関連。

use crate::hint::assert_unchecked;

/// スコア型。
///
/// 理論上の値域は `0..=2409` (最大値は 48 個全消し時)。
pub type Score = u32;

/// パーフェクト達成時に得られるボーナススコア。
pub const SCORE_PERFECT: Score = 200;

/// n 個の駒を消す着手による獲得スコアを返す。
///
/// `n >= 2` でなければならない。
pub const fn calc_score_erase(n: u32) -> Score {
    unsafe { assert_unchecked!(n >= 2) }

    (n - 1).pow(2)
}
