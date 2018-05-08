use nom;
use expr;

fn valid_id_char(i: char) -> bool {
    i.is_alphanumeric() || i == '-' || i == '.'
}

named! {
    id<&str, expr::Id>,
    do_parse!(
        id: take_while!(valid_id_char) >>
        (expr::Id::new(id)))
}

named! {
    license_id<&str, expr::Simple>,
    do_parse!(
        id: id >>
        or_later: opt!(char!('+')) >>
        (expr::Simple::LicenseId {
            id,
            or_later: or_later.is_some()
        }))
}

named! {
    license_ref<&str, expr::Simple>,
    do_parse!(
        document: opt!(do_parse!(
            tag!("DocumentRef-") >>
            id: id >>
            tag!(":") >>
            (id))) >>
        tag!("LicenseRef-") >>
        id: id >>
        (expr::Simple::LicenseRef { id, document }))
}

named! {
    simple<&str, expr::Simple>,
    alt!(license_id | license_ref)
}

named! {
    with<&str, expr::Compound>,
    ws!(do_parse!(
        license: simple >>
        tag!("WITH") >>
        exception: id >>
        (expr::Compound::With { license, exception })))
}

named! {
    and<&str, expr::Compound>,
    ws!(do_parse!(
        left: compound >>
        tag!("AND") >>
        right: compound >>
        (expr::Compound::And { left: Box::new(left), right: Box::new(right) })))
}

named! {
    or<&str, expr::Compound>,
    ws!(do_parse!(
        left: compound >>
        tag!("OR") >>
        right: compound >>
        (expr::Compound::Or { left: Box::new(left), right: Box::new(right) })))
}

named! {
    compound<&str, expr::Compound>,
    alt!(
        with | and | or
        | map!(simple, |license| expr::Compound::Simple { license })
        | delimited!(tag!("("), compound, tag!(")")))
}
