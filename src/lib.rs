//! # Bunka (分化)
//!
//! `bunka` は浮動小数点数値またはパーセント表記の文字列を受け取り、
//! 連分数展開アルゴリズムを用いて高精度な分数（分子 / 分母）に近似変換するためのライブラリおよびツールです。
//!
//! ## 特徴
//! - **連分数近似**: 最大分母の制限や許容誤差のパラメータを調整した分数近似。
//! - **柔軟な入力形式**: `0.142857` のような通常の小数のほか、`10%` のようなパーセント表示の文字列もパース可能。
//! - **直感的なインターフェース**: 数値やパーセント表示を入力して即座に結果を出力するCLIツールを内包。

pub mod cli;

/// 連分数展開アルゴリズムによる分数近似
///
/// 与えられた実数 `value` に最も近い分数 `(分子, 分母, 絶対誤差)` を、分母が `max_denominator` を超えない範囲で算出します。
///
/// # 引数
///
/// * `value` - 近似対象の浮動小数点数。
/// * `max_denominator` - 近似分数に許容される最大の分母値。
/// * `tolerance` - 近似を終了する許容誤差の閾値。
///
/// # 戻り値
///
/// `(i64, u64, f64)` のタプル。それぞれ `(分子, 分母, 近似後の絶対誤差)` を表します。
///
/// # 例
///
/// ```
/// use bunka::approximate_fraction;
/// let (num, den, error) = approximate_fraction(0.142857, 100000, 1e-6);
/// assert_eq!(num, 1);
/// assert_eq!(den, 7);
/// ```
pub fn approximate_fraction(value: f64, max_denominator: u64, tolerance: f64) -> (i64, u64, f64) {
    // 0の場合は 0/1 を即座に返す
    if value == 0.0 {
        return (0, 1, 0.0);
    }

    // 符号を判定し、絶対値で近似計算を進める
    let sign = if value < 0.0 { -1 } else { 1 };
    let target = value.abs();

    // 近似分数 (h_n / k_n) を算出するための漸化式の初期値設定
    // h1 = h_{n-1}, h2 = h_{n-2}
    // k1 = k_{n-1}, k2 = k_{n-2}
    let mut h1 = 1i64;
    let mut h2 = 0i64;
    let mut k1 = 0u64;
    let mut k2 = 1u64;

    // r は連分数展開の残差、a はその整数部分 ( floor(r) )
    let mut r = target;
    let mut a = r.floor() as i64;
    let mut step = 0;

    loop {
        // 漸化式: h_n = a_n * h_{n-1} + h_{n-2}, k_n = a_n * k_{n-1} + k_{n-2}
        let h = a * h1 + h2;
        let k = (a as u64) * k1 + k2;

        // 計算した分母が最大分母制限を超えた場合は、手前の近似値 (h1/k1) を採用するためループ終了
        if k > max_denominator {
            break;
        }

        // 次の計算ステップに向けて状態変数を更新
        h2 = h1;
        h1 = h;
        k2 = k1;
        k1 = k;

        // 現在の近似分数から実数値を算出し、元の値との絶対誤差を計算
        let approx_value = (h1 as f64 / k1 as f64) * (sign as f64);
        let error = (value - approx_value).abs();

        // 誤差が許容しきい値以下、または残差の小数部分がほぼゼロ（1e-11未満）になった場合は終了
        if error <= tolerance || (r - a as f64).abs() < 1e-11 {
            break;
        }

        // 残差の小数部分を取り出し、次の連分数係数を求めるための逆数を計算
        let diff = r - a as f64;
        if diff.abs() < 1e-11 {
            break;
        }
        r = 1.0 / diff;
        a = r.floor() as i64;

        // 異常な無限ループを防止するため、最大50ステップで制限
        step += 1;
        if step > 50 {
            break;
        }
    }

    // 最終的に求まった近似分数と、元の値に対する絶対誤差を返す
    let final_approx = (h1 as f64 / k1 as f64) * (sign as f64);
    (h1 * sign, k1, (value - final_approx).abs())
}

/// 文字列を浮動小数点数としてパースします。
///
/// 末尾に '%' が存在する場合、そのパーセント記号を取り除いて浮動小数点数としてパースし、
/// その値を 100.0 で割った値を返します。
/// 前後の空白、またはパーセント記号の前の空白はトリムされます。
///
/// # 例
///
/// ```
/// use bunka::parse_decimal_or_percent;
/// assert_eq!(parse_decimal_or_percent("10%").unwrap(), 0.1);
/// assert_eq!(parse_decimal_or_percent("0.142857").unwrap(), 0.142857);
/// ```
pub fn parse_decimal_or_percent(s: &str) -> Result<f64, std::num::ParseFloatError> {
    let trimmed = s.trim();
    if let Some(stripped) = trimmed.strip_suffix('%') {
        let val_part = stripped.trim();
        let val: f64 = val_part.parse()?;
        Ok(val / 100.0)
    } else {
        trimmed.parse()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::approx_constant)]
    fn test_approximate_fraction_positive() {
        let (num, den, _) = approximate_fraction(0.142857, 100000, 1e-6);
        assert_eq!(num, 1);
        assert_eq!(den, 7);

        let (num, den, _) = approximate_fraction(0.333333, 100000, 1e-6);
        assert_eq!(num, 1);
        assert_eq!(den, 3);

        let (num, den, _) = approximate_fraction(3.14159265, 100000, 1e-6);
        assert_eq!(num, 355);
        assert_eq!(den, 113);
    }

    #[test]
    fn test_approximate_fraction_zero() {
        let (num, den, _) = approximate_fraction(0.0, 100000, 1e-6);
        assert_eq!(num, 0);
        assert_eq!(den, 1);
    }

    #[test]
    fn test_approximate_fraction_negative() {
        let (num, den, _) = approximate_fraction(-0.5, 100000, 1e-6);
        assert_eq!(num, -1);
        assert_eq!(den, 2);
    }

    #[test]
    fn test_parse_decimal_or_percent() {
        assert_eq!(parse_decimal_or_percent("0.142857").unwrap(), 0.142857);
        assert_eq!(parse_decimal_or_percent("10%").unwrap(), 0.1);
        assert_eq!(parse_decimal_or_percent("  -5.5 % ").unwrap(), -0.055);
        assert!(parse_decimal_or_percent("abc").is_err());
        assert!(parse_decimal_or_percent("10%%").is_err());
    }
}
