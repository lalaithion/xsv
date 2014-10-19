use std::io;

use docopt;
use tabwriter::TabWriter;

use CliResult;
use config::Delimiter;
use util;

docopt!(Args, "
Prints the fields of the first row in the CSV data.

These names can be used in commands like 'select' to refer to columns in the
CSV data.

Usage:
    xsv headers [options] [<input>...]

headers options:
    -j, --just-names       Only show the header names (hide column index).
                           This is automatically enabled if more than one
                           input is given.
    --intersect            Shows the intersection of all headers in all of
                           the inputs given.

Common options:
    -h, --help             Display this message
    -d, --delimiter <arg>  The field delimiter for reading CSV data.
                           Must be a single character. [default: ,]
", arg_input: Vec<String>, flag_delimiter: Delimiter)

pub fn main(argv: &[&str]) -> CliResult<()> {
    let args: Args = try!(util::get_args(argv));
    let configs = try!(str| util::many_configs(args.arg_input.as_slice(),
                                               args.flag_delimiter, true));

    let num_inputs = configs.len();
    let mut headers = vec!();
    for conf in configs.into_iter() {
        let mut rdr = try!(io| conf.reader());
        for header in try!(csv| rdr.byte_headers()).into_iter() {
            if !args.flag_intersect || !headers.contains(&header) {
                headers.push(header);
            }
        }
    }

    let mut wtr: Box<Writer> =
        if args.flag_just_names {
            box io::stdout() as Box<Writer>
        } else {
            box TabWriter::new(io::stdout()) as Box<Writer>
        };
    for (i, header) in headers.into_iter().enumerate() {
        if num_inputs == 1 && !args.flag_just_names {
            try!(io| wtr.write_str((i + 1).to_string().as_slice()));
            try!(io| wtr.write_u8(b'\t'));
        }
        try!(io| wtr.write(header.as_slice()));
        try!(io| wtr.write_u8(b'\n'));
    }
    try!(io| wtr.flush());
    Ok(())
}