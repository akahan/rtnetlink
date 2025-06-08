// SPDX-License-Identifier: MIT

use netlink_packet_route::rule::RuleMessage;
use netlink_packet_route::{route, rule, AddressFamily};
use rtnetlink::{new_connection, Error, Handle};

#[tokio::main]
async fn main() -> Result<(), ()> {
    let (connection, handle, _) = new_connection().unwrap();
    tokio::spawn(connection);

    if let Err(e) = del_rule(handle).await {
        eprintln!("{e}");
    } else {
        println!("Route rule has been deleted")
    }
    Ok(())
}

async fn del_rule(handle: Handle) -> Result<(), Error> {
    let mut message = RuleMessage::default();
    message.header = rule::RuleHeader {
        family: AddressFamily::Inet,
        action: rule::RuleAction::ToTable,
        table: route::RouteHeader::RT_TABLE_MAIN,
        ..Default::default()
    };
    message.attributes = vec![
        rule::RuleAttribute::FwMark(0xab),
        rule::RuleAttribute::Table(route::RouteHeader::RT_TABLE_MAIN as u32),
        rule::RuleAttribute::Priority(78),
    ];

    let rule = handle.rule();
    rule.del(message).execute().await?;

    Ok(())
}

fn usage() {
    eprintln!(
        "\
usage: 
    cargo run --example add_rule -- <destination>/<prefix_length> <table_id> 

Note that you need to run this program as root:

    env CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUNNER='sudo -E' \\
        cargo run --example add_rule -- <destination>/<prefix_length> \
        <table_id>"
    );
}
