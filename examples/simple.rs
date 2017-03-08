extern crate macos_notifications;
use macos_notifications::*;

fn main() {
    let bundle = util::get_bundle_identifier("firefox");
    util::set_application(&bundle);
    let _sent = send_notification("Danger",
                                  Some("Will Robinson"),
                                  "Run away as fast as you can",
                                  Some("Blow"));
    let _ = send_notification("NOW", None, "Without subtitle", Some("Submarine"));

}