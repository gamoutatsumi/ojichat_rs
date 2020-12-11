use clap::*;
mod generator;
mod pattern;

fn main() {
    let app = App::new(crate_name!())
        .version(crate_version!())
        .author(crate_authors!())
        .about(crate_description!())
        .arg(Arg::from_usage("-e [number] '絵文字/顔文字の最大連続数 [default: 4]'"))
        .arg(Arg::from_usage("-p [level] '句読点挿入頻度レベル [min:0, max:3] [default: 0]'"))
        .arg(Arg::from_usage("[name] 'おじさんのメール相手'"))
        .setting(AppSettings::ColoredHelp);

    let matches = app.get_matches();

    if let Some(o) = matches.value_of("p") {
        println!("Value for level: {}", o)
    };

    if let Some(o) = matches.value_of("name") {
        println!("Value for name: {}", o)
    };

}
