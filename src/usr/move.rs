use crate::api::console::Style;
use crate::api::process::ExitCode;
use crate::usr;

pub fn main(args: &[&str]) -> Result<(), ExitCode> {
    let n = args.len();
    if n != 3 {
        help();
        return Err(ExitCode::UsageError);
    }
    for i in args.iter().take(n).skip(1) {
        match *i {
            "-h" | "--help" => {
                help();
                return Ok(());
            }
            _ => {
                crate::api::hfs::check_hfs_bounds(i)?;
                continue;
            }
        }
    }

    // TODO: Avoid doing copy+delete
    if usr::copy::main(args).is_ok() {
        usr::delete::main(&args[0..2])
    } else {
        Err(ExitCode::Failure)
    }
}

fn help() {
    let csi_option = Style::color("aqua");
    let csi_title = Style::color("yellow");
    let csi_reset = Style::reset();
    println!(
        "{}Usage:{} move {}<src> <dst>{}",
        csi_title, csi_reset, csi_option, csi_reset
    );
}
