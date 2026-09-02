//! コマンドライン（CLI）ユーザーインターフェースと引数解析を提供するモジュール。

use crate::approximate_fraction;
use std::env;
use std::process;

/// 解析されたCLIオプション
#[derive(Debug, PartialEq)]
pub struct CliOptions {
    /// 変換対象の数値またはパーセント表示文字列
    pub input_str: String,
    /// 最大分母制限
    pub max_den: u64,
    /// 許容誤差
    pub tolerance: f64,
    /// ヘルプメッセージ表示フラグ
    pub is_help: bool,
    /// バージョン表示フラグ
    pub is_version: bool,
}

/// コマンドライン引数の配列をパースします。
pub fn parse_cli_args<I, T>(args: I) -> Result<CliOptions, String>
where
    I: IntoIterator<Item = T>,
    T: AsRef<str>,
{
    let args_vec: Vec<String> = args.into_iter().map(|s| s.as_ref().to_owned()).collect();
    if args_vec.len() < 2 {
        return Err(
            "使用方法: MyBunka <小数点数> [オプション]\n例) MyBunka 0.142857  ->  1/7".to_string(),
        );
    }

    let val_str = &args_vec[1];
    if val_str == "--help" || val_str == "-h" {
        return Ok(CliOptions {
            input_str: String::new(),
            max_den: 100_000,
            tolerance: 1e-6,
            is_help: true,
            is_version: false,
        });
    }
    if val_str == "--version" || val_str == "-v" || val_str == "-V" {
        return Ok(CliOptions {
            input_str: String::new(),
            max_den: 100_000,
            tolerance: 1e-6,
            is_help: false,
            is_version: true,
        });
    }

    let mut max_den = 100_000u64;
    let mut tolerance = 1e-6f64;

    let mut i = 2;
    while i < args_vec.len() {
        match args_vec[i].as_str() {
            "--max-den" | "-d" => {
                if i + 1 < args_vec.len() {
                    max_den = match args_vec[i + 1].parse() {
                        Ok(n) if n > 0 => n,
                        _ => {
                            return Err(format!(
                                "エラー: 無効な最大分母指定です '{}'",
                                args_vec[i + 1]
                            ));
                        }
                    };
                    i += 2;
                } else {
                    return Err(format!(
                        "エラー: オプション '{}' に値が指定されていません",
                        args_vec[i]
                    ));
                }
            }
            "--tolerance" | "-t" => {
                if i + 1 < args_vec.len() {
                    tolerance = match args_vec[i + 1].parse() {
                        Ok(n) if n > 0.0 => n,
                        _ => {
                            return Err(format!(
                                "エラー: 無効な許容誤差指定です '{}'",
                                args_vec[i + 1]
                            ));
                        }
                    };
                    i += 2;
                } else {
                    return Err(format!(
                        "エラー: オプション '{}' に値が指定されていません",
                        args_vec[i]
                    ));
                }
            }
            _ => {
                return Err(format!("エラー: 未知のオプションです '{}'", args_vec[i]));
            }
        }
    }

    Ok(CliOptions {
        input_str: val_str.to_string(),
        max_den,
        tolerance,
        is_help: false,
        is_version: false,
    })
}

/// CLI版のエントリーポイント
pub fn run_cli() {
    let args: Vec<String> = env::args().collect();
    let opts = match parse_cli_args(args) {
        Ok(opts) => opts,
        Err(err_msg) => {
            eprintln!("{}", err_msg);
            process::exit(1);
        }
    };

    if opts.is_help {
        print_help();
        process::exit(0);
    }
    if opts.is_version {
        println!("MyBunka {}", env!("CARGO_PKG_VERSION"));
        process::exit(0);
    }

    let val: f64 = match crate::parse_decimal_or_percent(&opts.input_str) {
        Ok(n) => n,
        Err(_) => {
            eprintln!(
                "エラー: '{}' は無効な数値またはパーセント表記です",
                opts.input_str
            );
            process::exit(1);
        }
    };

    let (num, den, _) = approximate_fraction(val, opts.max_den, opts.tolerance);
    println!("{}/{}", num, den);
}

fn print_help() {
    println!("MyBunka - 小数点数から分数への近似変換ツール");
    println!();
    println!("使用方法:");
    println!("    MyBunka <小数点数> [オプション]");
    println!();
    println!("引数:");
    println!("    <小数点数>        分数に近似変換したい浮動小数点数");
    println!();
    println!("オプション:");
    println!("    -d, --max-den <値>  近似計算に使用する最大分母 (デフォルト: 100,000)");
    println!("    -t, --tolerance <値> 近似計算の許容誤差 (デフォルト: 1e-6)");
    println!("    -h, --help        このヘルプメッセージを表示して終了します");
    println!("    -v, -V, --version バージョン情報を表示して終了します");
    println!();
    println!("使用例:");
    println!("    MyBunka 0.142857   -> 1/7");
    println!("    MyBunka 3.14159265 -d 1000 -> 355/113");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_cli_args_basic() {
        let args = vec!["MyBunka", "0.142857"];
        let opts = parse_cli_args(args).unwrap();
        assert_eq!(opts.input_str, "0.142857");
        assert_eq!(opts.max_den, 100_000);
        assert_eq!(opts.tolerance, 1e-6);
        assert!(!opts.is_help);
        assert!(!opts.is_version);
    }

    #[test]
    fn test_parse_cli_args_options() {
        let args = vec!["MyBunka", "0.5", "-d", "1000", "-t", "1e-4"];
        let opts = parse_cli_args(args).unwrap();
        assert_eq!(opts.input_str, "0.5");
        assert_eq!(opts.max_den, 1000);
        assert_eq!(opts.tolerance, 1e-4);
    }

    #[test]
    fn test_parse_cli_args_help_version() {
        let opts = parse_cli_args(vec!["MyBunka", "-h"]).unwrap();
        assert!(opts.is_help);

        let opts = parse_cli_args(vec!["MyBunka", "--version"]).unwrap();
        assert!(opts.is_version);
    }

    #[test]
    fn test_parse_cli_args_invalid() {
        assert!(parse_cli_args(vec!["MyBunka"]).is_err());
        assert!(parse_cli_args(vec!["MyBunka", "0.5", "-d"]).is_err());
        assert!(parse_cli_args(vec!["MyBunka", "0.5", "--unknown"]).is_err());
    }
}
